// SPDX-License-Identifier: CC0-1.0

//! The Simplicity Human-Readable Encoding
//!
//! This module provides the ability to decode and encode [`NamedCommitNode`]s
//! in a human-readable format.
//!

mod error;
mod named_node;
mod parse;

use crate::dag::{DagLike, MaxSharing};
use crate::jet::Jet;
use crate::node::{self, CommitNode, NoWitness};
use crate::types;
use crate::{Cmr, ConstructNode, Ihr, Value};

use std::collections::HashMap;
use std::str;
use std::sync::Arc;

pub use self::error::{Error, ErrorSet};
pub use self::named_node::NamedCommitNode;

/// Line/column pair
///
/// There is a similar type provided by the `santiago` library but it does not implement
/// `Copy`, among many other traits, which makes it unergonomic to use. Santiago positions
/// can be converted using `.into()`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Position {
    line: usize,
    column: usize,
}

impl<'a> From<&'a santiago::lexer::Position> for Position {
    fn from(position: &'a santiago::lexer::Position) -> Position {
        Position {
            line: position.line,
            column: position.column,
        }
    }
}

impl From<santiago::lexer::Position> for Position {
    fn from(position: santiago::lexer::Position) -> Position {
        Position {
            line: position.line,
            column: position.column,
        }
    }
}

/// For named construct nodes, we abuse the `witness` combinator to support typed holes.
///
/// We do this because `witness` nodes have free source and target arrows, and
/// allow us to store arbitrary data in them using the generic witness type of
/// the node. Holes are characterized entirely by their source and target type,
/// just like witnesses, and are labelled by their name.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum WitnessOrHole {
    /// This witness is an actual witness combinator (with no witness data attached,
    /// that comes later)
    Witness,
    /// This is a typed hole, with the given name.
    TypedHole(Arc<str>),
}

impl WitnessOrHole {
    pub fn shallow_clone(&self) -> Self {
        match self {
            WitnessOrHole::Witness => WitnessOrHole::Witness,
            WitnessOrHole::TypedHole(name) => WitnessOrHole::TypedHole(Arc::clone(name)),
        }
    }
}

impl From<&'_ NoWitness> for WitnessOrHole {
    fn from(_: &NoWitness) -> Self {
        WitnessOrHole::Witness
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Forest<J: Jet> {
    roots: HashMap<Arc<str>, Arc<NamedCommitNode<J>>>,
}

impl<J: Jet> Forest<J> {
    /// Parses a forest from a string
    pub fn parse(s: &str) -> Result<Self, ErrorSet> {
        parse::parse(s).map(|roots| Forest { roots })
    }

    /// Parses a program from a bytestring
    pub fn from_program(root: Arc<CommitNode<J>>) -> Self {
        let root = NamedCommitNode::from_node(&root);
        let mut roots = HashMap::new();
        roots.insert("main".into(), root);
        Forest { roots }
    }

    /// Accessor for the map of roots of this forest
    pub fn roots(&self) -> &HashMap<Arc<str>, Arc<NamedCommitNode<J>>> {
        &self.roots
    }

    /// Serialize the program in human-readable form
    pub fn string_serialize(&self) -> String {
        struct Print {
            cmr: Cmr,
            ihr: Option<Ihr>,
            expr_str: String,  // The X = Y part
            arrow_str: String, // The :: A -> B part
        }
        fn print_lines(lines: &[Print], skip_before_end: bool) -> String {
            let mut ret = String::new();
            let expr_width = lines.iter().map(|line| line.expr_str.len()).max().unwrap();
            let arrow_width = lines.iter().map(|line| line.arrow_str.len()).max().unwrap();
            let last_line = lines.len();
            for (n, line) in lines.iter().enumerate() {
                ret += "\n";
                if skip_before_end && n == last_line - 1 {
                    ret += "\n";
                }
                ret += &format!("-- CMR: {}\n", line.cmr);
                if let Some(ihr) = line.ihr {
                    ret += &format!("-- IHR: {}\n", ihr);
                } else {
                    ret += "-- IHR: [undetermined]\n";
                }
                ret += &format!(
                    "{0:1$} {2:3$}\n",
                    line.expr_str, expr_width, line.arrow_str, arrow_width,
                );
            }
            ret
        }

        let mut witness_lines = vec![];
        let mut const_lines = vec![];
        let mut program_lines = vec![];
        // Pass 1: compute string data for every node
        for root in self.roots.values() {
            for data in root.as_ref().post_order_iter::<MaxSharing<_>>() {
                let node = data.node;
                let name = node.name();
                let mut expr_str = match node.inner() {
                    node::Inner::AssertR(cmr, _) => format!("{} := assertr #{}", name, cmr),
                    node::Inner::Fail(entropy) => format!("{} := fail {}", name, entropy),
                    node::Inner::Jet(ref j) => format!("{} := jet_{}", name, j),
                    node::Inner::Word(ref word) => {
                        format!("{} := const {}", name, word)
                    }
                    inner => format!("{} := {}", name, inner),
                };
                if let Some(child) = node.left_child() {
                    expr_str.push(' ');
                    expr_str.push_str(child.name());
                }
                if let Some(child) = node.right_child() {
                    expr_str.push(' ');
                    expr_str.push_str(child.name());
                } else if let node::Inner::AssertL(_, cmr) = node.inner() {
                    expr_str.push_str(" #");
                    expr_str.push_str(&cmr.to_string());
                } else if let node::Inner::Disconnect(_, hole_name) = node.inner() {
                    expr_str.push_str(&format!(" ?{}", hole_name));
                }

                let arrow = node.arrow();
                let arrow_str = format!(": {} -> {}", arrow.source, arrow.target).replace('×', "*"); // for human-readable encoding stick with ASCII

                let print = Print {
                    cmr: node.cmr(),
                    ihr: node.ihr(),
                    expr_str,
                    arrow_str,
                };
                if let node::Inner::Witness(..) = node.inner() {
                    witness_lines.push(print);
                } else if let node::Inner::Word(..) = node.inner() {
                    const_lines.push(print);
                } else {
                    program_lines.push(print);
                }
            }
        }

        // Pass 2: actually print everything
        let mut ret = String::new();
        if !witness_lines.is_empty() {
            ret += "--------------\n-- Witnesses\n--------------\n";
            ret += &print_lines(&witness_lines, false);
            ret += "\n";
        }
        if !const_lines.is_empty() {
            // FIXME detect scribes
            ret += "--------------\n-- Constants\n--------------\n";
            ret += &print_lines(&const_lines, false);
            ret += "\n";
        }
        if !program_lines.is_empty() {
            ret += "--------------\n-- Program code\n--------------\n";
            ret += &print_lines(&program_lines, true /* add a blank line before main */);
        }
        ret
    }

    /// Convert the forest into a witness node.
    ///
    /// Succeeds if the forest contains a "main" root and returns `None` otherwise.
    pub fn to_witness_node<'brand>(
        &self,
        inference_context: &types::Context<'brand>,
        witness: &HashMap<Arc<str>, Value>,
    ) -> Option<Arc<ConstructNode<'brand, J>>> {
        let main = self.roots.get("main")?;
        Some(main.to_construct_node(inference_context, witness, self.roots()))
    }
}

#[cfg(test)]
mod tests {
    use crate::human_encoding::Forest;
    use crate::jet::{Core, Jet};
    use crate::types;
    use crate::{BitMachine, Value};
    use core::borrow::Borrow;
    use std::collections::HashMap;
    use std::sync::Arc;

    fn assert_finalize_ok<J: Jet, T: Borrow<J::Transaction>>(
        s: &str,
        witness: &HashMap<Arc<str>, Value>,
        env: &J::Environment<T>,
    ) {
        types::Context::with_context(|ctx| {
            let program = Forest::<J>::parse(s)
                .expect("Failed to parse human encoding")
                .to_witness_node(&ctx, witness)
                .expect("Forest is missing expected root")
                .finalize_pruned(env)
                .expect("Failed to finalize");
            let mut mac = BitMachine::for_program(&program).expect("program has reasonable bounds");
            mac.exec(&program, env).expect("Failed to run program");
        });
    }

    fn assert_finalize_err<J: Jet, T: Borrow<J::Transaction>>(
        s: &str,
        witness: &HashMap<Arc<str>, Value>,
        env: &J::Environment<T>,
        err_msg: &'static str,
    ) {
        types::Context::with_context(|ctx| {
            let program = match Forest::<J>::parse(s)
                .expect("Failed to parse human encoding")
                .to_witness_node(&ctx, witness)
                .expect("Forest is missing expected root")
                .finalize_pruned(env)
            {
                Ok(program) => program,
                Err(error) => {
                    assert_eq!(&error.to_string(), err_msg);
                    return;
                }
            };
            let mut mac = BitMachine::for_program(&program).expect("program has reasonable bounds");
            match mac.exec(&program, env) {
                Ok(_) => panic!("Execution is expected to fail"),
                Err(error) => assert_eq!(&error.to_string(), err_msg),
            }
        });
    }

    #[test]
    fn executed_witness_with_value() {
        let s = "
            a := witness
            b := witness
            main := comp
                comp
                    pair a b
                    jet_lt_8
                jet_verify
        ";

        let a_less_than_b = HashMap::from([
            (Arc::from("a"), Value::u8(0x00)),
            (Arc::from("b"), Value::u8(0x01)),
        ]);
        assert_finalize_ok::<Core, _>(s, &a_less_than_b, &crate::jet::CoreEnv::EMPTY);

        let b_greater_equal_a = HashMap::from([
            (Arc::from("a"), Value::u8(0x01)),
            (Arc::from("b"), Value::u8(0x01)),
        ]);
        assert_finalize_err::<Core, _>(s, &b_greater_equal_a, &crate::jet::CoreEnv::EMPTY, "Jet failed during execution");
    }

    #[test]
    fn executed_witness_without_value() {
        let witness = HashMap::from([(Arc::from("wit1"), Value::u32(1337))]);
        assert_finalize_err::<Core, _>(
            "
                wit1 := witness : 1 -> 2^32
                wit2 := witness : 1 -> 2^32

                wits_are_equal := comp (pair wit1 wit2) jet_eq_32 : 1 -> 2
                main := comp wits_are_equal jet_verify            : 1 -> 1
            ",
            &witness,
            &crate::jet::CoreEnv::EMPTY,
            "Jet failed during execution",
        );
    }

    #[test]
    fn pruned_witness_without_value() {
        let s = "
            wit1 := witness : 1 -> 2
            wit2 := witness : 1 -> 2^64
            input := pair wit1 unit : 1 -> 2 * 1
            process := case (drop injr unit) (drop comp wit2 jet_all_64) : 2 * 1 -> 2
            main := comp input comp process jet_verify : 1 -> 1
        ";
        let wit2_is_pruned = HashMap::from([(Arc::from("wit1"), Value::u1(0))]);
        assert_finalize_ok::<Core, _>(s, &wit2_is_pruned, &crate::jet::CoreEnv::EMPTY);

        let wit2_is_missing = HashMap::from([(Arc::from("wit1"), Value::u1(1))]);
        assert_finalize_err::<Core, _>(s, &wit2_is_missing, &crate::jet::CoreEnv::EMPTY, "Jet failed during execution");

        let wit2_is_present = HashMap::from([
            (Arc::from("wit1"), Value::u1(1)),
            (Arc::from("wit2"), Value::u64(u64::MAX)),
        ]);
        assert_finalize_ok::<Core, _>(s, &wit2_is_present, &crate::jet::CoreEnv::EMPTY);
    }

    #[test]
    fn executed_hole_with_value() {
        let empty = HashMap::new();
        assert_finalize_ok::<Core, _>(
            "
                id1 := iden : 2^256 * 1 -> 2^256 * 1
                main := comp (disconnect id1 ?hole) unit
                hole := unit
            ",
            &empty,
            &crate::jet::CoreEnv::EMPTY,
        );
    }

    #[test]
    fn executed_hole_without_value() {
        let empty = HashMap::new();
        assert_finalize_err::<Core, _>(
            "
                wit1 := witness
                main := comp wit1 comp disconnect iden ?dis2 unit
            ",
            &empty,
            &crate::jet::CoreEnv::EMPTY,
            "disconnect node had one child (redeem time); must have two",
        );
    }

    #[test]
    fn witness_name_override() {
        let s = "
            wit1 := witness : 1 -> 2
            wit2 := wit1 : 1 -> 2
            main := comp wit2 jet_verify : 1 -> 1
        ";
        let wit1_populated = HashMap::from([(Arc::from("wit1"), Value::u1(1))]);
        assert_finalize_err::<Core, _>(s, &wit1_populated, &crate::jet::CoreEnv::EMPTY, "Jet failed during execution");

        let wit2_populated = HashMap::from([(Arc::from("wit2"), Value::u1(1))]);
        assert_finalize_ok::<Core, _>(s, &wit2_populated, &crate::jet::CoreEnv::EMPTY);
    }
}
