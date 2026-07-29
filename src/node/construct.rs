// SPDX-License-Identifier: CC0-1.0

use crate::dag::{InternalSharing, PostOrderIterItem};
use crate::jet::{Jet, JetEnvironment};
use crate::types::{self, Arrow};
use crate::{encode, BitIter, BitWriter, Cmr, FailEntropy, FinalizeError, RedeemNode, Value, Word};

use std::io;
use std::marker::PhantomData;
use std::sync::Arc;

use super::{
    Commit, CommitData, CommitNode, Converter, Inner, Marker, NoDisconnect, NoWitness, Node,
    Redeem, RedeemData,
};
use super::{CoreConstructible, DisconnectConstructible, WitnessConstructible};

/// ID used to share [`ConstructNode`]s.
///
/// This is impossible to construct, which is a promise that it is impossible
/// to share [`ConstructNode`]s.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum ConstructId {}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Construct<'brand> {
    /// Makes the type non-constructible.
    never: std::convert::Infallible,
    /// Required by Rust.
    phantom: std::marker::PhantomData<&'brand ()>,
}

impl<'brand> Marker for Construct<'brand> {
    type CachedData = ConstructData<'brand>;
    type Witness = Option<Value>;
    type Disconnect = Option<Arc<ConstructNode<'brand>>>;
    type SharingId = ConstructId;

    fn compute_sharing_id(_: Cmr, _: &ConstructData<'brand>) -> Option<ConstructId> {
        None
    }
}

pub type ConstructNode<'brand> = Node<Construct<'brand>>;

impl<'brand> ConstructNode<'brand> {
    /// Accessor for the node's arrow
    pub fn arrow(&self) -> &Arrow<'brand> {
        self.data.arrow()
    }

    /// Sets the source and target type of the node to unit
    pub fn set_arrow_to_program(&self) -> Result<(), types::Error> {
        let ctx = self.data.inference_context();
        let unit_ty = types::Type::unit(ctx);
        ctx.unify(
            &self.arrow().source,
            &unit_ty,
            "setting root source to unit",
        )?;
        ctx.unify(
            &self.arrow().target,
            &unit_ty,
            "setting root target to unit",
        )?;
        Ok(())
    }

    /// Convert a [`ConstructNode`] to a [`CommitNode`] by finalizing all of the types.
    ///
    /// Also sets the source and target type of this node to unit. This is almost
    /// certainly what you want, since the resulting `CommitNode` cannot be further
    /// composed, and needs to be 1->1 to go on-chain. But if you don't, call
    /// [`Self::finalize_types_non_program`] instead.
    pub fn finalize_types(&self) -> Result<Arc<CommitNode>, types::Error> {
        self.set_arrow_to_program()?;
        self.finalize_types_non_program()
    }

    /// Convert a [`ConstructNode`] to a [`CommitNode`] by finalizing all of the types.
    ///
    /// Does *not* sets the source and target type of this node to unit.
    pub fn finalize_types_non_program(&self) -> Result<Arc<CommitNode>, types::Error> {
        struct FinalizeTypes;

        impl<'brand> Converter<Construct<'brand>, Commit> for FinalizeTypes {
            type Error = types::Error;

            fn convert_witness(
                &mut self,
                _: &PostOrderIterItem<&ConstructNode>,
                _: &Option<Value>,
            ) -> Result<NoWitness, Self::Error> {
                Ok(NoWitness)
            }

            fn convert_disconnect(
                &mut self,
                _: &PostOrderIterItem<&ConstructNode>,
                _: Option<&Arc<CommitNode>>,
                _: &Option<Arc<ConstructNode>>,
            ) -> Result<NoDisconnect, Self::Error> {
                Ok(NoDisconnect)
            }

            fn convert_data(
                &mut self,
                data: &PostOrderIterItem<&ConstructNode>,
                inner: Inner<&Arc<CommitNode>, &NoDisconnect, &NoWitness>,
            ) -> Result<Arc<CommitData>, Self::Error> {
                let converted_data = inner.map(|node| node.cached_data());
                CommitData::new(&data.node.data.arrow, converted_data).map(Arc::new)
            }
        }

        self.convert::<InternalSharing, _, _>(&mut FinalizeTypes)
    }

    /// Finalize the witness program as an unpruned redeem program.
    ///
    /// Witness nodes must be populated with sufficient data,
    /// to ensure that the resulting redeem program successfully runs on the Bit Machine.
    /// Furthermore, **all** disconnected branches must be populated,
    /// even those that are not executed.
    ///
    /// The resulting redeem program is **not pruned**.
    ///
    /// ## See
    ///
    /// [`RedeemNode::prune`]
    pub fn finalize_unpruned(&self) -> Result<Arc<RedeemNode>, FinalizeError> {
        struct Finalizer(PhantomData<()>);

        impl<'brand> Converter<Construct<'brand>, Redeem> for Finalizer {
            type Error = FinalizeError;

            fn convert_witness(
                &mut self,
                data: &PostOrderIterItem<&ConstructNode>,
                wit: &Option<Value>,
            ) -> Result<Value, Self::Error> {
                if let Some(ref wit) = wit {
                    Ok(wit.shallow_clone())
                } else {
                    // We insert a zero value into unpopulated witness nodes,
                    // assuming that this node will later be pruned out of the program.
                    //
                    // Pruning requires running a program on the Bit Machine,
                    // which in turn requires a program with fully populated witness nodes.
                    // It would be horrible UX to force the caller to provide witness data
                    // even for unexecuted branches, so we insert zero values here.
                    //
                    // If this node is executed after all, then the caller can fix the witness
                    // data based on the returned execution error.
                    //
                    // Zero values may "accidentally" satisfy a program even if the caller
                    // didn't provide any witness data. However, this is only the case for the
                    // most trivial programs. The only place where we must be careful is our
                    // unit tests, which tend to include these kinds of trivial programs.
                    let ty = data
                        .node
                        .arrow()
                        .target
                        .finalize()
                        .map_err(FinalizeError::Type)?;
                    Ok(Value::zero(&ty))
                }
            }

            fn convert_disconnect(
                &mut self,
                _: &PostOrderIterItem<&ConstructNode>,
                maybe_converted: Option<&Arc<RedeemNode>>,
                _: &Option<Arc<ConstructNode>>,
            ) -> Result<Arc<RedeemNode>, Self::Error> {
                if let Some(child) = maybe_converted {
                    Ok(Arc::clone(child))
                } else {
                    Err(FinalizeError::DisconnectRedeemTime)
                }
            }

            fn convert_data(
                &mut self,
                data: &PostOrderIterItem<&ConstructNode>,
                inner: Inner<&Arc<RedeemNode>, &Arc<RedeemNode>, &Value>,
            ) -> Result<Arc<RedeemData>, Self::Error> {
                let converted_data = inner
                    .map(|node| node.cached_data())
                    .map_disconnect(|node| node.cached_data())
                    .map_witness(Value::shallow_clone);
                Ok(Arc::new(RedeemData::new(
                    data.node.arrow().finalize().map_err(FinalizeError::Type)?,
                    converted_data,
                )))
            }
        }

        self.convert::<InternalSharing, _, _>(&mut Finalizer(PhantomData))
    }

    /// Finalize the witness program as a pruned redeem program.
    ///
    /// Witness nodes must be populated with sufficient data,
    /// to ensure that the resulting redeem program successfully runs on the Bit Machine.
    /// Furthermore, **all** disconnected branches must be populated,
    /// even those that are not executed.
    ///
    /// The resulting redeem program is **pruned** based on the given transaction environment.
    ///
    /// ## See
    ///
    /// [`RedeemNode::prune`]
    pub fn finalize_pruned<JE: JetEnvironment>(
        &self,
        env: &JE,
    ) -> Result<Arc<RedeemNode>, FinalizeError> {
        let unpruned = self.finalize_unpruned()?;
        unpruned.prune(env).map_err(FinalizeError::Execution)
    }

    /// Decode a Simplicity expression from bits, without witness data.
    ///
    /// # Usage
    ///
    /// Use this method only if the serialization **does not** include the witness data.
    /// This means, the program simply has no witness during commitment,
    /// or the witness is provided by other means.
    ///
    /// If the serialization contains the witness data, then use [`crate::RedeemNode::decode()`].
    pub fn decode<I: Iterator<Item = u8>, J: Jet>(
        context: &types::Context<'brand>,
        mut bits: BitIter<I>,
    ) -> Result<Arc<Self>, crate::decode::Error> {
        let res = crate::decode::decode_expression::<I, J>(context, &mut bits)?;
        bits.close()?;
        Ok(res)
    }

    #[cfg(feature = "base64")]
    #[allow(clippy::should_implement_trait)] // returns Arc<Self>, needs tyctx
    pub fn from_str<J: Jet>(
        context: &types::Context<'brand>,
        s: &str,
    ) -> Result<Arc<Self>, crate::ParseError> {
        use crate::base64::engine::general_purpose;
        use crate::base64::Engine as _;

        let v = general_purpose::STANDARD
            .decode(s)
            .map_err(crate::ParseError::Base64)?;
        let iter = crate::BitIter::new(v.into_iter());
        Self::decode::<_, J>(context, iter)
            .map_err(crate::DecodeError::Decode)
            .map_err(crate::ParseError::Decode)
    }

    /// Encode a Simplicity expression to bits, with no witness data
    #[deprecated(since = "0.5.0", note = "use Self::encode_without_witness instead")]
    pub fn encode(&self, w: &mut BitWriter<&mut dyn io::Write>) -> io::Result<usize> {
        let program_bits = encode::encode_program(self, w)?;
        w.flush_all()?;
        Ok(program_bits)
    }
}

#[derive(Clone, Debug)]
pub struct ConstructData<'brand> {
    arrow: Arrow<'brand>,
}

impl<'brand> ConstructData<'brand> {
    /// Constructs a new [`ConstructData`] from an (unfinalized) type arrow
    pub fn new(arrow: Arrow<'brand>) -> Self {
        ConstructData { arrow }
    }

    /// Accessor for the node's arrow
    pub fn arrow(&self) -> &Arrow<'brand> {
        &self.arrow
    }
}

impl<'brand> CoreConstructible<'brand> for ConstructData<'brand> {
    fn comp_from_arrow(_: &Self, _: &Self, arrow: types::Arrow<'brand>) -> Self {
        ConstructData { arrow }
    }

    fn pair_from_arrow(_: &Self, _: &Self, arrow: types::Arrow<'brand>) -> Self {
        ConstructData { arrow }
    }

    fn iden(inference_context: &types::Context<'brand>) -> Self {
        ConstructData {
            arrow: Arrow::iden(inference_context),
        }
    }

    fn unit_from_arrow(arrow: types::Arrow<'brand>) -> Self {
        ConstructData { arrow }
    }

    fn injl(child: &Self) -> Self {
        ConstructData {
            arrow: Arrow::injl(&child.arrow),
        }
    }

    fn injr(child: &Self) -> Self {
        ConstructData {
            arrow: Arrow::injr(&child.arrow),
        }
    }

    fn take(child: &Self) -> Self {
        ConstructData {
            arrow: Arrow::take(&child.arrow),
        }
    }

    fn drop_(child: &Self) -> Self {
        ConstructData {
            arrow: Arrow::drop_(&child.arrow),
        }
    }

    fn case(left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(ConstructData {
            arrow: Arrow::case(&left.arrow, &right.arrow)?,
        })
    }

    fn assertl(left: &Self, _: Cmr) -> Result<Self, types::Error> {
        Ok(ConstructData {
            arrow: Arrow::assertl(&left.arrow)?,
        })
    }

    fn assertr(_: Cmr, right: &Self) -> Result<Self, types::Error> {
        Ok(ConstructData {
            arrow: Arrow::assertr(&right.arrow)?,
        })
    }

    fn fail(inference_context: &types::Context<'brand>, _: FailEntropy) -> Self {
        ConstructData {
            arrow: Arrow::fail(inference_context),
        }
    }

    fn const_word(inference_context: &types::Context<'brand>, word: Word) -> Self {
        ConstructData {
            arrow: Arrow::const_word(inference_context, &word),
        }
    }

    fn jet(inference_context: &types::Context<'brand>, jet: &dyn Jet) -> Self {
        ConstructData {
            arrow: Arrow::jet(inference_context, jet),
        }
    }

    fn arrow(&self) -> &Arrow<'brand> {
        &self.arrow
    }
}

impl<'brand> DisconnectConstructible<'brand, Option<Arc<ConstructNode<'brand>>>>
    for ConstructData<'brand>
{
    fn disconnect(
        left: &Self,
        right: &Option<Arc<ConstructNode<'brand>>>,
    ) -> Result<Self, types::Error> {
        let right = right.as_ref();
        Ok(ConstructData {
            arrow: Arrow::disconnect(&left.arrow, &right.map(|n| n.arrow()))?,
        })
    }
}

impl<'brand> WitnessConstructible<'brand, Option<Value>> for ConstructData<'brand> {
    fn witness(inference_context: &types::Context<'brand>, _witness: Option<Value>) -> Self {
        ConstructData {
            arrow: Arrow::witness(inference_context),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Final;
    use crate::Value;

    #[test]
    fn occurs_check_error() {
        types::Context::with_context(|ctx| {
            let iden = Arc::<ConstructNode>::iden(&ctx);
            let node = Arc::<ConstructNode>::disconnect(&iden, &Some(Arc::clone(&iden))).unwrap();

            assert!(matches!(
                node.finalize_types_non_program(),
                Err(types::Error::OccursCheck { .. }),
            ));
        });
    }

    #[test]
    fn occurs_check_2() {
        types::Context::with_context(|ctx| {
            // A more complicated occurs-check test that caused a deadlock in the past.
            let iden = Arc::<ConstructNode>::iden(&ctx);
            let injr = Arc::<ConstructNode>::injr(&iden);
            let pair = Arc::<ConstructNode>::pair(&injr, &iden).unwrap();
            let drop = Arc::<ConstructNode>::drop_(&pair);

            let case1 = Arc::<ConstructNode>::case(&drop, &drop).unwrap();
            let case2 = Arc::<ConstructNode>::case(&case1, &case1).unwrap();

            let comp1 = Arc::<ConstructNode>::comp(&case2, &case2).unwrap();
            let comp2 = Arc::<ConstructNode>::comp(&comp1, &case1).unwrap();

            assert!(matches!(
                comp2.finalize_types_non_program(),
                Err(types::Error::OccursCheck { .. }),
            ));
        });
    }

    #[test]
    fn occurs_check_3() {
        types::Context::with_context(|ctx| {
            // A similar example that caused a slightly different deadlock in the past.
            let wit = Arc::<ConstructNode>::witness(&ctx, None);
            let drop = Arc::<ConstructNode>::drop_(&wit);

            let comp1 = Arc::<ConstructNode>::comp(&drop, &drop).unwrap();
            let comp2 = Arc::<ConstructNode>::comp(&comp1, &comp1).unwrap();
            let comp3 = Arc::<ConstructNode>::comp(&comp2, &comp2).unwrap();
            let comp4 = Arc::<ConstructNode>::comp(&comp3, &comp3).unwrap();
            let comp5 = Arc::<ConstructNode>::comp(&comp4, &comp4).unwrap();

            let case = Arc::<ConstructNode>::case(&comp5, &comp4).unwrap();
            let drop2 = Arc::<ConstructNode>::drop_(&case);
            let case2 = Arc::<ConstructNode>::case(&drop2, &case).unwrap();
            let comp6 = Arc::<ConstructNode>::comp(&case2, &case2).unwrap();
            let case3 = Arc::<ConstructNode>::case(&comp6, &comp6).unwrap();

            let comp7 = Arc::<ConstructNode>::comp(&case3, &case3).unwrap();
            let comp8 = Arc::<ConstructNode>::comp(&comp7, &comp7).unwrap();

            assert!(matches!(
                comp8.finalize_types_non_program(),
                Err(types::Error::OccursCheck { .. }),
            ));
        });
    }

    #[test]
    fn type_check_error() {
        types::Context::with_context(|ctx| {
            let unit = Arc::<ConstructNode>::unit(&ctx);
            let case = Arc::<ConstructNode>::case(&unit, &unit).unwrap();

            assert!(matches!(
                Arc::<ConstructNode>::disconnect(&case, &Some(unit)),
                Err(types::Error::Bind { .. }),
            ));
        });
    }

    #[test]
    fn scribe() {
        // Ok to use same type inference context for all the below tests,
        // since everything has concrete types and anyway we only care
        // about CMRs, for which type inference is irrelevant.
        types::Context::with_context(|ctx| {
            let unit = Arc::<ConstructNode>::unit(&ctx);
            let bit0 = Arc::<ConstructNode>::const_word(&ctx, Word::u1(0));
            let bit1 = Arc::<ConstructNode>::const_word(&ctx, Word::u1(1));

            assert_eq!(
                unit.cmr(),
                Arc::<ConstructNode>::scribe(&ctx, &Value::unit()).cmr()
            );
            assert_eq!(
                bit0.cmr(),
                Arc::<ConstructNode>::scribe(&ctx, &Value::u1(0)).cmr()
            );
            assert_eq!(
                bit1.cmr(),
                Arc::<ConstructNode>::scribe(&ctx, &Value::u1(1)).cmr()
            );
            assert_eq!(
                Arc::<ConstructNode>::const_word(&ctx, Word::u2(1)).cmr(),
                Arc::<ConstructNode>::scribe(&ctx, &Value::u2(1)).cmr()
            );
            assert_eq!(
                Arc::<ConstructNode>::injl(&bit0).cmr(),
                Arc::<ConstructNode>::scribe(&ctx, &Value::left(Value::u1(0), Final::unit())).cmr()
            );
            assert_eq!(
                Arc::<ConstructNode>::injr(&bit1).cmr(),
                Arc::<ConstructNode>::scribe(&ctx, &Value::right(Final::unit(), Value::u1(1)))
                    .cmr()
            );
            assert_eq!(
                Arc::<ConstructNode>::pair(&unit, &unit).unwrap().cmr(),
                Arc::<ConstructNode>::scribe(&ctx, &Value::product(Value::unit(), Value::unit()))
                    .cmr()
            );
        });
    }

    #[test]
    fn regression_286_1() {
        // This is the smallest pure Simplicity program I was able to find that exhibits the bad
        // behavior seen in https://github.com/BlockstreamResearch/rust-simplicity/issues/286
        types::Context::with_context(|ctx| {
            let cmr = Cmr::from_byte_array([0xde; 32]);

            let u0 = Arc::<ConstructNode>::unit(&ctx);
            let i1 = Arc::<ConstructNode>::injl(&u0);
            let i2 = Arc::<ConstructNode>::injr(&i1);
            let i3 = Arc::<ConstructNode>::injr(&i2);
            let i4 = Arc::<ConstructNode>::injl(&i3);
            let u5 = Arc::<ConstructNode>::unit(&ctx);
            let i6 = Arc::<ConstructNode>::injl(&u5);
            let i7 = Arc::<ConstructNode>::injr(&i6);
            let p8 = Arc::<ConstructNode>::pair(&i4, &i7).unwrap();
            let u9 = Arc::<ConstructNode>::unit(&ctx);
            let a10 = Arc::<ConstructNode>::assertr(cmr, &u9).unwrap();
            let u11 = Arc::<ConstructNode>::unit(&ctx);
            let a12 = Arc::<ConstructNode>::assertr(cmr, &u11).unwrap();
            let a13 = Arc::<ConstructNode>::assertl(&a12, cmr).unwrap();
            let c14 = Arc::<ConstructNode>::case(&a10, &a13).unwrap();
            let c15 = Arc::<ConstructNode>::comp(&p8, &c14).unwrap();

            let finalized: Arc<CommitNode> = c15.finalize_types().unwrap();
            let prog = finalized.to_vec_without_witness();
            // In #286 we are encoding correctly...
            assert_eq!(
                hex::DisplayHex::as_hex(&prog[..]).to_string(),
                "dc920a28812b6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f243090e00b10e00680",
            );

            let prog = BitIter::from(prog);
            let decode = CommitNode::decode::<_, crate::jet::Core>(prog).unwrap();

            // ...but then during decoding we read the program incorrectly and this assertion fails.
            assert_eq!(finalized, decode);
        });
    }

    #[test]
    fn regression_286_2() {
        // This one is smaller because it starts with a witness node which has a large type.
        // This is a bit easier to grok but can't be serialized as a complete/valid program
        // without providing the witness data, which limits its ability to share with the
        // other libraries.
        //
        // It also exhibits the bug earlier than the other one -- it *should* just fail to
        // typecheck and not be constructible. So we can't get an encoding of it.
        types::Context::with_context(|ctx| {
            let w0 = Arc::<ConstructNode>::witness(&ctx, None);
            let i1 = Arc::<ConstructNode>::iden(&ctx);
            let d2 = Arc::<ConstructNode>::drop_(&i1);
            let i3 = Arc::<ConstructNode>::iden(&ctx);
            let i4 = Arc::<ConstructNode>::iden(&ctx);
            let t5 = Arc::<ConstructNode>::take(&i4);
            let ca6 = Arc::<ConstructNode>::case(&i3, &t5).unwrap();
            let ca7 = Arc::<ConstructNode>::case(&d2, &ca6).unwrap();
            let c8 = Arc::<ConstructNode>::comp(&w0, &ca7).unwrap();
            let u9 = Arc::<ConstructNode>::unit(&ctx);
            let c10 = Arc::<ConstructNode>::comp(&c8, &u9).unwrap();

            // In #286 we incorrectly succeed finalizing the types, and then encode a bad program.
            let err = c10.finalize_types().unwrap_err();
            assert!(matches!(err, types::Error::OccursCheck { .. }));
        });
    }
}
