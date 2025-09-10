// SPDX-License-Identifier: CC0-1.0

use crate::analysis::NodeBounds;
use crate::bit_machine::{ExecutionError, SetTracker};
use crate::dag::{DagLike, InternalSharing, MaxSharing, PostOrderIterItem};
use crate::jet::Jet;
use crate::types::{self, arrow::FinalArrow};
use crate::{encode, BitMachine};
use crate::{Amr, BitIter, BitWriter, Cmr, DecodeError, Ihr, Imr, Value};

use super::{
    Commit, CommitData, CommitNode, Construct, ConstructData, ConstructNode, Constructible,
    Converter, Hide, Inner, Marker, NoDisconnect, NoWitness, Node,
};

use core::borrow::Borrow;
use std::collections::HashSet;
use std::io;
use std::marker::PhantomData;
use std::sync::Arc;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Redeem<J> {
    /// Makes the type non-constructible.
    never: std::convert::Infallible,
    /// Required by Rust.
    phantom: std::marker::PhantomData<J>,
}

impl<J: Jet> Marker for Redeem<J> {
    type CachedData = Arc<RedeemData<J>>;
    type Witness = Value;
    type Disconnect = Arc<RedeemNode<J>>;
    type SharingId = Ihr;
    type Jet = J;

    fn compute_sharing_id(_: Cmr, cached_data: &Arc<RedeemData<J>>) -> Option<Ihr> {
        Some(cached_data.ihr)
    }
}

pub type RedeemNode<J> = Node<Redeem<J>>;

#[derive(Clone, Debug)]
pub struct RedeemData<J> {
    amr: Amr,
    imr: Imr,
    ihr: Ihr,
    arrow: FinalArrow,
    bounds: NodeBounds,
    /// This isn't really necessary, but it helps type inference if every
    /// struct has a \<J\> parameter, since it forces the choice of jets to
    /// be consistent without the user needing to specify it too many times.
    phantom: PhantomData<J>,
}

impl<J> PartialEq for RedeemData<J> {
    fn eq(&self, other: &Self) -> bool {
        self.ihr == other.ihr
    }
}
impl<J> Eq for RedeemData<J> {}

impl<J> std::hash::Hash for RedeemData<J> {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.ihr.hash(hasher)
    }
}

impl<J: Jet> RedeemData<J> {
    pub fn new(arrow: FinalArrow, inner: Inner<&Arc<Self>, J, &Arc<Self>, Value>) -> Self {
        let (amr, imr, bounds) = match inner {
            Inner::Iden => (
                Amr::iden(&arrow),
                Imr::iden(),
                NodeBounds::iden(arrow.source.bit_width()),
            ),
            Inner::Unit => (Amr::unit(&arrow), Imr::unit(), NodeBounds::unit()),
            Inner::InjL(child) => (
                Amr::injl(&arrow, child.amr),
                Imr::injl(child.imr),
                NodeBounds::injl(child.bounds),
            ),
            Inner::InjR(child) => (
                Amr::injr(&arrow, child.amr),
                Imr::injr(child.imr),
                NodeBounds::injr(child.bounds),
            ),
            Inner::Take(child) => (
                Amr::take(&arrow, child.amr),
                Imr::take(child.imr),
                NodeBounds::take(child.bounds),
            ),
            Inner::Drop(child) => (
                Amr::drop(&arrow, child.amr),
                Imr::drop(child.imr),
                NodeBounds::drop(child.bounds),
            ),
            Inner::Comp(left, right) => (
                Amr::comp(&arrow, &left.arrow, left.amr, right.amr),
                Imr::comp(left.imr, right.imr),
                NodeBounds::comp(left.bounds, right.bounds, left.arrow.target.bit_width()),
            ),
            Inner::Case(left, right) => (
                Amr::case(&arrow, left.amr, right.amr),
                Imr::case(left.imr, right.imr),
                NodeBounds::case(left.bounds, right.bounds),
            ),
            Inner::AssertL(left, r_cmr) => (
                Amr::assertl(&arrow, left.amr, r_cmr.into()),
                Imr::case(left.imr, r_cmr.into()),
                NodeBounds::assertl(left.bounds),
            ),
            Inner::AssertR(l_cmr, right) => (
                Amr::assertr(&arrow, l_cmr.into(), right.amr),
                Imr::case(l_cmr.into(), right.imr),
                NodeBounds::assertr(right.bounds),
            ),
            Inner::Pair(left, right) => (
                Amr::pair(&arrow, &left.arrow, &right.arrow, left.amr, right.amr),
                Imr::pair(left.imr, right.imr),
                NodeBounds::pair(left.bounds, right.bounds),
            ),
            Inner::Disconnect(left, right) => (
                Amr::disconnect(&arrow, &right.arrow, left.amr, right.amr),
                Imr::disconnect(left.imr, right.imr),
                NodeBounds::disconnect(
                    left.bounds,
                    right.bounds,
                    left.arrow.target.bit_width() - right.arrow.source.bit_width(),
                    left.arrow.source.bit_width(),
                    left.arrow.target.bit_width(),
                ),
            ),
            Inner::Witness(ref value) => (
                Amr::witness(&arrow, value),
                Imr::witness(&arrow, value),
                NodeBounds::witness(arrow.target.bit_width()),
            ),
            Inner::Fail(entropy) => (Amr::fail(entropy), Imr::fail(entropy), NodeBounds::fail()),
            Inner::Jet(jet) => (Amr::jet(jet), Imr::jet(jet), NodeBounds::jet(jet)),
            Inner::Word(ref val) => (
                Amr::const_word(val),
                Imr::const_word(val),
                NodeBounds::const_word(val),
            ),
        };

        RedeemData {
            amr,
            imr,
            ihr: Ihr::from_imr(imr, &arrow),
            arrow,
            bounds,
            phantom: PhantomData,
        }
    }
}

impl<J: Jet> RedeemNode<J> {
    /// Accessor for the node's AMR
    pub fn amr(&self) -> Amr {
        self.data.amr
    }

    /// Accessor for the node's IHR
    pub fn ihr(&self) -> Ihr {
        self.data.ihr
    }

    /// Accessor for the node's type arrow
    pub fn arrow(&self) -> &FinalArrow {
        &self.data.arrow
    }

    /// Accessor for the node's bit machine bounds
    pub fn bounds(&self) -> NodeBounds {
        self.data.bounds
    }

    /// Convert a [`RedeemNode`] back to a [`CommitNode`] by forgetting witnesses
    /// and cached data.
    pub fn unfinalize(&self) -> Result<Arc<CommitNode<J>>, types::Error> {
        struct Unfinalizer<J>(PhantomData<J>);

        impl<J: Jet> Converter<Redeem<J>, Commit<J>> for Unfinalizer<J> {
            type Error = types::Error;
            fn convert_witness(
                &mut self,
                _: &PostOrderIterItem<&RedeemNode<J>>,
                _: &Value,
            ) -> Result<NoWitness, Self::Error> {
                Ok(NoWitness)
            }

            fn convert_disconnect(
                &mut self,
                _: &PostOrderIterItem<&RedeemNode<J>>,
                _: Option<&Arc<CommitNode<J>>>,
                _: &Arc<RedeemNode<J>>,
            ) -> Result<NoDisconnect, Self::Error> {
                Ok(NoDisconnect)
            }

            fn convert_data(
                &mut self,
                data: &PostOrderIterItem<&RedeemNode<J>>,
                inner: Inner<&Arc<CommitNode<J>>, J, &NoDisconnect, &NoWitness>,
            ) -> Result<Arc<CommitData<J>>, Self::Error> {
                let converted_data = inner.map(|node| node.cached_data());
                Ok(Arc::new(CommitData::from_final(
                    data.node.data.arrow.shallow_clone(),
                    converted_data,
                )))
            }
        }

        self.convert::<MaxSharing<Redeem<J>>, _, _>(&mut Unfinalizer(PhantomData))
    }

    /// Convert a [`RedeemNode`] back into a [`ConstructNode`]
    /// by loosening the finalized types, witness data and disconnected branches.
    pub fn to_construct_node<'brand>(
        &self,
        inference_context: &types::Context<'brand>,
    ) -> Arc<ConstructNode<'brand, J>> {
        struct ToConstruct<'a, 'brand, J> {
            inference_context: &'a types::Context<'brand>,
            phantom: PhantomData<J>,
        }

        impl<'brand, J: Jet> Converter<Redeem<J>, Construct<'brand, J>> for ToConstruct<'_, 'brand, J> {
            type Error = ();

            fn convert_witness(
                &mut self,
                _: &PostOrderIterItem<&Node<Redeem<J>>>,
                witness: &Value,
            ) -> Result<Option<Value>, Self::Error> {
                Ok(Some(witness.clone()))
            }

            fn convert_disconnect(
                &mut self,
                _: &PostOrderIterItem<&Node<Redeem<J>>>,
                right: Option<&Arc<Node<Construct<'brand, J>>>>,
                _: &Arc<RedeemNode<J>>,
            ) -> Result<Option<Arc<Node<Construct<'brand, J>>>>, Self::Error> {
                Ok(right.cloned())
            }

            fn convert_data(
                &mut self,
                _: &PostOrderIterItem<&Node<Redeem<J>>>,
                inner: Inner<
                    &Arc<Node<Construct<'brand, J>>>,
                    J,
                    &Option<Arc<ConstructNode<'brand, J>>>,
                    &Option<Value>,
                >,
            ) -> Result<ConstructData<'brand, J>, Self::Error> {
                let inner = inner
                    .map(|node| node.cached_data())
                    .map_witness(|maybe_value| maybe_value.clone());
                Ok(ConstructData::from_inner(self.inference_context, inner)
                    .expect("types were already finalized"))
            }
        }

        self.convert::<InternalSharing, _, _>(&mut ToConstruct {
            inference_context,
            phantom: PhantomData,
        })
        .unwrap()
    }

    /// Prune the redeem program for the given transaction environment.
    ///
    /// Pruning works as follows:
    /// 1) Run the redeem program on the Bit Machine.
    /// 2) Mark all (un)used case branches using the IHR of the case node.
    /// 3) Rebuild the program and omit unused branches.
    ///
    /// The pruning result depends on the witness data (which is already part of the redeem program)
    /// and on the transaction environment. These two inputs determine which case branches are
    /// used and which are not. Pruning must be done for each transaction environment separately,
    /// starting from the same original, unpruned program. Pruning is a lossy process, so pruning
    /// an already pruned program is not sound.
    ///
    /// Pruning fails if the original, unpruned program fails to run on the Bit Machine (step 1).
    /// In this case, the witness data needs to be revised.
    /// The other pruning steps (2 & 3) never fail.
    pub fn prune<T>(&self, env: &J::Environment<T>) -> Result<Arc<RedeemNode<J>>, ExecutionError>
    where
        T: Borrow<J::Transaction>,
    {
        struct Pruner<'brand, J> {
            inference_context: types::Context<'brand>,
            tracker: SetTracker,
            phantom: PhantomData<J>,
        }

        impl<'brand, J: Jet> Converter<Redeem<J>, Construct<'brand, J>> for Pruner<'brand, J> {
            type Error = std::convert::Infallible;

            fn convert_witness(
                &mut self,
                _: &PostOrderIterItem<&RedeemNode<J>>,
                witness: &Value,
            ) -> Result<Option<Value>, Self::Error> {
                // The pruned type is not finalized at this point,
                // so we cannot prune the witness value.
                Ok(Some(witness.shallow_clone()))
            }

            fn convert_disconnect(
                &mut self,
                _: &PostOrderIterItem<&RedeemNode<J>>,
                right: Option<&Arc<ConstructNode<'brand, J>>>,
                _: &Arc<RedeemNode<J>>,
            ) -> Result<Option<Arc<ConstructNode<'brand, J>>>, Self::Error> {
                debug_assert!(
                    right.is_some(),
                    "disconnected branch should exist in unpruned redeem program"
                );
                Ok(right.map(Arc::clone))
            }

            fn prune_case(
                &mut self,
                data: &PostOrderIterItem<&RedeemNode<J>>,
                _left: &Arc<ConstructNode<J>>,
                _right: &Arc<ConstructNode<J>>,
            ) -> Result<Hide, Self::Error> {
                // The IHR of the pruned program may change,
                // but the Converter trait gives us access to the unpruned node (`data`).
                // The Bit Machine tracked (un)used case branches based on the unpruned IHR.
                match (
                    self.tracker.left().contains(&data.node.ihr()),
                    self.tracker.right().contains(&data.node.ihr()),
                ) {
                    (true, true) => Ok(Hide::Neither),
                    (false, true) => Ok(Hide::Left),
                    (true, false) => Ok(Hide::Right),
                    (false, false) => Ok(Hide::Neither), // case nodes that were never executed will be pruned out by their ancestors
                }
            }

            fn convert_data(
                &mut self,
                _: &PostOrderIterItem<&RedeemNode<J>>,
                inner: Inner<
                    &Arc<ConstructNode<'brand, J>>,
                    J,
                    &Option<Arc<ConstructNode<'brand, J>>>,
                    &Option<Value>,
                >,
            ) -> Result<ConstructData<'brand, J>, Self::Error> {
                let converted_inner = inner
                    .map(|node| node.cached_data())
                    .map_witness(Option::<Value>::clone);
                let retyped = ConstructData::from_inner(&self.inference_context, converted_inner)
                    .expect("pruned types should check out if unpruned types check out");
                Ok(retyped)
            }
        }

        struct Finalizer<J>(PhantomData<J>);

        impl<'brand, J: Jet> Converter<Construct<'brand, J>, Redeem<J>> for Finalizer<J> {
            type Error = std::convert::Infallible;

            fn convert_witness(
                &mut self,
                data: &PostOrderIterItem<&ConstructNode<J>>,
                witness: &Option<Value>,
            ) -> Result<Value, Self::Error> {
                let pruned_target_ty = data
                    .node
                    .arrow()
                    .target
                    .finalize()
                    .expect("pruned types should check out if unpruned types check out");
                let pruned_witness = witness
                    .as_ref()
                    .expect("witness node that originally stems from redeem program should be populated")
                    .prune(&pruned_target_ty)
                    .expect("pruned type should be shrunken version of unpruned type");
                Ok(pruned_witness)
            }

            fn convert_disconnect(
                &mut self,
                _: &PostOrderIterItem<&ConstructNode<J>>,
                right: Option<&Arc<RedeemNode<J>>>,
                _: &Option<Arc<ConstructNode<J>>>,
            ) -> Result<Arc<RedeemNode<J>>, Self::Error> {
                Ok(right
                    .map(Arc::clone)
                    .expect("disconnect node that originally stems from redeem program should have all branches"))
            }

            fn convert_data(
                &mut self,
                data: &PostOrderIterItem<&ConstructNode<J>>,
                inner: Inner<&Arc<RedeemNode<J>>, J, &Arc<RedeemNode<J>>, &Value>,
            ) -> Result<Arc<RedeemData<J>>, Self::Error> {
                // Finalize target types of witness nodes in advance so we can prune their values.
                let final_arrow = data
                    .node
                    .arrow()
                    .finalize()
                    .expect("pruned types should check out if unpruned types check out");
                let converted_inner = inner
                    .map(|node| node.cached_data())
                    .map_disconnect(|node| node.cached_data())
                    .map_witness(Value::shallow_clone);
                Ok(Arc::new(RedeemData::new(final_arrow, converted_inner)))
            }
        }

        // 1) Run the Bit Machine and mark (un)used branches.
        // This is the only fallible step in the pruning process.
        let mut mac = BitMachine::for_program(self)?;
        let tracker = mac.exec_prune(self, env)?;

        // 2) Prune out unused case branches.
        // Because the types of the pruned program may change,
        // we construct a temporary witness program with unfinalized types.
        types::Context::with_context(|inference_context| {
            let pruned_witness_program = self
                .convert::<InternalSharing, _, _>(&mut Pruner {
                    inference_context,
                    tracker,
                    phantom: PhantomData,
                })
                .expect("pruning unused branches is infallible");

            // 3) Finalize the types of the witness program.
            // We obtain the pruned redeem program.
            // Once the pruned type is finalized, we can proceed to prune witness values.
            Ok(pruned_witness_program
                .convert::<InternalSharing, _, _>(&mut Finalizer(PhantomData))
                .expect("finalization is infallible"))
        })
    }

    /// Decode a Simplicity program from bits, including the witness data.
    pub fn decode<I1, I2>(
        program: BitIter<I1>,
        mut witness: BitIter<I2>,
    ) -> Result<Arc<Self>, DecodeError>
    where
        I1: Iterator<Item = u8>,
        I2: Iterator<Item = u8>,
    {
        // 0. Set up a type to help with the call to `convert` below
        struct DecodeFinalizer<'bits, J: Jet, I: Iterator<Item = u8>> {
            bits: &'bits mut BitIter<I>,
            phantom: PhantomData<J>,
        }

        impl<'brand, J: Jet, I: Iterator<Item = u8>> Converter<Construct<'brand, J>, Redeem<J>>
            for DecodeFinalizer<'_, J, I>
        {
            type Error = DecodeError;
            fn convert_witness(
                &mut self,
                data: &PostOrderIterItem<&ConstructNode<J>>,
                _: &Option<Value>,
            ) -> Result<Value, Self::Error> {
                let arrow = data.node.data.arrow();
                let target_ty = arrow.target.finalize().map_err(DecodeError::Type)?;
                Value::from_compact_bits(self.bits, &target_ty)
                    .map_err(crate::decode::Error::from)
                    .map_err(DecodeError::Decode)
            }

            fn convert_disconnect(
                &mut self,
                _: &PostOrderIterItem<&ConstructNode<J>>,
                right: Option<&Arc<RedeemNode<J>>>,
                _: &Option<Arc<ConstructNode<J>>>,
            ) -> Result<Arc<RedeemNode<J>>, Self::Error> {
                if let Some(child) = right {
                    Ok(Arc::clone(child))
                } else {
                    Err(DecodeError::DisconnectRedeemTime)
                }
            }

            fn convert_data(
                &mut self,
                data: &PostOrderIterItem<&ConstructNode<J>>,
                inner: Inner<&Arc<RedeemNode<J>>, J, &Arc<RedeemNode<J>>, &Value>,
            ) -> Result<Arc<RedeemData<J>>, Self::Error> {
                let arrow = data
                    .node
                    .data
                    .arrow()
                    .finalize()
                    .map_err(DecodeError::Type)?;
                let converted_data = inner
                    .map(|node| node.cached_data())
                    .map_disconnect(|node| node.cached_data())
                    .map_witness(Value::shallow_clone);
                Ok(Arc::new(RedeemData::new(arrow, converted_data)))
            }
        }

        // 1. Decode program without witnesses as ConstructNode
        let program: Arc<Self> = types::Context::with_context(|ctx| {
            let construct =
                crate::ConstructNode::decode(&ctx, program).map_err(DecodeError::Decode)?;
            construct
                .set_arrow_to_program()
                .map_err(DecodeError::Type)?;

            // Importantly, we  use `InternalSharing` here to make sure that we respect
            // the sharing choices that were actually encoded in the bitstream.
            construct.convert::<InternalSharing, _, _>(&mut DecodeFinalizer {
                bits: &mut witness,
                phantom: PhantomData,
            })
        })?;

        // 3. Check that we read exactly as much witness data as we expected
        witness
            .close()
            .map_err(crate::decode::Error::BitIter)
            .map_err(DecodeError::Decode)?;

        // 4. Check sharing
        // This loop is equivalent to using `program.is_shared_as::<MaxSharing>()`
        // but is faster since it only runs a single iterator.
        let mut ihrs: HashSet<Ihr> = HashSet::new();
        for data in program.as_ref().post_order_iter::<InternalSharing>() {
            if !ihrs.insert(data.node.ihr()) {
                return Err(DecodeError::Decode(crate::decode::Error::SharingNotMaximal));
            }
        }

        Ok(program)
    }

    #[cfg(feature = "base64")]
    #[allow(clippy::should_implement_trait)] // returns Arc<Self>
    pub fn from_str(prog: &str, wit: &str) -> Result<Arc<Self>, crate::ParseError> {
        use crate::base64::engine::general_purpose;
        use crate::base64::Engine as _;
        use crate::hex::FromHex as _;

        let v = general_purpose::STANDARD
            .decode(prog)
            .map_err(crate::ParseError::Base64)?;
        let prog_iter = crate::BitIter::new(v.into_iter());

        let v = Vec::from_hex(wit).map_err(crate::ParseError::Hex)?;
        let wit_iter = crate::BitIter::new(v.into_iter());
        Self::decode(prog_iter, wit_iter).map_err(crate::ParseError::Decode)
    }

    /// Encode the program to bits.
    ///
    /// Includes witness data. Returns the number of written bits.
    #[deprecated(since = "0.5.0", note = "use Self::encode_with_witness instead")]
    pub fn encode<W1, W2>(
        &self,
        prog: &mut BitWriter<W1>,
        witness: &mut BitWriter<W2>,
    ) -> io::Result<usize>
    where
        W1: io::Write,
        W2: io::Write,
    {
        let sharing_iter = self.post_order_iter::<MaxSharing<Redeem<J>>>();
        let program_bits = encode::encode_program(self, prog)?;
        prog.flush_all()?;
        let witness_bits = encode::encode_witness(sharing_iter.into_witnesses(), witness)?;
        witness.flush_all()?;
        Ok(program_bits + witness_bits)
    }

    /// Encode the program and witness data to byte vectors.
    #[deprecated(since = "0.5.0", note = "use Self::to_vec_with_witness instead")]
    pub fn encode_to_vec(&self) -> (Vec<u8>, Vec<u8>) {
        let mut ret_1 = vec![];
        let mut ret_2 = vec![];
        self.encode_with_witness(&mut ret_1, &mut ret_2).unwrap();
        (ret_1, ret_2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::human_encoding::Forest;
    use crate::jet::Core;
    use crate::node::SimpleFinalizer;
    use crate::types::Final;
    use hex::DisplayHex;
    use std::collections::HashMap;
    use std::fmt;

    #[cfg_attr(not(feature = "base64"), allow(unused_variables))]
    #[track_caller]
    fn assert_program_deserializable<J: Jet>(
        prog_bytes: &[u8],
        witness_bytes: &[u8],
        cmr_str: &str,
        amr_str: &str,
        ihr_str: &str,
        b64_str: &str,
    ) -> Arc<RedeemNode<J>> {
        let prog_hex = prog_bytes.as_hex();
        let witness_hex = witness_bytes.as_hex();

        let prog = BitIter::from(prog_bytes);
        let witness = BitIter::from(witness_bytes);
        let prog = match RedeemNode::<J>::decode(prog, witness) {
            Ok(prog) => prog,
            Err(e) => panic!("program {} failed: {}", prog_hex, e),
        };

        assert_eq!(
            prog.cmr().to_string(),
            cmr_str,
            "CMR mismatch (got {} expected {}) for program {}",
            prog.cmr(),
            cmr_str,
            prog_hex,
        );

        assert_eq!(
            prog.amr().to_string(),
            amr_str,
            "AMR mismatch (got {} expected {}) for program {}",
            prog.amr(),
            amr_str,
            prog_hex,
        );
        assert_eq!(
            prog.ihr().to_string(),
            ihr_str,
            "IHR mismatch (got {} expected {}) for program {}",
            prog.ihr(),
            ihr_str,
            prog_hex,
        );

        let (reser_prog, reser_witness) = prog.to_vec_with_witness();
        assert_eq!(
            prog_bytes,
            &reser_prog[..],
            "program {} reserialized as {}",
            prog_hex,
            reser_prog.as_hex(),
        );
        assert_eq!(
            witness_bytes,
            &reser_witness[..],
            "witness {} reserialized as {}",
            witness_hex,
            reser_witness.as_hex(),
        );

        #[cfg(feature = "base64")]
        {
            let disp = prog.display();
            assert_eq!(prog.to_string(), b64_str);
            assert_eq!(disp.program().to_string(), b64_str);
            assert_eq!(
                disp.witness().to_string(),
                witness_bytes.as_hex().to_string()
            );
        }

        prog
    }

    #[track_caller]
    fn assert_program_not_deserializable<J: Jet>(
        prog_bytes: &[u8],
        witness_bytes: &[u8],
        err: &dyn fmt::Display,
    ) {
        let prog_hex = prog_bytes.as_hex();
        let witness_hex = witness_bytes.as_hex();
        let err_str = err.to_string();

        let prog = BitIter::from(prog_bytes);
        let witness = BitIter::from(witness_bytes);
        match RedeemNode::<J>::decode(prog, witness) {
            Ok(prog) => panic!(
                "Program {} wit {} succeded (expected error {}). Program parsed as:\n{:?}",
                prog_hex, witness_hex, err, prog
            ),
            Err(e) if e.to_string() == err_str => {} // ok
            Err(e) => panic!(
                "Program {} wit {} failed with error {} (expected error {})",
                prog_hex, witness_hex, e, err
            ),
        };
    }

    #[test]
    fn encode_shared_witnesses() {
        // # Program code:
        // wit1 = witness :: 1 -> 2^32
        // wit2 = witness :: 1 -> 2^32
        //
        // wits_are_equal = comp (pair wit1 wit2) jet_eq_32 :: 1 -> 2
        // main = comp wits_are_equal jet_verify            :: 1 -> 1
        let eqwits = [0xcd, 0xdc, 0x51, 0xb6, 0xe2, 0x08, 0xc0, 0x40];
        let iter = BitIter::from(&eqwits[..]);
        let eqwits_prog = CommitNode::<Core>::decode(iter).unwrap();

        let eqwits_final = eqwits_prog
            .finalize(&mut SimpleFinalizer::new(std::iter::repeat(Value::u32(
                0xDEADBEEF,
            ))))
            .unwrap();
        let output = eqwits_final.to_vec_with_witness();

        assert_eq!(
            output,
            (
                [0xc9, 0xc4, 0x6d, 0xb8, 0x82, 0x30, 0x10].into(),
                [0xde, 0xad, 0xbe, 0xef].into(),
            ),
            "output {} {}",
            output.0.as_hex(),
            output.1.as_hex()
        );
    }

    #[test]
    fn decode_shared_witnesses() {
        // This program is exactly the output from the `encode_shared_witnesses` test.
        // The point of this is to make sure that our witness-unsharing logic doesn't
        // get confused here and try to read two witnesses when there are only one.
        assert_program_deserializable::<Core>(
            &[0xc9, 0xc4, 0x6d, 0xb8, 0x82, 0x30, 0x10],
            &[0xde, 0xad, 0xbe, 0xef],
            "d7969920eff9a1ed0359aaa8545b239c69969e22c304c645a7b49bcc976a40a8",
            "f7acbb077e7661a08384818bc8e3a275ed42ad446252575a35a35f71689fef78",
            "3ce4a6390b4e4bda6330acda4800e66e5d2cae0f5a2888564c706f2b910146b8",
            "ycRtuIIwEA==",
        );
    }

    #[test]
    fn unshared_child() {
        // # id1 and id2 should be shared, but are not!
        // id1 = iden          :: A -> A # cmr dbfefcfc...
        // id2 = iden          :: A -> A # cmr dbfefcfc...
        // cp3 = comp id1 id2  :: A -> A # cmr c1ae55b5...
        // main = comp cp3 cp3 :: A -> A # cmr 314e2879...
        assert_program_not_deserializable::<Core>(
            &[0xc1, 0x08, 0x04, 0x00],
            &[],
            &DecodeError::Decode(crate::decode::Error::SharingNotMaximal),
        );
    }

    #[test]
    fn witness_consumed() {
        // "main = unit", but with a witness attached. Found by fuzzer.
        let prog = BitIter::from(&[0x24][..]);
        let wit = BitIter::from(&[0x00][..]);
        match RedeemNode::<Core>::decode(prog, wit) {
            Err(DecodeError::Decode(crate::decode::Error::BitIter(
                crate::BitIterCloseError::TrailingBytes { first_byte: 0 },
            ))) => {} // ok,
            Err(e) => panic!("got incorrect error {e}"),
            Ok(_) => panic!("accepted program with bad witness length"),
        }
    }

    #[test]
    fn shared_grandchild() {
        // # This program repeats the node `cp2` three times; during iteration it will
        // # be placed on the stack as part of the initial `comp` combinator, but by
        // # the time we get to it, it will have already been yielded. Makes sure this
        // # does not confuse the iteration logic and break the decoded program structure.
        // id1 = iden
        // cp2 = comp id1 id1
        // cp3 = comp cp2 cp2
        // main = comp cp3 cp2
        assert_program_deserializable::<Core>(
            &[0xc1, 0x00, 0x00, 0x01, 0x00],
            &[],
            "8a54101335ca2cf7e933d74cdb15f99becc4e540799ba5e2d19c00c9d7219e71",
            "74e868bd640c250bc45522085158a9723fc7e277bb16a8d582c4012ebbb1f6f1",
            "39b8f72bd1539de87d26673890603d6548cfc8b68571d996bdf9b1d8b557bd35",
            "wQAAAQA=",
        );
    }

    #[test]
    #[rustfmt::skip]
    fn assert_lr() {
        // asst = assertl unit deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef
        // input0 = pair (injl unit) unit
        // main = comp input0 asst
        assert_program_deserializable::<Core>(
            &[
                0xcd, 0x24, 0x08, 0x4b, 0x6f, 0x56, 0xdf, 0x77,
                0xef, 0x56, 0xdf, 0x77, 0xef, 0x56, 0xdf, 0x77,
                0xef, 0x56, 0xdf, 0x77, 0xef, 0x56, 0xdf, 0x77,
                0xef, 0x56, 0xdf, 0x77, 0xef, 0x56, 0xdf, 0x77,
                0xef, 0x56, 0xdf, 0x77, 0x86, 0x01, 0x80,
            ],
            &[],
            "abdd773fc7a503908739b4a63198416fdd470948830cb5a6516b98fe0a3bfa85",
            "1362ee53ae75218ed51dc4bd46cdbfa585f934ac6c6c3ff787e27dce91ccd80b",
            "251c6778129e0f12da3f2388ab30184e815e9d9456b5931e54802a6715d9ca42",
            "zSQIS29W33fvVt9371bfd+9W33fvVt9371bfd+9W33fvVt93hgGA",
        );


        // asst = assertr deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef unit
        // input1 = pair (injr unit) unit
        // main = comp input1 asst
        assert_program_deserializable::<Core>(
            &[
                0xcd, 0x25, 0x08, 0x6d, 0xea, 0xdb, 0xee, 0xfd,
                0xea, 0xdb, 0xee, 0xfd, 0xea, 0xdb, 0xee, 0xfd,
                0xea, 0xdb, 0xee, 0xfd, 0xea, 0xdb, 0xee, 0xfd,
                0xea, 0xdb, 0xee, 0xfd, 0xea, 0xdb, 0xee, 0xfd,
                0xea, 0xdb, 0xee, 0xf4, 0x86, 0x01, 0x80,
            ],
            &[],
            "f6c678dfb180b94567a9d524e05fbc893f6905e0e3db931ff01dc2701e783d4c",
            "212d4fa3dbe2b33db1e11bb6f4cc973be5de0896a3775387a06056483b8feb0f",
            "7a583edcc733b6bba66998110be403ac61fab2d93fc09ba3c84ab2509b538043",
            "zSUIberb7v3q2+796tvu/erb7v3q2+796tvu/erb7v3q2+70hgGA",
        );
    }

    #[test]
    #[rustfmt::skip]
    fn disconnect() {
        // id1 = iden                 :: (2^256 * B) -> (2^256 * B)                       # cmr dbfefcfc...
        // pr2 = pair id1 id1         :: (2^256 * B) -> ((2^256 * B) * (2^256 * B))       # cmr a62c628c...
        // disc3 = disconnect pr2 pr2 :: B -> ((2^256 * B) * ((2^256 * B) * (2^256 * B))) # cmr d81d6f28...
        // ut4 = unit                 :: ((2^256 * B) * ((2^256 * B) * (2^256 * B))) -> 1 # cmr 62274a89...
        // main = comp disc3 ut4      :: B -> 1                                           # cmr a453360c...
        assert_program_deserializable::<Core>(
            &[0xc5, 0x02, 0x06, 0x24, 0x10],
            &[],
            "afe8f5f8bd3f64bfa51d2f29ffa22523604d9654c0d9862dbf2dc67ba097cbb2",
            "15239708cb7b448cedc6a0b6401dce86ed74084056dd95831928860dd0c3ca67",
            "9cdacb48b16e108ccbd6bcbce459a64056df285c2dc6e02dca6d13c4b1530fb0",
            "xQIGJBA=",
        );
    }

    #[test]
    #[rustfmt::skip]
    #[cfg(feature = "elements")]
    fn disconnect2() {
        // Program that Russell posted on IRC 2023-06-22 that tickled `Dag::right_child`
        // bug that had been introduced that day. Probably not the most minimal test
        // vector but might as well include it as it seems like an interesting program.
        //
        // # Witnesses
        // wit1 = witness :: 1 -> 2^512
        //
        // # Constants
        // const1 = word_jet 0x00000000000000000000003b78ce563f89a0ed9414f5aa28ad0d96d6795f9c63 :: 1 -> 2^256 # cmr a9e3dbca...
        //
        // # Program code
        // id1 = iden                 :: (2^256 * 1) -> (2^256 * 1)     # cmr dbfefcfc...
        // jt2 = jet_sig_all_hash     :: 1 -> 2^256                     # cmr 9902bc0f...
        // disc3 = disconnect id1 jt2 :: 1 -> 2^512                     # cmr 6968f10e...
        // pr4 = pair const1 disc3    :: 1 -> (2^256 * 2^512)           # cmr 378ad609...
        // pr5 = pair pr4 wit1        :: 1 -> ((2^256 * 2^512) * 2^512) # cmr 0d51ff00...
        // jt6 = jet_check_sig_verify :: ((2^256 * 2^512) * 2^512) -> 1 # cmr 297459d8...
        //
        // main = comp pr5 jt6        :: 1 -> 1                         # cmr 14a5e0cc...
        assert_program_deserializable::<crate::jet::Elements>(
            &[
                0xd3, 0x69, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3b, 0x78, 0xce,
                0x56, 0x3f, 0x89, 0xa0, 0xed, 0x94, 0x14, 0xf5, 0xaa, 0x28, 0xad, 0x0d, 0x96, 0xd6, 0x79, 0x5f,
                0x9c, 0x63, 0x47, 0x07, 0x02, 0xc0, 0xe2, 0x8d, 0x88, 0x10, 
            ],
            &[
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3b, 0x78, 0xce, 0x56, 0x3f,
                0x89, 0xa0, 0xed, 0x94, 0x14, 0xf5, 0xaa, 0x28, 0xad, 0x0d, 0x96, 0xd6, 0x79, 0x5f, 0x9c, 0x63,
                0xdb, 0x86, 0x8d, 0x45, 0xa0, 0xbc, 0x1d, 0x19, 0x01, 0x30, 0x2b, 0xc8, 0x7a, 0x87, 0x1c, 0xf1,
                0x58, 0xe2, 0xbd, 0xe2, 0xcf, 0xa6, 0x45, 0xa8, 0x95, 0xc1, 0xb4, 0x5d, 0x68, 0xea, 0x24, 0xc0, 
            ],
            "f3cd4537d7ebb201732203195b30b549b8dc0c2c6257b3a0d53bedb08ea02874",
            "107fa80454ed0f2d95d7c18d307912b1497505b98de47198fee23b5018efa544",
            "d52021c638ba742a90bead9b3055efd66091fb50bb131aa8b10eb7c13ef464d1",
            "02kAAAAAAAAAAAAAADt4zlY/iaDtlBT1qiitDZbWeV+cY0cHAsDijYgQ",
        );
    }

    #[test]
    #[rustfmt::skip]
    #[cfg(feature = "elements")]
    fn disconnect3() {
        // Yet another disconnect-based program that hit a bug in our AMR computation
        // (passing left arrow in place of right arrow to the AMR constructor.)
        // # Program code
        // id1 = iden                 :: (2^256 * 1) -> (2^256 * 1) # cmr dbfefcfc...
        // ut2 = unit                 :: 1 -> 1                     # cmr 62274a89...
        // jl3 = injl ut2             :: 1 -> 2                     # cmr bd0cce93...
        // disc4 = disconnect id1 jl3 :: 1 -> (2^256 * 2)           # cmr 6968f10e...
        // ut5 = unit                 :: (2^256 * 2) -> 1           # cmr 62274a89...
        // main = comp disc4 ut5      :: 1 -> 1                     # cmr a8c9cc7a...
        assert_program_deserializable::<crate::jet::Elements>(
            &[0xc9, 0x09, 0x20, 0x74, 0x90, 0x40],
            &[],
            "b689bdee289c8dd4e2e283358d187813363d441776cf826dafc27cc8a81ec441",
            "3c68660a1afde7982ce4aa9d499ad382bc32f5f9ad894a5e915f76e66303a25b",
            "85313720ee43ae0ee03f88b05e6d9e4494308c6897bdeb3e93b94559c3317484",
            "yQkgdJBA",
        );
    }

    #[test]
    #[cfg(feature = "elements")]
    fn decode_schnorr() {
        #[rustfmt::skip]
        let schnorr0 = vec![
            0xc6, 0xd5, 0xf2, 0x61, 0x14, 0x03, 0x24, 0xb1, 0x86, 0x20, 0x92, 0x68, 0x9f, 0x0b, 0xf1, 0x3a,
            0xa4, 0x53, 0x6a, 0x63, 0x90, 0x8b, 0x06, 0xdf, 0x33, 0x61, 0x0c, 0x03, 0xe2, 0x27, 0x79, 0xc0,
            0x6d, 0xf2, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0xe2, 0x8d, 0x8c, 0x04, 0x00,
        ];
        #[rustfmt::skip]
        let schnorr0_wit = vec![
            0xe9, 0x07, 0x83, 0x1f, 0x80, 0x84, 0x8d, 0x10, 0x69, 0xa5, 0x37, 0x1b, 0x40, 0x24, 0x10, 0x36,
            0x4b, 0xdf, 0x1c, 0x5f, 0x83, 0x07, 0xb0, 0x08, 0x4c, 0x55, 0xf1, 0xce, 0x2d, 0xca, 0x82, 0x15,
            0x25, 0xf6, 0x6a, 0x4a, 0x85, 0xea, 0x8b, 0x71, 0xe4, 0x82, 0xa7, 0x4f, 0x38, 0x2d, 0x2c, 0xe5,
            0xeb, 0xee, 0xe8, 0xfd, 0xb2, 0x17, 0x2f, 0x47, 0x7d, 0xf4, 0x90, 0x0d, 0x31, 0x05, 0x36, 0xc0,
        ];
        assert_program_deserializable::<crate::jet::Elements>(
            &schnorr0,
            &schnorr0_wit,
            "8a9e97676b24be7797d9ee0bf32dd76bcd78028e973025f785eae8dc91c8a0da",
            "ec97c8774cb6bfb381fdbbcc8d964380fb3a3b45779322624490d6231ae777a4",
            "ad7c38b16b9129646dc89b52cff144de94a80e383c4983b53de65e3575abcf38",
            "xtXyYRQDJLGGIJJonwvxOqRTamOQiwbfM2EMA+InecBt8gAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA4o2MBAA=",
        );
    }

    #[cfg(feature = "elements")]
    fn assert_correct_pruning<J: Jet, T: Borrow<J::Transaction>>(
        unpruned_prog: &str,
        unpruned_wit: &HashMap<Arc<str>, Value>,
        expected_pruned_prog: &str,
        expected_pruned_wit: &HashMap<Arc<str>, Value>,
        env: &J::Environment<T>,
    ) {
        let unpruned_program = types::Context::with_context(|ctx| {
            Forest::<J>::parse(unpruned_prog)
                .expect("unpruned program should parse")
                .to_witness_node(&ctx, unpruned_wit)
                .expect("unpruned program should have main")
                .finalize_unpruned()
                .expect("unpruned program should finalize")
        });
        let expected_unpruned_program = types::Context::with_context(|ctx| {
            Forest::<J>::parse(expected_pruned_prog)
                .expect("expected pruned program should parse")
                .to_witness_node(&ctx, expected_pruned_wit)
                .expect("expected pruned program should have main")
                .finalize_unpruned()
                .expect("expected pruned program should finalize")
        });

        let mut mac = BitMachine::for_program(&unpruned_program)
            .expect("unpruned program has reasonable bounds");
        let unpruned_output = mac
            .exec(&unpruned_program, env)
            .expect("unpruned program should run without failure");

        let pruned_program = unpruned_program
            .prune(env)
            .expect("pruning should not fail if execution succeeded");
        assert_eq!(
            pruned_program.ihr(),
            expected_unpruned_program.ihr(),
            "pruning result differs from expected result"
        );

        let mut mac =
            BitMachine::for_program(&pruned_program).expect("pruned program has reasonable bounds");
        let pruned_output = mac
            .exec(&pruned_program, env)
            .expect("pruned program should run without failure");
        assert_eq!(
            unpruned_output, pruned_output,
            "pruned program should return same output as unpruned program"
        );
    }

    #[test]
    #[cfg(feature = "elements")]
    fn prune() {
        let env = crate::jet::elements::ElementsEnv::dummy();

        /*
         * 1) Prune a product type / value
         */
        let unpruned_prog = r#"wit1 := witness : 1 -> 2
wit2 := witness : 1 -> 2^64 * 2^64
input := pair wit1 wit2 : 1 -> 2 * (2^64 * 2^64)
process := case (drop take jet_is_zero_64) (drop drop jet_is_zero_64) : 2 * (2^64 * 2^64) -> 2
main := comp input comp process jet_verify : 1 -> 1"#;

        // 1.1) Prune right
        let unpruned_wit = HashMap::from([
            (Arc::from("wit1"), Value::u1(0)),
            (
                Arc::from("wit2"),
                Value::product(Value::u64(0), Value::u64(0)),
            ),
        ]);
        let pruned_prog = r#"wit1 := witness : 1 -> 2
wit2 := witness : 1 -> 2^64 * 1 -- right component was pruned
input := pair wit1 wit2 : 1 -> 2 * (2^64 * 1)
process := assertl (drop take jet_is_zero_64) #{drop drop jet_is_zero_64} : 2 * (2^64 * 1) -> 2 -- case became assertl
main := comp input comp process jet_verify : 1 -> 1"#;
        let pruned_wit = HashMap::from([
            (Arc::from("wit1"), Value::u1(0)),
            (
                Arc::from("wit2"),
                Value::product(Value::u64(0), Value::unit()),
            ),
        ]);
        assert_correct_pruning::<crate::jet::Elements, _>(
            unpruned_prog,
            &unpruned_wit,
            pruned_prog,
            &pruned_wit,
            &env,
        );

        // 1.2) Prune left
        let unpruned_wit = HashMap::from([
            (Arc::from("wit1"), Value::u1(1)),
            (
                Arc::from("wit2"),
                Value::product(Value::u64(0), Value::u64(0)),
            ),
        ]);
        let pruned_prog = r#"wit1 := witness : 1 -> 2
wit2 := witness : 1 -> 1 * 2^64 -- left component was pruned
input := pair wit1 wit2 : 1 -> 2 * (1 * 2^64)
process := assertr #{drop take jet_is_zero_64} (drop drop jet_is_zero_64) : 2 * (1 * 2^64) -> 2 -- case became assertr
main := comp input comp process jet_verify : 1 -> 1"#;
        let pruned_wit = HashMap::from([
            (Arc::from("wit1"), Value::u1(1)),
            (
                Arc::from("wit2"),
                Value::product(Value::unit(), Value::u64(0)),
            ),
        ]);
        assert_correct_pruning::<crate::jet::Elements, _>(
            unpruned_prog,
            &unpruned_wit,
            pruned_prog,
            &pruned_wit,
            &env,
        );

        /*
         * 1) Prune a sum type / value
         */
        let prune_sum = r#"wit1 := witness : 1 -> 2^64 + 2^64
input := pair wit1 unit : 1 -> (2^64 + 2^64) * 1
process := case (take jet_is_zero_64) (take jet_is_zero_64) : (2^64 + 2^64) * 1 -> 2
main := comp input comp process jet_verify : 1 -> 1"#;

        // 1.1) Prune right
        let unpruned_wit =
            HashMap::from([(Arc::from("wit1"), Value::left(Value::u64(0), Final::u64()))]);
        let pruned_prog = r#"wit1 := witness : 1 -> 2^64 + 1 -- right sub type became unit
input := pair wit1 unit : 1 -> (2^64 + 1) * 1
process := assertl (take jet_is_zero_64) #{take jet_is_zero_64} : (2^64 + 1) * 1 -> 2 -- case became assertl
main := comp input comp process jet_verify : 1 -> 1"#;
        let pruned_wit =
            HashMap::from([(Arc::from("wit1"), Value::left(Value::u64(0), Final::unit()))]);
        assert_correct_pruning::<crate::jet::Elements, _>(
            prune_sum,
            &unpruned_wit,
            pruned_prog,
            &pruned_wit,
            &env,
        );

        // 1.2) Prune left
        let unpruned_wit = HashMap::from([(
            Arc::from("wit1"),
            Value::right(Final::unit(), Value::u64(0)),
        )]);
        let pruned_prog = r#"wit1 := witness : 1 -> 1 + 2^64 -- left sub type became unit
input := pair wit1 unit : 1 -> (1 + 2^64) * 1
process := assertr #{take jet_is_zero_64} (take jet_is_zero_64) : (1 + 2^64) * 1 -> 2 -- case became assertr
main := comp input comp process jet_verify : 1 -> 1"#;
        let pruned_wit = HashMap::from([(
            Arc::from("wit1"),
            Value::right(Final::unit(), Value::u64(0)),
        )]);
        assert_correct_pruning::<crate::jet::Elements, _>(
            prune_sum,
            &unpruned_wit,
            pruned_prog,
            &pruned_wit,
            &env,
        );
    }
}

#[cfg(bench)]
#[cfg(feature = "elements")]
mod benches {
    use super::*;

    use crate::bit_encoding::BitIter;
    use crate::jet::Elements;

    use test::{black_box, Bencher};

    #[bench]
    fn decode_fixed_program(bh: &mut Bencher) {
        let prog = &[
            0xd3, 0x69, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3b,
            0x78, 0xce, 0x56, 0x3f, 0x89, 0xa0, 0xed, 0x94, 0x14, 0xf5, 0xaa, 0x28, 0xad, 0x0d,
            0x96, 0xd6, 0x79, 0x5f, 0x9c, 0x63, 0x47, 0x07, 0x02, 0xc0, 0xe2, 0x8d, 0x88, 0x10,
        ][..];
        let witness = &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3b, 0x78, 0xce,
            0x56, 0x3f, 0x89, 0xa0, 0xed, 0x94, 0x14, 0xf5, 0xaa, 0x28, 0xad, 0x0d, 0x96, 0xd6,
            0x79, 0x5f, 0x9c, 0x63, 0xdb, 0x86, 0x8d, 0x45, 0xa0, 0xbc, 0x1d, 0x19, 0x01, 0x30,
            0x2b, 0xc8, 0x7a, 0x87, 0x1c, 0xf1, 0x58, 0xe2, 0xbd, 0xe2, 0xcf, 0xa6, 0x45, 0xa8,
            0x95, 0xc1, 0xb4, 0x5d, 0x68, 0xea, 0x24, 0xc0,
        ][..];
        bh.iter(|| {
            let prog = BitIter::from(prog);
            let witness = BitIter::from(witness);
            black_box(RedeemNode::<Elements>::decode(
                black_box(prog),
                black_box(witness),
            ))
            .unwrap();
        });
    }
}
