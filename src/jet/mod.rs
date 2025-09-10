// SPDX-License-Identifier: CC0-1.0

//! # Simplicity jets
//!
//! Jets are special nodes that read a value,
//! process it internally, and write an output value.
//! This evaluation happens in a black-box manner:
//! In terms of the Bit Machine, it is a one-step process.
//!
//! In practice, jets call foreign C code that is equivalent to some Simplicity DAG.
//! This speeds up evaluation tremendously.
//! Equivalence of C and Simplicity is proved using the _Verified Software Toolchain_.
//! Programs are also smaller in size because jets replace large, equivalent Simplicity DAGs.

#[cfg(feature = "bitcoin")]
pub mod bitcoin;
mod core;
#[cfg(feature = "elements")]
pub mod elements;
mod init;
pub mod type_name;

pub use self::core::CoreEnv;
#[cfg(feature = "bitcoin")]
pub use init::bitcoin::Bitcoin;
pub use init::core::Core;
#[cfg(feature = "elements")]
pub use init::elements::Elements;
use simplicity_sys::c_jets::frame_ffi::CFrameItem;

use crate::analysis::Cost;
use crate::decode;
use crate::jet::type_name::TypeName;
use crate::merkle::cmr::Cmr;
use crate::{BitIter, BitWriter};
use std::borrow::Borrow;
use std::hash::Hash;
use std::io::Write;

/// Generic error that a jet failed during its execution.
///
/// Failure could be due to a failed assertion, an illegal input, etc.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct JetFailed;

impl std::fmt::Display for JetFailed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("Jet failed during execution")
    }
}

impl std::error::Error for JetFailed {}

/// Family of jets that share an encoding scheme and execution environment.
///
/// Jets are single nodes that read an input,
/// process it internally using foreign C code _(black box)_,
/// and produce an output.
/// Jets may read values from their _environment_.
///
/// Jets are **always** leaves in a Simplicity DAG.
pub trait Jet:
    Copy + Eq + Ord + Hash + std::fmt::Debug + std::fmt::Display + std::str::FromStr + 'static
{
    type Transaction;
    /// Environment for jet to read from
    type Environment<T> where T: Borrow<Self::Transaction>;
    /// CJetEnvironment to interact with C FFI.
    type CJetEnvironment;

    /// Return the CMR of the jet.
    fn cmr(&self) -> Cmr;

    /// Return the source type of the jet.
    fn source_ty(&self) -> TypeName;

    /// Return the target type of the jet.
    fn target_ty(&self) -> TypeName;

    /// Encode the jet to bits.
    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> std::io::Result<usize>;

    /// Decode a jet from bits.
    fn decode<I: Iterator<Item = u8>>(bits: &mut BitIter<I>) -> Result<Self, decode::Error>;

    /// Obtains a C FFI compatible environment for the jet.
    fn c_jet_env<T>(env: &Self::Environment<T>) -> &Self::CJetEnvironment
        where T: Borrow<Self::Transaction>;

    /// Obtain the FFI C pointer for the jet.
    fn c_jet_ptr(&self) -> &dyn Fn(&mut CFrameItem, CFrameItem, &Self::CJetEnvironment) -> bool;

    /// Return the cost of the jet.
    fn cost(&self) -> Cost;
}

#[cfg(test)]
mod tests {
    use crate::jet::{Core, CoreEnv};
    use crate::node::{ConstructNode, CoreConstructible, JetConstructible};
    use crate::types;
    use crate::value::Word;
    use crate::{BitMachine, Value};
    use std::sync::Arc;

    #[test]
    fn test_ffi_jet() {
        types::Context::with_context(|ctx| {
            let two_words = Arc::<ConstructNode<_>>::comp(
                &Arc::<ConstructNode<_>>::pair(
                    &Arc::<ConstructNode<_>>::const_word(&ctx, Word::u32(2)),
                    &Arc::<ConstructNode<_>>::const_word(&ctx, Word::u32(16)),
                )
                .unwrap(),
                &Arc::<ConstructNode<_>>::jet(&ctx, Core::Add32),
            )
            .unwrap();
            assert_eq!(
                BitMachine::test_exec(two_words, &CoreEnv::EMPTY).expect("executing"),
                Value::product(
                    Value::u1(0),       // carry bit
                    Value::u32(2 + 16), // result
                ),
            );
        });
    }

    #[test]
    fn test_simple() {
        types::Context::with_context(|ctx| {
            let two_words = Arc::<ConstructNode<Core>>::pair(
                &Arc::<ConstructNode<_>>::const_word(&ctx, Word::u32(2)),
                &Arc::<ConstructNode<_>>::const_word(&ctx, Word::u16(16)),
            )
            .unwrap();
            assert_eq!(
                BitMachine::test_exec(two_words, &CoreEnv::EMPTY).expect("executing"),
                Value::product(Value::u32(2), Value::u16(16)),
            );
        });
    }
}
