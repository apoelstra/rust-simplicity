// SPDX-License-Identifier: CC0-1.0

use crate::dag::{DagLike, MaxSharing, NoSharing, PostOrderIterItem};
use crate::jet::Jet;
use crate::types::arrow::{Arrow, FinalArrow};
use crate::{encode, types, Value};
use crate::{Amr, BitIter, BitWriter, Cmr, DecodeError, Ihr, Imr};

use super::{
    Construct, ConstructData, ConstructNode, Constructible, Converter, Inner, Marker, NoDisconnect,
    NoWitness, Node, Redeem, RedeemNode,
};

use std::io;
use std::marker::PhantomData;
use std::sync::Arc;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Commit<J> {
    /// Makes the type non-constructible.
    never: std::convert::Infallible,
    /// Required by Rust.
    phantom: std::marker::PhantomData<J>,
}

impl<J: Jet> Marker for Commit<J> {
    type CachedData = Arc<CommitData<J>>;
    type Witness = NoWitness;
    type Disconnect = NoDisconnect;
    type SharingId = Ihr;
    type Jet = J;

    fn compute_sharing_id(_: Cmr, cached_data: &Arc<CommitData<J>>) -> Option<Ihr> {
        cached_data.ihr
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CommitData<J> {
    /// The source and target types of the node
    arrow: FinalArrow,
    /// The IMR of the node if it exists. This is distinct from its IHR; it is
    /// used during computation of the IHR.
    imr: Option<Imr>,
    /// The AMR of the node if it exists, meaning, if it is not (an ancestor of)
    /// a witness or disconnect node.
    amr: Option<Amr>,
    /// The IHR of the node if it exists, meaning, if it is not (an ancestor of)
    /// a witness or disconnect node.
    ihr: Option<Ihr>,
    /// This isn't really necessary, but it helps type inference if every
    /// struct has a \<J\> parameter, since it forces the choice of jets to
    /// be consistent without the user needing to specify it too many times.
    phantom: PhantomData<J>,
}

impl<J: Jet> CommitData<J> {
    /// Accessor for the node's arrow
    pub fn arrow(&self) -> &FinalArrow {
        &self.arrow
    }

    /// Accessor for the node's IHR, if known
    pub fn ihr(&self) -> Option<Ihr> {
        self.ihr
    }

    /// Helper function to compute a cached AMR
    fn incomplete_amr(
        inner: Inner<&Arc<Self>, J, &NoDisconnect, &NoWitness>,
        arrow: &FinalArrow,
    ) -> Option<Amr> {
        match inner {
            Inner::Iden => Some(Amr::iden(arrow)),
            Inner::Unit => Some(Amr::unit(arrow)),
            Inner::InjL(child) => child.amr.map(|amr| Amr::injl(arrow, amr)),
            Inner::InjR(child) => child.amr.map(|amr| Amr::injr(arrow, amr)),
            Inner::Take(child) => child.amr.map(|amr| Amr::take(arrow, amr)),
            Inner::Drop(child) => child.amr.map(|amr| Amr::drop(arrow, amr)),
            Inner::Comp(left, right) => left
                .amr
                .zip(right.amr)
                .map(|(a, b)| Amr::comp(arrow, &left.arrow, a, b)),
            Inner::Case(left, right) => {
                left.amr.zip(right.amr).map(|(a, b)| Amr::case(arrow, a, b))
            }
            Inner::AssertL(left, r_cmr) => left
                .amr
                .map(|l_amr| Amr::assertl(arrow, l_amr, r_cmr.into())),
            Inner::AssertR(l_cmr, right) => right
                .amr
                .map(|r_amr| Amr::assertr(arrow, l_cmr.into(), r_amr)),
            Inner::Pair(left, right) => left
                .amr
                .zip(right.amr)
                .map(|(a, b)| Amr::pair(arrow, &left.arrow, &right.arrow, a, b)),
            Inner::Disconnect(..) => None,
            Inner::Witness(..) => None,
            Inner::Fail(entropy) => Some(Amr::fail(entropy)),
            Inner::Jet(jet) => Some(Amr::jet(jet)),
            Inner::Word(ref val) => Some(Amr::const_word(val)),
        }
    }

    /// Helper function to compute a cached first-pass IHR
    fn imr(inner: Inner<&Arc<Self>, J, &NoDisconnect, &NoWitness>) -> Option<Imr> {
        match inner {
            Inner::Iden => Some(Imr::iden()),
            Inner::Unit => Some(Imr::unit()),
            Inner::InjL(child) => child.imr.map(Imr::injl),
            Inner::InjR(child) => child.imr.map(Imr::injr),
            Inner::Take(child) => child.imr.map(Imr::take),
            Inner::Drop(child) => child.imr.map(Imr::drop),
            Inner::Comp(left, right) => left.imr.zip(right.imr).map(|(a, b)| Imr::comp(a, b)),
            Inner::Case(left, right) => left.imr.zip(right.imr).map(|(a, b)| Imr::case(a, b)),
            Inner::AssertL(left, r_cmr) => left.imr.map(|l_ihr| Imr::case(l_ihr, r_cmr.into())),
            Inner::AssertR(l_cmr, right) => right.imr.map(|r_ihr| Imr::case(l_cmr.into(), r_ihr)),
            Inner::Pair(left, right) => left.imr.zip(right.imr).map(|(a, b)| Imr::pair(a, b)),
            Inner::Disconnect(..) => None,
            Inner::Witness(..) => None,
            Inner::Fail(entropy) => Some(Imr::fail(entropy)),
            Inner::Jet(jet) => Some(Imr::jet(jet)),
            Inner::Word(ref val) => Some(Imr::const_word(val)),
        }
    }

    pub fn new(
        arrow: &Arrow,
        inner: Inner<&Arc<Self>, J, &NoDisconnect, &NoWitness>,
    ) -> Result<Self, types::Error> {
        let final_arrow = arrow.finalize()?;
        let imr = Self::imr(inner.clone());
        let amr = Self::incomplete_amr(inner, &final_arrow);
        Ok(CommitData {
            imr,
            amr,
            ihr: imr.map(|ihr| Ihr::from_imr(ihr, &final_arrow)),
            arrow: final_arrow,
            phantom: PhantomData,
        })
    }

    pub fn from_final(
        arrow: FinalArrow,
        inner: Inner<&Arc<Self>, J, &NoDisconnect, &NoWitness>,
    ) -> Self {
        let imr = Self::imr(inner.clone());
        let amr = Self::incomplete_amr(inner, &arrow);
        CommitData {
            imr,
            amr,
            ihr: imr.map(|ihr| Ihr::from_imr(ihr, &arrow)),
            arrow,
            phantom: PhantomData,
        }
    }
}

pub type CommitNode<J> = Node<Commit<J>>;

impl<J: Jet> CommitNode<J> {
    /// Accessor for the node's arrow
    pub fn arrow(&self) -> &FinalArrow {
        &self.data.arrow
    }

    /// Accessor for the node's AMR, if known
    pub fn amr(&self) -> Option<Amr> {
        self.data.amr
    }

    /// Accessor for the node's IHR, if known
    pub fn ihr(&self) -> Option<Ihr> {
        self.data.ihr
    }

    /// Finalizes a DAG, by iterating through through it without sharing, attaching
    /// witnesses, and hiding branches.
    ///
    /// This is a thin wrapper around [`Node::convert`] which fixes a few types to make
    /// it easier to use.
    pub fn finalize<C: Converter<Commit<J>, Redeem<J>>>(
        &self,
        converter: &mut C,
    ) -> Result<Arc<RedeemNode<J>>, C::Error> {
        self.convert::<NoSharing, Redeem<J>, _>(converter)
    }

    /// Convert a [`CommitNode`] back to a [`ConstructNode`] by redoing type inference
    pub fn unfinalize_types<'brand>(
        &self,
        inference_context: &types::Context<'brand>,
    ) -> Result<Arc<ConstructNode<'brand, J>>, types::Error> {
        struct UnfinalizeTypes<'a, 'brand, J: Jet> {
            inference_context: &'a types::Context<'brand>,
            phantom: PhantomData<J>,
        }

        impl<'brand, J: Jet> Converter<Commit<J>, Construct<'brand, J>> for UnfinalizeTypes<'_, 'brand, J> {
            type Error = types::Error;
            fn convert_witness(
                &mut self,
                _: &PostOrderIterItem<&CommitNode<J>>,
                _: &NoWitness,
            ) -> Result<Option<Value>, Self::Error> {
                Ok(None)
            }

            fn convert_disconnect(
                &mut self,
                _: &PostOrderIterItem<&CommitNode<J>>,
                _: Option<&Arc<ConstructNode<'brand, J>>>,
                _: &NoDisconnect,
            ) -> Result<Option<Arc<ConstructNode<'brand, J>>>, Self::Error> {
                Ok(None)
            }

            fn convert_data(
                &mut self,
                _: &PostOrderIterItem<&CommitNode<J>>,
                inner: Inner<
                    &Arc<ConstructNode<'brand, J>>,
                    J,
                    &Option<Arc<ConstructNode<'brand, J>>>,
                    &Option<Value>,
                >,
            ) -> Result<ConstructData<'brand, J>, Self::Error> {
                let inner = inner
                    .map(|node| node.arrow())
                    .map_disconnect(|maybe_node| maybe_node.as_ref().map(|node| node.arrow()));
                let inner = inner.disconnect_as_ref(); // lol sigh rust
                Ok(ConstructData::new(Arrow::from_inner(
                    self.inference_context,
                    inner,
                )?))
            }
        }

        self.convert::<MaxSharing<Commit<J>>, _, _>(&mut UnfinalizeTypes {
            inference_context,
            phantom: PhantomData,
        })
    }

    /// Decode a Simplicity program from bits, without witness data.
    ///
    /// # Usage
    ///
    /// Use this method only if the serialization **does not** include the witness data.
    /// This means, the program simply has no witness during commitment,
    /// or the witness is provided by other means.
    ///
    /// If the serialization contains the witness data, then use [`RedeemNode::decode()`].
    pub fn decode<I: Iterator<Item = u8>>(bits: BitIter<I>) -> Result<Arc<Self>, DecodeError> {
        use crate::decode;

        // 1. Decode program with out witnesses.
        let program = types::Context::with_context(|ctx| {
            let construct =
                crate::ConstructNode::decode(&ctx, bits).map_err(DecodeError::Decode)?;
            construct.finalize_types().map_err(DecodeError::Type)
        })?;
        // 2. Do sharing check, using incomplete IHRs
        if program.as_ref().is_shared_as::<MaxSharing<Commit<J>>>() {
            Ok(program)
        } else {
            Err(DecodeError::Decode(decode::Error::SharingNotMaximal))
        }
    }

    #[cfg(feature = "base64")]
    #[allow(clippy::should_implement_trait)] // returns Arc<Self>
    pub fn from_str(s: &str) -> Result<Arc<Self>, crate::ParseError> {
        use crate::base64::engine::general_purpose;
        use crate::base64::Engine as _;

        let v = general_purpose::STANDARD
            .decode(s)
            .map_err(crate::ParseError::Base64)?;
        let iter = crate::BitIter::new(v.into_iter());
        Self::decode(iter).map_err(crate::ParseError::Decode)
    }

    /// Encode a Simplicity expression to bits without any witness data
    #[deprecated(since = "0.5.0", note = "use Self::encode_without_witness instead")]
    pub fn encode<W: io::Write>(&self, w: &mut BitWriter<W>) -> io::Result<usize> {
        let program_bits = encode::encode_program(self, w)?;
        w.flush_all()?;
        Ok(program_bits)
    }

    /// Encode a Simplicity program to a vector of bytes, without any witness data.
    #[deprecated(since = "0.5.0", note = "use Self::to_vec_without_witness instead")]
    pub fn encode_to_vec(&self) -> Vec<u8> {
        let mut program = Vec::<u8>::new();
        self.encode_without_witness(&mut program)
            .expect("write to vector never fails");
        debug_assert!(!program.is_empty());

        program
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use hex::DisplayHex;
    use std::fmt;

    use crate::decode::Error;
    use crate::human_encoding::Forest;
    use crate::jet::Core;
    use crate::node::SimpleFinalizer;
    use crate::{BitMachine, Value};

    #[cfg_attr(not(feature = "base64"), allow(unused_variables))]
    #[track_caller]
    fn assert_program_deserializable<J: Jet>(
        prog_str: &str,
        prog_bytes: &[u8],
        cmr_str: &str,
        b64_str: &str,
    ) -> Arc<CommitNode<J>> {
        let forest = match Forest::<J>::parse(prog_str) {
            Ok(forest) => forest,
            Err(e) => panic!("Failed to parse program `{}`: {}", prog_str, e),
        };
        assert_eq!(
            forest.roots().len(),
            1,
            "program `{}` has multiple roots",
            prog_str
        );
        let main = match forest.roots().get("main") {
            Some(root) => root,
            None => panic!("Program `{}` has no main", prog_str),
        };

        let prog_hex = prog_bytes.as_hex();
        let main_bytes = main.to_vec_without_witness();
        assert_eq!(
            prog_bytes,
            main_bytes,
            "Program string `{}` encoded to {} (expected {})",
            prog_str,
            main_bytes.as_hex(),
            prog_hex,
        );

        let iter = BitIter::from(prog_bytes);
        let prog = match CommitNode::<J>::decode(iter) {
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

        let reser_sink = prog.to_vec_without_witness();
        assert_eq!(
            prog_bytes,
            &reser_sink[..],
            "program {} reserialized as {}",
            prog_hex,
            reser_sink.as_hex(),
        );

        #[cfg(feature = "base64")]
        {
            assert_eq!(prog.to_string(), b64_str);
            assert_eq!(prog.display().program().to_string(), b64_str);
            assert_eq!(prog, CommitNode::from_str(b64_str).unwrap());
        }

        prog
    }

    #[track_caller]
    fn assert_program_not_deserializable<J: Jet>(prog: &[u8], err: &dyn fmt::Display) {
        let prog_hex = prog.as_hex();
        let err_str = err.to_string();

        let iter = BitIter::from(prog);
        match CommitNode::<J>::decode(iter) {
            Ok(prog) => panic!(
                "Program {} succeded (expected error {}). Program parsed as:\n{:?}",
                prog_hex, err, prog
            ),
            Err(e) if e.to_string() == err_str => {} // ok
            Err(e) => panic!(
                "Program {} failed with error {} (expected error {})",
                prog_hex, e, err
            ),
        };
    }

    #[test]
    fn canonical_order() {
        // "main = comp unit iden", but with the iden serialized before the unit
        // To obtain this test vector I temporarily swapped `get_left` and `get_right`
        // in the implementation of `PostOrderIter`
        assert_program_not_deserializable::<Core>(&[0xa8, 0x48, 0x10], &Error::NotInCanonicalOrder);

        // "main = iden", but prefixed by some unused nodes, the first of which is also iden.
        assert_program_not_deserializable::<Core>(
            &[0xc1, 0x00, 0x06, 0x20],
            &Error::NotInCanonicalOrder,
        );
    }

    #[test]
    fn hidden_node() {
        // main = hidden deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef
        #[rustfmt::skip]
        let hidden = [
            0x36, 0xf5, 0x6d, 0xf7, 0x7e, 0xf5, 0x6d, 0xf7,
            0x7e, 0xf5, 0x6d, 0xf7, 0x7e, 0xf5, 0x6d, 0xf7,
            0x7e, 0xf5, 0x6d, 0xf7, 0x7e, 0xf5, 0x6d, 0xf7,
            0x7e, 0xf5, 0x6d, 0xf7, 0x7e, 0xf5, 0x6d, 0xf7,
            78,
        ];
        assert_program_not_deserializable::<Core>(&hidden, &Error::HiddenNode);

        // main = comp witness hidden deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef
        let hidden = [
            0xae, 0xdb, 0xd5, 0xb7, 0xdd, 0xfb, 0xd5, 0xb7, 0xdd, 0xfb, 0xd5, 0xb7, 0xdd, 0xfb,
            0xd5, 0xb7, 0xdd, 0xfb, 0xd5, 0xb7, 0xdd, 0xfb, 0xd5, 0xb7, 0xdd, 0xfb, 0xd5, 0xb7,
            0xdd, 0xfb, 0xd5, 0xb7, 0xdd, 0xe0, 0x80,
        ];
        assert_program_not_deserializable::<Core>(&hidden, &Error::HiddenNode);
    }

    #[test]
    fn case_both_children_hidden() {
        // h1 = hidden deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef
        // main = case h1 h1
        #[rustfmt::skip]
        let hidden = [
            0x8d, 0xbd, 0x5b, 0x7d, 0xdf, 0xbd, 0x5b, 0x7d,
            0xdf, 0xbd, 0x5b, 0x7d, 0xdf, 0xbd, 0x5b, 0x7d,
            0xdf, 0xbd, 0x5b, 0x7d, 0xdf, 0xbd, 0x5b, 0x7d,
            0xdf, 0xbd, 0x5b, 0x7d, 0xdf, 0xbd, 0x5b, 0x7d,
            0xde, 0x10,
        ];
        assert_program_not_deserializable::<Core>(&hidden, &Error::BothChildrenHidden);
    }

    #[test]
    fn unshared_hidden() {
        // This program has a repeated hidden node, but all other sharing is correct
        // and the order is canonical, etc.
        #[rustfmt::skip]
        let hidden = [
            0xd6, 0xe9, 0x62, 0x56, 0x62, 0xc9, 0x38, 0x8a,
            0x44, 0x31, 0x85, 0xee, 0xc2, 0x2b, 0x91, 0x48,
            0x87, 0xe1, 0xfd, 0x18, 0x57, 0xc2, 0x8c, 0x4a,
            0x28, 0x44, 0x2f, 0xa8, 0x61, 0x5c, 0xa7, 0x6e,
            0x8c, 0xf9, 0x80, 0xc2, 0x18, 0x95, 0x98, 0xb2,
            0x4e, 0x22, 0x91, 0x0c, 0x61, 0x7b, 0xb0, 0x8a,
            0xe4, 0x52, 0x21, 0xf8, 0x7f, 0x46, 0x15, 0xf0,
            0xa3, 0x12, 0x8a, 0x11, 0x0b, 0xea, 0x18, 0x57,
            0x29, 0xdb, 0xa3, 0x3e, 0x60, 0x30, 0x2c, 0x00,
            0xd0, 0x48, 0x20,
        ];
        assert_program_not_deserializable::<Core>(&hidden, &Error::SharingNotMaximal);
    }

    #[test]
    fn shared_witnesses() {
        assert_program_deserializable::<Core>(
            "main := witness",
            &[0x38],
            "a0fc8debd6796917c86b77aded82e6c61649889ae8f2ed65b57b41aa9d90e375",
            "OA==",
        );

        #[rustfmt::skip]
        let bad_diff1s = vec![
            // Above program, but with both witness nodes shared (note they have
            // the same type and CMR)
            vec![
                0xda, 0xe2, 0x39, 0xa3, 0x10, 0x42, 0x0e, 0x05,
                0x71, 0x88, 0xa3, 0x6d, 0xc4, 0x11, 0x80, 0x80
            ],
            // Same program but with each `witness` replaced by `comp iden witness`, which
            // is semantically the same but buries the offending witness nodes a bit to
            // trip up naive sharing logic.
            vec![
                0xde, 0x87, 0x04, 0x08, 0xe6, 0x8c, 0x41, 0x08,
                0x38, 0x15, 0xc6, 0x22, 0x8d, 0xb7, 0x10, 0x46,
                0x02, 0x00,
            ],
        ];
        for bad_diff1 in bad_diff1s {
            assert_program_not_deserializable::<Core>(&bad_diff1, &Error::SharingNotMaximal);
        }

        #[rustfmt::skip]
        let diff1s = vec![
            (
                // Sharing corrected
                "
                    -- Program which demands two 32-bit witnesses, the first one == the second + 1
                    wit1 := witness : 1 -> 2^32
                    wit2 := witness : 1 -> 2^32

                    wit_diff := comp (comp (pair wit1 wit2) jet_subtract_32) (drop iden) : 1 -> 2^32
                    diff_is_one := comp (pair wit_diff jet_one_32) jet_eq_32             : 1 -> 2
                    main := comp diff_is_one jet_verify                                  : 1 -> 1
                ",
                vec![
                    0xdc, 0xee, 0x28, 0xe6, 0x8c, 0x41, 0x08, 0x38,
                    0x15, 0xc6, 0x22, 0x8d, 0xb7, 0x10, 0x46, 0x02,
                    0x00,
                ],
                // CMR not checked against C code, since C won't give us any data without witnesses
                "e9339a0d715c721bff752aedc02710cdf3399f3f8d86e64456e85a1bc06ecb7c",
                "3O4o5oxBCDgVxiKNtxBGAgA=",
            ),
            // Same program but with each `witness` replaced by `comp iden witness`.
            (
                "
                    -- Program which demands two 32-bit witnesses, the first one == the second + 1
                    wit1 := witness : 1 -> 2^32
                    wit2 := witness : 1 -> 2^32
                    compwit1 := comp iden wit1
                    compwit2 := comp iden wit2

                    wit_diff := comp (comp (pair compwit1 compwit2) jet_subtract_32) (drop iden)
                    diff_is_one := comp (pair wit_diff jet_one_32) jet_eq_32             : 1 -> 2
                    main := comp diff_is_one jet_verify                                  : 1 -> 1
                ",
                vec![
                    0xe0, 0x28, 0x70, 0x43, 0x83, 0x00, 0xab, 0x9a,
                    0x31, 0x04, 0x20, 0xe0, 0x57, 0x18, 0x8a, 0x36,
                    0xdc, 0x41, 0x18, 0x08,
                ],
                // CMR not checked against C code, since C won't give us any data without witnesses
                "d03bf350f406aef3af0d48e6533b3325ff86f18a36e0e73895a5cd6d6692b860",
                "4ChwQ4MAq5oxBCDgVxiKNtxBGAg=",
            )
        ];

        for (prog_str, diff1, cmr, b64) in diff1s {
            let diff1_prog = crate::node::commit::tests::assert_program_deserializable::<Core>(
                prog_str, &diff1, cmr, b64,
            );

            // Attempt to finalize, providing 32-bit witnesses 0, 1, ..., and then
            // counting how many were consumed afterward.
            let mut counter = 0..100;
            let witness_iter = (&mut counter).rev().map(Value::u32);
            let diff1_final = diff1_prog
                .finalize(&mut SimpleFinalizer::new(witness_iter))
                .unwrap();
            assert_eq!(counter, 0..98);

            // Execute the program to confirm that it worked
            let mut mac =
                BitMachine::for_program(&diff1_final).expect("program has reasonable bounds");
            mac.exec(&diff1_final, &crate::jet::CoreEnv::EMPTY).unwrap();
        }
    }

    #[test]
    fn extra_nodes() {
        // main = comp unit unit # but with an extra unconnected `unit` stuck on the beginning
        // I created this unit test by hand
        assert_program_not_deserializable::<Core>(&[0xa9, 0x48, 0x00], &Error::NotInCanonicalOrder);
    }

    #[test]
    fn regression_177() {
        // `case (drop iden) iden` from upstream occurs-check test. Has an infinitely sized
        // input type. Will fail trying to unify the input type with the unit type before
        // doing the occurs check, putting an infinitely-sized type into the error variant.
        //
        // The human-readable encoding will keep going and then also hit the occurs check.
        //
        // Check that both error types can be generated and printed in finite space/time.
        let bad_prog = "
            id := iden
            main := case (drop id) id
        ";
        match Forest::<Core>::parse(bad_prog) {
            Ok(_) => panic!("program should have failed"),
            Err(set) => {
                let mut errs_happened = (false, false);
                for err in set.iter() {
                    match err {
                        crate::human_encoding::Error::TypeCheck(e @ types::Error::Bind { .. }) => {
                            errs_happened.0 = true;
                            e.to_string();
                        }
                        crate::human_encoding::Error::TypeCheck(
                            e @ types::Error::OccursCheck { .. },
                        ) => {
                            errs_happened.1 = true;
                            e.to_string();
                        }
                        x => panic!("unexpected error {x:?}"),
                    }
                }
                assert_eq!(errs_happened, (true, true));
            }
        };
    }
}
