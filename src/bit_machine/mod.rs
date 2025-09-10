// SPDX-License-Identifier: CC0-1.0

//! Simplicity Execution
//!
//! Implementation of the Bit Machine, without TCO, as TCO precludes some
//! frame management optimizations which can be used to great benefit.
//!

mod frame;
mod limits;

use core::borrow::Borrow;
use std::collections::HashSet;
use std::error;
use std::fmt;
use std::sync::Arc;

use crate::jet::{Jet, JetFailed};
use crate::node::{self, RedeemNode};
use crate::types::Final;
use crate::{analysis, Ihr};
use crate::{Cmr, FailEntropy, Value};
use frame::Frame;
use simplicity_sys::ffi::UWORD;

pub use self::limits::LimitError;

/// An execution context for a Simplicity program
pub struct BitMachine {
    /// Space for bytes that read and write frames point to.
    /// (De)allocation happens LIFO from left to right
    data: Vec<u8>,
    /// Top of data stack; index of first unused bit
    next_frame_start: usize,
    /// Read frame stack
    read: Vec<Frame>,
    /// Write frame stack
    write: Vec<Frame>,
    /// Acceptable source type
    source_ty: Arc<Final>,
}

impl BitMachine {
    /// Construct a Bit Machine with enough space to execute the given program.
    pub fn for_program<J: Jet>(program: &RedeemNode<J>) -> Result<Self, LimitError> {
        LimitError::check_program(program)?;
        let io_width = program.arrow().source.bit_width() + program.arrow().target.bit_width();

        Ok(Self {
            data: vec![0; (io_width + program.bounds().extra_cells).div_ceil(8)],
            next_frame_start: 0,
            read: Vec::with_capacity(program.bounds().extra_frames + analysis::IO_EXTRA_FRAMES),
            write: Vec::with_capacity(program.bounds().extra_frames + analysis::IO_EXTRA_FRAMES),
            source_ty: program.arrow().source.clone(),
        })
    }

    #[cfg(test)]
    pub fn test_exec<J: Jet, T: Borrow<J::Transaction>>(
        program: Arc<crate::node::ConstructNode<J>>,
        env: &J::Environment<T>,
    ) -> Result<Value, ExecutionError> {
        use crate::node::SimpleFinalizer;

        let prog = program
            .finalize_types_non_program()
            .expect("finalizing types")
            .finalize(&mut SimpleFinalizer::new(None.into_iter()))
            .expect("finalizing");
        let mut mac = BitMachine::for_program(&prog).expect("program has reasonable bounds");
        mac.exec(&prog, env)
    }

    /// Push a new frame of given size onto the write frame stack
    fn new_frame(&mut self, len: usize) {
        debug_assert!(
            self.next_frame_start + len <= self.data.len() * 8,
            "Data out of bounds: number of cells"
        );
        debug_assert!(
            self.write.len() + self.read.len() < self.read.capacity(),
            "Stacks out of bounds: number of frames"
        );

        self.write.push(Frame::new(self.next_frame_start, len));
        self.next_frame_start += len;
    }

    /// Move the active write frame to the read frame stack
    fn move_frame(&mut self) {
        let mut _active_write_frame = self.write.pop().unwrap();
        _active_write_frame.reset_cursor();
        self.read.push(_active_write_frame);
    }

    /// Drop the active read frame
    fn drop_frame(&mut self) {
        let active_read_frame = self.read.pop().unwrap();
        self.next_frame_start -= active_read_frame.bit_width();
        assert_eq!(self.next_frame_start, active_read_frame.start());
    }

    /// Write a single bit to the active write frame
    fn write_bit(&mut self, bit: bool) {
        self.write
            .last_mut()
            .expect("Empty write frame stack")
            .write_bit(bit, &mut self.data);
    }

    /// Move the cursor of the active write frame forward by
    /// a specified number of bits
    fn skip(&mut self, n: usize) {
        // short circuit n = 0
        if n == 0 {
            return;
        }
        let idx = self.write.len() - 1;
        self.write[idx].move_cursor_forward(n);
    }

    /// Copy the given number of bits from the active read frame
    /// to the active write frame
    fn copy(&mut self, n: usize) {
        // short circuit n = 0
        if n == 0 {
            return;
        }
        let widx = self.write.len() - 1;
        let ridx = self.read.len() - 1;
        self.write[widx].copy_from(&self.read[ridx], n, &mut self.data);
    }

    /// Move the cursor of the active read frame forward
    /// by the given number of bits
    fn fwd(&mut self, n: usize) {
        // short circuit n = 0
        if n == 0 {
            return;
        }
        let idx = self.read.len() - 1;
        self.read[idx].move_cursor_forward(n);
    }

    /// Move the cursor of the active read frame back
    /// by the given number of bits
    fn back(&mut self, n: usize) {
        // short circuit n = 0
        if n == 0 {
            return;
        }
        let idx = self.read.len() - 1;
        self.read[idx].move_cursor_backward(n);
    }

    /// Write a big-endian u8 value to the active write frame
    fn write_u8(&mut self, value: u8) {
        self.write
            .last_mut()
            .expect("Empty write frame stack")
            .write_u8(value, &mut self.data);
    }

    /// Read a bit from the active read frame
    fn read_bit(&mut self) -> bool {
        self.read
            .last_mut()
            .expect("Empty read frame stack")
            .read_bit(&self.data)
    }

    /// Write a bit string to the active write frame
    fn write_bytes(&mut self, bytes: &[u8]) {
        for bit in bytes {
            self.write_u8(*bit);
        }
    }

    /// Write a value to the current write frame
    fn write_value(&mut self, val: &Value) {
        for bit in val.iter_padded() {
            self.write_bit(bit);
        }
    }

    /// Return the bit width of the active read frame.
    fn active_read_bit_width(&self) -> usize {
        self.read.last().map(|frame| frame.bit_width()).unwrap_or(0)
    }

    /// Return the bit width of the active write frame.
    fn active_write_bit_width(&self) -> usize {
        self.write
            .last()
            .map(|frame| frame.bit_width())
            .unwrap_or(0)
    }

    /// Add a read frame with some given value in it, as input to the
    /// program
    pub fn input(&mut self, input: &Value) -> Result<(), ExecutionError> {
        if !input.is_of_type(&self.source_ty) {
            return Err(ExecutionError::InputWrongType(self.source_ty.clone()));
        }
        // Unit value doesn't need extra frame
        if !input.is_empty() {
            self.new_frame(input.padded_len());
            self.write_value(input);
            self.move_frame();
        }
        Ok(())
    }

    /// Execute the given `program` on the Bit Machine, using the given environment.
    ///
    ///  ## Precondition
    ///
    /// The Bit Machine is constructed via [`Self::for_program()`] to ensure enough space.
    pub fn exec<J: Jet, Tx: Borrow<J::Transaction>>(
        &mut self,
        program: &RedeemNode<J>,
        env: &J::Environment<Tx>,
    ) -> Result<Value, ExecutionError> {
        self.exec_with_tracker(program, env, &mut NoTracker)
    }

    /// Execute the given `program` on the Bit Machine and track executed case branches.
    ///
    /// If the program runs successfully, then two sets of IHRs are returned:
    ///
    /// 1) The IHRs of case nodes whose _left_ branch was executed.
    /// 2) The IHRs of case nodes whose _right_ branch was executed.
    ///
    /// ## Precondition
    ///
    /// The Bit Machine is constructed via [`Self::for_program()`] to ensure enough space.
    pub(crate) fn exec_prune<J: Jet, Tx: Borrow<J::Transaction>>(
        &mut self,
        program: &RedeemNode<J>,
        env: &J::Environment<Tx>,
    ) -> Result<SetTracker, ExecutionError> {
        let mut tracker = SetTracker::default();
        self.exec_with_tracker(program, env, &mut tracker)?;
        Ok(tracker)
    }

    /// Execute the given `program` on the Bit Machine, using the given environment and tracker.
    ///
    ///  ## Precondition
    ///
    /// The Bit Machine is constructed via [`Self::for_program()`] to ensure enough space.
    pub fn exec_with_tracker<J: Jet, T: ExecTracker<J>, Tx: Borrow<J::Transaction>>(
        &mut self,
        program: &RedeemNode<J>,
        env: &J::Environment<Tx>,
        tracker: &mut T,
    ) -> Result<Value, ExecutionError> {
        enum CallStack<'a, J: Jet> {
            Goto(&'a RedeemNode<J>),
            MoveFrame,
            DropFrame,
            CopyFwd(usize),
            Back(usize),
        }

        // Not used, but useful for debugging, so keep it around
        impl<J: Jet> fmt::Debug for CallStack<'_, J> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    CallStack::Goto(ins) => write!(f, "goto {}", ins.inner()),
                    CallStack::MoveFrame => f.write_str("move frame"),
                    CallStack::DropFrame => f.write_str("drop frame"),
                    CallStack::CopyFwd(n) => write!(f, "copy/fwd {}", n),
                    CallStack::Back(n) => write!(f, "back {}", n),
                }
            }
        }

        if self.read.is_empty() != self.source_ty.is_empty() {
            return Err(ExecutionError::InputWrongType(self.source_ty.clone()));
        }

        let mut ip = program;
        let mut call_stack = vec![];

        let output_width = ip.arrow().target.bit_width();
        if output_width > 0 {
            self.new_frame(output_width);
        }

        'main_loop: loop {
            match ip.inner() {
                node::Inner::Unit => {}
                node::Inner::Iden => {
                    let size_a = ip.arrow().source.bit_width();
                    self.copy(size_a);
                }
                node::Inner::InjL(left) => {
                    let (b, c) = ip.arrow().target.as_sum().unwrap();
                    self.write_bit(false);
                    self.skip(b.pad_left(c));
                    call_stack.push(CallStack::Goto(left));
                }
                node::Inner::InjR(left) => {
                    let (b, c) = ip.arrow().target.as_sum().unwrap();
                    self.write_bit(true);
                    self.skip(b.pad_right(c));
                    call_stack.push(CallStack::Goto(left));
                }
                node::Inner::Pair(left, right) => {
                    call_stack.push(CallStack::Goto(right));
                    call_stack.push(CallStack::Goto(left));
                }
                node::Inner::Comp(left, right) => {
                    let size_b = left.arrow().target.bit_width();

                    self.new_frame(size_b);
                    call_stack.push(CallStack::DropFrame);
                    call_stack.push(CallStack::Goto(right));
                    call_stack.push(CallStack::MoveFrame);
                    call_stack.push(CallStack::Goto(left));
                }
                node::Inner::Disconnect(left, right) => {
                    let size_prod_256_a = left.arrow().source.bit_width();
                    let size_a = size_prod_256_a - 256;
                    let size_prod_b_c = left.arrow().target.bit_width();
                    let size_b = size_prod_b_c - right.arrow().source.bit_width();

                    self.new_frame(size_prod_256_a);
                    self.write_bytes(right.cmr().as_ref());
                    self.copy(size_a);
                    self.move_frame();
                    self.new_frame(size_prod_b_c);

                    // Remember that call stack pushes are executed in reverse order
                    call_stack.push(CallStack::DropFrame);
                    call_stack.push(CallStack::DropFrame);
                    call_stack.push(CallStack::Goto(right));
                    call_stack.push(CallStack::CopyFwd(size_b));
                    call_stack.push(CallStack::MoveFrame);
                    call_stack.push(CallStack::Goto(left));
                }
                node::Inner::Take(left) => call_stack.push(CallStack::Goto(left)),
                node::Inner::Drop(left) => {
                    let size_a = ip.arrow().source.as_product().unwrap().0.bit_width();
                    self.fwd(size_a);
                    call_stack.push(CallStack::Back(size_a));
                    call_stack.push(CallStack::Goto(left));
                }
                node::Inner::Case(..) | node::Inner::AssertL(..) | node::Inner::AssertR(..) => {
                    let in_frame = &self.read[self.read.len() - 1];
                    let choice_bit: bool = in_frame.peek_bit(&self.data);

                    let (sum_a_b, _c) = ip.arrow().source.as_product().unwrap();
                    let (a, b) = sum_a_b.as_sum().unwrap();

                    if tracker.is_track_debug_enabled() {
                        if let node::Inner::AssertL(_, cmr) = ip.inner() {
                            let mut bits = in_frame.as_bit_iter(&self.data);
                            // Skips 1 + max(a.bit_width, b.bit_width) - a.bit_width
                            bits.nth(a.pad_left(b))
                                .expect("AssertL: unexpected end of frame");
                            let value = Value::from_padded_bits(&mut bits, _c)
                                .expect("AssertL: decode `C` value");
                            tracker.track_dbg_call(cmr, value);
                        }
                    }

                    match (ip.inner(), choice_bit) {
                        (node::Inner::Case(_, right), true)
                        | (node::Inner::AssertR(_, right), true) => {
                            self.fwd(1 + a.pad_right(b));
                            call_stack.push(CallStack::Back(1 + a.pad_right(b)));
                            call_stack.push(CallStack::Goto(right));
                            tracker.track_right(ip.ihr());
                        }
                        (node::Inner::Case(left, _), false)
                        | (node::Inner::AssertL(left, _), false) => {
                            self.fwd(1 + a.pad_left(b));
                            call_stack.push(CallStack::Back(1 + a.pad_left(b)));
                            call_stack.push(CallStack::Goto(left));
                            tracker.track_left(ip.ihr());
                        }
                        (node::Inner::AssertL(_, r_cmr), true) => {
                            return Err(ExecutionError::ReachedPrunedBranch(*r_cmr))
                        }
                        (node::Inner::AssertR(l_cmr, _), false) => {
                            return Err(ExecutionError::ReachedPrunedBranch(*l_cmr))
                        }
                        _ => unreachable!(),
                    }
                }
                node::Inner::Witness(value) => self.write_value(value),
                node::Inner::Jet(jet) => self.exec_jet(*jet, env, tracker)?,
                node::Inner::Word(value) => self.write_value(value.as_value()),
                node::Inner::Fail(entropy) => {
                    return Err(ExecutionError::ReachedFailNode(*entropy))
                }
            }

            ip = loop {
                match call_stack.pop() {
                    Some(CallStack::Goto(next)) => break next,
                    Some(CallStack::MoveFrame) => self.move_frame(),
                    Some(CallStack::DropFrame) => self.drop_frame(),
                    Some(CallStack::CopyFwd(n)) => {
                        self.copy(n);
                        self.fwd(n);
                    }
                    Some(CallStack::Back(n)) => self.back(n),
                    None => break 'main_loop,
                };
            };
        }

        if output_width > 0 {
            let out_frame = self.write.last_mut().unwrap();
            out_frame.reset_cursor();
            let value = Value::from_padded_bits(
                &mut out_frame.as_bit_iter(&self.data),
                &program.arrow().target,
            )
            .expect("Decode value of output frame");

            Ok(value)
        } else {
            Ok(Value::unit())
        }
    }

    fn exec_jet<J: Jet, T: ExecTracker<J>, Tx: Borrow<J::Transaction>>(
        &mut self,
        jet: J,
        env: &J::Environment<Tx>,
        tracker: &mut T,
    ) -> Result<(), JetFailed> {
        use crate::ffi::c_jets::frame_ffi::{c_readBit, c_writeBit, CFrameItem};
        use crate::ffi::c_jets::uword_width;
        use crate::ffi::ffi::UWORD;

        /// Create new C read frame that contains `bit_width` many bits from active read frame.
        ///
        /// Return C read frame together with underlying buffer.
        ///
        /// ## Safety
        ///
        /// The returned frame must outlive its buffer or there is a dangling pointer.
        ///
        /// ## Panics
        ///
        /// Active read frame has fewer bits than `bit_width`.
        unsafe fn get_input_frame(
            mac: &mut BitMachine,
            bit_width: usize,
        ) -> (CFrameItem, Vec<UWORD>) {
            assert!(bit_width <= mac.active_read_bit_width());
            let uword_width = uword_width(bit_width);
            let mut buffer = vec![0; uword_width];

            // Copy bits from active read frame into input frame
            let buffer_end = buffer.as_mut_ptr().add(uword_width);
            let mut write_frame = CFrameItem::new_write(bit_width, buffer_end);
            for _ in 0..bit_width {
                let bit = mac.read_bit();
                c_writeBit(&mut write_frame, bit);
            }
            mac.back(bit_width);

            // Convert input frame into read frame
            let buffer_ptr = buffer.as_mut_ptr();
            let read_frame = CFrameItem::new_read(bit_width, buffer_ptr);

            (read_frame, buffer)
        }

        /// Create C write frame that is as wide as `bit_width`.
        ///
        /// Return C write frame together with underlying buffer.
        ///
        /// ## Safety
        ///
        /// The returned frame must outlive its buffer or there is a dangling pointer.
        unsafe fn get_output_frame(bit_width: usize) -> (CFrameItem, Vec<UWORD>) {
            let uword_width = uword_width(bit_width);
            let mut buffer = vec![0; uword_width];

            // Return output frame as write frame
            let buffer_end = buffer.as_mut_ptr().add(uword_width);
            let write_frame = CFrameItem::new_write(bit_width, buffer_end);

            (write_frame, buffer)
        }

        /// Write `bit_width` many bits from `buffer` into active write frame.
        ///
        /// ## Panics
        ///
        /// Active write frame has fewer bits than `bit_width`.
        ///
        /// Buffer has fewer than bits than `bit_width` (converted to UWORDs).
        fn update_active_write_frame(mac: &mut BitMachine, bit_width: usize, buffer: &[UWORD]) {
            assert!(bit_width <= mac.active_write_bit_width());
            assert!(uword_width(bit_width) <= buffer.len());
            let buffer_ptr = buffer.as_ptr();
            let mut read_frame = unsafe { CFrameItem::new_read(bit_width, buffer_ptr) };

            for _ in 0..bit_width {
                let bit = unsafe { c_readBit(&mut read_frame) };
                mac.write_bit(bit);
            }
        }

        // Sanity Check: This should never really fail, but still good to do
        if !simplicity_sys::c_jets::sanity_checks() {
            return Err(JetFailed);
        }

        let input_width = jet.source_ty().to_bit_width();
        let output_width = jet.target_ty().to_bit_width();
        // Input buffer is implicitly referenced by input read frame!
        // Same goes for output buffer
        let (input_read_frame, input_buffer) = unsafe { get_input_frame(self, input_width) };
        let (mut output_write_frame, output_buffer) = unsafe { get_output_frame(output_width) };

        let jet_fn = jet.c_jet_ptr();
        let c_env = J::c_jet_env(env);
        let success = jet_fn(&mut output_write_frame, input_read_frame, c_env);

        tracker.track_jet_call(&jet, &input_buffer, &output_buffer, success);

        if !success {
            Err(JetFailed)
        } else {
            update_active_write_frame(self, output_width, &output_buffer);
            Ok(())
        }
    }
}

/// A type that keeps track of Bit Machine execution.
///
/// The trait is implemented for [`SetTracker`], that tracks which case branches were executed,
/// and it is implemented for [`NoTracker`], which is a dummy tracker that is
/// optimized out by the compiler.
///
/// The trait enables us to turn tracking on or off depending on a generic parameter.
pub trait ExecTracker<J: Jet> {
    /// Track the execution of the left branch of the case node with the given `ihr`.
    fn track_left(&mut self, ihr: Ihr);

    /// Track the execution of the right branch of the case node with the given `ihr`.
    fn track_right(&mut self, ihr: Ihr);

    /// Track the execution of a `jet` call with the given `input_buffer`, `output_buffer`, and call result `success`.
    fn track_jet_call(
        &mut self,
        jet: &J,
        input_buffer: &[UWORD],
        output_buffer: &[UWORD],
        success: bool,
    );

    /// Track the potential execution of a `dbg!` call with the given `cmr` and `value`.
    fn track_dbg_call(&mut self, cmr: &Cmr, value: Value);

    /// Check if tracking debug calls is enabled.
    fn is_track_debug_enabled(&self) -> bool;
}

/// Tracker of executed left and right branches for each case node.
#[derive(Clone, Debug, Default)]
pub struct SetTracker {
    left: HashSet<Ihr>,
    right: HashSet<Ihr>,
}

impl SetTracker {
    /// Access the set of IHRs of case nodes whose left branch was executed.
    pub fn left(&self) -> &HashSet<Ihr> {
        &self.left
    }

    /// Access the set of IHRs of case nodes whose right branch was executed.
    pub fn right(&self) -> &HashSet<Ihr> {
        &self.right
    }
}

/// Tracker that does not do anything (noop).
#[derive(Copy, Clone, Debug)]
pub struct NoTracker;

impl<J: Jet> ExecTracker<J> for SetTracker {
    fn track_left(&mut self, ihr: Ihr) {
        self.left.insert(ihr);
    }

    fn track_right(&mut self, ihr: Ihr) {
        self.right.insert(ihr);
    }

    fn track_jet_call(&mut self, _: &J, _: &[UWORD], _: &[UWORD], _: bool) {}

    fn track_dbg_call(&mut self, _: &Cmr, _: Value) {}

    fn is_track_debug_enabled(&self) -> bool {
        false
    }
}

impl<J: Jet> ExecTracker<J> for NoTracker {
    fn track_left(&mut self, _: Ihr) {}

    fn track_right(&mut self, _: Ihr) {}

    fn track_jet_call(&mut self, _: &J, _: &[UWORD], _: &[UWORD], _: bool) {}

    fn track_dbg_call(&mut self, _: &Cmr, _: Value) {}

    fn is_track_debug_enabled(&self) -> bool {
        // Set flag to test frame decoding in unit tests
        cfg!(test)
    }
}

/// Errors related to simplicity Execution
#[derive(Debug)]
pub enum ExecutionError {
    /// Provided input is of wrong type
    InputWrongType(Arc<Final>),
    /// Reached a fail node
    ReachedFailNode(FailEntropy),
    /// Reached a pruned branch
    ReachedPrunedBranch(Cmr),
    /// Exceeded some program limit
    LimitExceeded(LimitError),
    /// Jet failed during execution
    JetFailed(JetFailed),
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExecutionError::InputWrongType(expected_ty) => {
                write!(f, "Expected input of type: {expected_ty}")
            }
            ExecutionError::ReachedFailNode(entropy) => {
                write!(f, "Execution reached a fail node: {}", entropy)
            }
            ExecutionError::ReachedPrunedBranch(hash) => {
                write!(f, "Execution reached a pruned branch: {}", hash)
            }
            ExecutionError::LimitExceeded(e) => e.fmt(f),
            ExecutionError::JetFailed(jet_failed) => jet_failed.fmt(f),
        }
    }
}

impl error::Error for ExecutionError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::InputWrongType(..)
            | Self::ReachedFailNode(..)
            | Self::ReachedPrunedBranch(..) => None,
            Self::LimitExceeded(ref e) => Some(e),
            Self::JetFailed(ref e) => Some(e),
        }
    }
}

impl From<LimitError> for ExecutionError {
    fn from(e: LimitError) -> Self {
        ExecutionError::LimitExceeded(e)
    }
}

impl From<JetFailed> for ExecutionError {
    fn from(jet_failed: JetFailed) -> Self {
        ExecutionError::JetFailed(jet_failed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "elements")]
    use crate::jet::{elements::ElementsEnv, Elements};
    #[cfg(feature = "elements")]
    use crate::{node::RedeemNode, BitIter};
    #[cfg(feature = "elements")]
    use hex::DisplayHex;

    #[cfg(feature = "elements")]
    fn run_program_elements(
        prog_bytes: &[u8],
        witness_bytes: &[u8],
        cmr_str: &str,
        amr_str: &str,
        ihr_str: &str,
    ) -> Result<Value, ExecutionError> {
        let prog_hex = prog_bytes.as_hex();

        let prog = BitIter::from(prog_bytes);
        let witness = BitIter::from(witness_bytes);
        let prog = match RedeemNode::<Elements>::decode(prog, witness) {
            Ok(prog) => prog,
            Err(e) => panic!("program {} failed: {}", prog_hex, e),
        };

        // Check Merkle roots; check AMR last because the AMR is a more complicated
        // calculation and historically has been much more likely to be wrong.
        assert_eq!(
            prog.cmr().to_string(),
            cmr_str,
            "CMR mismatch (got {} expected {}) for program {}",
            prog.cmr(),
            cmr_str,
            prog_hex,
        );
        assert_eq!(
            prog.amr().to_string(),
            amr_str,
            "AMR mismatch (got {} expected {}) for program {}",
            prog.amr(),
            amr_str,
            prog_hex,
        );
        assert_eq!(
            prog.ihr().to_string(),
            ihr_str,
            "IHR mismatch (got {} expected {}) for program {}",
            prog.ihr(),
            ihr_str,
            prog_hex,
        );

        // Try to run it on the bit machine and return the result
        let env = ElementsEnv::dummy();
        BitMachine::for_program(&prog)
            .expect("program has reasonable bounds")
            .exec(&prog, &env)
    }

    #[test]
    #[cfg(feature = "elements")]
    fn crash_regression1() {
        // cfe18fb44028870400
        // This program caused an array OOB.
        // # Witnesses
        // wit1 = witness :: (((1 + (1 + 2^256)) * (1 + (1 + 2^256))) + 1) -> 1
        //
        // # Program code
        // jt1 = jet_num_outputs    :: 1 -> 2^32                                          # cmr 447165a3...
        // jt2 = jet_issuance_token :: 2^32 -> (1 + (1 + 2^256))                          # cmr 85e9591c...
        // pr3 = pair jt2 jt2       :: 2^32 -> ((1 + (1 + 2^256)) * (1 + (1 + 2^256)))    # cmr 45d40848...
        // cp4 = comp jt1 pr3       :: 1 -> ((1 + (1 + 2^256)) * (1 + (1 + 2^256)))       # cmr 7bb1824f...
        // jl5 = injl cp4           :: 1 -> (((1 + (1 + 2^256)) * (1 + (1 + 2^256))) + 1) # cmr 277ee32c...
        //
        // main = comp jl5 wit1     :: 1 -> 1                                             # cmr 7050c4a6...

        let res = run_program_elements(
            &[0xcf, 0xe1, 0x8f, 0xb4, 0x40, 0x28, 0x87, 0x04, 0x00],
            &[],
            "0075f5368af7b453c2f7318493df12567bc6d733cf1ebb69a08ce93e1e529956",
            "0ac9baaee94e416c6598a271dfc18d6014f751ac300cfe69eb758c20f26c624b",
            "61f2cf59bfec55bb6e44cbae71c3ba225f1059f9a5679c0b42a7177daff52b5a",
        );
        assert_eq!(res.unwrap(), Value::unit());
    }

    #[test]
    fn crash_regression2() {
        use crate::node::{CoreConstructible as _, JetConstructible as _};

        type Node<'brand> = Arc<crate::ConstructNode<'brand, crate::jet::Core>>;

        crate::types::Context::with_context(|ctx| {
            let mut bomb = Node::jet(
                &ctx,
                crate::jet::Core::Ch8, // arbitrary jet with nonzero output size
            );
            for _ in 0..100 {
                bomb = Node::pair(&bomb, &bomb).unwrap();
            }
            let _ = bomb.finalize_pruned(&crate::jet::CoreEnv::EMPTY);
        });
    }
}
