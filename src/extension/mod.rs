// Rust Simplicity Library
// Written in 2020 by
//   Andrew Poelstra <apoelstra@blockstream.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! # Extensions
//!
//! Extensions to the Simplicity language needed for blockchain support
//!

pub mod dummy;
#[cfg(feature = "bitcoin")]
pub mod bitcoin;
pub mod jets;

use std::{fmt, io};

use {encode, exec};
use bititer::BitIter;
use cmr::Cmr;
use Error;

#[cfg(not(feature = "bitcoin"))]
pub use self::dummy as bitcoin;

/// Types used by Bitcoin/Elements primitives and hardcoded jets
///
/// This enum is needed since the type inference engine does not have direct
/// access to Bitcoin/Elements node types (these do not even exist without
/// the correct feature flag. Therefore we need accessors in this module which
/// can introspect the node and determine the correct source/target type.
///
/// On the other hand, we cannot use the actual `Type` from type.rs for this
/// purpose because this is private to the type inference module.
pub enum TypeName {
    One,
    Word32,
    SWord32,
    TwoTimesWord32,
    Word64,
    SWord64,
    Word64TimesTwo,
    Word128,
    Word256,
    SWord256,
    Word256Word32,
    SWord256Word32,
    Word256Word512,
}

/// Trait representing an extension (Bitcoin or Elements) to Simplicity
pub trait Node: Sized + fmt::Display {
    /// Transaction environment
    type TxEnv;

    /// Decode a node from a bit iterator
    fn decode<I: Iterator<Item = u8>>(
        iter: &mut BitIter<I>,
    ) -> Result<Self, Error>;

    /// Encode a node into a bit writer
    fn encode<W: encode::BitWrite>(&self, w: &mut W) -> io::Result<usize>;

    /// Execute the node in a Bit Machine; assuming the surrounding
    /// program has typechecked, this cannot fail
    fn exec(&self, &mut exec::BitMachine, txenv: &Self::TxEnv);

    /// Return the CMR of the node
    fn cmr(&self) -> Cmr;

    /// The name of the source type of this node
    fn source_type(&self) -> TypeName;

    /// The name of the target type of this node
    fn target_type(&self) -> TypeName;
}

