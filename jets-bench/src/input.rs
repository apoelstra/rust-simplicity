use std::sync::Arc;
use std::{cmp, io, iter};

use rand::distributions as dist;
use rand::rngs::ThreadRng;
use rand::{Rng, RngCore};
use simplicity::ffi::c_jets::frame_ffi::c_writeBit;
use simplicity::ffi::CFrameItem;
use simplicity::hashes::Hash;
use simplicity::jet::Elements;
use simplicity::types::{self, CompleteBound};
use simplicity::Value;

pub fn random_value(ty: &types::Final, rng: &mut ThreadRng) -> Value {
    enum StackItem<'a> {
        Type(&'a types::Final),
        LeftSum(Arc<types::Final>),
        RightSum(Arc<types::Final>),
        Product,
    }

    let mut call_stack = vec![StackItem::Type(ty)];
    let mut value_stack = Vec::new();

    while let Some(top) = call_stack.pop() {
        match top {
            StackItem::Type(ty) => match &ty.bound() {
                CompleteBound::Unit => value_stack.push(Value::unit()),
                CompleteBound::Sum(left, right) => {
                    if rng.gen() {
                        call_stack.push(StackItem::LeftSum(Arc::clone(right)));
                        call_stack.push(StackItem::Type(left));
                    } else {
                        call_stack.push(StackItem::RightSum(Arc::clone(left)));
                        call_stack.push(StackItem::Type(right));
                    }
                }
                CompleteBound::Product(left, right) => {
                    call_stack.push(StackItem::Product);
                    call_stack.push(StackItem::Type(right));
                    call_stack.push(StackItem::Type(left));
                }
            },
            StackItem::LeftSum(right) => {
                let left = value_stack.pop().unwrap();
                value_stack.push(Value::left(left, right));
            }
            StackItem::RightSum(left) => {
                let right = value_stack.pop().unwrap();
                value_stack.push(Value::right(left, right));
            }
            StackItem::Product => {
                let right = value_stack.pop().unwrap();
                let left = value_stack.pop().unwrap();
                value_stack.push(Value::product(left, right));
            }
        }
    }
    debug_assert!(value_stack.len() == 1);
    value_stack.pop().unwrap()
}

/// Maximum number of bytes needed to represent a value used in the benchmarks.
///
/// If this value is too low it'll cause a panic. In that case just increase it.
const MAX_VALUE_BYTES: usize = 616; // Sha256Ctx8Add512

/// A "flat value" represented as a sequence of bits.
///
/// The representation is that used in the Bit Machine; so for a given type,
/// the number of bits is always constant, though for some values some bits
/// may be unused (in sum types where one type is larger than the other).
///
/// It is **not** the representation used in witness encoding, which is much
/// more compact, and compresses sums.
#[derive(Clone)]
pub struct FlatValue {
    inner: [u8; MAX_VALUE_BYTES],
    len_bytes: usize,
    len_bits: usize,
}

impl FlatValue {
    /// Creates a new random value of the given type.
    ///
    /// # Panics
    ///
    /// If the type to be written is too large. In this case, edit the source
    /// to increase the `MAX_VALUE_BYTES` constant.
    pub fn random(ty: &types::Final) -> Self {
        Self::random_n_bits(ty.bit_width())
    }

    /// Creates an all-zero flat value of the given byte length.
    pub fn zero_n_bytes<const N: usize>() -> Self
    where
        dist::Standard: dist::Distribution<[u8; N]>,
    {
        assert!(
            N <= MAX_VALUE_BYTES,
            "tried to generate a flat value of {} bytes; max {}; please increase MAX_VALUE_BYTES",
            N,
            MAX_VALUE_BYTES,
        );

        FlatValue {
            inner: [0; MAX_VALUE_BYTES],
            len_bytes: N,
            len_bits: 8 * N,
        }
    }

    /// Creates a random flat value of the given byte length using the thread-local RNG.
    pub fn random_n_bytes<const N: usize>() -> Self
    where
        dist::Standard: dist::Distribution<[u8; N]>,
    {
        let bytes = rand::random::<[u8; N]>();
        let mut ret = Self::zero_n_bytes::<N>();
        ret.inner[..N].copy_from_slice(&bytes);
        ret
    }

    /// Creates a zero flat value of the given bit-length using the thread-local RNG.
    pub fn zero_n_bits(bit_width: usize) -> Self {
        let byte_width = (bit_width + 7) / 8;

        assert!(
            byte_width <= MAX_VALUE_BYTES,
            "type of width {} bits too large; please increase MAX_VALUE_BYTES from {} ({} bits)",
            bit_width,
            MAX_VALUE_BYTES,
            8 * MAX_VALUE_BYTES,
        );

        FlatValue {
            inner: [0; MAX_VALUE_BYTES],
            len_bytes: byte_width,
            len_bits: bit_width,
        }
    }

    /// Given a value, prefix a 0 before it.
    pub fn prefix_zero(mut self) -> Self {
        assert!(
            self.len_bits < 8 * MAX_VALUE_BYTES - 1,
            "no room to push another bit; max number of bytes {}",
            MAX_VALUE_BYTES,
        );

        // shift everything right
        let mut carry = 0;
        for byte in self.inner[..self.len_bytes + 1].iter_mut() {
            let tmp = *byte;
            *byte = (*byte >> 1) | carry;
            carry = tmp << 7;
        }
        if self.len_bits % 8 == 0 {
            self.len_bytes += 1;
        }
        self.len_bits += 1;
        self
    }

    /// Given a value, prefix a 1 before it.
    pub fn prefix_one(self) -> Self {
        let mut ret = self.prefix_zero();
        ret.inner[0] |= 0x80;
        ret
    }

    /// Helper function to zero out the unused bits of the last byte of the value.
    fn zero_top_byte(&mut self) {
        if self.len_bits % 8 != 0 {
            self.inner[self.len_bytes - 1] &= (1 << (8 - self.len_bits % 8)) - 1;
        }
    }

    /// Creates a random flat value of the given bit-length using the thread-local RNG.
    pub fn random_n_bits(bit_width: usize) -> Self {
        let byte_width = (bit_width + 7) / 8;

        let mut ret = Self::zero_n_bits(bit_width);
        rand::thread_rng().fill_bytes(&mut ret.inner[..byte_width]);
        ret.zero_top_byte();
        ret
    }

    pub fn left_shift(&mut self, bits: usize) {
        assert!(bits < 8, "cannot shift more than 8 bits at once");
        if bits == 0 {
            return;
        }

        let mut last_byte = 0;
        for byte in self.inner.iter_mut().rev() {
            let tmp = *byte >> (8 - bits);
            *byte <<= bits;
            *byte |= last_byte;
            last_byte = tmp;
        }

        if self.len_bits % 8 >= bits {
            self.len_bytes -= 1;
        }
        self.len_bits -= bits;
    }

    /// Creates a new value as a product of two existing values.
    pub fn product<'s, I: IntoIterator<Item = &'s Self>>(items: I) -> Self {
        let mut ret = FlatValue {
            inner: [0; MAX_VALUE_BYTES],
            len_bytes: 0,
            len_bits: 0,
        };

        for item in items {
            let new_len_bits = ret.len_bits + item.len_bits;
            let new_len_bytes = (new_len_bits + 7) / 8;
            assert!(
                new_len_bytes <= MAX_VALUE_BYTES,
                "length {} bytes too large; please increase MAX_VALUE_BYTES from {} ({} bits)",
                new_len_bytes,
                MAX_VALUE_BYTES,
                8 * MAX_VALUE_BYTES,
            );

            let right_shift = ret.len_bits % 8;
            if right_shift == 0 {
                item.write_to(&mut ret.inner[ret.len_bytes..], 0).unwrap();
            } else {
                let last_byte = ret.inner[ret.len_bytes];
                item.write_to(&mut ret.inner[ret.len_bytes - 1..], right_shift)
                    .unwrap();
                ret.inner[ret.len_bytes - 1] |= last_byte;
            }

            ret.len_bytes = new_len_bytes;
            ret.len_bits = new_len_bits;
        }

        ret
    }

    /// Given a value of length 2n, break it into two values of length n.
    pub fn split_in_half(&self) -> (Self, Self) {
        assert!(
            self.len_bits % 2 == 0,
            "must have an even length object to split in half"
        );
        assert!(
            self.len_bits % 16 == 0,
            "FIXME right now we only support splitting even numbers of bytes"
        );
        let mut lhs = FlatValue::zero_n_bits(self.len_bits / 2);
        let mut rhs = FlatValue::zero_n_bits(self.len_bits / 2);

        lhs.inner[..self.len_bytes / 2].copy_from_slice(&self.inner[..self.len_bytes / 2]);
        rhs.inner[..self.len_bytes / 2]
            .copy_from_slice(&self.inner[self.len_bytes / 2..self.len_bytes]);

        (lhs, rhs)
    }

    /// Writes the value into some sort of writer, first right-shifting it by up to 8 bits.
    ///
    /// # Panics
    ///
    /// If the right-shift is 8 or more.
    pub fn write_to<W: io::Write>(&self, mut dest: W, right_shift: usize) -> io::Result<()> {
        assert!(right_shift < 8, "right shift {} must be < 8", right_shift);
        if right_shift == 0 {
            dest.write_all(&self.inner[..self.len_bytes])
        } else {
            let mut last_byte = 0;
            for &byte in &self.inner[..self.len_bytes] {
                dest.write_all(&[(last_byte << (8 - right_shift)) | (byte >> right_shift)])?;
                last_byte = byte;
            }
            Ok(())
        }
    }

    /// Creates an iterator over the bits of the value
    pub fn bit_iter(&self) -> FlatValueBitIter<'_> {
        FlatValueBitIter {
            inner: self,
            n_read: 0,
        }
    }

    /// Internal helper function that just calls a jet on a flatvalue to compute
    /// a new flatvalue. We don't use this for benchmarks because it doesn't mess
    /// with alignment and isn't very careful about type correctness/sizing.
    fn call_jet(&self, jet: Elements, dest_bits: usize) -> Self {
        use core::{mem, ptr};
        use simplicity::ffi::c_jets::uword_width;
        use simplicity::jet::Jet as _;

        assert!(dest_bits <= 8 * MAX_VALUE_BYTES);
        let mut ret = Self::zero_n_bits(dest_bits);

        assert_eq!(
            self.len_bits % 8,
            0,
            "don't support calling jets with non-byte-aligned input",
        );

        unsafe {
            use simplicity::elements::hashes::Hash as _;

            let mut dst_inner = [0usize; MAX_VALUE_BYTES / mem::size_of::<usize>()];
            let mut src_inner = [0usize; MAX_VALUE_BYTES / mem::size_of::<usize>()];

            let mut src_bytes = self.inner;
            // See below block comment on the write frame for justification of this
            // weird byte-swapping ritual.
            src_bytes[..self.len_bytes].reverse();
            ptr::copy_nonoverlapping(
                src_bytes.as_ptr(),
                src_inner.as_mut_ptr() as *mut u8,
                MAX_VALUE_BYTES,
            );
            for us in &mut src_inner {
                *us = usize::from_be(us.swap_bytes());
            }

            let src_read_frame = CFrameItem::new_read(self.len_bits, src_inner.as_ptr());
            let mut dst_write_frame = CFrameItem::new_write(
                dest_bits,
                dst_inner.as_mut_ptr().add(uword_width(dest_bits)),
            );

            let ctrl_blk: [u8; 33] = [
                0xc0, 0xeb, 0x04, 0xb6, 0x8e, 0x9a, 0x26, 0xd1, 0x16, 0x04, 0x6c, 0x76, 0xe8, 0xff,
                0x47, 0x33, 0x2f, 0xb7, 0x1d, 0xda, 0x90, 0xff, 0x4b, 0xef, 0x53, 0x70, 0xf2, 0x52,
                0x26, 0xd3, 0xbc, 0x09, 0xfc,
            ];
            let env = simplicity::jet::elements::ElementsEnv::new(
                Arc::new(simplicity::elements::Transaction {
                    version: 0,
                    lock_time: simplicity::elements::LockTime::ZERO,
                    input: vec![],
                    output: vec![],
                }),
                vec![],
                0,
                simplicity::Cmr::unit(),
                simplicity::elements::taproot::ControlBlock::from_slice(&ctrl_blk).unwrap(),
                None,
                simplicity::elements::BlockHash::all_zeros(),
            );

            // We can assert this because in our sampling code jets should never
            // fail. In the benchmarking code they might.
            assert!(jet.c_jet_ptr()(
                &mut dst_write_frame,
                src_read_frame,
                Elements::c_jet_env(&env)
            ));
            // The write frame winds up as an array of usizes with all bytes in
            // reverse order. (The bytes of the usizes are in reverse order due
            // to endianness, but also the usizes themselves are in reverse
            // order for whatever reason.)If the number of bits written was not
            // a multiple of 8, then the final usize will be incomplete and its
            // **least** significant byte(s) will be 0 and of the nonzero byte
            // the **most significant bit(s)** will be 0.
            //
            // To solve this, we first convert the backward usize array to a
            // backward u8 array...
            for us in &mut dst_inner {
                *us = us.swap_bytes().to_be();
            }
            ptr::copy_nonoverlapping(
                dst_inner.as_ptr() as *mut u8,
                ret.inner.as_mut_ptr(),
                MAX_VALUE_BYTES,
            );

            // We then reverse the backward byte array, which leaves us with a
            // correct-direction byte array which may be *right*-shifted.
            if dest_bits % 8 == 0 {
                ret.inner[..dest_bits / 8].reverse();
            } else {
                ret.inner[..(dest_bits + 7) / 8].reverse();
                ret.len_bits += 8 - dest_bits % 8; // correct for "correction" by left_shift
                ret.left_shift(8 - dest_bits % 8);
            }
        }
        ret
    }
}

pub struct FlatValueBitIter<'fv> {
    inner: &'fv FlatValue,
    n_read: usize,
}

impl<'fv> Iterator for FlatValueBitIter<'fv> {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n_read >= self.inner.len_bits {
            None
        } else {
            let byte = self.inner.inner[self.n_read / 8];
            let bit = byte & (1 << (7 - self.n_read % 8)) != 0;
            self.n_read += 1;
            Some(bit)
        }
    }
}

#[derive(Clone)]
pub enum InputSampling {
    /// Uniform random distribution of bit strings based on encoded values over the source type
    Random,
    /// A given, fixed bit string (whose length is multiple of 8)
    /// Worst-case inputs
    Fixed(Value),
    /// Custom sampling method, read first src type bits from input
    /// Useful for cases where we want to sample inputs according to some distributions
    Custom(Arc<dyn Fn() -> Value>),
}

impl InputSampling {
    pub fn write_sample(
        &self,
        src_frame: &mut CFrameItem,
        src_ty: &types::Final,
        rng: &mut ThreadRng,
    ) {
        let mut write_bit = |bit: bool| unsafe { c_writeBit(src_frame, bit) };

        match self {
            InputSampling::Random => {
                let value = random_value(src_ty, rng);
                for bit in value.iter_padded() {
                    write_bit(bit);
                }
            }
            InputSampling::Fixed(value) => {
                for bit in value.iter_padded() {
                    write_bit(bit);
                }
            }
            InputSampling::Custom(gen_bytes) => {
                let value = gen_bytes();
                for bit in value.iter_padded() {
                    write_bit(bit);
                }
            }
        }
    }
}

/// An object which can be used as an input.
pub trait InputSample {
    /// Number of distinct distributions from which the object can be sampled.
    fn n_distributions(&self) -> usize;

    /// Display string for the distribution
    fn distribution_name(&self, dist: usize) -> String;

    /// Sample from the requested distribution, using the thread-local random
    /// number generator if needed.
    ///
    /// The `dist` value chooses which distribution to use from the range 0 through
    /// [`Self::n_distributions`] minus 1. The exact meanings vary from object to
    /// object and should not be relied upon *except* that 0 should always represent
    /// "a uniform valid value" or something close to it.
    ///
    /// # Panics
    ///
    /// May panic if the requested `dist` is out of range. Call [`Self::n_distributions`]
    /// to learn the maximum value.
    ///
    /// May panic if `n_bits` does not make sense for the type in question.
    fn sample(&self, dist: usize, n_bits: usize) -> FlatValue;
}

pub trait InputSampleExactSize: InputSample {
    fn n_bits(&self) -> usize;

    fn sample_exact(&self, dist: usize) -> FlatValue {
        self.sample(dist, self.n_bits())
    }
}

pub struct Unit;

impl InputSample for Unit {
    fn n_distributions(&self) -> usize {
        1
    }

    fn distribution_name(&self, _: usize) -> String {
        "unit".into()
    }

    fn sample(&self, _: usize, n_bits: usize) -> FlatValue {
        assert_eq!(n_bits, 0, "tried to sample nonzero-sized unit");
        FlatValue::zero_n_bits(0)
    }
}

impl InputSampleExactSize for Unit {
    fn n_bits(&self) -> usize {
        0
    }
}

pub struct UniformBits;
pub struct UniformBitsExact<const N: usize>;

impl InputSample for UniformBits {
    fn n_distributions(&self) -> usize {
        3
    }

    fn distribution_name(&self, dist: usize) -> String {
        match dist {
            0 => "ufmbits".to_string(),
            1 => "lo-bits".to_string(),
            2 => "hi-bits".to_string(),
            x => panic!("no distribution {x} for UniformBytes"),
        }
    }

    fn sample(&self, dist: usize, n_bits: usize) -> FlatValue {
        assert!(
            n_bits > 0,
            "tried to sample 0 uniform bits; use Unit rather than UniformBits"
        );
        match dist {
            // uniform
            0 => FlatValue::random_n_bits(n_bits),
            // low bit-weight values
            1 => {
                let mut rng = rand::thread_rng();
                let mut ret = FlatValue::zero_n_bits(n_bits);

                for _ in 0..rng.gen_range(0..cmp::min(8, n_bits)) {
                    let bit = rng.gen_range(0..n_bits);
                    ret.inner[bit / 8] |= 1 << (bit % 8);
                }

                ret
            }
            // high bit-weight values
            2 => {
                let mut rng = rand::thread_rng();
                let mut ret = FlatValue::zero_n_bits(n_bits);

                for byte in &mut ret.inner[..ret.len_bytes] {
                    *byte = 0xff;
                }
                for _ in 0..rng.gen_range(0..cmp::min(8, n_bits)) {
                    let bit = rng.gen_range(0..n_bits);
                    ret.inner[bit / 8] &= !(1 << (bit % 8));
                }

                ret
            }
            x => panic!("no distribution {x} for UniformBytes"),
        }
    }
}

impl<const N: usize> InputSample for UniformBitsExact<N> {
    fn n_distributions(&self) -> usize {
        UniformBits.n_distributions()
    }

    fn distribution_name(&self, dist: usize) -> String {
        UniformBits.distribution_name(dist)
    }

    fn sample(&self, dist: usize, n_bits: usize) -> FlatValue {
        assert_eq!(
            n_bits,
            self.n_bits(),
            "requested {} bits for an exact-sized {}-bit string",
            n_bits,
            self.n_bits(),
        );
        UniformBits.sample(dist, n_bits)
    }
}

impl<const N: usize> InputSampleExactSize for UniformBitsExact<N> {
    fn n_bits(&self) -> usize {
        N
    }
}

/// A variable-length buffer able to hold up to 8^(N + 1) - 1 bits.
///
/// So, e.g. if N is 8 then it can hold between 0 and 511 bits inclusive,
/// in multiples of 8.
///
/// The buffer is encoded as a product of tuples of halving size. In Rust
/// terms, when N = 2 it looks like
///
/// ```text
///     (Option<(u8, u8, u8, u8), Option<(u8, u8)>, Option<u8>)
/// ```
///
/// And you can see that it can hold up to 7 bits. If there is a smaller
/// number, there is a unique pattern of Some/None that can express that.
///
/// In Russell's terms:
///
/// * a Simplicity buffer of type (TWO^8)^<2^(n+1) as [`Value`].
/// * The notation X^<2 is notation for the type (S X)
/// * The notation X^<(2*n) is notation for the type S (X^n) * X^<n
///
/// Cannot represent >= 2**16 bytes. 0 <= n < 16 is a Simplicity consensus rule.
///
/// The total length is therefore always a 2^n less 1, for the actual bits,
/// plus n, for the sum-type markers.
///
/// In the benchmarks this is only ever used with N = 8, for the sha256 context
/// buffer, which is variable-length between 0 and 511 bits long.
pub struct VarLengthBuffer;
pub struct VarLengthBufferExact<const N: usize>;

impl InputSample for VarLengthBuffer {
    fn n_distributions(&self) -> usize {
        2
    }

    fn distribution_name(&self, dist: usize) -> String {
        match dist {
            0 => "varbuf-rndlen".into(),
            1 => "varbuf-maxlen".into(),
            x => panic!("no distribution {x} for VarLengthBuffer"),
        }
    }

    fn sample(&self, dist: usize, n_bits: usize) -> FlatValue {
        let mut n = None;
        for i in 0..16 {
            if n_bits == (8 << (i + 1)) - 8 + i + 1 {
                n = Some(i);
            }
        }
        let n = match n {
            Some(n) => n,
            None => panic!(
                "n_bits must be of the form 2^n + n, not {} for n in 0..16",
                n_bits,
            ),
        };

        let mut rng = rand::thread_rng();

        let max_value_len_bytes = 1 << (n + 1);
        let value_len_bytes = match dist {
            // uniform
            0 => rng.gen_range(0..=max_value_len_bytes),
            1 => max_value_len_bytes,
            x => panic!("no distribution {x} for VarLengthBuffer"),
        };

        // Construct the value *as put on the bit machine*, which means
        // that the units show up as long strings of 0s rather than just
        // a single 0 bit (as would be done for the witness encoding).
        let mut ret = FlatValue::zero_n_bits(0);
        for i in (0..n + 1).rev() {
            let rhs = if value_len_bytes & (1 << i) == 0 {
                FlatValue::zero_n_bits(8 << i).prefix_zero()
            } else {
                FlatValue::random_n_bits(8 << i).prefix_one()
            };
            ret = FlatValue::product(&[ret, rhs]);
        }
        assert_eq!(ret.len_bits, n_bits); // sanity check
        ret
    }
}

impl<const N: usize> InputSample for VarLengthBufferExact<N> {
    fn n_distributions(&self) -> usize {
        VarLengthBuffer.n_distributions()
    }

    fn distribution_name(&self, dist: usize) -> String {
        VarLengthBuffer.distribution_name(dist)
    }

    fn sample(&self, dist: usize, n_bits: usize) -> FlatValue {
        assert_eq!(
            n_bits,
            self.n_bits(),
            "requested an {}-bit sample from a varlength buffer with N = {} (len = {})",
            n_bits,
            N,
            self.n_bits(),
        );
        VarLengthBuffer.sample(dist, n_bits)
    }
}
impl<const N: usize> InputSampleExactSize for VarLengthBufferExact<N> {
    fn n_bits(&self) -> usize {
        (8 << (N + 1)) - 8 + N + 1
    }
}

pub struct EqProduct<L>(pub L);

impl<L: InputSample> InputSample for EqProduct<L> {
    fn n_distributions(&self) -> usize {
        // both random
        self.0.n_distributions() * self.0.n_distributions()
            // both equal, and each a bit-inverse of the other
            + 2 * self.0.n_distributions()
    }

    fn distribution_name(&self, dist: usize) -> String {
        let ndist = self.0.n_distributions();
        let ndist2 = ndist * ndist;

        if dist < ndist2 {
            format!(
                "{}_+_{}",
                self.0.distribution_name(dist / ndist),
                self.0.distribution_name(dist % ndist),
            )
        } else if dist < ndist2 + ndist {
            format!("{}_+_self", self.0.distribution_name(dist - ndist2),)
        } else {
            format!(
                "{}_+_inv_self",
                self.0.distribution_name(dist - ndist2 - ndist),
            )
        }
    }

    fn sample(&self, dist: usize, n_bits: usize) -> FlatValue {
        assert_eq!(n_bits % 2, 0, "EqProducts samples must have even size");

        let ndist = self.0.n_distributions();
        let ndist2 = ndist * ndist;

        if dist < ndist2 {
            FlatValue::product(&[
                self.0.sample(dist / ndist, n_bits / 2),
                self.0.sample(dist % ndist, n_bits / 2),
            ])
        } else if dist < ndist2 + ndist {
            let samp = self.0.sample(dist - ndist2, n_bits / 2);
            FlatValue::product(iter::repeat(&samp).take(2))
        } else if dist < ndist2 + 2 * ndist {
            let samp = self.0.sample(dist - ndist2 - ndist, n_bits / 2);
            let mut samp_inv = samp.clone();
            for byte in &mut samp_inv.inner[..samp_inv.len_bytes] {
                *byte = !*byte;
            }
            samp_inv.zero_top_byte();

            FlatValue::product(&[samp, samp_inv])
        } else {
            panic!(
                "invalid distribution request {} (max n^2 + 2n = {} with n = {}",
                dist,
                ndist * ndist + 2 * ndist,
                ndist,
            )
        }
    }
}

impl<L: InputSampleExactSize> InputSampleExactSize for EqProduct<L> {
    fn n_bits(&self) -> usize {
        2 * self.0.n_bits()
    }
}

pub struct PrefixBit<L>(pub L);

impl<L: InputSample> InputSample for PrefixBit<L> {
    fn n_distributions(&self) -> usize {
        2 * self.0.n_distributions()
    }

    fn distribution_name(&self, dist: usize) -> String {
        let ndist = self.0.n_distributions();
        if dist < ndist {
            format!("0{}", self.0.distribution_name(dist))
        } else {
            format!("1{}", self.0.distribution_name(dist - ndist))
        }
    }

    fn sample(&self, dist: usize, n_bits: usize) -> FlatValue {
        let ndist = self.0.n_distributions();

        if dist < ndist {
            self.0.sample(dist, n_bits - 1).prefix_zero()
        } else if dist < 2 * ndist {
            self.0.sample(dist - ndist, n_bits - 1).prefix_zero()
        } else {
            panic!(
                "invalid distribution request {} (max 2n = {})",
                dist,
                2 * ndist,
            )
        }
    }
}

impl<L: InputSampleExactSize> InputSampleExactSize for PrefixBit<L> {
    fn n_bits(&self) -> usize {
        1 + self.0.n_bits()
    }
}

pub struct GenericProduct<L, R>(pub L, pub R);

impl<L: InputSampleExactSize, R: InputSample> InputSample for GenericProduct<L, R> {
    fn n_distributions(&self) -> usize {
        self.0.n_distributions() * self.1.n_distributions()
    }

    fn distribution_name(&self, dist: usize) -> String {
        format!(
            "{}_+_{}",
            self.0.distribution_name(dist / self.1.n_distributions()),
            self.1.distribution_name(dist % self.1.n_distributions()),
        )
    }

    fn sample(&self, dist: usize, n_bits: usize) -> FlatValue {
        assert!(
            n_bits >= self.0.n_bits(),
            "tried to sample a product of fewer bits ({}) than left factor ({})",
            n_bits,
            self.0.n_bits(),
        );
        FlatValue::product(&[
            self.0
                .sample(dist / self.1.n_distributions(), self.0.n_bits()),
            self.1
                .sample(dist % self.1.n_distributions(), n_bits - self.0.n_bits()),
        ])
    }
}

impl<L: InputSampleExactSize, R: InputSampleExactSize> InputSampleExactSize
    for GenericProduct<L, R>
{
    fn n_bits(&self) -> usize {
        self.0.n_bits() + self.1.n_bits()
    }
}

pub struct Sha256Ctx;

impl InputSample for Sha256Ctx {
    fn n_distributions(&self) -> usize {
        // var-len buffer, 64-bit length, 256-bit midstate
        GenericProduct(
            VarLengthBufferExact::<5>,
            GenericProduct(UniformBitsExact::<64>, UniformBitsExact::<256>),
        )
        .n_distributions()
    }

    fn distribution_name(&self, dist: usize) -> String {
        format!(
            "sha256ctx_{}",
            GenericProduct(
                VarLengthBufferExact::<5>,
                GenericProduct(UniformBitsExact::<64>, UniformBitsExact::<256>),
            )
            .distribution_name(dist)
        )
    }

    fn sample(&self, dist: usize, n_bits: usize) -> FlatValue {
        assert_eq!(
            n_bits,
            self.n_bits(),
            "tried to construct sha256ctx (size {} bits) with {} bits",
            self.n_bits(),
            n_bits,
        );
        GenericProduct(
            VarLengthBufferExact::<5>,
            GenericProduct(UniformBitsExact::<64>, UniformBitsExact::<256>),
        )
        .sample(dist, n_bits)
    }
}

impl InputSampleExactSize for Sha256Ctx {
    fn n_bits(&self) -> usize {
        GenericProduct(
            VarLengthBufferExact::<5>,
            GenericProduct(UniformBitsExact::<64>, UniformBitsExact::<256>),
        )
        .n_bits()
    }
}

pub struct Fe;

impl InputSample for Fe {
    fn n_distributions(&self) -> usize {
        // uniform, plus an extra "out of range" distribution
        4
    }

    fn distribution_name(&self, dist: usize) -> String {
        match dist {
            // uniform and low-bit-weight
            0..=2 => format!("fe_{}", UniformBits.distribution_name(dist)),
            3 => "fe_outofrange".into(),
            x => panic!("no distribution {x} for Fe"),
        }
    }

    fn sample(&self, dist: usize, n_bits: usize) -> FlatValue {
        assert_eq!(
            n_bits, 256,
            "tried to sample a FE with {} bits, not 256",
            n_bits
        );

        match dist {
            // uniform and low-bit-weight
            0..=2 => {
                let mut ret = UniformBits.sample(dist, 256);
                // lazy way to ensure in-range, though this misses the highest allowable values
                ret.inner[27] &= !1; // flip 2^32 bit to zero
                ret
            }
            // high bit-weight values, i.e. "out of range"
            3 => {
                // Field order: 2^256 - 2^32 - 2^9 - 2^8 - 2^7 - 2^6 - 2^4 - 1
                // We will cheat by sampling uniformly in the range 2^256 - 2^32 up to 2^256, which
                // loses 977 values. For benchmarking purposes these values shouldn't matter.
                let mut ret = FlatValue::zero_n_bits(n_bits);
                for bit in &mut ret.inner[0..27] {
                    *bit = 0xff;
                }
                ret.inner[27] = 0xff; // in fe order this is fe; we set to ff to be out of range
                rand::thread_rng().fill_bytes(&mut ret.inner[28..32]);
                ret
            }
            x => panic!("no distribution {x} for Fe"),
        }
    }
}

impl InputSampleExactSize for Fe {
    fn n_bits(&self) -> usize {
        256
    }
}

pub struct Scalar;

impl InputSample for Scalar {
    fn n_distributions(&self) -> usize {
        // uniform, plus an extra "out of range" distribution
        4
    }

    fn distribution_name(&self, dist: usize) -> String {
        match dist {
            // uniform and low-bit-weight
            0..=2 => format!("sc_{}", UniformBits.distribution_name(dist)),
            3 => "scalar_outofrange".into(),
            x => panic!("no distribution {x} for Scalar"),
        }
    }

    fn sample(&self, dist: usize, n_bits: usize) -> FlatValue {
        assert_eq!(
            n_bits, 256,
            "tried to sample a FE with {} bits, not 256",
            n_bits
        );

        match dist {
            // uniform and low-bit-weight
            0..=2 => {
                let mut ret = UniformBits.sample(dist, 256);
                // lazy way to ensure in-range, though this misses the highest allowable values
                ret.inner[27] &= !1; // flip 2^32 bit to zero
                ret
            }
            // high bit-weight values, i.e. "out of range"
            3 => {
                // Scalar order: 0xffffffff ffffffff ffffffff fffffffe baaedce6 af48a03b bfd25e8c d0364141
                // Again, for out-of-order points we cheat by changing that high fe to a ff then
                // sampling uniformly past that.
                let mut ret = FlatValue::zero_n_bits(n_bits);
                for bit in &mut ret.inner[0..15] {
                    *bit = 0xff;
                }
                ret.inner[15] = 0xff; // in scalar order this is fe; we set to ff to be out of range
                rand::thread_rng().fill_bytes(&mut ret.inner[16..32]);
                ret
            }
            x => panic!("no distribution {x} for Scalar"),
        }
    }
}

impl InputSampleExactSize for Scalar {
    fn n_bits(&self) -> usize {
        256
    }
}

/// A group element represented as an x, y pair of Fes.
///
/// There is no sanctioned way to represent infinity as a ge, as no jets
/// output them. But you can represent invalid points, including (0, 0),
/// and the jets are all defined to do something with such points.w
pub struct Ge;

impl InputSample for Ge {
    fn n_distributions(&self) -> usize {
        // To avoid an exponential blowup, we are not exhaustive here. Instead we sample
        // all fes from the same distribution. We add one extra distribution for "valid
        // fes but off-curve points".
        1 + Fe.n_distributions()
    }

    fn distribution_name(&self, dist: usize) -> String {
        if dist == 0 {
            "ge_oncurve".into()
        } else {
            format!("ge_offc_{}", Fe.distribution_name(dist - 1))
        }
    }

    fn sample(&self, dist: usize, n_bits: usize) -> FlatValue {
        assert_eq!(n_bits, 2 * 256, "attempt to sample GE with {} bits", n_bits);

        if dist == 0 {
            // valid fes, on-curve point
            loop {
                let x = Fe.sample(0, 256);
                let x2 = x.call_jet(Elements::FeSquare, 256);
                let x3 = FlatValue::product(&[x.clone(), x2]).call_jet(Elements::FeMultiply, 256);

                let mut seven = FlatValue::zero_n_bits(256);
                seven.inner[31] = 7;
                let x3_7 = FlatValue::product(&[x3, seven]).call_jet(Elements::FeAdd, 256);
                let mut y = x3_7.call_jet(Elements::FeSquareRoot, 257);

                if y.inner[0] & 0x80 != 0 {
                    y.left_shift(1);
                    if rand::random() {
                        y = y.call_jet(Elements::FeNegate, 256);
                    }

                    break FlatValue::product(&[x, y]);
                }
            }
        } else {
            // Valid fes but off-curve point, OR invalid fes in some way.
            // Either way we just sample fes and don't try to make them fit the curve.
            let dist = dist - 1;
            FlatValue::product(&[Fe.sample(dist, 256), Fe.sample(dist, 256)])
        }
    }
}

impl InputSampleExactSize for Ge {
    fn n_bits(&self) -> usize {
        2 * 256
    }
}

/// A group element represented as an x, y, z triple of Fes.
///
/// If z == 0 then the point represents "infinity".
pub struct Gej;

impl InputSample for Gej {
    fn n_distributions(&self) -> usize {
        // To avoid an exponential blowup, we are not exhaustive here. Instead we sample
        // all fes from the same distribution. There are two possibilities for each FE
        // distribution: full points (sample x y z) and infinite points (sample x y and
        // set z = 0).
        //
        // Finally we add an extra "valid fes but off-curve point" distribution.
        1 + 2 * Fe.n_distributions()
    }

    fn distribution_name(&self, dist: usize) -> String {
        if dist == 0 {
            "gej_oncurve".into()
        } else if dist < 1 + Fe.n_distributions() {
            format!("gej_offc_{}", Fe.distribution_name(dist - 1))
        } else {
            format!(
                "gej_zero_{}",
                Fe.distribution_name(dist - 1 - Fe.n_distributions())
            )
        }
    }

    fn sample(&self, dist: usize, n_bits: usize) -> FlatValue {
        assert_eq!(
            n_bits,
            3 * 256,
            "attempt to sample GEJ with {} bits",
            n_bits
        );

        if dist == 0 {
            // valid fes, on-curve point
            let (ge_x, ge_y) = Ge.sample(0, 512).split_in_half();

            let z = Fe.sample(0, 256);
            let z2 = z.call_jet(Elements::FeSquare, 256);
            let z3 =
                FlatValue::product(&[z.clone(), z2.clone()]).call_jet(Elements::FeMultiply, 256);

            let xj = FlatValue::product(&[ge_x, z2]).call_jet(Elements::FeMultiply, 256);
            let yj = FlatValue::product(&[ge_y, z3]).call_jet(Elements::FeMultiply, 256);
            FlatValue::product(&[xj, yj, z])
        } else {
            // Valid fes but off-curve point, OR invalid fes in some way.
            // Either way we just sample fes and don't try to make them fit the curve.
            let dist = dist - 1;

            if dist < Fe.n_distributions() {
                FlatValue::product(&[
                    Fe.sample(dist, 256),
                    Fe.sample(dist, 256),
                    Fe.sample(dist, 256),
                ])
            } else {
                FlatValue::product(&[
                    Fe.sample(dist - Fe.n_distributions(), 256),
                    Fe.sample(dist - Fe.n_distributions(), 256),
                    FlatValue::zero_n_bits(256),
                ])
            }
        }
    }
}

impl InputSampleExactSize for Gej {
    fn n_bits(&self) -> usize {
        3 * 256
    }
}

/// A group element represented as a y parity bit and an x coordinate.
///
/// If z == 0 then the point represents "infinity".
pub struct Point;

impl InputSample for Point {
    fn n_distributions(&self) -> usize {
        Ge.n_distributions()
    }

    fn distribution_name(&self, dist: usize) -> String {
        format!("pt_{}", Ge.distribution_name(dist))
    }

    fn sample(&self, dist: usize, n_bits: usize) -> FlatValue {
        assert_eq!(
            n_bits, 257,
            "attempt to sample compact point with {} bits",
            n_bits
        );

        let (ge_x, ge_y) = Ge.sample(dist, 512).split_in_half();
        if ge_y.inner[31] & 1 == 0 {
            ge_x.prefix_zero()
        } else {
            ge_x.prefix_one()
        }
    }
}

impl InputSampleExactSize for Point {
    fn n_bits(&self) -> usize {
        257
    }
}

pub struct Bip340Signature;

impl InputSample for Bip340Signature {
    fn n_distributions(&self) -> usize {
        // valid signatures and invalid signatures; we don't bother sampling
        // things with out-of-range or otherwise invalidly encoded values
        // lead to early returns, and we know that actual signature verification
        // will be overwhelmingly more expensive than that
        2
    }

    fn distribution_name(&self, dist: usize) -> String {
        if dist == 0 {
            "bip340_valid".into()
        } else {
            "bip340_invalid".into()
        }
    }

    fn sample(&self, dist: usize, n_bits: usize) -> FlatValue {
        use simplicity::bitcoin;

        assert_eq!(
            n_bits, 1024,
            "attempt to sample BIP-340 signature with {} bits",
            n_bits
        );

        if dist == 0 {
            // valid sigs
            let secp_ctx = bitcoin::secp256k1::Secp256k1::new();
            let keypair = bitcoin::key::Keypair::new(&secp_ctx, &mut rand::thread_rng());
            let xpk = bitcoin::key::XOnlyPublicKey::from_keypair(&keypair);

            let msg = bitcoin::secp256k1::Message::from_digest_slice(&rand::random::<[u8; 32]>())
                .unwrap();
            let sig = secp_ctx.sign_schnorr(&msg, &keypair);
            //assert!(secp_ctx.verify_schnorr(&sig, &msg, &xpk.0).is_ok());

            let mut ret = FlatValue::zero_n_bits(1024);
            ret.inner[..32].copy_from_slice(&xpk.0.serialize());
            ret.inner[32..96].copy_from_slice(sig.as_ref());
            ret.inner[96..128].copy_from_slice(&msg[..]);
            ret
        } else if dist == 1 {
            let (ge_x, _) = Ge.sample(0, 512).split_in_half();
            // invalid sigs
            FlatValue::product(&[ge_x, UniformBits.sample(0, 768)])
        } else {
            panic!("invalid distribution {} for BIP 340 signature", dist)
        }
    }
}

impl InputSampleExactSize for Bip340Signature {
    fn n_bits(&self) -> usize {
        1024
    }
}

pub struct CheckSigSignature;

impl InputSample for CheckSigSignature {
    fn n_distributions(&self) -> usize {
        // valid signatures and invalid signatures; we don't bother sampling
        // things with out-of-range or otherwise invalidly encoded values
        // lead to early returns, and we know that actual signature verification
        // will be overwhelmingly more expensive than that
        2
    }

    fn distribution_name(&self, dist: usize) -> String {
        if dist == 0 {
            "checksig_valid".into()
        } else {
            "checksig_invalid".into()
        }
    }

    fn sample(&self, dist: usize, n_bits: usize) -> FlatValue {
        use simplicity::{bitcoin, hashes};

        fn tagged_hash(tag: &[u8], msg_block: [u8; 64]) -> hashes::sha256::Hash {
            use simplicity::hashes::{Hash as _, HashEngine as _};

            let tag_hash = hashes::sha256::Hash::hash(tag);
            let block = [tag_hash.to_byte_array(), tag_hash.to_byte_array()].concat();
            let mut engine = hashes::sha256::Hash::engine();
            engine.input(&block);
            engine.input(&msg_block);

            hashes::sha256::Hash::from_engine(engine)
        }

        assert_eq!(
            n_bits, 1280,
            "attempt to sample checksig signature with {} bits",
            n_bits
        );

        if dist == 0 {
            // valid sigs
            let secp_ctx = bitcoin::secp256k1::Secp256k1::signing_only();
            let keypair = bitcoin::key::Keypair::new(&secp_ctx, &mut rand::thread_rng());
            let xpk = bitcoin::key::XOnlyPublicKey::from_keypair(&keypair);

            let msg = [0xab; 64];
            let hashed_msg = tagged_hash(b"Simplicity\x1fSignature", msg);
            let hashed_msg = bitcoin::secp256k1::Message::from_digest(hashed_msg.to_byte_array());
            let sig = secp_ctx.sign_schnorr(&hashed_msg, &keypair);

            let mut ret = FlatValue::zero_n_bits(1024);
            ret.inner[..32].copy_from_slice(&xpk.0.serialize());
            ret.inner[32..96].copy_from_slice(sig.as_ref());
            ret.inner[96..160].copy_from_slice(&msg[..]);
            ret
        } else if dist == 1 {
            let (ge_x, _) = Ge.sample(0, 512).split_in_half();
            // invalid sigs
            FlatValue::product(&[ge_x, UniformBits.sample(0, 1024)])
        } else {
            panic!("invalid distribution {} for BIP 340 signature", dist)
        }
    }
}

impl InputSampleExactSize for CheckSigSignature {
    fn n_bits(&self) -> usize {
        1280
    }
}

pub struct DivMod12864Input;

impl InputSample for DivMod12864Input {
    fn n_distributions(&self) -> usize {
        // divmod128_64 has specific preconditions (unlike the other divmod jets).
        // If these are not met then it does a quick shortcut error return.
        2
    }

    fn distribution_name(&self, dist: usize) -> String {
        if dist == 0 {
            "uniform".into()
        } else {
            "divmod12864precondition".into()
        }
    }

    fn sample(&self, dist: usize, n_bits: usize) -> FlatValue {
        assert_eq!(
            n_bits, 192,
            "attempt to sample divmod128_64 input with {} bits (expected 192)",
            n_bits
        );

        if dist == 0 {
            UniformBits.sample(0, 192)
        } else if dist == 1 {
            // The input to div_mod_128_64 is a 128-bit integer `a` followed by a 64-bit integer `b`.
            // It has two preconditions:
            //     1. The high bit of b must be set.
            //     2. ah (the high 64 bits of a) is strictly less than b.
            //
            // See the implementation of `simplicity_div_mod_128_64` in libsimplicity C/jets.c
            // for how these are implemented.

            // To implement this, we sample two random 64-bit strings, with one of them
            // having its high bit forced to 1. The higher of the two (which will always
            // have its high bit 1) is b, and the other one is ah.
            let sample_1 = FlatValue::random_n_bits(63).prefix_one();
            let sample_2 = FlatValue::random_n_bits(64);
            for (bit1, bit2) in sample_1.bit_iter().zip(sample_2.bit_iter()) {
                match (bit1, bit2) {
                    (false, false) | (true, true) => {} // both equal
                    (true, false) => return FlatValue::product(&[sample_2, UniformBits.sample(0, 64), sample_1]),
                    (false, true) => return FlatValue::product(&[sample_1, UniformBits.sample(0, 64), sample_2]),
                }
            }
            unreachable!("if we get here, two uniform 63-bit samples were exactly equal")
        } else {
            panic!("invalid distribution {} for div_mod_128_64", dist)
        }
    }
}

impl InputSampleExactSize for DivMod12864Input {
    fn n_bits(&self) -> usize {
        192
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use simplicity::bitcoin::secp256k1;

    #[test]
    fn sample_ge() {
        let val = Ge.sample(0, 512);

        let mut arr = [0; 65];
        arr[0] = 4;
        arr[1..65].copy_from_slice(&val.inner[..64]);
        assert!(secp256k1::PublicKey::from_slice(&arr[..]).is_ok());

        let neg_ge = val.call_jet(Elements::GeNegate, 512);
        assert_eq!(&val.inner[..32], &neg_ge.inner[..32]);

        let neg_neg_ge = neg_ge.call_jet(Elements::GeNegate, 512);
        assert_eq!(&val.inner[..64], &neg_neg_ge.inner[..64]);
    }

    #[test]
    fn sample_gej() {
        let _val = Gej.sample(0, 768);
    }

    #[test]
    fn sample_ge_buffer() {
        use crate::{ElementsEnvSampling, JetBuffer, JetParams};
        use simplicity::jet::Jet;

        let env = ElementsEnvSampling::null().env();
        let jet = Elements::PointVerify1;

        let src_ty = jet.source_ty().to_final();
        let tgt_ty = jet.target_ty().to_final();
        let params = JetParams::for_sample(
            0,
            &GenericProduct(GenericProduct(GenericProduct(Scalar, Point), Scalar), Point),
        );
        let mut buffer = JetBuffer::new(&src_ty, &tgt_ty, &params);

        let (src, mut dst) = buffer.write(&src_ty, &params, &mut rand::thread_rng());

        jet.c_jet_ptr()(&mut dst, src, env.c_tx_env());
    }

    #[test]
    #[should_panic]
    fn linear_verify() {
        let input = FlatValue::product(&[
            Scalar.sample(0, 256),
            Ge.sample(0, 512),
            Scalar.sample(0, 256),
            Ge.sample(0, 512),
        ]);
        input.call_jet(Elements::LinearVerify1, 1);
    }
}
