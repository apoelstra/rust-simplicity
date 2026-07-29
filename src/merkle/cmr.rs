// SPDX-License-Identifier: CC0-1.0

use crate::jet::Jet;
#[cfg(feature = "elements")]
use crate::node::{CoreConstructible, DisconnectConstructible, NoDisconnect, WitnessConstructible};
use crate::types::{self, Arrow, Error};
use crate::value::Word;
use crate::{FailEntropy, Tmr};
use hashes::sha256::Midstate;

use super::{bip340_iv, impl_mr_type, MidstateExt as _};

impl_mr_type! {
    /// Commitment Merkle root
    ///
    /// A Merkle root that commits to a node's combinator and recursively its children.
    ///
    /// Importantly, witness data and right disconnect branches are _not_ included in the hash.
    /// This makes these elements malleable while preserving program identity (SegWit, delegation).
    ///
    /// Uniquely identifies a program's structure in terms of combinators at commitment time.
    #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct Cmr([u8; 32]);
}

impl From<Tmr> for Cmr {
    fn from(tmr: Tmr) -> Self {
        Cmr::from_byte_array(tmr.to_byte_array())
    }
}

impl Cmr {
    /// Produce a CMR for an iden combinator
    pub const fn iden() -> Self {
        Self::from_byte_array(Self::IDEN_IV.to_parts().0)
    }

    /// Produce a CMR for a unit combinator
    pub const fn unit() -> Self {
        Self::from_byte_array(Self::UNIT_IV.to_parts().0)
    }

    /// Produce a CMR for an injl combinator
    pub fn injl(child: Cmr) -> Self {
        Self::INJL_IV.update_0_then_32(child).into_merkle_root()
    }

    /// Produce a CMR for an injr combinator
    pub fn injr(child: Cmr) -> Self {
        Self::INJR_IV.update_0_then_32(child).into_merkle_root()
    }

    /// Produce a CMR for a take combinator
    pub fn take(child: Cmr) -> Self {
        Self::TAKE_IV.update_0_then_32(child).into_merkle_root()
    }

    /// Produce a CMR for a drop combinator
    pub fn drop(child: Cmr) -> Self {
        Self::DROP_IV.update_0_then_32(child).into_merkle_root()
    }

    /// Produce a CMR for a comp combinator
    pub fn comp(left: Cmr, right: Cmr) -> Self {
        Self::COMP_IV.zz_update_2x32(left, right).into_merkle_root()
    }

    /// Produce a CMR for a case combinator
    pub fn case(left: Cmr, right: Cmr) -> Self {
        Self::CASE_IV.zz_update_2x32(left, right).into_merkle_root()
    }

    /// Produce a CMR for a pair combinator
    pub fn pair(left: Cmr, right: Cmr) -> Self {
        Self::PAIR_IV.zz_update_2x32(left, right).into_merkle_root()
    }

    /// Produce a CMR for a disconnect combinator
    pub fn disconnect(left: Cmr) -> Self {
        Self::DISCONNECT_IV
            .update_0_then_32(left)
            .into_merkle_root()
    }

    /// Produce a CMR for a witness combinator
    pub const fn witness() -> Self {
        Self::from_byte_array(Self::WITNESS_IV.to_parts().0)
    }

    /// Produce a CMR for a fail combinator
    pub fn fail(entropy: FailEntropy) -> Self {
        Self::FAIL_IV
            .zz_update_64(entropy.to_byte_array())
            .into_merkle_root()
    }

    /// Produce a CMR for a jet
    pub fn jet(jet: &dyn Jet) -> Self {
        jet.cmr()
    }

    /// Compute the CMR of a constant word jet
    ///
    /// This is equal to the IHR of the equivalent scribe, converted to a CMR in
    /// the usual way for jets.
    pub fn const_word(word: &Word) -> Self {
        let w = 1 + word.n();

        let mut cmr_stack = Vec::with_capacity(33);
        // 1. Compute the CMR for the `scribe` corresponding to this word jet
        for (bit_idx, bit) in word.iter().enumerate() {
            cmr_stack.push(Cmr::BITS[usize::from(bit)]);
            let mut j = bit_idx;
            while j & 1 == 1 {
                let right_cmr = cmr_stack.pop().unwrap();
                let left_cmr = cmr_stack.pop().unwrap();
                cmr_stack.push(Cmr::pair(left_cmr, right_cmr));
                j >>= 1;
            }
        }
        assert_eq!(cmr_stack.len(), 1);

        let ihr_iv = Self::CONST_WORD_IV;
        let ihr_pass1 = ihr_iv.update_0_then_32(cmr_stack[0]);
        // 2. Add TMRs to get the pass-two IHR
        let ihr_pass2 = ihr_pass1.zz_update_2x32(Tmr::unit(), Tmr::TWO_TWO_N[w - 1]);
        // 3. Convert to a jet CMR
        bip340_iv(b"Simplicity\x1fJet")
            .update_weight_then_32(word.len() as u64, ihr_pass2.to_parts().0)
            .into_merkle_root()
    }

    #[rustfmt::skip]
    const UNIT_IV: Midstate = Midstate::new([
        0xc4, 0x0a, 0x10, 0x26, 0x3f, 0x74, 0x36, 0xb4,
        0x16, 0x0a, 0xcb, 0xef, 0x1c, 0x36, 0xfb, 0xa4,
        0xbe, 0x4d, 0x95, 0xdf, 0x18, 0x1a, 0x96, 0x8a,
        0xfe, 0xab, 0x5e, 0xac, 0x24, 0x7a, 0xdf, 0xf7,
    ], 64);

    #[rustfmt::skip]
    const IDEN_IV: Midstate = Midstate::new([
        0x54, 0x1a, 0x1a, 0x69, 0xbd, 0x4b, 0xcb, 0xda,
        0x7f, 0x34, 0x31, 0x0e, 0x30, 0x78, 0xf7, 0x26,
        0x44, 0x31, 0x22, 0xfb, 0xcc, 0x1c, 0xb5, 0x36,
        0x0c, 0x78, 0x64, 0xec, 0x0d, 0x32, 0x3a, 0xc0,
    ], 64);

    #[rustfmt::skip]
    const INJL_IV: Midstate = Midstate::new([
        0x54, 0xe9, 0x1d, 0x18, 0xd8, 0xf8, 0x1f, 0x6d,
        0x29, 0x86, 0xbb, 0x58, 0x47, 0x9a, 0x54, 0xeb,
        0x63, 0x0e, 0x95, 0x23, 0xb6, 0x9e, 0xe8, 0x53,
        0x29, 0x80, 0xd0, 0x55, 0x58, 0x19, 0x4f, 0x15,
    ], 64);

    #[rustfmt::skip]
    const INJR_IV: Midstate = Midstate::new([
        0xd7, 0x0f, 0xfd, 0xce, 0x97, 0x77, 0x7b, 0x4d,
        0xfe, 0x31, 0xfd, 0x9f, 0xf5, 0xd0, 0x17, 0xa6,
        0x30, 0x5d, 0x7e, 0xc6, 0x0d, 0xf3, 0xb1, 0xbf,
        0x6d, 0x25, 0xe8, 0x16, 0x33, 0xde, 0xd4, 0xbf,
    ], 64);

    #[rustfmt::skip]
    const TAKE_IV: Midstate = Midstate::new([
        0x50, 0x5f, 0xc0, 0x81, 0xb5, 0xba, 0x2a, 0xcd,
        0x09, 0x50, 0x67, 0xc3, 0xdf, 0xb8, 0xea, 0x12,
        0x6f, 0xa1, 0x5d, 0x55, 0xcb, 0x21, 0x1e, 0x6a,
        0xed, 0x34, 0xe8, 0xd1, 0xe3, 0x7a, 0xf0, 0xfa,
    ], 64);

    #[rustfmt::skip]
    const DROP_IV: Midstate = Midstate::new([
        0x8a, 0x30, 0x8d, 0x38, 0xa1, 0x13, 0xa2, 0x60,
        0xb4, 0xc7, 0x14, 0x5a, 0xbd, 0xc5, 0x22, 0x4d,
        0xeb, 0x70, 0x13, 0x79, 0x59, 0x0e, 0x0c, 0x8c,
        0x38, 0x86, 0x0b, 0xab, 0x12, 0x71, 0xa8, 0xa8,
    ], 64);

    #[rustfmt::skip]
    const COMP_IV: Midstate = Midstate::new([
        0x57, 0xec, 0x23, 0xa2, 0xa4, 0x77, 0x8e, 0x01,
        0x58, 0xa6, 0x21, 0x7a, 0xea, 0x3e, 0xf7, 0x42,
        0x8b, 0xa0, 0x90, 0x92, 0x73, 0xb9, 0x73, 0xfa,
        0x14, 0x32, 0xa9, 0x27, 0x84, 0x3e, 0x92, 0x7a,
    ], 64);

    #[rustfmt::skip]
    const CASE_IV: Midstate = Midstate::new([
        0x29, 0x5e, 0x2a, 0x6d, 0xc8, 0xc5, 0xce, 0x59,
        0xe4, 0xed, 0xcf, 0xe9, 0xb4, 0xd8, 0xf7, 0x64,
        0x13, 0x3a, 0xa5, 0x51, 0x4b, 0xd3, 0xee, 0x8b,
        0x4b, 0x75, 0xec, 0x8f, 0x4d, 0xeb, 0x08, 0xbe,
    ], 64);

    #[rustfmt::skip]
    const PAIR_IV: Midstate = Midstate::new([
        0x7d, 0x5e, 0x6d, 0xac, 0x15, 0xb1, 0x42, 0x8a,
        0x0d, 0x26, 0x0c, 0x94, 0x29, 0xdb, 0xe8, 0x89,
        0x65, 0x93, 0xf3, 0x1f, 0x70, 0x86, 0x27, 0xee,
        0x75, 0xb2, 0x7e, 0xee, 0xfd, 0xd0, 0x50, 0x05,
    ], 64);

    #[rustfmt::skip]
    const DISCONNECT_IV: Midstate = Midstate::new([
        0x35, 0x33, 0x8b, 0x5b, 0x81, 0x74, 0x0c, 0x6d,
        0x67, 0xdc, 0x1e, 0xa3, 0xc8, 0x31, 0xe4, 0xc0,
        0xaf, 0xd8, 0x64, 0x09, 0xbc, 0x04, 0xd0, 0xdd,
        0x43, 0x24, 0xb7, 0xd9, 0xd5, 0x83, 0xf4, 0xeb,
    ], 64);

    #[rustfmt::skip]
    const WITNESS_IV: Midstate = Midstate::new([
        0xa0, 0xfc, 0x8d, 0xeb, 0xd6, 0x79, 0x69, 0x17,
        0xc8, 0x6b, 0x77, 0xad, 0xed, 0x82, 0xe6, 0xc6,
        0x16, 0x49, 0x88, 0x9a, 0xe8, 0xf2, 0xed, 0x65,
        0xb5, 0x7b, 0x41, 0xaa, 0x9d, 0x90, 0xe3, 0x75,
    ], 64);

    #[rustfmt::skip]
    const FAIL_IV: Midstate = Midstate::new([
        0x22, 0x83, 0xc1, 0x81, 0x9e, 0x69, 0x2f, 0x96,
        0x85, 0xfe, 0x95, 0x40, 0x76, 0xc5, 0x16, 0x7c,
        0x03, 0xbd, 0xe7, 0xcc, 0xda, 0xab, 0x00, 0x5e,
        0x55, 0x36, 0x12, 0x2e, 0x18, 0xf7, 0x23, 0x7a,
    ], 64);

    #[rustfmt::skip]
    const CONST_WORD_IV: Midstate = Midstate::new([
        0x0c, 0x5b, 0xc1, 0xce, 0xc8, 0xf1, 0x41, 0x85,
        0x0e, 0x33, 0x3e, 0x28, 0xea, 0x01, 0xc0, 0x5a,
        0xc6, 0x42, 0xeb, 0x30, 0xb4, 0x9e, 0x69, 0x2c,
        0x56, 0xb1, 0x22, 0xbb, 0x69, 0x49, 0xc5, 0xcd,
    ], 64);

    /// CMRs for the bits 0 and 1 -- injl(unit) and injr(unit) respectively
    #[rustfmt::skip]
    const BITS: [Cmr; 2] = [
        Cmr([
            0x88, 0x81, 0xaf, 0xf5, 0x16, 0x0c, 0xc0, 0xc9,
            0xf8, 0xec, 0xea, 0xd8, 0xb4, 0x01, 0xfa, 0x97,
            0xee, 0xf5, 0xfc, 0x60, 0x75, 0x2e, 0x98, 0xd2,
            0x47, 0x56, 0x1a, 0x4d, 0xa6, 0xce, 0x96, 0x5e,
        ]),
        Cmr([
            0xa0, 0x43, 0x8b, 0x72, 0x36, 0x48, 0x72, 0x7b,
            0x3f, 0x2d, 0x18, 0x5f, 0xcd, 0x95, 0x69, 0xe0,
            0x22, 0xa4, 0x47, 0x8e, 0xb2, 0x5f, 0xdf, 0xa5,
            0x38, 0xea, 0xc5, 0x9d, 0x81, 0x7c, 0x31, 0x1c,
        ]),
    ];
}

/// Wrapper around a CMR which allows it to be constructed with the
/// `*Constructible*` traits, allowing CMRs to be computed using the
/// same generic construction code that nodes are.
#[cfg(feature = "elements")] // only used by policy module
pub struct ConstructibleCmr<'brand> {
    pub cmr: Cmr,
    pub arrow: Arrow<'brand>,
}

#[cfg(feature = "elements")] // only used by policy module
impl<'brand> CoreConstructible<'brand> for ConstructibleCmr<'brand> {
    fn iden(inference_context: &types::Context<'brand>) -> Self {
        ConstructibleCmr {
            cmr: Cmr::iden(),
            arrow: Arrow::iden(inference_context),
        }
    }

    fn unit_from_arrow(arrow: Arrow<'brand>) -> Self {
        ConstructibleCmr {
            cmr: Cmr::unit(),
            arrow,
        }
    }

    fn injl(child: &Self) -> Self {
        ConstructibleCmr {
            cmr: Cmr::injl(child.cmr),
            arrow: Arrow::injl(child.arrow()),
        }
    }

    fn injr(child: &Self) -> Self {
        ConstructibleCmr {
            cmr: Cmr::injr(child.cmr),
            arrow: Arrow::injr(child.arrow()),
        }
    }

    fn take(child: &Self) -> Self {
        ConstructibleCmr {
            cmr: Cmr::take(child.cmr),
            arrow: Arrow::take(child.arrow()),
        }
    }

    fn drop_(child: &Self) -> Self {
        ConstructibleCmr {
            cmr: Cmr::drop(child.cmr),
            arrow: Arrow::drop_(child.arrow()),
        }
    }

    fn comp_from_arrow(left: &Self, right: &Self, arrow: Arrow<'brand>) -> Self {
        ConstructibleCmr {
            cmr: Cmr::comp(left.cmr, right.cmr),
            arrow,
        }
    }

    fn case(left: &Self, right: &Self) -> Result<Self, Error> {
        Ok(ConstructibleCmr {
            cmr: Cmr::case(left.cmr, right.cmr),
            arrow: Arrow::case(left.arrow(), right.arrow())?,
        })
    }

    fn assertl(left: &Self, right: Cmr) -> Result<Self, Error> {
        Ok(ConstructibleCmr {
            cmr: Cmr::case(left.cmr, right),
            arrow: Arrow::assertl(left.arrow())?,
        })
    }

    fn assertr(left: Cmr, right: &Self) -> Result<Self, Error> {
        Ok(ConstructibleCmr {
            cmr: Cmr::case(left, right.cmr),
            arrow: Arrow::assertr(right.arrow())?,
        })
    }

    fn pair_from_arrow(left: &Self, right: &Self, arrow: Arrow<'brand>) -> Self {
        ConstructibleCmr {
            cmr: Cmr::pair(left.cmr, right.cmr),
            arrow,
        }
    }

    fn fail(inference_context: &types::Context<'brand>, entropy: FailEntropy) -> Self {
        ConstructibleCmr {
            cmr: Cmr::fail(entropy),
            arrow: Arrow::fail(inference_context),
        }
    }

    fn const_word(inference_context: &types::Context<'brand>, word: Word) -> Self {
        ConstructibleCmr {
            cmr: Cmr::const_word(&word),
            arrow: Arrow::const_word(inference_context, &word),
        }
    }

    fn jet(inference_context: &types::Context<'brand>, jet: &dyn Jet) -> Self {
        ConstructibleCmr {
            cmr: jet.cmr(),
            arrow: Arrow::jet(inference_context, jet),
        }
    }

    fn arrow(&self) -> &Arrow<'brand> {
        &self.arrow
    }
}

#[cfg(feature = "elements")] // only used by policy module
impl<'brand, X> DisconnectConstructible<'brand, X> for ConstructibleCmr<'brand> {
    // Specifically with disconnect we don't check for consistency between the
    // type inference context of the disconnected node, if any, and that of
    // the left node. The idea is, from the point of view of (Constructible)Cmr,
    // the right child of disconnect doesn't even exist.
    fn disconnect(left: &Self, _right: &X) -> Result<Self, Error> {
        Ok(ConstructibleCmr {
            cmr: Cmr::disconnect(left.cmr),
            arrow: Arrow::disconnect(&left.arrow, &NoDisconnect)?,
        })
    }
}

#[cfg(feature = "elements")] // only used by policy module
impl<'brand, W> WitnessConstructible<'brand, W> for ConstructibleCmr<'brand> {
    fn witness(inference_context: &types::Context<'brand>, _: W) -> Self {
        ConstructibleCmr {
            arrow: Arrow::witness(inference_context),
            cmr: Cmr::witness(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::node::{ConstructNode, CoreConstructible};
    use crate::types;

    use std::str::FromStr;
    use std::sync::Arc;

    #[test]
    fn cmr_display_unit() {
        types::Context::with_context(|ctx| {
            let c = Arc::<ConstructNode>::unit(&ctx);

            assert_eq!(
                c.cmr().to_string(),
                "c40a10263f7436b4160acbef1c36fba4be4d95df181a968afeab5eac247adff7"
            );
            assert_eq!(format!("{:.8}", c.cmr()), "c40a1026");

            assert_eq!(
                Cmr::from_str("c40a10263f7436b4160acbef1c36fba4be4d95df181a968afeab5eac247adff7"),
                Ok(c.cmr()),
            );
        });
    }

    #[test]
    fn fixed_const_word_cmr() {
        // Checked against C implementation
        let bit0 = Word::u1(0);
        #[rustfmt::skip]
        assert_eq!(
            Cmr::const_word(&bit0),
            Cmr::from_str("a51cfd799d0bc368f48208032fc3881953f35aa7fd2b985cb237cbad143e30d2").unwrap(),
        );
    }

    #[test]
    fn bit_cmr() {
        types::Context::with_context(|ctx| {
            let unit = Arc::<ConstructNode>::unit(&ctx);
            let bit0 = Arc::<ConstructNode>::injl(&unit);
            assert_eq!(bit0.cmr(), Cmr::BITS[0]);

            let bit1 = Arc::<ConstructNode>::injr(&unit);
            assert_eq!(bit1.cmr(), Cmr::BITS[1]);
        });
    }

    #[test]
    fn ivs() {
        fn check_iv(target: Midstate, s: &'static str) {
            let name = s
                .trim_start_matches("Simplicity\x1f")
                .strip_prefix("Commitment\x1f")
                .unwrap_or("const_word");
            // Uncomment this if the IVs ever change
            /*
            let target = Cmr(bip340_iv(s.as_bytes()));
            println!("    #[rustfmt::skip]");
            println!("    const {}_IV: Midstate = Midstate::new([", name.to_ascii_uppercase());
            print!("       "); for ch in &target.0[0..8] { print!(" 0x{:02x},", ch); }; println!();
            print!("       "); for ch in &target.0[8..16] { print!(" 0x{:02x},", ch); }; println!();
            print!("       "); for ch in &target.0[16..24] { print!(" 0x{:02x},", ch); }; println!();
            print!("       "); for ch in &target.0[24..32] { print!(" 0x{:02x},", ch); }; println!();
            println!("    ], 64);");
            println!();
            */
            assert_eq!(
                target,
                bip340_iv(s.as_bytes()),
                "mismatch on IV for {}",
                name
            );
        }

        check_iv(Cmr::UNIT_IV, "Simplicity\x1fCommitment\x1funit");
        check_iv(Cmr::IDEN_IV, "Simplicity\x1fCommitment\x1fiden");
        check_iv(Cmr::INJL_IV, "Simplicity\x1fCommitment\x1finjl");
        check_iv(Cmr::INJR_IV, "Simplicity\x1fCommitment\x1finjr");
        check_iv(Cmr::TAKE_IV, "Simplicity\x1fCommitment\x1ftake");
        check_iv(Cmr::DROP_IV, "Simplicity\x1fCommitment\x1fdrop");
        check_iv(Cmr::COMP_IV, "Simplicity\x1fCommitment\x1fcomp");
        check_iv(Cmr::CASE_IV, "Simplicity\x1fCommitment\x1fcase");
        check_iv(Cmr::PAIR_IV, "Simplicity\x1fCommitment\x1fpair");
        check_iv(Cmr::DISCONNECT_IV, "Simplicity\x1fCommitment\x1fdisconnect");
        check_iv(Cmr::WITNESS_IV, "Simplicity\x1fCommitment\x1fwitness");
        check_iv(Cmr::FAIL_IV, "Simplicity\x1fCommitment\x1ffail");
        check_iv(Cmr::CONST_WORD_IV, "Simplicity\x1fIdentity");
    }

    #[test]
    fn const_bits() {
        /// The scribe expression, as defined in the Simplicity tech report.
        fn scribe<'brand>(ctx: &types::Context<'brand>, bit: u8) -> Arc<ConstructNode<'brand>> {
            match bit {
                0 => Arc::<ConstructNode>::injl(&Arc::<ConstructNode>::unit(ctx)),
                1 => Arc::<ConstructNode>::injr(&Arc::<ConstructNode>::unit(ctx)),
                _ => panic!("Unexpected bit {bit}"),
            }
        }

        fn check_bit(target: Cmr, index: u8) {
            // Uncomment this if the IVs ever change
            /*
            let target = scribe(index).cmr();
            println!("    Cmr(Midstate([");
            print!("       "); for ch in &target.0[0..8] { print!(" 0x{:02x},", ch); }; println!();
            print!("       "); for ch in &target.0[8..16] { print!(" 0x{:02x},", ch); }; println!();
            print!("       "); for ch in &target.0[16..24] { print!(" 0x{:02x},", ch); }; println!();
            print!("       "); for ch in &target.0[24..32] { print!(" 0x{:02x},", ch); }; println!();
            println!("    ])),");
            */
            types::Context::with_context(|ctx| {
                assert_eq!(
                    target,
                    scribe(&ctx, index).cmr(),
                    "mismatch on CMR for bit {index}",
                );
            });
        }

        for index in 0..2u8 {
            check_bit(Cmr::BITS[usize::from(index)], index)
        }
    }
}
