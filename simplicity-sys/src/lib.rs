// SPDX-License-Identifier: CC0-1.0

pub mod c_jets;
pub use c_jets::CFrameItem;
pub use c_jets::{bitcoin, elements};

// Temporary to keep the Haskell-generated code compiling
pub use c_jets::elements::CTxEnv as CElementsTxEnv;

pub mod alloc;
pub mod ffi;
#[cfg(feature = "test-utils")]
pub mod tests;
