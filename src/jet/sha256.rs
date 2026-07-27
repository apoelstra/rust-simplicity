// SPDX-License-Identifier: CC0-1.0

use super::{Core, Jet};

/// A type of jet capable of SHA256 calculations.
pub trait Sha256Jet {
    /// The `Low1` (false) jet.
    fn low_1() -> &'static dyn Jet;

    /// The `High1` (true) jet.
    fn high_1() -> &'static dyn Jet;

    /// The `Sha256Block` jet.
    fn sha256_block() -> &'static dyn Jet;
}

/// We have identical trait implementations for the three jet types, so we invoke this macro,
/// with different type aliases in scope, which always generates identical code for the type
/// alias.
macro_rules! impl_sha256jet_for_JetTy {
    () => {
        impl Sha256Jet for JetTy {
            fn low_1() -> &'static dyn Jet {
                &Self::Low1
            }

            fn high_1() -> &'static dyn Jet {
                &Self::High1
            }

            fn sha256_block() -> &'static dyn Jet {
                &Self::Sha256Block
            }
        }
    };
}
use impl_sha256jet_for_JetTy;

#[cfg(feature = "bitcoin")]
mod bitcoin {
    use super::{Jet, Sha256Jet};

    type JetTy = super::super::Bitcoin;
    super::impl_sha256jet_for_JetTy!();
}

mod core {
    use super::{Jet, Sha256Jet};

    type JetTy = super::Core;
    super::impl_sha256jet_for_JetTy!();
}

#[cfg(feature = "elements")]
mod elements {
    use super::{Jet, Sha256Jet};

    type JetTy = super::super::Elements;
    super::impl_sha256jet_for_JetTy!();
}
