// SPDX-License-Identifier: CC0-1.0

//! Parsing

mod ast;

use crate::dag::{Dag, DagLike, InternalSharing};
use crate::jet::Jet;
use crate::node;
use crate::types::{self, Type};
use std::collections::HashMap;
use std::mem;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use super::named_node::{NamedCommitNode, NamedConstructNode, Namer};
use super::{Position, WitnessOrHole};

use super::{Error, ErrorSet};

#[derive(Clone)]
struct UnresolvedExpression<'brand, J: Jet> {
    inner: UnresolvedInner<'brand, J>,
    position: Position,
}

impl<'brand, J: Jet> UnresolvedExpression<'brand, J> {
    fn from_name(name: Arc<str>, position: Position) -> Self {
        UnresolvedExpression {
            inner: UnresolvedInner::NoExpr {
                name,
                user_source_types: vec![],
                user_target_types: vec![],
            },
            position,
        }
    }

    fn from_inline_expression(expr_inner: ast::ExprInner<J>, position: Position) -> Self {
        UnresolvedExpression {
            inner: UnresolvedInner::Inline { expr_inner },
            position,
        }
    }

    fn add_expression(
        &mut self,
        expr_inner: ast::ExprInner<J>,
        position: Position,
    ) -> Result<(), ErrorSet> {
        match self.inner {
            UnresolvedInner::NoExpr {
                ref name,
                ref mut user_source_types,
                ref mut user_target_types,
            } => {
                self.inner = UnresolvedInner::Named {
                    name: Arc::clone(name),
                    user_source_types: mem::take(user_source_types),
                    user_target_types: mem::take(user_target_types),
                    expr_inner,
                };
                Ok(())
            }
            UnresolvedInner::Inline { .. } => unreachable!(),
            UnresolvedInner::Named { ref name, .. } => Err(ErrorSet::single(
                position,
                Error::NameRepeated(Arc::clone(name)),
            )),
        }
    }

    fn add_source_type(&mut self, ty: Type<'brand>) {
        match self.inner {
            UnresolvedInner::NoExpr {
                ref mut user_source_types,
                ..
            } => {
                user_source_types.push(ty);
            }
            UnresolvedInner::Inline { .. } => unreachable!(),
            UnresolvedInner::Named {
                ref mut user_source_types,
                ..
            } => {
                user_source_types.push(ty);
            }
        }
    }

    fn add_target_type(&mut self, ty: Type<'brand>) {
        match self.inner {
            UnresolvedInner::NoExpr {
                ref mut user_target_types,
                ..
            } => {
                user_target_types.push(ty);
            }
            UnresolvedInner::Inline { .. } => unreachable!(),
            UnresolvedInner::Named {
                ref mut user_target_types,
                ..
            } => {
                user_target_types.push(ty);
            }
        }
    }
}

#[derive(Clone)]
enum UnresolvedInner<'brand, J: Jet> {
    NoExpr {
        name: Arc<str>,
        user_source_types: Vec<Type<'brand>>,
        user_target_types: Vec<Type<'brand>>,
    },
    Inline {
        expr_inner: ast::ExprInner<J>,
    },
    Named {
        name: Arc<str>,
        user_source_types: Vec<Type<'brand>>,
        user_target_types: Vec<Type<'brand>>,
        expr_inner: ast::ExprInner<J>,
    },
}

struct ResolvedExpression<'brand, J: Jet> {
    inner: ResolvedInner<'brand, J>,
    position: Position,

    name: Option<Arc<str>>,
    user_source_types: Arc<[Type<'brand>]>,
    user_target_types: Arc<[Type<'brand>]>,

    in_degree: AtomicUsize,
}

impl<'brand, J: Jet> DagLike for &'_ ResolvedExpression<'brand, J> {
    type Node = ResolvedExpression<'brand, J>;
    fn data(&self) -> &ResolvedExpression<'brand, J> {
        self
    }

    fn as_dag_node(&self) -> Dag<Self> {
        match self.inner {
            ResolvedInner::Missing { .. } | ResolvedInner::NoExpr { .. } => Dag::Nullary,
            ResolvedInner::Reference(ref child) => Dag::Unary(child),
            ResolvedInner::AssertL(ref left, ResolvedCmr::Expr(ref right))
            | ResolvedInner::AssertR(ResolvedCmr::Expr(ref left), ref right) => {
                Dag::Binary(left, right)
            }
            ResolvedInner::AssertL(ref child, ResolvedCmr::Literal)
            | ResolvedInner::AssertR(ResolvedCmr::Literal, ref child) => Dag::Unary(child),
            ResolvedInner::Inline(ref inner) => inner.as_dag().map(|node| node),
        }
    }
}

enum ResolvedCmr<'brand, J: Jet> {
    Expr(Arc<ResolvedExpression<'brand, J>>),
    Literal,
}

enum ResolvedInner<'brand, J: Jet> {
    /// A reference to a missing expression
    Missing { name: Arc<str> },
    /// A reference to a name with no associated expression
    NoExpr { name: Arc<str> },
    /// A reference to another expression
    Reference(Arc<ResolvedExpression<'brand, J>>),
    /// A left assertion (referring to the CMR of an expression on the right)
    AssertL(Arc<ResolvedExpression<'brand, J>>, ResolvedCmr<'brand, J>),
    /// A right assertion (referring to the CMR of an expression on the left)
    AssertR(ResolvedCmr<'brand, J>, Arc<ResolvedExpression<'brand, J>>),
    /// An inline expression
    Inline(
        node::Inner<
            Arc<ResolvedExpression<'brand, J>>,
            J,
            Arc<ResolvedExpression<'brand, J>>,
            WitnessOrHole,
        >,
    ),
}

pub fn parse<J: Jet + 'static>(
    program: &str,
) -> Result<HashMap<Arc<str>, Arc<NamedCommitNode<J>>>, ErrorSet> {
    types::Context::with_context(|ctx| parse_inner(ctx, program))
}

fn parse_inner<J: Jet + 'static>(
    inference_context: types::Context<'_>,
    program: &str,
) -> Result<HashMap<Arc<str>, Arc<NamedCommitNode<J>>>, ErrorSet> {
    let mut errors = ErrorSet::new();
    // **
    // Step 1: Read expressions into HashMap, checking for dupes and illegal names.
    // **
    let mut unresolved_map = HashMap::<Arc<str>, UnresolvedExpression<J>>::new();
    for line in ast::parse_line_vector(program)? {
        if line.name.as_ref() == "_" || line.name.starts_with("prim") {
            errors.add(line.position, Error::NameIllegal(Arc::clone(&line.name)));
            continue;
        }

        let entry =
            unresolved_map
                .entry(line.name.clone())
                .or_insert(UnresolvedExpression::from_name(
                    Arc::clone(&line.name),
                    line.position,
                ));

        if let Some(expr) = line.expression {
            if let Err(eset) = entry.add_expression(expr.inner, expr.position) {
                errors.merge(&eset)
            }
        }
        if let Some(ty) = line.arrow.0 {
            entry.add_source_type(ty.reify(&inference_context));
        }
        if let Some(ty) = line.arrow.1 {
            entry.add_target_type(ty.reify(&inference_context));
        }
    }

    // **
    // Step 2: Resolve every name.
    // We need to manually recurse rather than using `dag::PostOrderIter` because
    // we may have multiple disconnected components.
    // **
    let mut resolved_map =
        HashMap::<Arc<str>, Arc<ResolvedExpression<J>>>::with_capacity(unresolved_map.len());

    while let Some(name) = unresolved_map.keys().next() {
        let name = Arc::clone(name);
        let expr = unresolved_map.remove(&name).unwrap();

        #[derive(Clone)]
        struct StackItem<'brand, J: Jet> {
            expr: UnresolvedExpression<'brand, J>,
            name: Option<Arc<str>>,
            done_children: bool,
        }

        // Stack of unresolved entries, which are named expressions from `resolved_map`
        // as well as children of those named expressions (which themselves may be
        // either named or inline).
        let mut stack = vec![];
        // Stack of resolved inline entries. Somewhat confusingly, when resolving inline
        // expressions, we put them on the stack and use some careful accounting to make
        // sure that they're popped in the right place. When resolving named expressions,
        // we just stick the result into `resolved_map` and later refer to it by name.
        //
        // The reason we need to do both is that we are working with a "DAG forest" where
        // named expressions may be accessible from multiple roots. This means that when
        // we encounter a named expression, that expression might've already been resolved,
        // and there is no "intrinsic" way to distinguish these cases. So tracking state
        // using a stack would be pretty complicated.
        //
        // On the other hand, inline expressions do not have names or any other identifying
        // characteristics except the order in which they appear. So for these we need to
        // use the `inline_stack` to keep track of which ones we've already resolved..
        let mut inline_stack: Vec<Arc<ResolvedExpression<J>>> = vec![];
        stack.push(StackItem {
            expr,
            name: Some(Arc::clone(&name)),
            done_children: false,
        });

        while let Some(mut stack_item) = stack.pop() {
            // First, recurse on any children
            if !stack_item.done_children {
                stack_item.done_children = true;
                stack.push(stack_item.clone());

                let push_ast_expr = |stack: &mut Vec<_>, expr: &ast::Expression<_>| {
                    stack.push(StackItem {
                        expr: UnresolvedExpression::from_inline_expression(
                            expr.inner.clone(),
                            expr.position,
                        ),
                        name: None,
                        done_children: false,
                    })
                };

                match stack_item.expr.inner {
                    UnresolvedInner::NoExpr { .. } => {}
                    UnresolvedInner::Inline { ref expr_inner, .. }
                    | UnresolvedInner::Named { ref expr_inner, .. } => match expr_inner {
                        ast::ExprInner::Reference(ref ref_name) => {
                            if let Some(child) = unresolved_map.remove(ref_name) {
                                stack.push(StackItem {
                                    expr: child,
                                    name: Some(Arc::clone(ref_name)),
                                    done_children: false,
                                });
                            }
                        }
                        ast::ExprInner::AssertL(ref left, ref cmr) => {
                            push_ast_expr(&mut stack, left);
                            if let ast::AstCmr::Expr(ref right) = cmr {
                                push_ast_expr(&mut stack, right);
                            }
                        }
                        ast::ExprInner::AssertR(ref cmr, ref right) => {
                            if let ast::AstCmr::Expr(ref left) = cmr {
                                push_ast_expr(&mut stack, left);
                            }
                            push_ast_expr(&mut stack, right);
                        }
                        ast::ExprInner::Inline(ref inner) => match inner.as_dag() {
                            Dag::Nullary => {}
                            Dag::Unary(child) => push_ast_expr(&mut stack, child),
                            Dag::Binary(left, right) => {
                                push_ast_expr(&mut stack, left);
                                push_ast_expr(&mut stack, right);
                            }
                        },
                    },
                }
                continue;
            }

            let mut convert_expr_inner = |expr_inner: &ast::ExprInner<J>| match expr_inner {
                ast::ExprInner::Reference(ref ref_name) => {
                    if let Some(referent) = resolved_map.get(ref_name) {
                        referent.in_degree.fetch_add(1, Ordering::SeqCst);
                        ResolvedInner::Reference(Arc::clone(referent))
                    } else {
                        ResolvedInner::Missing {
                            name: Arc::clone(ref_name),
                        }
                    }
                }
                ast::ExprInner::AssertL(_, ref cmr) => {
                    let left = inline_stack.pop().unwrap();
                    left.in_degree.fetch_add(1, Ordering::SeqCst);

                    let right = match cmr {
                        ast::AstCmr::Expr(..) => {
                            let right = inline_stack.pop().unwrap();
                            right.in_degree.fetch_add(1, Ordering::SeqCst);
                            ResolvedCmr::Expr(right)
                        }
                        ast::AstCmr::Literal => ResolvedCmr::Literal,
                    };
                    ResolvedInner::AssertL(left, right)
                }
                ast::ExprInner::AssertR(ref cmr, _) => {
                    let left = match cmr {
                        ast::AstCmr::Expr(..) => {
                            let left = inline_stack.pop().unwrap();
                            left.in_degree.fetch_add(1, Ordering::SeqCst);
                            ResolvedCmr::Expr(left)
                        }
                        ast::AstCmr::Literal => ResolvedCmr::Literal,
                    };

                    let right = inline_stack.pop().unwrap();
                    right.in_degree.fetch_add(1, Ordering::SeqCst);
                    ResolvedInner::AssertR(left, right)
                }
                ast::ExprInner::Inline(ref inner) => ResolvedInner::Inline(
                    inner
                        .as_ref()
                        .map(|_| {
                            let child = inline_stack.pop().unwrap();
                            child.in_degree.fetch_add(1, Ordering::SeqCst);
                            child
                        })
                        .map_disconnect(|_| {
                            let child = inline_stack.pop().unwrap();
                            child.in_degree.fetch_add(1, Ordering::SeqCst);
                            child
                        })
                        .map_witness(WitnessOrHole::shallow_clone),
                ),
            };

            // Then, convert the node. At this point if we are missing any children
            // it is because there was a resolution error, i.e. the expression
            // references a child that doesn't exist.
            let resolved: ResolvedExpression<J> = match stack_item.expr.inner {
                UnresolvedInner::NoExpr {
                    ref name,
                    ref user_source_types,
                    ref user_target_types,
                } => ResolvedExpression {
                    inner: ResolvedInner::NoExpr {
                        name: Arc::clone(name),
                    },
                    position: stack_item.expr.position,
                    name: Some(Arc::clone(name)),
                    user_source_types: Arc::from(&user_source_types[..]),
                    user_target_types: Arc::from(&user_target_types[..]),
                    in_degree: AtomicUsize::new(0),
                },
                UnresolvedInner::Inline { ref expr_inner } => ResolvedExpression {
                    inner: convert_expr_inner(expr_inner),
                    position: stack_item.expr.position,
                    name: None,
                    user_source_types: Arc::from([]),
                    user_target_types: Arc::from([]),
                    in_degree: AtomicUsize::new(0),
                },
                UnresolvedInner::Named {
                    ref expr_inner,
                    ref name,
                    ref user_source_types,
                    ref user_target_types,
                } => ResolvedExpression {
                    inner: convert_expr_inner(expr_inner),
                    position: stack_item.expr.position,
                    name: Some(Arc::clone(name)),
                    user_source_types: Arc::from(&user_source_types[..]),
                    user_target_types: Arc::from(&user_target_types[..]),
                    in_degree: AtomicUsize::new(0),
                },
            };

            if let Some(name) = stack_item.name {
                resolved_map.insert(name, Arc::new(resolved));
            } else {
                inline_stack.push(Arc::new(resolved));
            }
        }
        assert!(inline_stack.is_empty());
    }
    drop(unresolved_map);

    // ** Step 3: convert each DAG of names/expressions into a DAG of NamedNodes.
    let mut roots = HashMap::<Arc<str>, Arc<NamedCommitNode<J>>>::new();
    for (name, expr) in &resolved_map {
        if expr.in_degree.load(Ordering::SeqCst) > 0 {
            continue;
        }

        let mut namer = Namer::new();
        let mut converted: Vec<Option<Arc<NamedConstructNode<J>>>> = vec![];
        for data in expr.as_ref().post_order_iter::<InternalSharing>() {
            let left = data
                .left_index
                .and_then(|idx| Option::<Arc<_>>::clone(&converted[idx]));
            let right = data
                .right_index
                .and_then(|idx| Option::<Arc<_>>::clone(&converted[idx]));

            let maybe_inner = match data.node.inner {
                ResolvedInner::Missing { ref name, .. } => {
                    errors.add(data.node.position, Error::NameMissing(Arc::clone(name)));
                    None
                }
                ResolvedInner::NoExpr { ref name, .. } => {
                    errors.add(data.node.position, Error::NameIncomplete(Arc::clone(name)));
                    None
                }
                ResolvedInner::Reference(..) => {
                    // For chains of references we make an effort to preserve the name.
                    // So if you have main := a; a := b; b := c, then the `main` node
                    // will retain the name `main` (and absent any other references,
                    // the `a` and `b` names will simply be dropped).
                    let mut child = left;
                    if let Some(name) = data.node.name.as_ref() {
                        child = child.map(|node| Arc::new(node.renamed(Arc::clone(name))));
                    }
                    converted.push(child);
                    continue;
                }
                ResolvedInner::AssertL(..) => left.zip(right).map(|(left, right)| {
                    let cmr = right.cmr();
                    node::Inner::AssertL(left, cmr)
                }),
                ResolvedInner::AssertR(..) => left.zip(right).map(|(left, right)| {
                    let cmr = left.cmr();
                    node::Inner::AssertR(cmr, right)
                }),
                ResolvedInner::Inline(ref inner) => inner
                    .as_ref()
                    .map_left_right(|_| left, |_| right.clone())
                    .map_disconnect(|_| right)
                    .map_witness(WitnessOrHole::shallow_clone)
                    .transpose_disconnect()
                    .and_then(node::Inner::transpose),
            };

            let inner = match maybe_inner {
                Some(inner) => inner,
                None => {
                    converted.push(None);
                    continue;
                }
            };

            let name = Option::<Arc<str>>::clone(&data.node.name)
                .unwrap_or_else(|| Arc::from(namer.assign_name(inner.as_ref()).as_str()));

            let node = NamedConstructNode::new(
                &inference_context,
                Arc::clone(&name),
                data.node.position,
                Arc::clone(&data.node.user_source_types),
                Arc::clone(&data.node.user_target_types),
                inner,
            );

            match node {
                Ok(node) => {
                    let node = Arc::new(node);
                    converted.push(Some(node));
                }
                Err(e) => {
                    errors.add(data.node.position, e);
                    converted.push(None);
                }
            }
        }

        if let Some(Some(root)) = converted.pop() {
            let finalized = if &**name == "main" {
                root.finalize_types_main()
            } else {
                root.finalize_types_non_main()
            };

            match finalized {
                Ok(root) => {
                    roots.insert(Arc::clone(name), root);
                }
                Err(errs) => errors.merge(&errs),
            }
        }
    }

    // From each root, count the number of ways that each witness node can be reached
    for root in roots.values() {
        let mut counts: Vec<HashMap<Arc<str>, usize>> = vec![];
        for data in root.as_ref().post_order_iter::<InternalSharing>() {
            let mut new_counts = HashMap::new();

            if let Some(idx) = data.left_index {
                for (name, count) in &counts[idx] {
                    *new_counts.entry(Arc::clone(name)).or_insert(0) += count;
                }
            }
            if let Some(idx) = data.right_index {
                for (name, count) in &counts[idx] {
                    *new_counts.entry(Arc::clone(name)).or_insert(0) += count;
                }
            }

            if matches!(
                data.node.inner(),
                node::Inner::Disconnect(_, _) | node::Inner::Witness(_)
            ) {
                *new_counts.entry(Arc::clone(data.node.name())).or_insert(0) += 1;
            }

            counts.push(new_counts);
        }

        let accumulated = counts.last().unwrap();
        for (name, count) in accumulated {
            if *count > 1 {
                errors.add_no_position(Error::WitnessDisconnectRepeated {
                    name: Arc::clone(name),
                    count: *count,
                });
            }
        }
    }

    if errors.is_empty() {
        Ok(roots)
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::dag::MaxSharing;
    use crate::human_encoding::Forest;
    use crate::jet::{Core, Jet};
    use crate::node::Inner;
    use crate::value::Word;
    use crate::{BitMachine, Value};

    fn assert_cmr_witness<J: Jet, T: core::borrow::Borrow<J::Transaction>>(
        s: &str,
        cmr: &str,
        witness: &HashMap<Arc<str>, Value>,
        env: &J::Environment<T>,
    ) {
        match parse::<J>(s) {
            Ok(forest) => {
                assert_eq!(forest.len(), 1);
                let main = &forest["main"];
                assert_eq!(main.cmr().to_string(), cmr);

                let program = types::Context::with_context(|ctx| {
                    main.to_construct_node(&ctx, witness, &forest)
                        .finalize_unpruned()
                        .expect("finalize")
                });

                let mut mac =
                    BitMachine::for_program(&program).expect("program has reasonable bounds");
                mac.exec(&program, env).expect("execute");
            }
            Err(errs) => {
                println!("\nErrors:\n{}", errs);
                panic!("Failed to parse program:\n{}", s);
            }
        }
    }

    fn error_contains<T>(res: Result<T, ErrorSet>, s: &str) {
        match res {
            Ok(_) => panic!("expected error, but result was Ok"),
            Err(errs) => {
                let e_msg = errs.to_string();
                assert!(
                    e_msg.contains(s),
                    "Did not find search string {} in error string: {}",
                    s,
                    e_msg,
                );
            }
        }
    }

    fn assert_const<J: Jet>(s: &str, word: Word) {
        match parse::<J>(s) {
            Ok(forest) => {
                assert_eq!(forest.len(), 1);
                let main = &forest["main"];

                for data in main.clone().post_order_iter::<MaxSharing<_>>() {
                    if let Inner::Word(parsed_word) = data.node.inner() {
                        assert_eq!(&word, parsed_word);
                    }
                }
            }
            Err(errs) => {
                println!("\nErrors:\n{}", errs);
                panic!("Failed to parse program:\n{}", s);
            }
        }
    }

    #[test]
    fn errors() {
        error_contains(
            parse::<Core>("main := a"),
            "name `a` is referred to but does not exist",
        );
        error_contains(
            parse::<Core>("main := unit : 1 -> 2"),
            "failed to apply bound `2` to existing bound `1`",
        );
        error_contains(
            parse::<Core>("main := pair unit unit"),
            "failed to apply bound `1 × 1` to existing bound `1`",
        );
    }

    #[test]
    fn simple_program() {
        let empty = HashMap::new();
        assert_cmr_witness::<Core, _>(
            "main := unit",
            "c40a10263f7436b4160acbef1c36fba4be4d95df181a968afeab5eac247adff7",
            &empty,
            &crate::jet::CoreEnv::EMPTY,
        );

        let witness = HashMap::from([
            (Arc::from("wit1"), Value::u32(0x00010203)),
            (Arc::from("wit2"), Value::u32(0x00010203)),
        ]);
        assert_cmr_witness::<Core, _>(
            "
                wit1 := witness : 1 -> 2^32
                wit2 := witness : 1 -> 2^32

                wits_are_equal := comp (pair wit1 wit2) jet_eq_32 : 1 -> 2
                main := comp wits_are_equal jet_verify            : 1 -> 1
            ",
            "d7969920eff9a1ed0359aaa8545b239c69969e22c304c645a7b49bcc976a40a8",
            &witness,
            &crate::jet::CoreEnv::EMPTY,
        );
    }

    #[test]
    fn circular_program() {
        error_contains(
            parse::<Core>("main := comp main unit"),
            "name `main` is referred to but does not exist",
        );
    }

    #[test]
    fn preserve_name() {
        let program = &Forest::<Core>::parse("main := x    x := unit").unwrap();
        assert!(program.string_serialize().contains("main := unit"));
    }

    #[test]
    #[cfg(feature = "elements")]
    fn bip340_program() {
        use crate::jet::elements::ElementsEnv;
        use crate::jet::Elements;

        let empty = HashMap::new();
        let dummy = ElementsEnv::dummy();
        assert_cmr_witness::<Elements, _>(
            "main := unit",
            "c40a10263f7436b4160acbef1c36fba4be4d95df181a968afeab5eac247adff7",
            &empty,
            &dummy,
        );

        // See https://github.com/bitcoin/bips/blob/master/bip-0340/test-vectors.csv
        let sig = [
            0xe9, 0x07, 0x83, 0x1f, 0x80, 0x84, 0x8d, 0x10, 0x69, 0xa5, 0x37, 0x1b, 0x40, 0x24,
            0x10, 0x36, 0x4b, 0xdf, 0x1c, 0x5f, 0x83, 0x07, 0xb0, 0x08, 0x4c, 0x55, 0xf1, 0xce,
            0x2d, 0xca, 0x82, 0x15, 0x25, 0xf6, 0x6a, 0x4a, 0x85, 0xea, 0x8b, 0x71, 0xe4, 0x82,
            0xa7, 0x4f, 0x38, 0x2d, 0x2c, 0xe5, 0xeb, 0xee, 0xe8, 0xfd, 0xb2, 0x17, 0x2f, 0x47,
            0x7d, 0xf4, 0x90, 0x0d, 0x31, 0x05, 0x36, 0xc0,
        ];

        let signature = HashMap::from([(Arc::from("wit1"), Value::u512(sig))]);
        assert_cmr_witness::<Elements, _>(
            "
                -- Witnesses
                wit1 := witness : 1 -> 2^512

                -- Constants
                const1 := const 0xf9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f90000000000000000000000000000000000000000000000000000000000000000 : 1 -> 2^512 -- ef40cb78

                -- Program code
                pr1 := pair const1 wit1    : 1 -> (2^512 * 2^512) -- 8080ed5e
                jt2 := jet_bip_0340_verify : (2^512 * 2^512) -> 1 -- af924cbe

                main := comp pr1 jt2       : 1 -> 1               -- 3f6422da
            ",
           "8a9e97676b24be7797d9ee0bf32dd76bcd78028e973025f785eae8dc91c8a0da",
            &signature,
            &dummy
        );
    }

    #[test]
    fn const_word() {
        let human_words = [
            ("0b0", Word::u1(0b0)),
            ("0b1", Word::u1(0b1)),
            ("0b00", Word::u2(0b00)),
            ("0b11", Word::u2(0b11)),
            ("0b0000", Word::u4(0b0000)),
            ("0b1111", Word::u4(0b1111)),
            ("0b00000000", Word::u8(0b00000000)),
            ("0b11111111", Word::u8(0b11111111)),
            (
                "0b00000001001000110100010101100111",
                Word::u32(u32::from_be_bytes([
                    0b00000001, 0b00100011, 0b01000101, 0b01100111,
                ])),
            ),
            ("0x0", Word::u4(0x0)),
            ("0xf", Word::u4(0xf)),
            ("0x00", Word::u8(0x00)),
            ("0xff", Word::u8(0xff)),
            (
                "0xdeadbeef",
                Word::u32(u32::from_be_bytes([0xde, 0xad, 0xbe, 0xef])),
            ),
        ];

        for (human, word) in human_words {
            let s = format!("main := comp const {human} unit");
            assert_const::<Core>(s.as_str(), word);
        }
    }

    #[test]
    fn duplicate_witness_in_disconnected_branch() {
        error_contains(
            parse::<Core>(
                "
                wit1 := witness
                main := comp wit1 comp disconnect iden ?dis2 unit

                wit1 := witness
                dis2 := wit1
            ",
            ),
            "name `wit1` occured mulitple times",
        );
    }
}
