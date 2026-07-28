// SPDX-License-Identifier: CC0-1.0

//! "Type Promise" `ConstructNodes`
//!
//! Thes module contains the [`TypePromise`] type, which is a transparent wrapper around
//! any constructible node type (such as [`ConstructNode`] which makes a promise to the Rust
//! compiler that the node has the given source and/or target types. These promises can be
//! created from existing nodes by [`TypePromise::new`], [`TypePromise::promise_source_type`]
//! or [`TypePromise::promise_target_type`], which check the revelant types and return an
//! errro if they don't match. Or they can be created "by construction" by calling methods
//! like [`TypePromise::unit`] which directly create nodes that have a given type, and
//! promise this.
//!
//! To support this, the module also defines a Rust analogue of the Simplicity type system
//! in the form of the [`Free`], [`Unit`], [`Sum`] and [`Product`] types, which each implement
//! the [`Type`] trait. These types can usefully represent both "free" and "complete" Simplicity
//! types, which are sufficient to represent scribes (constant words) and composition with jets
//! (including hash functions).

use core::convert::Infallible;
use core::marker::PhantomData;
use core::ops::Deref;
use std::sync::Arc;

use super::{CoreConstructible, NoWitness, WitnessConstructible};
use crate::jet::Sha256Jet;
use crate::node::Node;
use crate::types::{self, Context, Final};
use crate::{Tmr, Word};

/// Rust representation of a free (unbound) Simplicity type.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Free {}

/// Rust representation of the Simplicity unit type.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Unit {}

/// Rust representation of the Simplicity sum type.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Sum<L, R> {
    /// Makes the type non-constructible.
    never: Infallible,
    /// Required by Rust.
    phantom: PhantomData<(L, R)>,
}

/// Rust representation of the Simplicity product type.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Product<L, R> {
    /// Makes the type non-constructible.
    never: Infallible,
    /// Required by Rust.
    phantom: PhantomData<(L, R)>,
}

pub trait Type {
    const IS_FREE: bool;
    const TMR: Option<Tmr>;
    fn runtime_ty<'brand>(ctx: &Context<'brand>) -> types::Type<'brand>;
}

impl Type for Free {
    const IS_FREE: bool = true;
    const TMR: Option<Tmr> = None;

    fn runtime_ty<'brand>(ctx: &Context<'brand>) -> types::Type<'brand> {
        types::Type::free(ctx, "compile_time_known".to_owned())
    }
}

impl Type for Unit {
    const IS_FREE: bool = false;
    const TMR: Option<Tmr> = Some(Tmr::unit());

    fn runtime_ty<'brand>(ctx: &Context<'brand>) -> types::Type<'brand> {
        types::Type::unit(ctx)
    }
}

pub trait UniqueType: Type {}
impl UniqueType for Unit {}

pub trait Unification<Ty> {}

// This lets us treat the "gensym types" returned from `promise_source` and `promise_target`
// as equal. Frustratingly we cannot do the same thing for `U: Type` because this would cause
// a 'conflicting impl' error with the Product/Sum impls below. This means that when you want
// to call pair(), for example, on a generic U, you need to add an explicit
//
//     where (U, U): Unification<U>
//
// bound on U. See for example `quote_single` in merkle/cmr.rs.
impl<U: UniqueType> Unification<U> for (U, U) {}
// The Rust compiler needs all the Free cases to be explicit to avoid 'conflicting impl' errors
impl Unification<Free> for (Free, Free) {}
impl Unification<Unit> for (Unit, Free) {}
impl Unification<Unit> for (Free, Unit) {}
impl<Ty1: Type, Ty2: Type> Unification<Sum<Ty1, Ty2>> for (Sum<Ty1, Ty2>, Free) {}
impl<Ty1: Type, Ty2: Type> Unification<Sum<Ty1, Ty2>> for (Free, Sum<Ty1, Ty2>) {}
impl<Ty1: Type, Ty2: Type> Unification<Product<Ty1, Ty2>> for (Product<Ty1, Ty2>, Free) {}
impl<Ty1: Type, Ty2: Type> Unification<Product<Ty1, Ty2>> for (Free, Product<Ty1, Ty2>) {}
impl<L, R, L1, R1, L2, R2> Unification<Sum<L, R>> for (Sum<L1, R1>, Sum<L2, R2>)
where
    (L1, L2): Unification<L>,
    (R1, R2): Unification<R>,
{
}
impl<L, R, L1, R1, L2, R2> Unification<Product<L, R>> for (Product<L1, R1>, Product<L2, R2>)
where
    (L1, L2): Unification<L>,
    (R1, R2): Unification<R>,
{
}

impl<L: Type, R: Type> Type for Product<L, R> {
    const IS_FREE: bool = false;
    const TMR: Option<Tmr> = match (L::TMR, R::TMR) {
        (Some(ltmr), Some(rtmr)) => Some(Tmr::product_unoptimized(ltmr, rtmr)),
        _ => None,
    };

    fn runtime_ty<'brand>(ctx: &Context<'brand>) -> types::Type<'brand> {
        if L::TMR == R::TMR {
            // Special case all the bitstrings of length up to 512. The overwhelmingly
            // common cases will be u256 and u512, and this check does both of those
            // without recomputing any hashes at runtime.
            if L::TMR == Some(Tmr::TWO_TWO_N[0]) {
                return types::Type::complete(ctx, Final::two_two_n_fixed::<1>());
            }
            if L::TMR == Some(Tmr::TWO_TWO_N[1]) {
                return types::Type::complete(ctx, Final::two_two_n_fixed::<2>());
            }
            if L::TMR == Some(Tmr::TWO_TWO_N[2]) {
                return types::Type::complete(ctx, Final::two_two_n_fixed::<3>());
            }
            if L::TMR == Some(Tmr::TWO_TWO_N[3]) {
                return types::Type::complete(ctx, Final::two_two_n_fixed::<4>());
            }
            if L::TMR == Some(Tmr::TWO_TWO_N[4]) {
                return types::Type::complete(ctx, Final::two_two_n_fixed::<5>());
            }
            if L::TMR == Some(Tmr::TWO_TWO_N[5]) {
                return types::Type::complete(ctx, Final::two_two_n_fixed::<6>());
            }
            if L::TMR == Some(Tmr::TWO_TWO_N[6]) {
                return types::Type::complete(ctx, Final::two_two_n_fixed::<7>());
            }
            if L::TMR == Some(Tmr::TWO_TWO_N[7]) {
                return types::Type::complete(ctx, Final::two_two_n_fixed::<8>());
            }
            if L::TMR == Some(Tmr::TWO_TWO_N[8]) {
                return types::Type::complete(ctx, Final::two_two_n_fixed::<9>());
            }
            // Maybe we also want to do successors of these, which would let us also shortcut
            // buffer8s and ctx8s.
        }
        types::Type::product(ctx, L::runtime_ty(ctx), R::runtime_ty(ctx))
    }
}

impl<L: Type, R: Type> Type for Sum<L, R> {
    const IS_FREE: bool = false;
    const TMR: Option<Tmr> = match (L::TMR, R::TMR) {
        (Some(ltmr), Some(rtmr)) => Some(Tmr::sum_unoptimized(ltmr, rtmr)),
        _ => None,
    };

    fn runtime_ty<'brand>(ctx: &Context<'brand>) -> types::Type<'brand> {
        if L::TMR == Some(Tmr::unit()) && R::TMR == Some(Tmr::unit()) {
            // Special-case bits.
            types::Type::complete(ctx, Final::two_two_n_fixed::<0>())
        } else {
            types::Type::sum(ctx, L::runtime_ty(ctx), R::runtime_ty(ctx))
        }
    }
}

/// The type 2 (one bit)
pub type U1 = Sum<Unit, Unit>;

/// The type 2^2 (pair of bits)
pub type U2 = Product<U1, U1>;

/// The type 2^4 (one nybble)
pub type U4 = Product<U2, U2>;

/// The type 2^8 (one byte)
pub type U8 = Product<U4, U4>;

/// The type 2^16
pub type U16 = Product<U8, U8>;

/// The type 2^32
pub type U32 = Product<U16, U16>;

/// The type 2^64
pub type U64 = Product<U32, U32>;

/// The type 2^128
pub type U128 = Product<U64, U64>;

/// The type 2^256
pub type U256 = Product<U128, U128>;

/// The type 2^512
pub type U512 = Product<U256, U256>;

/// Wrapper around a node type which makes a compile-time promise about both its source and target
/// types.
#[repr(transparent)]
pub struct TypePromise<STy, TTy, C> {
    inner: C,
    phantom: PhantomData<(STy, TTy)>,
}

impl<STy, TTy, C> Deref for TypePromise<STy, TTy, C> {
    type Target = C;
    fn deref(&self) -> &Self::Target {
        unsafe {
            // SAFETY: has #[repr(transparent)] with C as the inner type
            core::mem::transmute(self)
        }
    }
}

impl<LTy, RTy, C> TypePromise<LTy, RTy, C> {
    /// Drop the type promise and return the underlying node.
    pub fn forget_type_promise(self) -> C {
        self.inner
    }
}

impl<'brand, LTy: Type, RTy: Type, C: CoreConstructible<'brand>> TypePromise<LTy, RTy, C> {
    /// Promise that both the source and target Simplicity types of a node are some compile-time
    /// known value.
    pub fn new(inner: C) -> Option<Self> {
        let src_ok_free = LTy::IS_FREE && inner.arrow().source.is_free();
        let src_ok_complete = LTy::TMR.is_some() && LTy::TMR == inner.arrow().source.tmr();
        let tgt_ok_free = RTy::IS_FREE && inner.arrow().target.is_free();
        let tgt_ok_complete = RTy::TMR.is_some() && RTy::TMR == inner.arrow().target.tmr();

        if (src_ok_free || src_ok_complete) && (tgt_ok_free || tgt_ok_complete) {
            Some(Self {
                inner,
                phantom: PhantomData,
            })
        } else {
            None
        }
    }

    /// Sets the source type of the promise to some other, compatible source type.
    pub fn bind_source_type<LTy2>(self) -> TypePromise<LTy2, RTy, C>
    where
        (LTy, LTy2): Unification<LTy2>,
    {
        TypePromise {
            inner: self.inner,
            phantom: PhantomData,
        }
    }

    /// Sets the target type of the promise to some other, compatible target type.
    pub fn bind_target_type<RTy2>(self) -> TypePromise<LTy, RTy2, C>
    where
        (RTy, RTy2): Unification<RTy2>,
    {
        TypePromise {
            inner: self.inner,
            phantom: PhantomData,
        }
    }
}

// Constructors for various promises. Many of these we just put on `TypePromise<Free, Free>`
// even though they don't actually use two free types anywhere, so that when the user types
// `TypePromise` Rust won't struggle to infer types. It'll just call the method defined in
// this block.
impl<'brand, C: CoreConstructible<'brand>> TypePromise<Free, Free, C> {
    /// Promise that the source Simplicity type of a node is some compile-time known value.
    ///
    /// Makes no promise about the target type; it is set to a placeholder type which cannot
    /// be unified with any other type.
    pub fn promise_source_type<LTy: Type>(
        inner: C,
    ) -> Option<TypePromise<LTy, impl UniqueType, C>> {
        let ok_free = LTy::IS_FREE && inner.arrow().source.is_free();
        let ok_complete = LTy::TMR.is_some() && LTy::TMR == inner.arrow().source.tmr();

        if ok_free || ok_complete {
            // Here we set the target type to unit, but this is never visible. The caller will
            // see an "impl UniqueType" which will not compare equal to any other type.
            Some(TypePromise {
                inner,
                phantom: PhantomData::<(LTy, Unit)>,
            })
        } else {
            None
        }
    }

    /// Promise that the target Simplicity type of a node is some compile-time known value.
    ///
    /// Makes no promise about the source type; it is set to a placeholder type which cannot
    /// be unified with any other type.
    pub fn promise_target_type<RTy: Type>(
        inner: C,
    ) -> Option<TypePromise<impl UniqueType, RTy, C>> {
        let ok_free = RTy::IS_FREE && inner.arrow().target.is_free();
        let ok_complete = RTy::TMR.is_some() && RTy::TMR == inner.arrow().target.tmr();

        if ok_free || ok_complete {
            // Here we set the source type to unit, but this is never visible. The caller will
            // see an "impl UniqueType" which will not compare equal to any other type.
            Some(TypePromise {
                inner,
                phantom: PhantomData::<(Unit, RTy)>,
            })
        } else {
            None
        }
    }

    /// Constructs the iden combinator.
    pub fn iden(ctx: &types::Context<'brand>) -> Self {
        Self {
            inner: C::iden(ctx),
            phantom: PhantomData,
        }
    }
}

impl<'brand, M, LTy, RTy1, RTy2> TypePromise<LTy, Product<RTy1, RTy2>, Arc<Node<M>>>
where
    M: crate::node::Marker,
    M::CachedData: CoreConstructible<'brand>,
{
    /// Constructs the 'pair' combinator.
    pub fn pair1(
        left: &TypePromise<LTy, RTy1, Arc<Node<M>>>,
        right: &TypePromise<LTy, RTy2, Arc<Node<M>>>,
    ) -> Self {
        let l_arrow = left.inner.arrow().shallow_clone();
        let r_arrow = right.inner.arrow().shallow_clone();
        let ctx = l_arrow.inference_context;
        let new_arrow = types::arrow::Arrow {
            source: l_arrow.source.shallow_clone(),
            target: types::Type::product(&ctx, l_arrow.target, r_arrow.target),
            inference_context: ctx,
        };

        TypePromise {
            inner: Arc::<Node<M>>::pair_from_arrow(left, right, new_arrow),
            phantom: PhantomData,
        }
    }

    /// Constructs the 'pair' combinator.
    pub fn pair<LTy1, LTy2>(
        left: &TypePromise<LTy1, RTy1, Arc<Node<M>>>,
        right: &TypePromise<LTy2, RTy2, Arc<Node<M>>>,
    ) -> Self
    where
        (LTy1, LTy2): Unification<LTy>,
    {
        let l_arrow = left.inner.arrow().shallow_clone();
        let r_arrow = right.inner.arrow().shallow_clone();
        let ctx = l_arrow.inference_context;
        let new_arrow = types::arrow::Arrow {
            source: l_arrow.source.shallow_clone(),
            target: types::Type::product(&ctx, l_arrow.target, r_arrow.target),
            inference_context: ctx,
        };

        TypePromise {
            inner: Arc::<Node<M>>::pair_from_arrow(left, right, new_arrow),
            phantom: PhantomData,
        }
    }
}

impl<'brand, C: CoreConstructible<'brand>> TypePromise<Free, Unit, C> {
    /// Constructs the unit combinator/value.
    pub fn unit(ctx: &types::Context<'brand>) -> Self {
        Self {
            inner: C::unit(ctx),
            phantom: PhantomData,
        }
    }
}

impl<'brand, C: CoreConstructible<'brand>> TypePromise<Unit, U1, C> {
    /// Constructs the false value.
    pub fn _false<J: Sha256Jet>(ctx: &types::Context<'brand>) -> Self {
        Self {
            inner: C::jet(ctx, J::low_1()),
            phantom: PhantomData,
        }
    }

    /// Constructs the true value.
    pub fn _true<J: Sha256Jet>(ctx: &types::Context<'brand>) -> Self {
        Self {
            inner: C::jet(ctx, J::high_1()),
            phantom: PhantomData,
        }
    }

    /// Constructs a 1-bit constant word.
    pub fn const_word(ctx: &types::Context<'brand>, bit: bool) -> Self {
        Self {
            inner: C::const_word(ctx, Word::u1(u8::from(bit))),
            phantom: PhantomData,
        }
    }
}

impl<'brand, C: CoreConstructible<'brand>> TypePromise<Unit, U2, C> {
    /// Constructs a 2-bit constant word.
    ///
    /// ## Panics
    ///
    /// The value is out of range.
    pub fn const_word(ctx: &types::Context<'brand>, word: u8) -> Self {
        Self {
            inner: C::const_word(ctx, Word::u2(word)),
            phantom: PhantomData,
        }
    }
}

impl<'brand, C: CoreConstructible<'brand>> TypePromise<Unit, U4, C> {
    /// Constructs a 4-bit constant word.
    ///
    /// ## Panics
    ///
    /// The value is out of range.
    pub fn const_word(ctx: &types::Context<'brand>, word: u8) -> Self {
        Self {
            inner: C::const_word(ctx, Word::u4(word)),
            phantom: PhantomData,
        }
    }
}

impl<'brand, C: CoreConstructible<'brand>> TypePromise<Unit, U8, C> {
    /// Constructs a constant byte.
    pub fn const_word(ctx: &types::Context<'brand>, byte: u8) -> Self {
        Self {
            inner: C::const_word(ctx, Word::u8(byte)),
            phantom: PhantomData,
        }
    }
}

impl<'brand, C: CoreConstructible<'brand>> TypePromise<Unit, U16, C> {
    /// Constructs a constant byte.
    pub fn const_word(ctx: &types::Context<'brand>, word: u16) -> Self {
        Self {
            inner: C::const_word(ctx, Word::u16(word)),
            phantom: PhantomData,
        }
    }
}

impl<'brand, C: CoreConstructible<'brand>> TypePromise<Unit, U32, C> {
    /// Constructs a constant byte.
    pub fn const_word(ctx: &types::Context<'brand>, word: u32) -> Self {
        Self {
            inner: C::const_word(ctx, Word::u32(word)),
            phantom: PhantomData,
        }
    }
}

impl<'brand, C: CoreConstructible<'brand>> TypePromise<Unit, U64, C> {
    /// Constructs a constant byte.
    pub fn const_word(ctx: &types::Context<'brand>, word: u64) -> Self {
        Self {
            inner: C::const_word(ctx, Word::u64(word)),
            phantom: PhantomData,
        }
    }
}

impl<'brand, C: CoreConstructible<'brand>> TypePromise<Unit, U128, C> {
    /// Constructs a constant byte.
    pub fn const_word(ctx: &types::Context<'brand>, word: u128) -> Self {
        Self {
            inner: C::const_word(ctx, Word::u128(word)),
            phantom: PhantomData,
        }
    }
}

impl<'brand, C: CoreConstructible<'brand>> TypePromise<Unit, U256, C> {
    /// Constructs a constant byte.
    pub fn const_word(ctx: &types::Context<'brand>, bytes: [u8; 32]) -> Self {
        Self {
            inner: C::const_word(ctx, Word::u256(bytes)),
            phantom: PhantomData,
        }
    }
}

impl<'brand, C: CoreConstructible<'brand>> TypePromise<Unit, U512, C> {
    /// Constructs a constant byte.
    pub fn const_word(ctx: &types::Context<'brand>, bytes: [u8; 64]) -> Self {
        Self {
            inner: C::const_word(ctx, Word::u512(bytes)),
            phantom: PhantomData,
        }
    }
}

impl<'brand, C: CoreConstructible<'brand>> TypePromise<Product<U256, U512>, U256, C> {
    /// Constructs the `sha256_block` jet.
    pub fn sha256_block<J: Sha256Jet>(ctx: &types::Context<'brand>) -> Self {
        Self {
            inner: C::jet(ctx, J::sha256_block()),
            phantom: PhantomData,
        }
    }
}

impl<'brand, M, LTy, RTy> TypePromise<LTy, RTy, Arc<Node<M>>>
where
    M: crate::node::Marker,
    M::CachedData: CoreConstructible<'brand>,
{
    /// Constructs the 'injl' combinator.
    pub fn injl(&self) -> TypePromise<Sum<LTy, Free>, RTy, Arc<Node<M>>> {
        TypePromise {
            inner: Arc::<Node<M>>::injl(&self.inner),
            phantom: PhantomData,
        }
    }

    /// Constructs the 'injr' combinator.
    pub fn injr(&self) -> TypePromise<Sum<Free, LTy>, RTy, Arc<Node<M>>> {
        TypePromise {
            inner: Arc::<Node<M>>::injr(&self.inner),
            phantom: PhantomData,
        }
    }

    /// Constructs the 'take' combinator.
    pub fn take(&self) -> TypePromise<Product<LTy, Free>, RTy, Arc<Node<M>>> {
        TypePromise {
            inner: Arc::<Node<M>>::take(&self.inner),
            phantom: PhantomData,
        }
    }

    /// Constructs the 'drop' combinator.
    pub fn drop_(&self) -> TypePromise<Product<Free, LTy>, RTy, Arc<Node<M>>> {
        TypePromise {
            inner: Arc::<Node<M>>::drop_(&self.inner),
            phantom: PhantomData,
        }
    }

    /// Constructs the 'comp' combinator.
    pub fn comp<MidTy>(
        left: &TypePromise<LTy, MidTy, Arc<Node<M>>>,
        right: &TypePromise<MidTy, RTy, Arc<Node<M>>>,
    ) -> Self {
        let l_arrow = left.inner.arrow();
        let inference_context = l_arrow.inference_context.shallow_clone();
        let new_arrow = types::arrow::Arrow {
            source: left.arrow().source.shallow_clone(),
            target: right.arrow().target.shallow_clone(),
            inference_context,
        };

        Self {
            inner: Arc::<Node<M>>::comp_from_arrow(left, right, new_arrow),
            phantom: PhantomData,
        }
    }
}

impl<'brand, M> TypePromise<Free, Free, Arc<Node<M>>>
where
    M: crate::node::Marker,
    M::Witness: From<NoWitness>,
    M::CachedData: WitnessConstructible<'brand, M::Witness>,
{
    /// Constructs a witness node with free source and target types.
    ///
    /// If you have nontrivial witness data, use [`TypePromise::witness`] instead. But see the
    /// caveats on that method.
    pub fn free_witness(ctx: &Context<'brand>) -> Self {
        Self {
            inner: Arc::<Node<M>>::witness(ctx, NoWitness.into()),
            phantom: PhantomData,
        }
    }
}

impl<'brand, M> TypePromise<Free, Free, Arc<Node<M>>>
where
    M: crate::node::Marker,
    M::CachedData: WitnessConstructible<'brand, M::Witness>,
{
    /// Constructs a witness node with free source and target types.
    ///
    /// As with the ordinary [`WitnessConstructible::witness`] constructor, this method does
    /// **not** check that the provided witness value has a compatible type (if it has a type
    /// at all) with the witness node. If these are not compatible, construction of the
    /// program will fail at the final step, when converting to a [`simplicity::RedeemNode`].
    pub fn witness(ctx: &Context<'brand>, wit: M::Witness) -> Self {
        Self {
            inner: Arc::<Node<M>>::witness(ctx, wit),
            phantom: PhantomData,
        }
    }
}

impl<'brand, M, RTy> TypePromise<Unit, RTy, Arc<Node<M>>>
where
    M: crate::node::Marker,
    M::CachedData: CoreConstructible<'brand>,
    RTy: Type,
{
    /// Takes a node L with unit source and returns (unit; L). In other words, precomposes
    /// the node with unit, thereby erasing its original source type.
    ///
    /// Replaces the source type with the type `LTy` specified by the caller. After calling
    /// this method, the "correct" source type of the contained node is still a free variable,
    /// so if the source is not constrained by further compositions, it will eventually (during
    /// pruning) be replaced by unit, regardless of what `LTy` was. This is harmless and
    /// probably not even observable, but potentially confusing.
    ///
    /// To avoid confusion, we recommend immediately following this method by another
    /// combinator like `pair` which will actually fix the type to what it's being promised as.
    pub fn precompose_unit<LTy: Type>(&self) -> TypePromise<LTy, RTy, Arc<Node<M>>> {
        let ctx = self.inner.inference_context();

        let new_source = LTy::runtime_ty(ctx);
        let new_unit_arrow = types::Arrow {
            source: new_source.shallow_clone(),
            target: types::Type::unit(ctx),
            inference_context: ctx.shallow_clone(),
        };
        let new_unit = Arc::unit_from_arrow(new_unit_arrow);

        let new_arrow = types::arrow::Arrow {
            source: new_source,
            target: self.inner.arrow().target.shallow_clone(),
            inference_context: ctx.shallow_clone(),
        };
        TypePromise::<LTy, RTy, _> {
            inner: Arc::comp_from_arrow(&new_unit, &self.inner, new_arrow),
            phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::node::ConstructNode;

    #[test]
    fn compile_pair() {
        types::Context::with_context(|ctx| {
            let unit1 = TypePromise::<_, _, Arc<ConstructNode>>::unit(&ctx);
            let unit2 = TypePromise::<_, _, Arc<ConstructNode>>::unit(&ctx);
            let _ = TypePromise::pair(&unit1, &unit2);
        })
    }
}
