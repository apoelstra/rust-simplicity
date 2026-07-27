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
pub mod core;
#[cfg(feature = "elements")]
pub mod elements;
mod init;
mod sha256;
pub mod type_name;

pub use self::core::CoreEnv;
#[cfg(feature = "bitcoin")]
pub use crate::jet::bitcoin::BitcoinEnv;
#[cfg(feature = "elements")]
pub use elements::ElementsTxEnv;

#[cfg(feature = "bitcoin")]
pub use init::bitcoin::Bitcoin;
pub use init::core::Core;
#[cfg(feature = "elements")]
pub use init::elements::Elements;
pub use sha256::Sha256Jet;
use simplicity_sys::c_jets::frame_ffi::CFrameItem;

use crate::analysis::Cost;
use crate::decode;
use crate::jet::type_name::TypeName;
use crate::merkle::cmr::Cmr;
use crate::{BitIter, BitWriter};
use std::any::{Any, TypeId};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
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

/// An environment for jets to read.
pub trait JetEnvironment {
    /// The type of jet that this environment supports.
    type Jet: Jet;

    /// CJetEnvironment to interact with C FFI.
    type CJetEnvironment;

    /// Obtains a C FFI compatible environment for the jet.
    fn c_jet_env(&self) -> &Self::CJetEnvironment;

    /// Obtain the FFI C pointer for the jet.
    fn c_jet_ptr(
        jet: &Self::Jet,
    ) -> fn(&mut CFrameItem, CFrameItem, &Self::CJetEnvironment) -> bool;
}

/// Family of jets that share an encoding scheme and execution environment.
///
/// Jets are single nodes that read an input,
/// process it internally using foreign C code _(black box)_,
/// and produce an output.
/// Jets may read values from their _environment_.
///
/// Jets are **always** leaves in a Simplicity DAG.
pub trait Jet: DynJet + std::fmt::Debug + std::fmt::Display + Send + Sync + 'static {
    /// Return the CMR of the jet.
    fn cmr(&self) -> Cmr;

    /// Return the source type of the jet.
    fn source_ty(&self) -> TypeName;

    /// Return the target type of the jet.
    fn target_ty(&self) -> TypeName;

    /// Encode the jet to bits.
    fn encode(&self, w: &mut BitWriter<&mut dyn Write>) -> std::io::Result<usize>;

    /// Decode a jet from bits.
    fn decode<I: Iterator<Item = u8>>(bits: &mut BitIter<I>) -> Result<Self, decode::Error>
    where
        Self: Sized;

    /// Return the cost of the jet.
    fn cost(&self) -> Cost;

    /// Parse a jet from a string.
    fn parse(s: &str) -> Result<Self, crate::Error>
    where
        Self: Sized;
}

/// Lets `Box<dyn Jet>` work with `Clone`, `Eq`, `Ord`, and `Hash`.
pub trait DynJet {
    fn as_any(&self) -> &dyn Any;
    fn dyn_clone(&self) -> Box<dyn Jet>;
    fn dyn_eq(&self, other: &dyn Jet) -> bool;
    fn dyn_cmp(&self, other: &dyn Jet) -> Ordering;
    fn dyn_hash(&self, state: &mut dyn Hasher);
}

impl<T: Jet + Clone + Ord + Hash> DynJet for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dyn_clone(&self) -> Box<dyn Jet> {
        Box::new(self.clone())
    }

    fn dyn_eq(&self, other: &dyn Jet) -> bool {
        other
            .as_any()
            .downcast_ref::<Self>()
            .is_some_and(|other| self == other)
    }

    fn dyn_cmp(&self, other: &dyn Jet) -> Ordering {
        match other.as_any().downcast_ref::<Self>() {
            Some(other) => self.cmp(other),
            None => TypeId::of::<Self>().cmp(&other.as_any().type_id()),
        }
    }

    fn dyn_hash(&self, mut state: &mut dyn Hasher) {
        self.hash(&mut state)
    }
}

impl Clone for Box<dyn Jet> {
    fn clone(&self) -> Self {
        (**self).dyn_clone()
    }
}

impl PartialEq for Box<dyn Jet> {
    fn eq(&self, other: &Self) -> bool {
        (**self).dyn_eq(&**other)
    }
}

impl Eq for Box<dyn Jet> {}

impl PartialOrd for Box<dyn Jet> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Box<dyn Jet> {
    fn cmp(&self, other: &Self) -> Ordering {
        (**self).dyn_cmp(&**other)
    }
}

impl Hash for Box<dyn Jet> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (**self).dyn_hash(state)
    }
}

#[cfg(test)]
mod tests {
    use crate::jet::{Core, CoreEnv};
    use crate::node::{ConstructNode, CoreConstructible as _};
    use crate::types;
    use crate::value::Word;
    use crate::{BitMachine, Value};
    use std::sync::Arc;

    #[test]
    fn test_ffi_jet() {
        types::Context::with_context(|ctx| {
            let two_words = Arc::<ConstructNode>::comp(
                &Arc::<ConstructNode>::pair(
                    &Arc::<ConstructNode>::const_word(&ctx, Word::u32(2)),
                    &Arc::<ConstructNode>::const_word(&ctx, Word::u32(16)),
                )
                .unwrap(),
                &Arc::<ConstructNode>::jet(&ctx, &Core::Add32),
            )
            .unwrap();
            assert_eq!(
                BitMachine::test_exec(two_words, &CoreEnv::new()).expect("executing"),
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
            let two_words = Arc::<ConstructNode>::pair(
                &Arc::<ConstructNode>::const_word(&ctx, Word::u32(2)),
                &Arc::<ConstructNode>::const_word(&ctx, Word::u16(16)),
            )
            .unwrap();
            assert_eq!(
                BitMachine::test_exec(two_words, &CoreEnv::new()).expect("executing"),
                Value::product(Value::u32(2), Value::u16(16)),
            );
        });
    }
}
