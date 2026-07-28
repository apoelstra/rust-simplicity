// SPDX-License-Identifier: CC0-1.0

//! Simplicity Program Nodes
//!
//! The types in this module are used to represent Simplicity expressions as
//! DAGs. The nodes of this DAG are individual combinators along with some
//! cached data about the node, which depends on the specific node type.
//!
//! All nodes represent the root of an expression. Expressions whose source
//! and target types are both unit are called "programs". Generally speaking,
//! any nodes that users hold directly will be programs.
//!
//! There are three main node types:
//!
//! 1. [`crate::RedeemNode`] represents a Simplicity node as it exists on the blockchain.
//!    Every witness node is populated with a correctly-typed [`Value`], every
//!    disconnect node has a child, every non-executed branch of a `case` combinator
//!    is pruned, and there is nothing you can do to modify the expression.
//!
//!    `RedeemNode`s can be executed on the bit machine and have complete types
//!    and resource bounds, and can be serialized in the consensus bit-encoding.
//!
//! 2. [`CommitNode`] represents a Simplicity node as it is *committed to* on
//!    the blockchain. This means that witness and (TODO) disconnect nodes are not
//!    populated, but type inference is complete and resource bounds are
//!    available. `case` combinators may have pruned children (in which case they
//!    are instead considered `assertl` or `assertr` combinators), or not.
//!
//!    There is a bit-encoding for `CommitNode`s which is essentially only used
//!    by this library. It consists of the bit-encoding.of the combinators, fully
//!    shared *except* that witness and disconnect nodes (and their ancestors)
//!    are unshared. No witness data is included.
//!
//!    TODO there is also a human-readable encoding.
//!
//! 3. [`ConstructNode`] represents an "under-construction" Simplicity expression.
//!    These nodes' types are not necessarily complete, and are inferred as the
//!    program is constructed. This is the only node that you can programmatically
//!    construct. It has no encoding, human-readable or bitwise, and is intended
//!    to exist only ephemerally.
//!
//! The following conversions are possible between the node types:
//!
//! 1. [`ConstructNode::finalize_types`] converts a [`ConstructNode`] to a
//!    [`CommitNode`] by setting any free variables, as well as the source and
//!    target of the root node, to unit.
//!
//!    This conversion requires no input from the user but may fail with a type
//!    error, in case of infinitely-sized types or in the case that the unit
//!    bounds cannot be applied.
//!
//! 2. [`CommitNode::finalize`] converts a [`CommitNode`] to a [`RedeemNode`]
//!    by attaching witnesses to each witness node, and deciding whether to hide
//!    branches for each `case` node.
//!
//! 3. [`CommitNode::unfinalize_types`] converts a [`CommitNode`] to a
//!    [`ConstructNode`] by throwing away all types and re-inferring them. It
//!    cannot fail.
//!
//! 4. [`RedeemNode::unfinalize`] converts a [`RedeemNode`] to a [`CommitNode`]
//!    by throwing away witness and (TODO) disconnect data. It cannot recover
//!    pruned branches so is of limited usefulness, but it is included for
//!    completeness.
//!

use crate::dag::{DagLike, MaxSharing, SharingTracker};
use crate::encode;
use crate::jet::Jet;
use crate::types::Arrow;
use crate::{types, BitWriter, Cmr, FailEntropy, HasCmr, Value};

use std::sync::Arc;
use std::{fmt, hash, io};

mod commit;
mod construct;
mod convert;
mod disconnect;
mod display;
mod hiding;
mod inner;
mod redeem;
pub mod type_promise;

use crate::value::Word;
pub use commit::{Commit, CommitData, CommitNode};
pub use construct::{Construct, ConstructData, ConstructNode};
pub use convert::{Converter, Hide, SimpleFinalizer};
pub use disconnect::{Disconnectable, NoDisconnect};
pub use display::{Display, DisplayAsGraph, DisplayExpr, GraphFormat, SharingLevel};
pub use hiding::Hiding;
pub use inner::Inner;
pub use redeem::{Redeem, RedeemData, RedeemNode};

/// Lots of existing code imports [`JetConstructible`] just to get the `jet` method.
/// Almost all such code also imports [`CoreConstructible`], which now gives them
/// this method, so this code will continue to work as long as there is something
/// called [`JetConstructible`] to import.
#[doc(hidden)]
#[deprecated(note = "subsumed by CoreConstructible")]
pub enum JetConstructible {}

// This trait should only be implemented on empty types, so we can demand
// every trait bound under the sun. Doing so will make #[derive]s easier
// for downstream users.
pub trait Marker:
    Copy + Clone + PartialEq + Eq + PartialOrd + Ord + fmt::Debug + hash::Hash
{
    /// Precomputed data about the node, such as its type arrow or various Merkle roots.
    type CachedData: Clone;

    /// Type of witness data attached to DAGs of this node type. Typically either [`Value`]
    /// or [`NoWitness`].
    type Witness: Clone;

    /// Type of disconnect data attached to DAGs of this node type.
    type Disconnect: Disconnectable<Node<Self>> + Clone;

    /// A type which uniquely identifies a node, for purposes of sharing
    /// during iteration over the DAG.
    type SharingId: hash::Hash + Clone + Eq;

    /// Yields the sharing ID for a given type, starting from its CMR and its cached data.
    ///
    /// If the type cannot be uniquely identified (e.g. because it is missing data), then
    /// this method returns `None`. In this case, the node will not be shared with any
    /// other node.
    fn compute_sharing_id(cmr: Cmr, cached_data: &Self::CachedData) -> Option<Self::SharingId>;
}

/// Null data type used as dummy for [`Marker::Witness`]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct NoWitness;

impl From<NoWitness> for Option<Value> {
    fn from(_: NoWitness) -> Self {
        None
    }
}

pub trait Constructible<'brand, X, W>:
    DisconnectConstructible<'brand, X>
    + WitnessConstructible<'brand, W>
    + CoreConstructible<'brand>
    + Sized
{
    fn from_inner(
        inference_context: &types::Context<'brand>,
        inner: Inner<&Self, &X, W>,
    ) -> Result<Self, types::Error> {
        match inner {
            Inner::Iden => Ok(Self::iden(inference_context)),
            Inner::Unit => Ok(Self::unit(inference_context)),
            Inner::InjL(child) => Ok(Self::injl(child)),
            Inner::InjR(child) => Ok(Self::injr(child)),
            Inner::Take(child) => Ok(Self::take(child)),
            Inner::Drop(child) => Ok(Self::drop_(child)),
            Inner::Comp(left, right) => Self::comp(left, right),
            Inner::Case(left, right) => Self::case(left, right),
            Inner::AssertL(left, r_cmr) => Self::assertl(left, r_cmr),
            Inner::AssertR(l_cmr, right) => Self::assertr(l_cmr, right),
            Inner::Pair(left, right) => Self::pair(left, right),
            Inner::Disconnect(left, right) => Self::disconnect(left, right),
            Inner::Fail(entropy) => Ok(Self::fail(inference_context, entropy)),
            Inner::Word(ref w) => Ok(Self::const_word(inference_context, w.shallow_clone())),
            Inner::Jet(j) => Ok(Self::jet(inference_context, j.as_ref())),
            Inner::Witness(w) => Ok(Self::witness(inference_context, w)),
        }
    }
}

impl<'brand, X, W, T> Constructible<'brand, X, W> for T where
    T: DisconnectConstructible<'brand, X>
        + WitnessConstructible<'brand, W>
        + CoreConstructible<'brand>
        + Sized
{
}

pub trait CoreConstructible<'brand>: Sized {
    fn unit_from_arrow(arrow: Arrow<'brand>) -> Self;
    fn comp_from_arrow(left: &Self, right: &Self, arrow: Arrow<'brand>) -> Self;
    fn pair_from_arrow(left: &Self, right: &Self, arrow: Arrow<'brand>) -> Self;

    fn iden(inference_context: &types::Context<'brand>) -> Self;

    fn unit(inference_context: &types::Context<'brand>) -> Self {
        Self::unit_from_arrow(Arrow::unit(inference_context))
    }

    fn injl(child: &Self) -> Self;
    fn injr(child: &Self) -> Self;
    fn take(child: &Self) -> Self;
    fn drop_(child: &Self) -> Self;

    fn comp(left: &Self, right: &Self) -> Result<Self, types::Error> {
        let arrow = Arrow::comp(left.arrow(), right.arrow())?;
        Ok(Self::comp_from_arrow(left, right, arrow))
    }

    fn case(left: &Self, right: &Self) -> Result<Self, types::Error>;
    fn assertl(left: &Self, right: Cmr) -> Result<Self, types::Error>;
    fn assertr(left: Cmr, right: &Self) -> Result<Self, types::Error>;

    fn pair(left: &Self, right: &Self) -> Result<Self, types::Error> {
        let arrow = Arrow::pair(left.arrow(), right.arrow())?;
        Ok(Self::pair_from_arrow(left, right, arrow))
    }

    fn fail(inference_context: &types::Context<'brand>, entropy: FailEntropy) -> Self;
    fn const_word(inference_context: &types::Context<'brand>, word: Word) -> Self;
    fn jet(inference_context: &types::Context<'brand>, jet: &dyn Jet) -> Self;

    /// Accessor for the type inference context used to create the object.
    #[inline]
    fn inference_context(&self) -> &types::Context<'brand> {
        &self.arrow().inference_context
    }
    fn arrow(&self) -> &types::Arrow<'brand>;

    /// Create an expression that produces the given `value`.
    ///
    /// The expression is minimized by using as many word jets as possible.
    fn scribe(ctx: &types::Context<'brand>, value: &Value) -> Self {
        use crate::value::ValueRef;

        #[derive(Debug, Clone)]
        enum Task<'v> {
            Process(ValueRef<'v>),
            MakeLeft,
            MakeRight,
            MakeProduct,
        }

        let mut input = vec![Task::Process(value.as_ref())];
        let mut output = vec![];
        while let Some(top) = input.pop() {
            match top {
                Task::Process(value) => {
                    if value.is_unit() {
                        output.push(Self::unit(ctx));
                    } else if let Some(word) = value.to_word() {
                        output.push(Self::const_word(ctx, word));
                    } else if let Some(left) = value.as_left() {
                        input.push(Task::MakeLeft);
                        input.push(Task::Process(left));
                    } else if let Some(right) = value.as_right() {
                        input.push(Task::MakeRight);
                        input.push(Task::Process(right));
                    } else if let Some((left, right)) = value.as_product() {
                        input.push(Task::MakeProduct);
                        input.push(Task::Process(right));
                        input.push(Task::Process(left));
                    }
                }
                Task::MakeLeft => {
                    let inner = output.pop().unwrap();
                    output.push(Self::injl(&inner));
                }
                Task::MakeRight => {
                    let inner = output.pop().unwrap();
                    output.push(Self::injr(&inner));
                }
                Task::MakeProduct => {
                    let right = output.pop().unwrap();
                    let left = output.pop().unwrap();
                    // simfony::PairBuilder would remove this `.expect()` call
                    output.push(
                        Self::pair(&left, &right).expect(
                            "`pair` should always succeed because input type is unrestricted",
                        ),
                    );
                }
            }
        }
        debug_assert_eq!(output.len(), 1);
        output.pop().unwrap()
    }

    /// Create a DAG that takes any input and returns bit `0` as constant output.
    ///
    /// _Overall type: A → 2_
    fn bit_false(inference_context: &types::Context<'brand>) -> Self {
        let unit = Self::unit(inference_context);
        Self::injl(&unit)
    }

    /// Create a DAG that takes any input and returns bit `1` as constant output.
    ///
    /// _Overall type: A → 2_
    fn bit_true(inference_context: &types::Context<'brand>) -> Self {
        let unit = Self::unit(inference_context);
        Self::injr(&unit)
    }

    /// Create a DAG that takes a bit and an input,
    /// such that the `left` child is evaluated on the input if the bit is `1` _(if branch)_
    /// and the `right` child is evaluated on the input otherwise _(else branch)_.
    ///
    /// _Overall type: 2 × A → B where `left`: A → B and `right`: A → B_
    ///
    /// _Type inference will fail if children are not of the correct type._
    fn cond(left: &Self, right: &Self) -> Result<Self, types::Error> {
        let drop_left = Self::drop_(left);
        let drop_right = Self::drop_(right);
        Self::case(&drop_right, &drop_left)
    }

    /// Create a DAG that asserts that its child returns `true`, and fails otherwise.
    /// The `hash` identifies the assertion and is returned upon failure.
    ///
    /// _Overall type: A → 1 where `child`: A → 2_
    ///
    /// _Type inference will fail if children are not of the correct type._
    fn assert(child: &Self, hash: Cmr) -> Result<Self, types::Error> {
        let unit = Self::unit(child.inference_context());
        let pair_child_unit = Self::pair(child, &unit)?;
        let assertr_hidden_unit = Self::assertr(hash, &unit)?;

        Self::comp(&pair_child_unit, &assertr_hidden_unit)
    }

    /// Create a DAG that computes Boolean _NOT_ of the `child`.
    ///
    /// _Overall type: A → 2 where `child`: A → 2_
    ///
    /// _Type inference will fail if children are not of the correct type._
    #[allow(clippy::should_implement_trait)]
    fn not(child: &Self) -> Result<Self, types::Error> {
        let unit = Self::unit(child.inference_context());
        let pair_child_unit = Self::pair(child, &unit)?;
        let bit_true = Self::bit_true(child.inference_context());
        let bit_false = Self::bit_false(child.inference_context());
        let case_true_false = Self::case(&bit_true, &bit_false)?;

        Self::comp(&pair_child_unit, &case_true_false)
    }

    /// Create a DAG that computes Boolean _AND_ of the `left` and `right` child.
    ///
    /// _Overall type: A → 2 where `left`: A → 2 and `right`: A → 2_
    ///
    /// _Type inference will fail if children are not of the correct type._
    fn and(left: &Self, right: &Self) -> Result<Self, types::Error> {
        left.inference_context()
            .check_eq(right.inference_context())?;
        let iden = Self::iden(left.inference_context());
        let pair_left_iden = Self::pair(left, &iden)?;
        let bit_false = Self::bit_false(left.inference_context());
        let drop_right = Self::drop_(right);
        let case_false_right = Self::case(&bit_false, &drop_right)?;

        Self::comp(&pair_left_iden, &case_false_right)
    }

    /// Create a DAG that computes Boolean _OR_ of the `left` and `right`.
    ///
    /// _Overall type: A → 2 where `left`: A → 2 and `right`: A → 2_
    ///
    /// _Type inference will fail if children are not of the correct type._
    fn or(left: &Self, right: &Self) -> Result<Self, types::Error> {
        left.inference_context()
            .check_eq(right.inference_context())?;
        let iden = Self::iden(left.inference_context());
        let pair_left_iden = Self::pair(left, &iden)?;
        let drop_right = Self::drop_(right);
        let bit_true = Self::bit_true(left.inference_context());
        let case_right_true = Self::case(&drop_right, &bit_true)?;

        Self::comp(&pair_left_iden, &case_right_true)
    }
}

pub trait DisconnectConstructible<'brand, X>: Sized {
    fn disconnect(left: &Self, right: &X) -> Result<Self, types::Error>;
}

pub trait WitnessConstructible<'brand, W>: Sized {
    fn witness(inference_context: &types::Context<'brand>, witness: W) -> Self;
}

/// A node in a Simplicity expression.
///
/// There are three node types provided by this library: `ConstructNode`, `CommitNode`,
/// and `RedeemNode`, which represent Simplicty programs during construction, at
/// commitment time, and at redemption time, respectively.
///
/// This generic structure is used to define conversions and mapping functions over
/// nodes and DAGs, and allows users to define their own custom node types.
///
/// For equality and hashing purposes, nodes are characterized entirely by their
/// CMR and cached data. Users who create custom nodes should define a custom type
/// for [`Marker::CachedData`] and think carefully about whether and how to
/// implement the [`std::hash::Hash`] or equality traits.
pub struct Node<N: Marker> {
    inner: Inner<Arc<Node<N>>, N::Disconnect, N::Witness>,
    cmr: Cmr,
    data: N::CachedData,
}

impl<N: Marker> PartialEq for Node<N>
where
    N::CachedData: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.cmr == other.cmr && self.data == other.data
    }
}
impl<N: Marker> Eq for Node<N> where N::CachedData: Eq {}

impl<N: Marker> hash::Hash for Node<N>
where
    N::CachedData: hash::Hash,
{
    fn hash<H: hash::Hasher>(&self, h: &mut H) {
        self.cmr.hash(h);
        self.data.hash(h);
    }
}

impl<N: Marker> fmt::Debug for Node<N>
where
    for<'a> &'a Node<N>: DagLike,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.post_order_iter::<MaxSharing<N>>().into_display(
            f,
            |node, f| fmt::Display::fmt(&node.inner, f),
            |_, _| Ok(()),
        )
    }
}

#[cfg(feature = "base64")]
impl<N: Marker> fmt::Display for Node<N> {
    /// Displays the program data in base64.
    ///
    /// If you need witness data as well, then use [`Node::display`].
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::base64::display::Base64Display;
        use crate::base64::engine::general_purpose;

        let v = self.to_vec_without_witness();
        Base64Display::new(&v, &general_purpose::STANDARD).fmt(f)
    }
}

impl<N: Marker> HasCmr for Node<N> {
    fn cmr(&self) -> Cmr {
        self.cmr
    }
}

impl<N: Marker> HasCmr for Arc<Node<N>> {
    fn cmr(&self) -> Cmr {
        self.cmr
    }
}

impl<'brand, N> CoreConstructible<'brand> for Arc<Node<N>>
where
    N: Marker,
    N::CachedData: CoreConstructible<'brand>,
{
    fn comp_from_arrow(left: &Self, right: &Self, arrow: Arrow<'brand>) -> Self {
        Arc::new(Node {
            cmr: Cmr::comp(left.cmr(), right.cmr()),
            data: N::CachedData::comp_from_arrow(left.cached_data(), right.cached_data(), arrow),
            inner: Inner::Comp(Arc::clone(left), Arc::clone(right)),
        })
    }

    fn pair_from_arrow(left: &Self, right: &Self, arrow: Arrow<'brand>) -> Self {
        Arc::new(Node {
            cmr: Cmr::pair(left.cmr(), right.cmr()),
            data: N::CachedData::pair_from_arrow(left.cached_data(), right.cached_data(), arrow),
            inner: Inner::Pair(Arc::clone(left), Arc::clone(right)),
        })
    }

    fn iden(inference_context: &types::Context<'brand>) -> Self {
        Arc::new(Node {
            cmr: Cmr::iden(),
            data: N::CachedData::iden(inference_context),
            inner: Inner::Iden,
        })
    }

    fn unit_from_arrow(arrow: Arrow<'brand>) -> Self {
        Arc::new(Node {
            cmr: Cmr::unit(),
            data: N::CachedData::unit_from_arrow(arrow),
            inner: Inner::Unit,
        })
    }

    fn injl(child: &Self) -> Self {
        Arc::new(Node {
            cmr: Cmr::injl(child.cmr()),
            data: N::CachedData::injl(&child.data),
            inner: Inner::InjL(Arc::clone(child)),
        })
    }

    fn injr(child: &Self) -> Self {
        Arc::new(Node {
            cmr: Cmr::injr(child.cmr()),
            data: N::CachedData::injr(&child.data),
            inner: Inner::InjR(Arc::clone(child)),
        })
    }

    fn take(child: &Self) -> Self {
        Arc::new(Node {
            cmr: Cmr::take(child.cmr()),
            data: N::CachedData::take(&child.data),
            inner: Inner::Take(Arc::clone(child)),
        })
    }

    fn drop_(child: &Self) -> Self {
        Arc::new(Node {
            cmr: Cmr::drop(child.cmr()),
            data: N::CachedData::drop_(&child.data),
            inner: Inner::Drop(Arc::clone(child)),
        })
    }

    fn case(left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(Arc::new(Node {
            cmr: Cmr::case(left.cmr(), right.cmr()),
            data: N::CachedData::case(&left.data, &right.data)?,
            inner: Inner::Case(Arc::clone(left), Arc::clone(right)),
        }))
    }

    fn assertl(left: &Self, r_cmr: Cmr) -> Result<Self, types::Error> {
        Ok(Arc::new(Node {
            cmr: Cmr::case(left.cmr(), r_cmr),
            data: N::CachedData::assertl(&left.data, r_cmr)?,
            inner: Inner::AssertL(Arc::clone(left), r_cmr),
        }))
    }

    fn assertr(l_cmr: Cmr, right: &Self) -> Result<Self, types::Error> {
        Ok(Arc::new(Node {
            cmr: Cmr::case(l_cmr, right.cmr()),
            data: N::CachedData::assertr(l_cmr, &right.data)?,
            inner: Inner::AssertR(l_cmr, Arc::clone(right)),
        }))
    }

    fn fail(inference_context: &types::Context<'brand>, entropy: FailEntropy) -> Self {
        Arc::new(Node {
            cmr: Cmr::fail(entropy),
            data: N::CachedData::fail(inference_context, entropy),
            inner: Inner::Fail(entropy),
        })
    }

    fn const_word(inference_context: &types::Context<'brand>, word: Word) -> Self {
        Arc::new(Node {
            cmr: Cmr::const_word(&word),
            data: N::CachedData::const_word(inference_context, word.shallow_clone()),
            inner: Inner::Word(word),
        })
    }

    fn jet(inference_context: &types::Context<'brand>, jet: &dyn Jet) -> Self {
        Arc::new(Node {
            cmr: Cmr::jet(jet),
            data: N::CachedData::jet(inference_context, jet),
            inner: Inner::Jet(jet.dyn_clone()),
        })
    }

    fn arrow(&self) -> &Arrow<'brand> {
        self.data.arrow()
    }
}

impl<'brand, N> DisconnectConstructible<'brand, N::Disconnect> for Arc<Node<N>>
where
    N: Marker,
    N::CachedData: DisconnectConstructible<'brand, N::Disconnect>,
{
    fn disconnect(left: &Self, right: &N::Disconnect) -> Result<Self, types::Error> {
        Ok(Arc::new(Node {
            cmr: Cmr::disconnect(left.cmr()),
            data: N::CachedData::disconnect(&left.data, right)?,
            inner: Inner::Disconnect(Arc::clone(left), right.clone()),
        }))
    }
}

impl<'brand, N> WitnessConstructible<'brand, N::Witness> for Arc<Node<N>>
where
    N: Marker,
    N::CachedData: WitnessConstructible<'brand, N::Witness>,
{
    fn witness(inference_context: &types::Context<'brand>, value: N::Witness) -> Self {
        Arc::new(Node {
            cmr: Cmr::witness(),
            data: N::CachedData::witness(inference_context, value.clone()),
            inner: Inner::Witness(value),
        })
    }
}

impl<N: Marker> Node<N> {
    /// Accessor for the node's "inner value", i.e. its combinator
    pub fn inner(&self) -> &Inner<Arc<Node<N>>, N::Disconnect, N::Witness> {
        &self.inner
    }

    /// Accessor for the node's CMR
    pub fn cmr(&self) -> Cmr {
        self.cmr
    }

    /// Accessor for the node's cached data
    pub fn sharing_id(&self) -> Option<N::SharingId> {
        N::compute_sharing_id(self.cmr, &self.data)
    }

    /// Accessor for the node's cached data
    pub fn cached_data(&self) -> &N::CachedData {
        &self.data
    }

    /// Contruct a node from its constituent parts.
    ///
    /// This method can be used to directly costruct a node. It will compute the CMR
    /// automatically based on the value of `inner` but requires that `cached_data`
    /// be provided.
    ///
    /// If available, [`Constructible`] and its dependent traits will be easier to
    /// use.
    pub fn from_parts(
        inner: Inner<Arc<Self>, N::Disconnect, N::Witness>,
        data: N::CachedData,
    ) -> Self {
        let cmr = match &inner {
            Inner::Unit => Cmr::unit(),
            Inner::Iden => Cmr::iden(),
            Inner::InjL(ref c) => Cmr::injl(c.cmr()),
            Inner::InjR(ref c) => Cmr::injr(c.cmr()),
            Inner::Take(ref c) => Cmr::take(c.cmr()),
            Inner::Drop(ref c) => Cmr::drop(c.cmr()),
            Inner::Comp(ref cl, ref cr) => Cmr::comp(cl.cmr(), cr.cmr()),
            Inner::Case(ref cl, ref cr) => Cmr::case(cl.cmr(), cr.cmr()),
            Inner::AssertL(ref c, cmr) => Cmr::case(c.cmr(), *cmr),
            Inner::AssertR(cmr, ref c) => Cmr::case(*cmr, c.cmr()),
            Inner::Pair(ref cl, ref cr) => Cmr::pair(cl.cmr(), cr.cmr()),
            Inner::Disconnect(ref cl, _) => Cmr::disconnect(cl.cmr()),
            Inner::Witness(_) => Cmr::witness(),
            Inner::Fail(entropy) => Cmr::fail(*entropy),
            Inner::Jet(j) => Cmr::jet(j.as_ref()),
            Inner::Word(ref w) => Cmr::const_word(w),
        };

        Node { cmr, inner, data }
    }

    /// Generic conversion function from one type of node to another, with the
    /// ability to prune during the conversion.
    ///
    /// Parameterized over what kind of sharing to use when iterating over the
    /// DAG, and what conversion logic to use.
    ///
    /// See the documentation for [`Converter`] for details.
    pub fn convert<S, M, C>(&self, converter: &mut C) -> Result<Arc<Node<M>>, C::Error>
    where
        S: for<'a> SharingTracker<&'a Self> + Default,
        M: Marker,
        C: Converter<N, M>,
    {
        let mut converted: Vec<Arc<Node<M>>> = vec![];
        for data in self.post_order_iter::<S>() {
            // First, tell the converter about the iterator state..
            converter.visit_node(&data);

            // Construct an Inner<usize> where pointers are replaced by indices.
            // Note that `map_left_right`'s internal logic will ensure that these
            // `unwrap`s are only called when they will succeed.
            let indexed_inner: Inner<usize, &N::Disconnect, &N::Witness> = data
                .node
                .inner
                .as_ref()
                .map_left_right(|_| data.left_index.unwrap(), |_| data.right_index.unwrap());

            // Then, convert witness data, if this is a witness node.
            let witness_inner: Inner<&usize, &&N::Disconnect, M::Witness> = indexed_inner
                .as_ref()
                .map_witness_result(|wit| converter.convert_witness(&data, wit))?;

            // Then convert disconnect nodes data.
            let maybe_converted = data.right_index.map(|idx| &converted[idx]);
            let witness_inner: Inner<&usize, M::Disconnect, M::Witness> = witness_inner
                .map_disconnect_result(|disc| {
                    converter.convert_disconnect(&data, maybe_converted, disc)
                })?;

            // Then put the converted nodes in place (it's easier to do this in this
            // order because of the way the reference types work out).
            let converted_inner: Inner<Arc<Node<M>>, M::Disconnect, M::Witness> =
                witness_inner.map(|idx| Arc::clone(&converted[*idx]));

            // Next, prune case nodes into asserts, if applicable
            let pruned_inner = if let Inner::Case(left, right) = converted_inner {
                let hide = converter.prune_case(&data, &left, &right)?;

                match hide {
                    Hide::Neither => Inner::Case(left, right),
                    Hide::Left => Inner::AssertR(left.cmr(), right),
                    Hide::Right => Inner::AssertL(left, right.cmr()),
                }
            } else {
                converted_inner
            };

            // Finally, construct the node
            converted.push(Arc::new(Node {
                data: converter.convert_data(&data, pruned_inner.as_ref())?,
                cmr: data.node.cmr,
                inner: pruned_inner,
            }));
        }
        Ok(converted.pop().unwrap())
    }

    pub fn display(&self) -> Display<'_, N> {
        Display::from(self)
    }

    /// Display the Simplicity expression as a linear string.
    ///
    /// The linear string has no sharing and may be **exponentially larger**
    /// than the originally shared expression!
    pub fn display_expr(&self) -> DisplayExpr<'_, N> {
        DisplayExpr::from(self)
    }

    /// Display the Simplicity expression as a graph in the given format and sharing level.
    ///
    /// This is the general form of [`display_as_dot`](Node::display_as_dot) and
    /// [`display_as_mermaid`](Node::display_as_mermaid). Use those convenience methods for
    /// the common case of DOT or Mermaid output with no sharing.
    ///
    /// The `format` field of the returned [`DisplayAsGraph`] can be changed after construction,
    /// and the [`fmt::Display`] impl will use whatever `format` and `sharing` are set at that
    /// point. See also [`DisplayAsGraph::to_dot_string`] and [`DisplayAsGraph::to_mermaid_string`]
    /// to render to a specific format regardless of the stored `format` field.
    pub fn display_as_graph(
        &self,
        format: GraphFormat,
        sharing_level: SharingLevel,
    ) -> DisplayAsGraph<'_, N> {
        DisplayAsGraph::new(self, format, sharing_level)
    }

    /// Display the Simplicity expression as a Graphviz DOT graph.
    ///
    /// The DOT output can be rendered with `dot -Tsvg` or similar tools.
    /// Shared nodes appear once in the graph with multiple incoming edges.
    pub fn display_as_dot(&self) -> DisplayAsGraph<'_, N> {
        DisplayAsGraph::new(self, GraphFormat::Dot, SharingLevel::None)
    }

    /// Display the Simplicity expression as a Mermaid diagram.
    ///
    /// The Mermaid output can be rendered in Markdown or the Mermaid live editor.
    /// Shared nodes appear once in the diagram with multiple incoming edges.
    pub fn display_as_mermaid(&self) -> DisplayAsGraph<'_, N> {
        DisplayAsGraph::new(self, GraphFormat::Mermaid, SharingLevel::None)
    }

    /// Encode a Simplicity expression to bits without any witness data.
    pub fn encode_without_witness(&self, prog: &mut dyn io::Write) -> io::Result<usize> {
        let mut w = BitWriter::new(prog);
        let program_bits = encode::encode_program(self, &mut w)?;
        w.flush_all()?;
        Ok(program_bits)
    }

    /// Encode a Simplicity expression to a vector of bytes, without any witness data.
    pub fn to_vec_without_witness(&self) -> Vec<u8> {
        let mut program = Vec::<u8>::new();
        self.encode_without_witness(&mut program)
            .expect("Vec::write is infallible");
        debug_assert!(!program.is_empty());
        program
    }
}

impl<N: Marker<Witness = Value>> Node<N> {
    /// Encode the program and witness data to bits.
    ///
    /// Returns the number of written bits for the program and witness, respectively.
    pub fn encode_with_witness(
        &self,
        prog: &mut dyn io::Write,
        witness: &mut dyn io::Write,
    ) -> io::Result<(usize, usize)> {
        let mut prog = BitWriter::new(prog);
        let mut witness = BitWriter::new(witness);

        let sharing_iter = self.post_order_iter::<MaxSharing<N>>();
        let program_bits = encode::encode_program(self, &mut prog)?;
        prog.flush_all()?;
        let witness_bits = encode::encode_witness(sharing_iter.into_witnesses(), &mut witness)?;
        witness.flush_all()?;
        Ok((program_bits, witness_bits))
    }

    /// Encode the program and witness data to byte vectors.
    pub fn to_vec_with_witness(&self) -> (Vec<u8>, Vec<u8>) {
        let mut ret_1 = vec![];
        let mut ret_2 = vec![];
        self.encode_with_witness(&mut ret_1, &mut ret_2)
            .expect("Vec::write is infallible");
        (ret_1, ret_2)
    }
}

impl<N: Marker> Drop for Node<N> {
    fn drop(&mut self) {
        // Note: this is basically identical to the drop impl for types::Incomplete.
        fn push_children<N: Marker>(
            stack: &mut Vec<Arc<Node<N>>>,
            inner: Inner<Arc<Node<N>>, N::Disconnect, N::Witness>,
        ) {
            use crate::dag::Dag;
            match inner.into_dag() {
                Dag::Nullary => {}
                Dag::Unary(child) => stack.push(child),
                Dag::Binary(left, right) => {
                    stack.push(left);
                    stack.push(right);
                }
            }
        }

        let mut stack = Vec::new();
        push_children(&mut stack, std::mem::replace(&mut self.inner, Inner::Unit));
        while let Some(child) = stack.pop() {
            if let Some(mut child) = Arc::into_inner(child) {
                push_children(&mut stack, std::mem::replace(&mut child.inner, Inner::Unit));
            }
        }
    }
}

#[cfg(test)]
#[cfg(all(feature = "test-utils", feature = "elements"))]
mod tests {
    use ffi::tests::TestData;

    use crate::analysis::Cost;
    use crate::ffi;
    use crate::jet::Elements;
    use crate::BitIter;
    use crate::RedeemNode;

    fn check_merkle_roots(test: &TestData) {
        let prog = BitIter::from(test.prog.as_slice());
        let witness = BitIter::from(test.witness.as_slice());
        ffi::tests::run_program(
            &test.prog,
            &test.witness,
            ffi::tests::TestUpTo::CheckOneOne,
            None,
            None,
        )
        .unwrap();
        let prog = RedeemNode::decode::<_, _, Elements>(prog, witness).unwrap();
        assert_eq!(prog.cmr().to_byte_array(), test.cmr);
        assert_eq!(prog.amr().to_byte_array(), test.amr);
        assert_eq!(prog.ihr().to_byte_array(), test.ihr);
        assert_eq!(prog.bounds().cost, Cost::from_milliweight(test.cost))
    }

    #[test]
    fn progs_cmr() {
        let schnorr0 = ffi::tests::schnorr0_test_data();
        let schnorr6 = ffi::tests::schnorr6_test_data();
        let ctx8_unpruned = ffi::tests::ctx8_unpruned_test_data();
        let ctx8_pruned = ffi::tests::ctx8_pruned_test_data();
        // check_merkle_roots(&hash_block); Need 1 -> 1 for now.
        check_merkle_roots(&schnorr0);
        check_merkle_roots(&schnorr6);
        check_merkle_roots(&ctx8_unpruned);
        check_merkle_roots(&ctx8_pruned);
    }
}
