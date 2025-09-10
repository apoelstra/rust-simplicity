// SPDX-License-Identifier: CC0-1.0

use crate::dag::{InternalSharing, PostOrderIterItem};
use crate::jet::Jet;
use crate::types::{self, arrow::Arrow};
use crate::{encode, BitIter, BitWriter, Cmr, FailEntropy, FinalizeError, RedeemNode, Value, Word};

use core::borrow::Borrow;
use std::io;
use std::marker::PhantomData;
use std::sync::Arc;

use super::{
    Commit, CommitData, CommitNode, Converter, Inner, Marker, NoDisconnect, NoWitness, Node,
    Redeem, RedeemData,
};
use super::{CoreConstructible, DisconnectConstructible, JetConstructible, WitnessConstructible};

/// ID used to share [`ConstructNode`]s.
///
/// This is impossible to construct, which is a promise that it is impossible
/// to share [`ConstructNode`]s.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum ConstructId {}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Construct<'brand, J> {
    /// Makes the type non-constructible.
    never: std::convert::Infallible,
    /// Required by Rust.
    phantom: std::marker::PhantomData<&'brand J>,
}

impl<'brand, J: Jet> Marker for Construct<'brand, J> {
    type CachedData = ConstructData<'brand, J>;
    type Witness = Option<Value>;
    type Disconnect = Option<Arc<ConstructNode<'brand, J>>>;
    type SharingId = ConstructId;
    type Jet = J;

    fn compute_sharing_id(_: Cmr, _: &ConstructData<J>) -> Option<ConstructId> {
        None
    }
}

pub type ConstructNode<'brand, J> = Node<Construct<'brand, J>>;

impl<'brand, J: Jet> ConstructNode<'brand, J> {
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
    pub fn finalize_types(&self) -> Result<Arc<CommitNode<J>>, types::Error> {
        self.set_arrow_to_program()?;
        self.finalize_types_non_program()
    }

    /// Convert a [`ConstructNode`] to a [`CommitNode`] by finalizing all of the types.
    ///
    /// Does *not* sets the source and target type of this node to unit.
    pub fn finalize_types_non_program(&self) -> Result<Arc<CommitNode<J>>, types::Error> {
        struct FinalizeTypes<J: Jet>(PhantomData<J>);

        impl<'brand, J: Jet> Converter<Construct<'brand, J>, Commit<J>> for FinalizeTypes<J> {
            type Error = types::Error;

            fn convert_witness(
                &mut self,
                _: &PostOrderIterItem<&ConstructNode<J>>,
                _: &Option<Value>,
            ) -> Result<NoWitness, Self::Error> {
                Ok(NoWitness)
            }

            fn convert_disconnect(
                &mut self,
                _: &PostOrderIterItem<&ConstructNode<J>>,
                _: Option<&Arc<CommitNode<J>>>,
                _: &Option<Arc<ConstructNode<J>>>,
            ) -> Result<NoDisconnect, Self::Error> {
                Ok(NoDisconnect)
            }

            fn convert_data(
                &mut self,
                data: &PostOrderIterItem<&ConstructNode<J>>,
                inner: Inner<&Arc<CommitNode<J>>, J, &NoDisconnect, &NoWitness>,
            ) -> Result<Arc<CommitData<J>>, Self::Error> {
                let converted_data = inner.map(|node| node.cached_data());
                CommitData::new(&data.node.data.arrow, converted_data).map(Arc::new)
            }
        }

        self.convert::<InternalSharing, _, _>(&mut FinalizeTypes(PhantomData))
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
    pub fn finalize_unpruned(&self) -> Result<Arc<RedeemNode<J>>, FinalizeError> {
        struct Finalizer<J>(PhantomData<J>);

        impl<'brand, J: Jet> Converter<Construct<'brand, J>, Redeem<J>> for Finalizer<J> {
            type Error = FinalizeError;

            fn convert_witness(
                &mut self,
                data: &PostOrderIterItem<&ConstructNode<J>>,
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
                _: &PostOrderIterItem<&ConstructNode<J>>,
                maybe_converted: Option<&Arc<RedeemNode<J>>>,
                _: &Option<Arc<ConstructNode<J>>>,
            ) -> Result<Arc<RedeemNode<J>>, Self::Error> {
                if let Some(child) = maybe_converted {
                    Ok(Arc::clone(child))
                } else {
                    Err(FinalizeError::DisconnectRedeemTime)
                }
            }

            fn convert_data(
                &mut self,
                data: &PostOrderIterItem<&ConstructNode<J>>,
                inner: Inner<&Arc<RedeemNode<J>>, J, &Arc<RedeemNode<J>>, &Value>,
            ) -> Result<Arc<RedeemData<J>>, Self::Error> {
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
    pub fn finalize_pruned<T: Borrow<J::Transaction>>(
        &self,
        env: &J::Environment<T>,
    ) -> Result<Arc<RedeemNode<J>>, FinalizeError> {
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
    pub fn decode<I: Iterator<Item = u8>>(
        context: &types::Context<'brand>,
        mut bits: BitIter<I>,
    ) -> Result<Arc<Self>, crate::decode::Error> {
        let res = crate::decode::decode_expression(context, &mut bits)?;
        bits.close()?;
        Ok(res)
    }

    #[cfg(feature = "base64")]
    #[allow(clippy::should_implement_trait)] // returns Arc<Self>, needs tyctx
    pub fn from_str(
        context: &types::Context<'brand>,
        s: &str,
    ) -> Result<Arc<Self>, crate::ParseError> {
        use crate::base64::engine::general_purpose;
        use crate::base64::Engine as _;

        let v = general_purpose::STANDARD
            .decode(s)
            .map_err(crate::ParseError::Base64)?;
        let iter = crate::BitIter::new(v.into_iter());
        Self::decode(context, iter)
            .map_err(crate::DecodeError::Decode)
            .map_err(crate::ParseError::Decode)
    }

    /// Encode a Simplicity expression to bits, with no witness data
    #[deprecated(since = "0.5.0", note = "use Self::encode_without_witness instead")]
    pub fn encode<W: io::Write>(&self, w: &mut BitWriter<W>) -> io::Result<usize> {
        let program_bits = encode::encode_program(self, w)?;
        w.flush_all()?;
        Ok(program_bits)
    }
}

#[derive(Clone, Debug)]
pub struct ConstructData<'brand, J> {
    arrow: Arrow<'brand>,
    /// This isn't really necessary, but it helps type inference if every
    /// struct has a \<J\> parameter, since it forces the choice of jets to
    /// be consistent without the user needing to specify it too many times.
    phantom: PhantomData<J>,
}

impl<'brand, J: Jet> ConstructData<'brand, J> {
    /// Constructs a new [`ConstructData`] from an (unfinalized) type arrow
    pub fn new(arrow: Arrow<'brand>) -> Self {
        ConstructData {
            arrow,
            phantom: PhantomData,
        }
    }

    /// Accessor for the node's arrow
    pub fn arrow(&self) -> &Arrow<'brand> {
        &self.arrow
    }
}

impl<'brand, J> CoreConstructible<'brand> for ConstructData<'brand, J> {
    fn iden(inference_context: &types::Context<'brand>) -> Self {
        ConstructData {
            arrow: Arrow::iden(inference_context),
            phantom: PhantomData,
        }
    }

    fn unit(inference_context: &types::Context<'brand>) -> Self {
        ConstructData {
            arrow: Arrow::unit(inference_context),
            phantom: PhantomData,
        }
    }

    fn injl(child: &Self) -> Self {
        ConstructData {
            arrow: Arrow::injl(&child.arrow),
            phantom: PhantomData,
        }
    }

    fn injr(child: &Self) -> Self {
        ConstructData {
            arrow: Arrow::injr(&child.arrow),
            phantom: PhantomData,
        }
    }

    fn take(child: &Self) -> Self {
        ConstructData {
            arrow: Arrow::take(&child.arrow),
            phantom: PhantomData,
        }
    }

    fn drop_(child: &Self) -> Self {
        ConstructData {
            arrow: Arrow::drop_(&child.arrow),
            phantom: PhantomData,
        }
    }

    fn comp(left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(ConstructData {
            arrow: Arrow::comp(&left.arrow, &right.arrow)?,
            phantom: PhantomData,
        })
    }

    fn case(left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(ConstructData {
            arrow: Arrow::case(&left.arrow, &right.arrow)?,
            phantom: PhantomData,
        })
    }

    fn assertl(left: &Self, right: Cmr) -> Result<Self, types::Error> {
        Ok(ConstructData {
            arrow: Arrow::assertl(&left.arrow, right)?,
            phantom: PhantomData,
        })
    }

    fn assertr(left: Cmr, right: &Self) -> Result<Self, types::Error> {
        Ok(ConstructData {
            arrow: Arrow::assertr(left, &right.arrow)?,
            phantom: PhantomData,
        })
    }

    fn pair(left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(ConstructData {
            arrow: Arrow::pair(&left.arrow, &right.arrow)?,
            phantom: PhantomData,
        })
    }

    fn fail(inference_context: &types::Context<'brand>, entropy: FailEntropy) -> Self {
        ConstructData {
            arrow: Arrow::fail(inference_context, entropy),
            phantom: PhantomData,
        }
    }

    fn const_word(inference_context: &types::Context<'brand>, word: Word) -> Self {
        ConstructData {
            arrow: Arrow::const_word(inference_context, word),
            phantom: PhantomData,
        }
    }

    fn inference_context(&self) -> &types::Context<'brand> {
        self.arrow.inference_context()
    }
}

impl<'brand, J: Jet> DisconnectConstructible<'brand, Option<Arc<ConstructNode<'brand, J>>>>
    for ConstructData<'brand, J>
{
    fn disconnect(
        left: &Self,
        right: &Option<Arc<ConstructNode<'brand, J>>>,
    ) -> Result<Self, types::Error> {
        let right = right.as_ref();
        Ok(ConstructData {
            arrow: Arrow::disconnect(&left.arrow, &right.map(|n| n.arrow()))?,
            phantom: PhantomData,
        })
    }
}

impl<'brand, J> WitnessConstructible<'brand, Option<Value>> for ConstructData<'brand, J> {
    fn witness(inference_context: &types::Context<'brand>, _witness: Option<Value>) -> Self {
        ConstructData {
            arrow: Arrow::witness(inference_context, NoWitness),
            phantom: PhantomData,
        }
    }
}

impl<'brand, J: Jet> JetConstructible<'brand, J> for ConstructData<'brand, J> {
    fn jet(inference_context: &types::Context<'brand>, jet: J) -> Self {
        ConstructData {
            arrow: Arrow::jet(inference_context, jet),
            phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jet::Core;
    use crate::types::Final;
    use crate::Value;

    #[test]
    fn occurs_check_error() {
        types::Context::with_context(|ctx| {
            let iden = Arc::<ConstructNode<Core>>::iden(&ctx);
            let node =
                Arc::<ConstructNode<Core>>::disconnect(&iden, &Some(Arc::clone(&iden))).unwrap();

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
            let iden = Arc::<ConstructNode<Core>>::iden(&ctx);
            let injr = Arc::<ConstructNode<Core>>::injr(&iden);
            let pair = Arc::<ConstructNode<Core>>::pair(&injr, &iden).unwrap();
            let drop = Arc::<ConstructNode<Core>>::drop_(&pair);

            let case1 = Arc::<ConstructNode<Core>>::case(&drop, &drop).unwrap();
            let case2 = Arc::<ConstructNode<Core>>::case(&case1, &case1).unwrap();

            let comp1 = Arc::<ConstructNode<Core>>::comp(&case2, &case2).unwrap();
            let comp2 = Arc::<ConstructNode<Core>>::comp(&comp1, &case1).unwrap();

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
            let wit = Arc::<ConstructNode<Core>>::witness(&ctx, None);
            let drop = Arc::<ConstructNode<Core>>::drop_(&wit);

            let comp1 = Arc::<ConstructNode<Core>>::comp(&drop, &drop).unwrap();
            let comp2 = Arc::<ConstructNode<Core>>::comp(&comp1, &comp1).unwrap();
            let comp3 = Arc::<ConstructNode<Core>>::comp(&comp2, &comp2).unwrap();
            let comp4 = Arc::<ConstructNode<Core>>::comp(&comp3, &comp3).unwrap();
            let comp5 = Arc::<ConstructNode<Core>>::comp(&comp4, &comp4).unwrap();

            let case = Arc::<ConstructNode<Core>>::case(&comp5, &comp4).unwrap();
            let drop2 = Arc::<ConstructNode<Core>>::drop_(&case);
            let case2 = Arc::<ConstructNode<Core>>::case(&drop2, &case).unwrap();
            let comp6 = Arc::<ConstructNode<Core>>::comp(&case2, &case2).unwrap();
            let case3 = Arc::<ConstructNode<Core>>::case(&comp6, &comp6).unwrap();

            let comp7 = Arc::<ConstructNode<Core>>::comp(&case3, &case3).unwrap();
            let comp8 = Arc::<ConstructNode<Core>>::comp(&comp7, &comp7).unwrap();

            assert!(matches!(
                comp8.finalize_types_non_program(),
                Err(types::Error::OccursCheck { .. }),
            ));
        });
    }

    #[test]
    fn type_check_error() {
        types::Context::with_context(|ctx| {
            let unit = Arc::<ConstructNode<Core>>::unit(&ctx);
            let case = Arc::<ConstructNode<Core>>::case(&unit, &unit).unwrap();

            assert!(matches!(
                Arc::<ConstructNode<Core>>::disconnect(&case, &Some(unit)),
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
            let unit = Arc::<ConstructNode<Core>>::unit(&ctx);
            let bit0 = Arc::<ConstructNode<Core>>::const_word(&ctx, Word::u1(0));
            let bit1 = Arc::<ConstructNode<Core>>::const_word(&ctx, Word::u1(1));

            assert_eq!(
                unit.cmr(),
                Arc::<ConstructNode<Core>>::scribe(&ctx, &Value::unit()).cmr()
            );
            assert_eq!(
                bit0.cmr(),
                Arc::<ConstructNode<Core>>::scribe(&ctx, &Value::u1(0)).cmr()
            );
            assert_eq!(
                bit1.cmr(),
                Arc::<ConstructNode<Core>>::scribe(&ctx, &Value::u1(1)).cmr()
            );
            assert_eq!(
                Arc::<ConstructNode<Core>>::const_word(&ctx, Word::u2(1)).cmr(),
                Arc::<ConstructNode<Core>>::scribe(&ctx, &Value::u2(1)).cmr()
            );
            assert_eq!(
                Arc::<ConstructNode<Core>>::injl(&bit0).cmr(),
                Arc::<ConstructNode<Core>>::scribe(&ctx, &Value::left(Value::u1(0), Final::unit()))
                    .cmr()
            );
            assert_eq!(
                Arc::<ConstructNode<Core>>::injr(&bit1).cmr(),
                Arc::<ConstructNode<Core>>::scribe(
                    &ctx,
                    &Value::right(Final::unit(), Value::u1(1))
                )
                .cmr()
            );
            assert_eq!(
                Arc::<ConstructNode<Core>>::pair(&unit, &unit)
                    .unwrap()
                    .cmr(),
                Arc::<ConstructNode<Core>>::scribe(
                    &ctx,
                    &Value::product(Value::unit(), Value::unit())
                )
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

            let u0 = Arc::<ConstructNode<Core>>::unit(&ctx);
            let i1 = Arc::<ConstructNode<Core>>::injl(&u0);
            let i2 = Arc::<ConstructNode<Core>>::injr(&i1);
            let i3 = Arc::<ConstructNode<Core>>::injr(&i2);
            let i4 = Arc::<ConstructNode<Core>>::injl(&i3);
            let u5 = Arc::<ConstructNode<Core>>::unit(&ctx);
            let i6 = Arc::<ConstructNode<Core>>::injl(&u5);
            let i7 = Arc::<ConstructNode<Core>>::injr(&i6);
            let p8 = Arc::<ConstructNode<Core>>::pair(&i4, &i7).unwrap();
            let u9 = Arc::<ConstructNode<Core>>::unit(&ctx);
            let a10 = Arc::<ConstructNode<Core>>::assertr(cmr, &u9).unwrap();
            let u11 = Arc::<ConstructNode<Core>>::unit(&ctx);
            let a12 = Arc::<ConstructNode<Core>>::assertr(cmr, &u11).unwrap();
            let a13 = Arc::<ConstructNode<Core>>::assertl(&a12, cmr).unwrap();
            let c14 = Arc::<ConstructNode<Core>>::case(&a10, &a13).unwrap();
            let c15 = Arc::<ConstructNode<Core>>::comp(&p8, &c14).unwrap();

            let finalized: Arc<CommitNode<_>> = c15.finalize_types().unwrap();
            let prog = finalized.to_vec_without_witness();
            // In #286 we are encoding correctly...
            assert_eq!(
                hex::DisplayHex::as_hex(&prog).to_string(),
                "dc920a28812b6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f243090e00b10e00680",
            );

            let prog = BitIter::from(prog);
            let decode = CommitNode::<Core>::decode(prog).unwrap();

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
            let w0 = Arc::<ConstructNode<Core>>::witness(&ctx, None);
            let i1 = Arc::<ConstructNode<Core>>::iden(&ctx);
            let d2 = Arc::<ConstructNode<Core>>::drop_(&i1);
            let i3 = Arc::<ConstructNode<Core>>::iden(&ctx);
            let i4 = Arc::<ConstructNode<Core>>::iden(&ctx);
            let t5 = Arc::<ConstructNode<Core>>::take(&i4);
            let ca6 = Arc::<ConstructNode<Core>>::case(&i3, &t5).unwrap();
            let ca7 = Arc::<ConstructNode<Core>>::case(&d2, &ca6).unwrap();
            let c8 = Arc::<ConstructNode<Core>>::comp(&w0, &ca7).unwrap();
            let u9 = Arc::<ConstructNode<Core>>::unit(&ctx);
            let c10 = Arc::<ConstructNode<Core>>::comp(&c8, &u9).unwrap();

            // In #286 we incorrectly succeed finalizing the types, and then encode a bad program.
            let err = c10.finalize_types().unwrap_err();
            assert!(matches!(err, types::Error::OccursCheck { .. }));
        });
    }
}
