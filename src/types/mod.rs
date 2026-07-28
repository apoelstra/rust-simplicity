// SPDX-License-Identifier: CC0-1.0

//! Types and Type Inference
//!
//! Every Simplicity expression has two types associated with it: a source and
//! a target. We refer to this pair of types as an "arrow". The types are
//! inferred from the structure of the program.
//!
//! Simplicity types are one of three things
//!   * A unit type, which has one value
//!   * A sum of two other types
//!   * A product of two other types
//!
//! During type inference, types are initially "free", meaning that there are
//! no constraints on what they will eventually be. The program structure then
//! imposes additional constraints (for example, the `comp` combinator requires
//! that its left child's target type be the same as its right child's source
//! type), and by unifying all these constraints, all types can be inferred.
//!
//! Type inference is done progressively during construction of Simplicity
//! expressions. It is completed by the [`Type::finalize`] method, which
//! recursively completes types by setting any remaining free variables to unit.
//! If any type constraints are incompatible with each other (e.g. a type is
//! bound to be both a product and a sum type) then inference fails at that point
//! by returning an error.
//!
//! In addition to completing types [`Type::finalize`], does one additional
//! check, the "occurs check", to ensures that there are no infinitely-sized
//! types. Such types occur when a type has itself as a child, are illegal in
//! Simplicity, and could not be represented by our data structures.
//!

// In this module, during inference types are characterized by their [`Bound`],
// which describes the constraints on the type. The bound of a type can be
// obtained by the [`Type::bound`] method, and is an enum with four variants:
//
// * [`Bound::Free`] means that the type has no constraints; it is a free
//   variable. The type has a name which can be used to identify it in error
//   messages.
// * [`Bound::Sum`] and [`Bound::Product`] means that the the type is a sum
//   (resp. product) of two other types, which are characterized by their
//   own bounds.
// * [`Bound::Complete`] means that the type has no free variables at all,
//   and has an already-computed [`Final`] structure suitable for use in
//   contexts that require complete types. (Unit types are always complete,
//   and therefore use this variant rather than getting their own.)
//
// During inference, it is possible for a type to be complete, in the sense
// of having no free variables, without its bound being [`Bound::Complete`].
// This occurs, for example, if a type is a sum of two incomplete types, then
// the child types are completed during type inference on an unrelated part
// of the type hierarchy. The type would then have a [`Bound::Sum`] with two
// children, both of which are complete.
//
// The inference engine makes an effort to notice when this happens and set
// the bound of complete types to [`Bound::Complete`], but since type inference
// is inherently non-local this cannot always be done.
//
// When the distinction matters, we say a type is "finalized" only if its bound
// is `Complete` and "complete" if it has no free variables. But the distinction
// usually does not matter, so we prefer to use the word "complete".
//
// There are three main types in this module:
//   * [`Type`] is the main type representing a Simplicity type, whether it is
//     complete or not. Its main methods are [`Type::bound`] which returns the
//     current state of the type and [`Type::bind`] which adds a new constraint
//     to the type.
//   * `Final` is a mutex-free structure that can be obtained from a complete
//     type. It includes the TMR and the complete bound describing the type.
//   * `Bound` defines the structure of a type: whether it is free, complete,
//     or a sum or product of other types.
//

use self::union_bound::{PointerLike, UbElement, WithGhostToken};
use crate::dag::{DagLike, NoSharing};
use crate::Tmr;

use std::fmt;
use std::sync::Arc;

pub mod arrow;
mod context;
mod final_data;
mod incomplete;
mod precomputed;
mod union_bound;
mod variable;

pub use arrow::{Arrow, FinalArrow};
pub use context::{BoundRef, Context};
pub use final_data::{CompleteBound, Final, TypeTooLargeError};
pub use incomplete::Incomplete;

/// Error type for simplicity
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum Error {
    /// An attempt to bind a type conflicted with an existing bound on the type
    Bind {
        existing_bound: Arc<Incomplete>,
        new_bound: Arc<Incomplete>,
        hint: &'static str,
    },
    /// Two unequal complete types were attempted to be unified
    CompleteTypeMismatch {
        type1: Arc<Final>,
        type2: Arc<Final>,
        hint: &'static str,
    },
    /// A type is recursive (i.e., occurs within itself), violating the "occurs check"
    OccursCheck { infinite_bound: Arc<Incomplete> },
    /// Attempted to combine two nodes which had different type inference
    /// contexts. This is probably a programming error.
    InferenceContextMismatch,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Bind {
                ref existing_bound,
                ref new_bound,
                hint,
            } => {
                write!(
                    f,
                    "failed to apply bound `{}` to existing bound `{}`: {}",
                    new_bound, existing_bound, hint,
                )
            }
            Error::CompleteTypeMismatch {
                ref type1,
                ref type2,
                hint,
            } => {
                write!(
                    f,
                    "attempted to unify unequal types `{}` and `{}`: {}",
                    type1, type2, hint,
                )
            }
            Error::OccursCheck { infinite_bound } => {
                write!(f, "infinitely-sized type {}", infinite_bound,)
            }
            Error::InferenceContextMismatch => {
                f.write_str("attempted to combine two nodes with different type inference contexts")
            }
        }
    }
}

impl std::error::Error for Error {}

/// The state of a [`Type`] based on all constraints currently imposed on it.
#[derive(Clone)]
enum Bound<'brand> {
    /// Fully-unconstrained type
    Free(String),
    /// Fully-constrained (i.e. complete) type, which has no free variables.
    Complete(Arc<Final>),
    /// A sum of two other types
    Sum(TypeInner<'brand>, TypeInner<'brand>),
    /// A product of two other types
    Product(TypeInner<'brand>, TypeInner<'brand>),
}

impl Bound<'_> {
    /// Clones the `Bound`.
    ///
    /// This is the same as just calling `.clone()` but has a different name to
    /// emphasize that what's being cloned is (at most) a pair of ref-counted
    /// pointers.
    pub fn shallow_clone(&self) -> Self {
        self.clone()
    }
}

/// Source or target type of a Simplicity expression.
///
/// Internally this type is essentially just a refcounted pointer; it is
/// therefore quite cheap to clone, but be aware that cloning will not
/// actually create a new independent type, just a second pointer to the
/// first one.
#[derive(Clone)]
pub struct Type<'brand> {
    /// Handle to the type context.
    ctx: Context<'brand>,
    /// The actual contents of the type.
    inner: TypeInner<'brand>,
}

#[derive(Clone)]
struct TypeInner<'brand> {
    /// A set of constraints, which maintained by the union-bound algorithm and
    /// is progressively tightened as type inference proceeds.
    bound: UbElement<'brand, BoundRef<'brand>>,
}

impl TypeInner<'_> {
    fn shallow_clone(&self) -> Self {
        self.clone()
    }
}

impl<'brand> Type<'brand> {
    /// Return an unbound type with the given name
    pub fn free(ctx: &Context<'brand>, name: String) -> Self {
        Self::wrap_bound(ctx, ctx.alloc_free(name))
    }

    /// Create the unit type.
    pub fn unit(ctx: &Context<'brand>) -> Self {
        Self::wrap_bound(ctx, ctx.alloc_unit())
    }

    /// Create the type `2^(2^n)` for the given `n`.
    ///
    /// The type is precomputed and fast to access.
    pub fn two_two_n(ctx: &Context<'brand>, n: usize) -> Self {
        Self::complete(ctx, precomputed::nth_power_of_2(n))
    }

    /// Create the sum of the given `left` and `right` types.
    pub fn sum(ctx: &Context<'brand>, left: Self, right: Self) -> Self {
        Self::wrap_bound(ctx, ctx.alloc_sum(left, right))
    }

    /// Create the product of the given `left` and `right` types.
    pub fn product(ctx: &Context<'brand>, left: Self, right: Self) -> Self {
        Self::wrap_bound(ctx, ctx.alloc_product(left, right))
    }

    /// Create a complete type.
    pub fn complete(ctx: &Context<'brand>, final_data: Arc<Final>) -> Self {
        Self::wrap_bound(ctx, ctx.alloc_complete(final_data))
    }

    fn wrap_bound(ctx: &Context<'brand>, bound: BoundRef<'brand>) -> Self {
        Type {
            ctx: ctx.shallow_clone(),
            inner: TypeInner {
                bound: UbElement::new(bound),
            },
        }
    }

    /// Clones the `Type`.
    ///
    /// This is the same as just calling `.clone()` but has a different name to
    /// emphasize that what's being cloned is merely a ref-counted pointer.
    pub fn shallow_clone(&self) -> Self {
        self.clone()
    }

    /// Accessor for the TMR of this type, if it is final
    pub fn tmr(&self) -> Option<Tmr> {
        self.final_data().map(|data| data.tmr())
    }

    /// Accessor for the data of this type, if it is complete
    pub fn final_data(&self) -> Option<Arc<Final>> {
        let root = self.ctx.get_root_ref(&self.inner.bound);
        let bound = self.ctx.get(&root);
        if let Bound::Complete(ref data) = bound {
            Some(Arc::clone(data))
        } else {
            None
        }
    }

    /// Whether this type is a free type.
    pub fn is_free(&self) -> bool {
        let root = self.ctx.get_root_ref(&self.inner.bound);
        let bound = self.ctx.get(&root);
        matches!(bound, Bound::Free(_))
    }

    /// Whether this type is known to be final
    ///
    /// During type inference this may be false even though the type is, in fact,
    /// complete, since its children may have been unified to a complete type. To
    /// ensure a type is complete, call [`Type::finalize`].
    pub fn is_final(&self) -> bool {
        self.final_data().is_some()
    }

    /// Converts a type as-is to an `Incomplete` type for use in an error.
    pub fn to_incomplete(&self) -> Arc<Incomplete> {
        let root = self.ctx.get_root_ref(&self.inner.bound);
        Incomplete::from_bound_ref(&self.ctx, root)
    }

    /// Attempts to finalize the type. Returns its TMR on success.
    pub fn finalize(&self) -> Result<Arc<Final>, Error> {
        let root = self.ctx.get_root_ref(&self.inner.bound);
        let bound = self.ctx.get(&root);
        if let Bound::Complete(ref data) = bound {
            return Ok(Arc::clone(data));
        }

        // First, do occurs-check to ensure that we have no infinitely sized types.
        if let Some(infinite_bound) = Incomplete::occurs_check(&self.ctx, root.shallow_clone()) {
            return Err(Error::OccursCheck { infinite_bound });
        }

        // Now that we know our types have finite size, we can safely use a
        // post-order iterator to finalize them.
        let mut finalized = vec![];
        for data in (&self.ctx, root).post_order_iter::<NoSharing>() {
            let bound_get = data.node.0.get(&data.node.1);
            let final_data = match bound_get {
                Bound::Free(_) => Final::unit(),
                Bound::Complete(ref arc) => Arc::clone(arc),
                Bound::Sum(..) => Final::sum(
                    Arc::clone(&finalized[data.left_index.unwrap()]),
                    Arc::clone(&finalized[data.right_index.unwrap()]),
                ),
                Bound::Product(..) => Final::product(
                    Arc::clone(&finalized[data.left_index.unwrap()]),
                    Arc::clone(&finalized[data.right_index.unwrap()]),
                ),
            };

            if !matches!(bound_get, Bound::Complete(..)) {
                self.ctx
                    .reassign_non_complete(data.node.1, Bound::Complete(Arc::clone(&final_data)));
            }
            finalized.push(final_data);
        }
        Ok(finalized.pop().unwrap())
    }

    /// Return a vector containing the types 2^(2^i) for i from 0 to n-1.
    pub fn powers_of_two(ctx: &Context<'brand>, n: usize) -> Vec<Self> {
        let mut ret = Vec::with_capacity(n);

        let unit = Type::unit(ctx);
        let mut two = Type::sum(ctx, unit.shallow_clone(), unit);
        for _ in 0..n {
            ret.push(two.shallow_clone());
            two = Type::product(ctx, two.shallow_clone(), two);
        }
        ret
    }
}

const MAX_DISPLAY_DEPTH: usize = 64;
const MAX_DISPLAY_LENGTH: usize = 10000;

impl fmt::Debug for Type<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let root = self.ctx.get_root_ref(&self.inner.bound);
        for data in (&self.ctx, root).verbose_pre_order_iter::<NoSharing>(Some(MAX_DISPLAY_DEPTH)) {
            if data.index > MAX_DISPLAY_LENGTH {
                write!(f, "... [truncated type after {} nodes]", MAX_DISPLAY_LENGTH)?;
                return Ok(());
            }
            if data.depth == MAX_DISPLAY_DEPTH {
                if data.n_children_yielded == 0 {
                    f.write_str("...")?;
                }
                continue;
            }
            let bound = data.node.0.get(&data.node.1);
            match (bound, data.n_children_yielded) {
                (Bound::Free(ref s), _) => f.write_str(s)?,
                (Bound::Complete(ref comp), _) => fmt::Debug::fmt(comp, f)?,
                (Bound::Sum(..), 0) | (Bound::Product(..), 0) => {
                    if data.index > 0 {
                        f.write_str("(")?;
                    }
                }
                (Bound::Sum(..), 2) | (Bound::Product(..), 2) => {
                    if data.index > 0 {
                        f.write_str(")")?
                    }
                }
                (Bound::Sum(..), _) => f.write_str(" + ")?,
                (Bound::Product(..), _) => f.write_str(" × ")?,
            }
        }
        Ok(())
    }
}

impl fmt::Display for Type<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let root = self.ctx.get_root_ref(&self.inner.bound);
        for data in (&self.ctx, root).verbose_pre_order_iter::<NoSharing>(Some(MAX_DISPLAY_DEPTH)) {
            if data.index > MAX_DISPLAY_LENGTH {
                write!(f, "... [truncated type after {} nodes]", MAX_DISPLAY_LENGTH)?;
                return Ok(());
            }
            if data.depth == MAX_DISPLAY_DEPTH {
                if data.n_children_yielded == 0 {
                    f.write_str("...")?;
                }
                continue;
            }
            let bound = data.node.0.get(&data.node.1);
            match (bound, data.n_children_yielded) {
                (Bound::Free(ref s), _) => f.write_str(s)?,
                (Bound::Complete(ref comp), _) => fmt::Display::fmt(comp, f)?,
                (Bound::Sum(..), 0) | (Bound::Product(..), 0) => {
                    if data.index > 0 {
                        f.write_str("(")?;
                    }
                }
                (Bound::Sum(..), 2) | (Bound::Product(..), 2) => {
                    if data.index > 0 {
                        f.write_str(")")?
                    }
                }
                (Bound::Sum(..), _) => f.write_str(" + ")?,
                (Bound::Product(..), _) => f.write_str(" × ")?,
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::jet::Core;
    use crate::node::{ConstructNode, CoreConstructible as _};

    #[test]
    fn inference_failure() {
        Context::with_context(|ctx| {
            // unit: A -> 1
            let unit = Arc::<ConstructNode>::unit(&ctx); // 1 -> 1

            // Force unit to be 1->1
            Arc::<ConstructNode>::comp(&unit, &unit).unwrap();

            // take unit: 1 * B -> 1
            let take_unit = Arc::<ConstructNode>::take(&unit); // 1*1 -> 1

            // Pair will try to unify 1 and 1*B
            Arc::<ConstructNode>::pair(&unit, &take_unit).unwrap_err();
            // Trying to do it again should not work.
            Arc::<ConstructNode>::pair(&unit, &take_unit).unwrap_err();
        });
    }

    #[test]
    fn memory_leak() {
        Context::with_context(|ctx| {
            let iden = Arc::<ConstructNode>::iden(&ctx);
            let drop = Arc::<ConstructNode>::drop_(&iden);
            let case = Arc::<ConstructNode>::case(&iden, &drop).unwrap();

            let _ = format!("{:?}", case.arrow().source);
        });
    }

    #[test]
    fn check_jet_same_type_ok() {
        Context::with_context(|ctx| {
            let _ = Arc::<ConstructNode>::jet(&ctx, &Core::Add32);
            let _ = Arc::<ConstructNode>::jet(&ctx, &Core::Subtract32);
        });
    }

    #[cfg(feature = "elements")]
    #[test]
    #[should_panic(expected = "mixed jet types in context")]
    fn check_jet_different_types_panics() {
        use crate::jet::Elements;

        Context::with_context(|ctx| {
            let _ = Arc::<ConstructNode>::jet(&ctx, &Core::Add32);
            let _ = Arc::<ConstructNode>::jet(&ctx, &Elements::Add32);
        });
    }
}
