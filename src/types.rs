use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::{cmp, fmt, mem};

use extension;
use Node;

#[derive(Clone)]
enum Type {
    Unit,
    Sum(RcVar, RcVar),
    Product(RcVar, RcVar),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FinalTypeInner {
    Unit,
    Sum(Arc<FinalType>, Arc<FinalType>),
    Product(Arc<FinalType>, Arc<FinalType>),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct FinalType {
    pub ty: FinalTypeInner,
    pub bit_width: usize,
}

impl fmt::Display for FinalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.ty {
            FinalTypeInner::Unit => f.write_str("ε"),
            FinalTypeInner::Sum(ref a, ref b) => write!(f, "({} + {})", a, b),
            FinalTypeInner::Product(ref a, ref b) => write!(f, "({} × {})", a, b),
        }
    }
}

impl FinalType {
    pub fn bit_width(&self) -> usize {
        self.bit_width
    }

    fn from_var(var: RcVar) -> Arc<FinalType> {
        let var = find_root(var);
        let mut var_borr = var.borrow_mut();

        let existing_type = match var_borr.var {
            Variable::Free => Type::Unit,
            Variable::Bound(ref ty, ref mut occurs_check) => {
                assert!(!*occurs_check); // TODO set an error here
                *occurs_check = true;
                ty.clone()
            }
            Variable::EqualTo(..) => unreachable!(),
            Variable::Finalized(ref done) => return done.clone(),
        };

        let (sub1, sub2) = match existing_type {
            Type::Unit => {
                let ret = Arc::new(FinalType {
                    ty: FinalTypeInner::Unit,
                    bit_width: 0,
                });
                var_borr.var = Variable::Finalized(ret.clone());
                return ret;
            }
            Type::Sum(ref sub1, ref sub2) => (sub1.clone(), sub2.clone()),
            Type::Product(ref sub1, ref sub2) => (sub1.clone(), sub2.clone()),
        };

        let sub1 = find_root(sub1.clone());
        let sub2 = find_root(sub2.clone());

        let sub1_borr = sub1.borrow_mut();
        let final1 = match sub1_borr.var {
            Variable::Free => Arc::new(FinalType {
                ty: FinalTypeInner::Unit,
                bit_width: 0,
            }),
            Variable::Bound(..) => {
                drop(sub1_borr);
                FinalType::from_var(sub1.clone())
            }
            Variable::EqualTo(..) => unreachable!(),
            Variable::Finalized(ref f1) => {
                let ret = f1.clone();
                drop(sub1_borr);
                ret
            }
        };

        let sub2_borr = sub2.borrow_mut();
        let final2 = match sub2_borr.var {
            Variable::Free => Arc::new(FinalType {
                ty: FinalTypeInner::Unit,
                bit_width: 0,
            }),
            Variable::Bound(..) => {
                drop(sub2_borr);
                FinalType::from_var(sub2)
            }
            Variable::EqualTo(..) => unreachable!(),
            Variable::Finalized(ref f2) => {
                let ret = f2.clone();
                drop(sub2_borr);
                ret
            }
        };

        let ret = match existing_type {
            Type::Unit => unreachable!(),
            Type::Sum(..) => Arc::new(FinalType {
                bit_width: 1 + cmp::max(final1.bit_width, final2.bit_width),
                ty: FinalTypeInner::Sum(final1, final2),
            }),
            Type::Product(..) => Arc::new(FinalType {
                bit_width: final1.bit_width + final2.bit_width,
                ty: FinalTypeInner::Product(final1, final2),
            }),
        };
        var_borr.var = Variable::Finalized(ret.clone());
        ret
    }
}

#[derive(Clone)]
enum Variable {
    /// Free variable
    Free,
    /// Bound to some type (which may itself contain other free variables,
    /// or not). Contains a boolean which is only used by the finalization
    /// function, for the occurs-check
    Bound(Type, bool),
    /// Equal to another variable (the included `RcVar` is the "parent"
    /// pointer in union-find terms)
    EqualTo(RcVar),
    /// Complete type has been set in place
    Finalized(Arc<FinalType>),
}

struct UnificationVar {
    var: Variable,
    rank: usize,
}

type RcVar = Rc<RefCell<UnificationVar>>;

impl UnificationVar {
    fn free() -> UnificationVar {
        UnificationVar {
            var: Variable::Free,
            rank: 0,
        }
    }

    fn concrete(ty: Type) -> UnificationVar {
        UnificationVar {
            var: Variable::Bound(ty, false),
            rank: 0,
        }
    }
}

fn bind(rcvar: &RcVar, ty: Type) {
    // Cloning a `Variable` is cheap, as the nontrivial variants merely
    // hold `Rc`s
    let self_var = rcvar.borrow().var.clone();
    match self_var {
        Variable::Free => rcvar.borrow_mut().var = Variable::Bound(ty, false),
        Variable::EqualTo(..) => unreachable!(
            "Tried to bind unification variable which was not \
             the representative of its equivalence class"
        ),
        Variable::Finalized(..) => unreachable!(),
        Variable::Bound(self_ty, _) => match (self_ty, ty) {
            (Type::Unit, Type::Unit) => {}
            (Type::Sum(al1, al2), Type::Sum(be1, be2))
            | (Type::Product(al1, al2), Type::Product(be1, be2)) => {
                unify(al1, be1);
                unify(al2, be2);
            }
            (a, b) => {
                let self_s = match a {
                    Type::Unit => "unit",
                    Type::Sum(..) => "sum",
                    Type::Product(..) => "prod",
                };
                let b_s = match b {
                    Type::Unit => "unit",
                    Type::Sum(..) => "sum",
                    Type::Product(..) => "prod",
                };
                panic!("unification failure {} vs {}", self_s, b_s)
            }
        },
    }
}

fn find_root(mut node: RcVar) -> RcVar {
    loop {
        // Double-assignment needed for pre-NLL borrowck reasons
        let parent = match node.borrow().var {
            Variable::EqualTo(ref parent) => Some(parent.clone()),
            _ => None,
        };
        let parent = match parent {
            Some(x) => x,
            _ => break node,
        };

        // Extra scope for pre-NLL borrowck reasons
        {
            let parent_borr = parent.borrow();
            if let Variable::EqualTo(ref grandparent) = parent_borr.var {
                node.borrow_mut().var = Variable::EqualTo(grandparent.clone());
            }
        }

        node = parent;
    }
}

fn unify(mut alpha: RcVar, mut beta: RcVar) {
    alpha = find_root(alpha);
    beta = find_root(beta);

    // Already unified, done
    if Rc::ptr_eq(&alpha, &beta) {
        return;
    }

    // Adjust ranks for union-find path halving
    if alpha.borrow().rank < beta.borrow().rank {
        mem::swap(&mut alpha, &mut beta);
    } else if alpha.borrow().rank == beta.borrow().rank {
        alpha.borrow_mut().rank += 1;
    }

    // Do the unification
    let be_var = {
        let mut be_borr = beta.borrow_mut();
        mem::replace(&mut be_borr.var, Variable::EqualTo(alpha.clone()))
    };
    match be_var {
        Variable::Free => {} // nothing to do
        Variable::Bound(be_type, _) => bind(&alpha, be_type),
        Variable::EqualTo(..) => unreachable!(),
        Variable::Finalized(..) => unreachable!(),
    }
}

#[derive(Clone)]
struct UnificationArrow {
    source: Rc<RefCell<UnificationVar>>,
    target: Rc<RefCell<UnificationVar>>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct TypedNode<Witness> {
    pub node: Node<Witness>,
    pub source_ty: Arc<FinalType>,
    pub target_ty: Arc<FinalType>,
}

/// Attach types to all nodes in a program
pub fn type_check<Witness: ::std::fmt::Debug>(
    program: Vec<Node<Witness>>,
) -> Vec<TypedNode<Witness>> {
    if program.is_empty() {
        return vec![];
    }

    // Produce all powers of two as types
    let two_0 = Rc::new(RefCell::new(UnificationVar::concrete(Type::Unit)));
    let two_1 = Rc::new(RefCell::new(UnificationVar::concrete(Type::Sum(
        two_0.clone(),
        two_0.clone(),
    ))));
    let two_2 = Rc::new(RefCell::new(UnificationVar::concrete(Type::Product(
        two_1.clone(),
        two_1.clone(),
    ))));
    let two_4 = Rc::new(RefCell::new(UnificationVar::concrete(Type::Product(
        two_2.clone(),
        two_2.clone(),
    ))));
    let two_8 = Rc::new(RefCell::new(UnificationVar::concrete(Type::Product(
        two_4.clone(),
        two_4.clone(),
    ))));
    let two_16 = Rc::new(RefCell::new(UnificationVar::concrete(Type::Product(
        two_8.clone(),
        two_8.clone(),
    ))));
    let two_32 = Rc::new(RefCell::new(UnificationVar::concrete(Type::Product(
        two_16.clone(),
        two_16.clone(),
    ))));
    let two_64 = Rc::new(RefCell::new(UnificationVar::concrete(Type::Product(
        two_32.clone(),
        two_32.clone(),
    ))));
    let two_128 = Rc::new(RefCell::new(UnificationVar::concrete(Type::Product(
        two_64.clone(),
        two_64.clone(),
    ))));
    let two_256 = Rc::new(RefCell::new(UnificationVar::concrete(Type::Product(
        two_128.clone(),
        two_128.clone(),
    ))));
    let two_256_32 = Rc::new(RefCell::new(UnificationVar::concrete(Type::Product(
        two_256.clone(),
        two_32.clone(),
    ))));

    // Convenience closure for getting types for extensions
    let type_from_name = &|name: extension::TypeName| {
        match name {
            extension::TypeName::One => Type::Unit,
            extension::TypeName::Word32
                => Type::Product(two_16.clone(), two_16.clone()),
            extension::TypeName::SWord32
                => Type::Sum(two_0.clone(), two_32.clone()),
            extension::TypeName::Word64
                => Type::Product(two_32.clone(), two_32.clone()),
            extension::TypeName::SWord64
                => Type::Sum(two_0.clone(), two_64.clone()),
            extension::TypeName::Word256
                => Type::Product(two_128.clone(), two_128.clone()),
            extension::TypeName::SWord256
                => Type::Sum(two_0.clone(), two_256.clone()),
            extension::TypeName::Word256Word32
                => Type::Product(two_256.clone(), two_32.clone()),
            extension::TypeName::SWord256Word32
                => Type::Sum(two_0.clone(), two_256_32.clone()),
        }
    };

    let mut rcs = Vec::<Rc<UnificationArrow>>::with_capacity(program.len());
    let mut finals = Vec::<TypedNode<Witness>>::with_capacity(program.len());

    // Compute most general unifier for all types in the DAG
    for program_node in &program {
        let node = UnificationArrow {
            source: Rc::new(RefCell::new(UnificationVar::free())),
            target: Rc::new(RefCell::new(UnificationVar::free())),
        };

        match *program_node {
            Node::Iden => unify(node.source.clone(), node.target.clone()),
            Node::Unit => bind(&node.target, Type::Unit),
            Node::InjL(i) => {
                unify(node.source.clone(), rcs[i].source.clone());
                let target_type = Type::Sum(
                    rcs[i].target.clone(),
                    Rc::new(RefCell::new(UnificationVar::free())),
                );
                bind(&node.target, target_type);
            }
            Node::InjR(i) => {
                unify(node.source.clone(), rcs[i].source.clone());
                let target_type = Type::Sum(
                    Rc::new(RefCell::new(UnificationVar::free())),
                    rcs[i].target.clone(),
                );
                bind(&node.target, target_type);
            }
            Node::Take(i) => {
                unify(node.target.clone(), rcs[i].target.clone());
                let target_type = Type::Product(
                    rcs[i].source.clone(),
                    Rc::new(RefCell::new(UnificationVar::free())),
                );
                bind(&node.source, target_type);
            }
            Node::Drop(i) => {
                unify(node.target.clone(), rcs[i].target.clone());
                let target_type = Type::Product(
                    Rc::new(RefCell::new(UnificationVar::free())),
                    rcs[i].source.clone(),
                );
                bind(&node.source, target_type);
            }
            Node::Comp(i, j) => {
                unify(node.source.clone(), rcs[i].source.clone());
                unify(rcs[i].target.clone(), rcs[j].source.clone());
                unify(node.target.clone(), rcs[j].target.clone());
            }
            Node::Case(i, j) => {
                let var1 = Rc::new(RefCell::new(UnificationVar::free()));
                let var2 = Rc::new(RefCell::new(UnificationVar::free()));
                let var3 = Rc::new(RefCell::new(UnificationVar::free()));

                let sum12_ty = Type::Sum(var1.clone(), var2.clone());
                let sum12_var = Rc::new(RefCell::new(UnificationVar::free()));
                bind(&sum12_var, sum12_ty);

                let source_ty = Type::Product(sum12_var, var3.clone());
                bind(&node.source, source_ty);
                if let Node::Hidden(..) = program[i] {
                } else {
                    bind(
                        &find_root(rcs[i].source.clone()),
                        Type::Product(var1.clone(), var3.clone()),
                    );
                    unify(node.target.clone(), rcs[i].target.clone());
                }
                if let Node::Hidden(..) = program[j] {
                } else {
                    bind(
                        &find_root(rcs[j].source.clone()),
                        Type::Product(var2.clone(), var3.clone()),
                    );
                    unify(node.target.clone(), rcs[j].target.clone());
                }
            }
            Node::Pair(i, j) => {
                unify(node.source.clone(), rcs[i].source.clone());
                unify(node.source.clone(), rcs[j].source.clone());
                bind(
                    &node.target,
                    Type::Product(rcs[i].target.clone(), rcs[j].target.clone()),
                );
            }
            Node::Witness(..) => {
                // No type constraints
            }
            Node::Hidden(..) => {
                // No type constraints
            }
            Node::Bitcoin(ref bn) => {
                bind(&node.source, type_from_name(bn.source_type()));
                bind(&node.target, type_from_name(bn.target_type()));
            },
            ref x => unimplemented!("haven't implemented {:?}", x),
        };

        rcs.push(Rc::new(node));
    }

    // Finalize, setting all unconstrained types to `Unit` and doing the
    // occurs check. (All the magic happens inside `FinalType::from_var`.)
    for (idx, node) in program.into_iter().enumerate() {
        finals.push(TypedNode {
            node: node,
            source_ty: FinalType::from_var(rcs[idx].source.clone()),
            target_ty: FinalType::from_var(rcs[idx].target.clone()),
        });
    }

    finals
}