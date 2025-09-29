use std::sync::Arc;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand::rngs::ThreadRng;
use simplicity::bitcoin;
use simplicity::jet::bitcoin::BitcoinEnv;
use simplicity::jet::{Bitcoin, Jet};
use simplicity::types;
use simplicity::types::Final;
use simplicity::Value;
use simplicity_bench::input::{
    self, EqProduct, GenericProduct, InputSample, PrefixBit, Sha256Ctx, UniformBits,
    UniformBitsExact,
};
use simplicity_bench::{
    genesis_pegin, BitcoinEnvSampling, BenchSample, InputSampling, JetBuffer, JetParams, SimplicityCtx8,
    SimplicityEncode,
};

const NUM_RANDOM_SAMPLES: usize = 100;

/// Number of inputs and outputs in the tx
/// RATIONALE: One input to spend a asset, one input to pay fees, one input
/// to interact with the contract
/// Two outputs for asset(dest, change), two outputs for bitcoin (dest, change)
/// One output for fees, and one output for the contract
///
/// Why these constants don't matter: (FOR NOW)?
///
/// All jets actually use some pre-computed cache which does not depend on
/// the number of inputs and outputs. We have already pre-calculated all the
/// inputs and outputs. There is no iteration over jets, only
pub const NUM_TX_INPUTS: usize = 3;
pub const NUM_TX_OUTPUTS: usize = 6;

/// Worst case env for each jet
#[derive(PartialEq, Eq)]
enum BitcoinBenchEnvType {
    /// None
    None,
    /// Force the environment to have an annex
    Annex,
    /// Random env, we don't care about issuances, pegins or conf data
    /// These jets use pre-cached data and don't care about how the data
    /// was constructed
    Random,
}

impl BitcoinBenchEnvType {
    fn env(&self) -> BitcoinEnv<Arc<bitcoin::Transaction>> {
        let mut env_sampler = BitcoinEnvSampling::null();
        if *self != BitcoinBenchEnvType::None {
            env_sampler = env_sampler
                .n_inputs(NUM_TX_INPUTS)
                .n_outputs(NUM_TX_OUTPUTS);

            if *self == BitcoinBenchEnvType::Annex {
                env_sampler = env_sampler.with_annex();
            }
        };
        env_sampler.env()
    }
}

fn jet_arrow(jet: Bitcoin) -> (Arc<types::Final>, Arc<types::Final>) {
    let src_ty = jet.source_ty().to_final();
    let tgt_ty = jet.target_ty().to_final();
    (src_ty, tgt_ty)
}

// Separate out heavy jets to run them more times in our benchmark.
fn is_heavy_jet(jet: Bitcoin) -> bool {
    // Hashes
    match jet {
        Bitcoin::HashToCurve |
        Bitcoin::Sha256Iv |
        Bitcoin::Sha256Block |
        Bitcoin::Sha256Ctx8Init |
        Bitcoin::Sha256Ctx8Add1 |
        Bitcoin::Sha256Ctx8Add2 |
        Bitcoin::Sha256Ctx8Add4 |
        Bitcoin::Sha256Ctx8Add8 |
        Bitcoin::Sha256Ctx8Add16 |
        Bitcoin::Sha256Ctx8Add32 |
        Bitcoin::Sha256Ctx8Add64 |
        Bitcoin::Sha256Ctx8Add128 |
        Bitcoin::Sha256Ctx8Add256 |
        Bitcoin::Sha256Ctx8Add512 |
        Bitcoin::Sha256Ctx8AddBuffer511 |
        Bitcoin::Sha256Ctx8Finalize |
        Bitcoin::Swu |
        // Jets for secp FE
        Bitcoin::FeNormalize |
        Bitcoin::FeNegate |
        Bitcoin::FeAdd |
        Bitcoin::FeSquare |
        Bitcoin::FeMultiply |
        Bitcoin::FeMultiplyBeta |
        Bitcoin::FeInvert |
        Bitcoin::FeSquareRoot |
        Bitcoin::FeIsZero |
        Bitcoin::FeIsOdd |
        // Jets for secp scalars
        Bitcoin::ScalarNormalize |
        Bitcoin::ScalarNegate |
        Bitcoin::ScalarAdd |
        Bitcoin::ScalarSquare |
        Bitcoin::ScalarMultiply |
        Bitcoin::ScalarMultiplyLambda |
        Bitcoin::ScalarInvert |
        Bitcoin::ScalarIsZero |
        // Jets for secp gej points
        Bitcoin::GejInfinity |
        Bitcoin::GejRescale |
        Bitcoin::GejNormalize |
        Bitcoin::GejNegate |
        Bitcoin::GeNegate |
        Bitcoin::GejDouble |
        Bitcoin::GejAdd |
        Bitcoin::GejGeAddEx |
        Bitcoin::GejGeAdd |
        Bitcoin::GejIsInfinity |
        Bitcoin::GejEquiv |
        Bitcoin::GejGeEquiv |
        Bitcoin::GejXEquiv |
        Bitcoin::GejYIsOdd |
        Bitcoin::GejIsOnCurve |
        // Other jets
        Bitcoin::GeIsOnCurve |
        Bitcoin::Scale |
        Bitcoin::Generate |
        Bitcoin::LinearCombination1 |
        Bitcoin::LinearVerify1 |
        Bitcoin::Decompress |
        Bitcoin::PointVerify1 |
        // Signature jets
        Bitcoin::CheckSigVerify |
        Bitcoin::Bip0340Verify  => true,
        _ => false,
    }
}

#[rustfmt::skip]
fn bench(c: &mut Criterion) {
    // Sanity Check: This should never really fail, but still good to do
    if !simplicity::ffi::c_jets::sanity_checks() {
        panic!("Sanity checks failed");
    }

    // Initialize set of all jets
    let mut jet_checker = simplicity_bench::check_all_jets::JetChecker::initialize();

    let mut rng = ThreadRng::default();
    let mut count = 0;

    let arr: [(Bitcoin, &dyn InputSample); 368] = [
        // Bit logics
        (Bitcoin::Verify, &UniformBits),
        // low
        (Bitcoin::Low1, &input::Unit),
        (Bitcoin::Low8, &input::Unit),
        (Bitcoin::Low16, &input::Unit),
        (Bitcoin::Low32, &input::Unit),
        (Bitcoin::Low64, &input::Unit),
        // high
        (Bitcoin::High1, &input::Unit),
        (Bitcoin::High8, &input::Unit),
        (Bitcoin::High16, &input::Unit),
        (Bitcoin::High32, &input::Unit),
        (Bitcoin::High64, &input::Unit),
        // complement
        (Bitcoin::Complement1, &UniformBits),
        (Bitcoin::Complement8, &UniformBits),
        (Bitcoin::Complement16, &UniformBits),
        (Bitcoin::Complement32, &UniformBits),
        (Bitcoin::Complement64, &UniformBits),
        // and
        (Bitcoin::And1, &EqProduct(UniformBits)),
        (Bitcoin::And8, &EqProduct(UniformBits)),
        (Bitcoin::And16, &EqProduct(UniformBits)),
        (Bitcoin::And32, &EqProduct(UniformBits)),
        (Bitcoin::And64, &EqProduct(UniformBits)),
        // or
        (Bitcoin::Or1, &EqProduct(UniformBits)),
        (Bitcoin::Or8, &EqProduct(UniformBits)),
        (Bitcoin::Or16, &EqProduct(UniformBits)),
        (Bitcoin::Or32, &EqProduct(UniformBits)),
        (Bitcoin::Or64, &EqProduct(UniformBits)),
        // xor
        (Bitcoin::Xor1, &EqProduct(UniformBits)),
        (Bitcoin::Xor8, &EqProduct(UniformBits)),
        (Bitcoin::Xor16, &EqProduct(UniformBits)),
        (Bitcoin::Xor32, &EqProduct(UniformBits)),
        (Bitcoin::Xor64, &EqProduct(UniformBits)),
        // maj
        (Bitcoin::Maj1, &UniformBits),
        (Bitcoin::Maj8, &UniformBits),
        (Bitcoin::Maj16, &UniformBits),
        (Bitcoin::Maj32, &UniformBits),
        (Bitcoin::Maj64, &UniformBits),
        // xor xor
        (Bitcoin::XorXor1, &UniformBits),
        (Bitcoin::XorXor8, &UniformBits),
        (Bitcoin::XorXor16, &UniformBits),
        (Bitcoin::XorXor32, &UniformBits),
        (Bitcoin::XorXor64, &UniformBits),
        // ch
        (Bitcoin::Ch1, &UniformBits),
        (Bitcoin::Ch8, &UniformBits),
        (Bitcoin::Ch16, &UniformBits),
        (Bitcoin::Ch32, &UniformBits),
        (Bitcoin::Ch64, &UniformBits),
        // left shift
        (Bitcoin::LeftShift8, &UniformBits),
        (Bitcoin::LeftShift16, &UniformBits),
        (Bitcoin::LeftShift32, &UniformBits),
        (Bitcoin::LeftShift64, &UniformBits),
        (Bitcoin::LeftShiftWith8, &UniformBits),
        (Bitcoin::LeftShiftWith16, &UniformBits),
        (Bitcoin::LeftShiftWith32, &UniformBits),
        (Bitcoin::LeftShiftWith64, &UniformBits),
        // right shift
        (Bitcoin::RightShift8, &UniformBits),
        (Bitcoin::RightShift16, &UniformBits),
        (Bitcoin::RightShift32, &UniformBits),
        (Bitcoin::RightShift64, &UniformBits),
        (Bitcoin::RightShiftWith8, &UniformBits),
        (Bitcoin::RightShiftWith16, &UniformBits),
        (Bitcoin::RightShiftWith32, &UniformBits),
        (Bitcoin::RightShiftWith64, &UniformBits),
        // full left shift
        (Bitcoin::FullLeftShift8_1, &UniformBits),
        (Bitcoin::FullLeftShift8_2, &UniformBits),
        (Bitcoin::FullLeftShift8_4, &UniformBits),
        (Bitcoin::FullLeftShift16_1, &UniformBits),
        (Bitcoin::FullLeftShift16_2, &UniformBits),
        (Bitcoin::FullLeftShift16_4, &UniformBits),
        (Bitcoin::FullLeftShift16_8, &UniformBits),
        (Bitcoin::FullLeftShift32_1, &UniformBits),
        (Bitcoin::FullLeftShift32_2, &UniformBits),
        (Bitcoin::FullLeftShift32_4, &UniformBits),
        (Bitcoin::FullLeftShift32_8, &UniformBits),
        (Bitcoin::FullLeftShift32_16, &UniformBits),
        (Bitcoin::FullLeftShift64_1, &UniformBits),
        (Bitcoin::FullLeftShift64_2, &UniformBits),
        (Bitcoin::FullLeftShift64_4, &UniformBits),
        (Bitcoin::FullLeftShift64_8, &UniformBits),
        (Bitcoin::FullLeftShift64_16, &UniformBits),
        (Bitcoin::FullLeftShift64_32, &UniformBits),
        // full right shift
        (Bitcoin::FullRightShift8_1, &UniformBits),
        (Bitcoin::FullRightShift8_2, &UniformBits),
        (Bitcoin::FullRightShift8_4, &UniformBits),
        (Bitcoin::FullRightShift16_1, &UniformBits),
        (Bitcoin::FullRightShift16_2, &UniformBits),
        (Bitcoin::FullRightShift16_4, &UniformBits),
        (Bitcoin::FullRightShift16_8, &UniformBits),
        (Bitcoin::FullRightShift32_1, &UniformBits),
        (Bitcoin::FullRightShift32_2, &UniformBits),
        (Bitcoin::FullRightShift32_4, &UniformBits),
        (Bitcoin::FullRightShift32_8, &UniformBits),
        (Bitcoin::FullRightShift32_16, &UniformBits),
        (Bitcoin::FullRightShift64_1, &UniformBits),
        (Bitcoin::FullRightShift64_2, &UniformBits),
        (Bitcoin::FullRightShift64_4, &UniformBits),
        (Bitcoin::FullRightShift64_8, &UniformBits),
        (Bitcoin::FullRightShift64_16, &UniformBits),
        (Bitcoin::FullRightShift64_32, &UniformBits),
        // left rotate
        (Bitcoin::LeftRotate8, &UniformBits),
        (Bitcoin::LeftRotate16, &UniformBits),
        (Bitcoin::LeftRotate32, &UniformBits),
        (Bitcoin::LeftRotate64, &UniformBits),
        // right rotate
        (Bitcoin::RightRotate8, &UniformBits),
        (Bitcoin::RightRotate16, &UniformBits),
        (Bitcoin::RightRotate32, &UniformBits),
        (Bitcoin::RightRotate64, &UniformBits),
        // left extend
        (Bitcoin::LeftExtend1_8, &UniformBits),
        (Bitcoin::LeftExtend1_16, &UniformBits),
        (Bitcoin::LeftExtend1_32, &UniformBits),
        (Bitcoin::LeftExtend1_64, &UniformBits),
        (Bitcoin::LeftExtend8_16, &UniformBits),
        (Bitcoin::LeftExtend8_32, &UniformBits),
        (Bitcoin::LeftExtend8_64, &UniformBits),
        (Bitcoin::LeftExtend16_32, &UniformBits),
        (Bitcoin::LeftExtend16_64, &UniformBits),
        (Bitcoin::LeftExtend32_64, &UniformBits),
        // right extend
        // no right-extend for 1-bit values
        (Bitcoin::RightExtend8_16, &UniformBits),
        (Bitcoin::RightExtend8_32, &UniformBits),
        (Bitcoin::RightExtend8_64, &UniformBits),
        (Bitcoin::RightExtend16_32, &UniformBits),
        (Bitcoin::RightExtend16_64, &UniformBits),
        (Bitcoin::RightExtend32_64, &UniformBits),
        // left pad
        (Bitcoin::LeftPadLow1_8, &UniformBits),
        (Bitcoin::LeftPadLow1_16, &UniformBits),
        (Bitcoin::LeftPadLow1_32, &UniformBits),
        (Bitcoin::LeftPadLow1_64, &UniformBits),
        (Bitcoin::LeftPadLow8_16, &UniformBits),
        (Bitcoin::LeftPadLow8_32, &UniformBits),
        (Bitcoin::LeftPadLow8_64, &UniformBits),
        (Bitcoin::LeftPadLow16_32, &UniformBits),
        (Bitcoin::LeftPadLow16_64, &UniformBits),
        (Bitcoin::LeftPadLow32_64, &UniformBits),
        (Bitcoin::LeftPadHigh1_8, &UniformBits),
        (Bitcoin::LeftPadHigh1_16, &UniformBits),
        (Bitcoin::LeftPadHigh1_32, &UniformBits),
        (Bitcoin::LeftPadHigh1_64, &UniformBits),
        (Bitcoin::LeftPadHigh8_16, &UniformBits),
        (Bitcoin::LeftPadHigh8_32, &UniformBits),
        (Bitcoin::LeftPadHigh8_64, &UniformBits),
        (Bitcoin::LeftPadHigh16_32, &UniformBits),
        (Bitcoin::LeftPadHigh16_64, &UniformBits),
        (Bitcoin::LeftPadHigh32_64, &UniformBits),
        // right pad
        (Bitcoin::RightPadLow1_8, &UniformBits),
        (Bitcoin::RightPadLow1_16, &UniformBits),
        (Bitcoin::RightPadLow1_32, &UniformBits),
        (Bitcoin::RightPadLow1_64, &UniformBits),
        (Bitcoin::RightPadLow8_16, &UniformBits),
        (Bitcoin::RightPadLow8_32, &UniformBits),
        (Bitcoin::RightPadLow8_64, &UniformBits),
        (Bitcoin::RightPadLow16_32, &UniformBits),
        (Bitcoin::RightPadLow16_64, &UniformBits),
        (Bitcoin::RightPadLow32_64, &UniformBits),
        (Bitcoin::RightPadHigh1_8, &UniformBits),
        (Bitcoin::RightPadHigh1_16, &UniformBits),
        (Bitcoin::RightPadHigh1_32, &UniformBits),
        (Bitcoin::RightPadHigh1_64, &UniformBits),
        (Bitcoin::RightPadHigh8_16, &UniformBits),
        (Bitcoin::RightPadHigh8_32, &UniformBits),
        (Bitcoin::RightPadHigh8_64, &UniformBits),
        (Bitcoin::RightPadHigh16_32, &UniformBits),
        (Bitcoin::RightPadHigh16_64, &UniformBits),
        (Bitcoin::RightPadHigh32_64, &UniformBits),
        // leftmost
        (Bitcoin::Leftmost8_1, &UniformBits),
        (Bitcoin::Leftmost8_2, &UniformBits),
        (Bitcoin::Leftmost8_4, &UniformBits),
        (Bitcoin::Leftmost16_1, &UniformBits),
        (Bitcoin::Leftmost16_2, &UniformBits),
        (Bitcoin::Leftmost16_4, &UniformBits),
        (Bitcoin::Leftmost16_8, &UniformBits),
        (Bitcoin::Leftmost32_1, &UniformBits),
        (Bitcoin::Leftmost32_2, &UniformBits),
        (Bitcoin::Leftmost32_4, &UniformBits),
        (Bitcoin::Leftmost32_8, &UniformBits),
        (Bitcoin::Leftmost32_16, &UniformBits),
        (Bitcoin::Leftmost64_1, &UniformBits),
        (Bitcoin::Leftmost64_2, &UniformBits),
        (Bitcoin::Leftmost64_4, &UniformBits),
        (Bitcoin::Leftmost64_8, &UniformBits),
        (Bitcoin::Leftmost64_16, &UniformBits),
        (Bitcoin::Leftmost64_32, &UniformBits),
        // rightmost
        (Bitcoin::Rightmost8_1, &UniformBits),
        (Bitcoin::Rightmost8_2, &UniformBits),
        (Bitcoin::Rightmost8_4, &UniformBits),
        (Bitcoin::Rightmost16_1, &UniformBits),
        (Bitcoin::Rightmost16_2, &UniformBits),
        (Bitcoin::Rightmost16_4, &UniformBits),
        (Bitcoin::Rightmost16_8, &UniformBits),
        (Bitcoin::Rightmost32_1, &UniformBits),
        (Bitcoin::Rightmost32_2, &UniformBits),
        (Bitcoin::Rightmost32_4, &UniformBits),
        (Bitcoin::Rightmost32_8, &UniformBits),
        (Bitcoin::Rightmost32_16, &UniformBits),
        (Bitcoin::Rightmost64_1, &UniformBits),
        (Bitcoin::Rightmost64_2, &UniformBits),
        (Bitcoin::Rightmost64_4, &UniformBits),
        (Bitcoin::Rightmost64_8, &UniformBits),
        (Bitcoin::Rightmost64_16, &UniformBits),
        (Bitcoin::Rightmost64_32, &UniformBits),
        // some
        (Bitcoin::Some1, &UniformBits),
        (Bitcoin::Some8, &UniformBits),
        (Bitcoin::Some16, &UniformBits),
        (Bitcoin::Some32, &UniformBits),
        (Bitcoin::Some64, &UniformBits),
        // all
        (Bitcoin::All8, &UniformBits),
        (Bitcoin::All16, &UniformBits),
        (Bitcoin::All32, &UniformBits),
        (Bitcoin::All64, &UniformBits),
        // one
        (Bitcoin::One8, &input::Unit),
        (Bitcoin::One16, &input::Unit),
        (Bitcoin::One32, &input::Unit),
        (Bitcoin::One64, &input::Unit),
        // eq
        (Bitcoin::Eq1, &EqProduct(UniformBits)),
        (Bitcoin::Eq8, &EqProduct(UniformBits)),
        (Bitcoin::Eq16, &EqProduct(UniformBits)),
        (Bitcoin::Eq32, &EqProduct(UniformBits)),
        (Bitcoin::Eq64, &EqProduct(UniformBits)),
        (Bitcoin::Eq256, &EqProduct(UniformBits)),
        // Arithmetic
        // add
        (Bitcoin::Add8, &EqProduct(UniformBits)),
        (Bitcoin::Add16, &EqProduct(UniformBits)),
        (Bitcoin::Add32, &EqProduct(UniformBits)),
        (Bitcoin::Add64, &EqProduct(UniformBits)),
        // full add
        (Bitcoin::FullAdd8, &PrefixBit(EqProduct(UniformBits))),
        (Bitcoin::FullAdd16, &PrefixBit(EqProduct(UniformBits))),
        (Bitcoin::FullAdd32, &PrefixBit(EqProduct(UniformBits))),
        (Bitcoin::FullAdd64, &PrefixBit(EqProduct(UniformBits))),
        // full increment
        (Bitcoin::FullIncrement8, &PrefixBit(UniformBits)),
        (Bitcoin::FullIncrement16, &PrefixBit(UniformBits)),
        (Bitcoin::FullIncrement32, &PrefixBit(UniformBits)),
        (Bitcoin::FullIncrement64, &PrefixBit(UniformBits)),
        // increment
        (Bitcoin::Increment8, &UniformBits),
        (Bitcoin::Increment16, &UniformBits),
        (Bitcoin::Increment32, &UniformBits),
        (Bitcoin::Increment64, &UniformBits),
        // subtract
        (Bitcoin::Subtract8, &EqProduct(UniformBits)),
        (Bitcoin::Subtract16, &EqProduct(UniformBits)),
        (Bitcoin::Subtract32, &EqProduct(UniformBits)),
        (Bitcoin::Subtract64, &EqProduct(UniformBits)),
        // full subtract
        (Bitcoin::FullSubtract8, &PrefixBit(EqProduct(UniformBits))),
        (Bitcoin::FullSubtract16, &PrefixBit(EqProduct(UniformBits))),
        (Bitcoin::FullSubtract32, &PrefixBit(EqProduct(UniformBits))),
        (Bitcoin::FullSubtract64, &PrefixBit(EqProduct(UniformBits))),
        // negate
        (Bitcoin::Negate8, &UniformBits),
        (Bitcoin::Negate16, &UniformBits),
        (Bitcoin::Negate32, &UniformBits),
        (Bitcoin::Negate64, &UniformBits),
        // full decrement
        (Bitcoin::FullDecrement8, &PrefixBit(UniformBits)),
        (Bitcoin::FullDecrement16, &PrefixBit(UniformBits)),
        (Bitcoin::FullDecrement32, &PrefixBit(UniformBits)),
        (Bitcoin::FullDecrement64, &PrefixBit(UniformBits)),
        // decrement
        (Bitcoin::Decrement8, &UniformBits),
        (Bitcoin::Decrement16, &UniformBits),
        (Bitcoin::Decrement32, &UniformBits),
        (Bitcoin::Decrement64, &UniformBits),
        // multiply
        (Bitcoin::Multiply8, &EqProduct(UniformBits)),
        (Bitcoin::Multiply16, &EqProduct(UniformBits)),
        (Bitcoin::Multiply32, &EqProduct(UniformBits)),
        (Bitcoin::Multiply64, &EqProduct(UniformBits)),
        // full multiply
        (Bitcoin::FullMultiply8, &EqProduct(UniformBits)),
        (Bitcoin::FullMultiply16, &EqProduct(UniformBits)),
        (Bitcoin::FullMultiply32, &EqProduct(UniformBits)),
        (Bitcoin::FullMultiply64, &EqProduct(UniformBits)),
        // is zero
        (Bitcoin::IsZero8, &UniformBits),
        (Bitcoin::IsZero16, &UniformBits),
        (Bitcoin::IsZero32, &UniformBits),
        (Bitcoin::IsZero64, &UniformBits),
        // is one
        (Bitcoin::IsOne8, &UniformBits),
        (Bitcoin::IsOne16, &UniformBits),
        (Bitcoin::IsOne32, &UniformBits),
        (Bitcoin::IsOne64, &UniformBits),
        // le
        (Bitcoin::Le8, &EqProduct(UniformBits)),
        (Bitcoin::Le16, &EqProduct(UniformBits)),
        (Bitcoin::Le32, &EqProduct(UniformBits)),
        (Bitcoin::Le64, &EqProduct(UniformBits)),
        // lt
        (Bitcoin::Lt8, &EqProduct(UniformBits)),
        (Bitcoin::Lt16, &EqProduct(UniformBits)),
        (Bitcoin::Lt32, &EqProduct(UniformBits)),
        (Bitcoin::Lt64, &EqProduct(UniformBits)),
        // min
        (Bitcoin::Min8, &EqProduct(UniformBits)),
        (Bitcoin::Min16, &EqProduct(UniformBits)),
        (Bitcoin::Min32, &EqProduct(UniformBits)),
        (Bitcoin::Min64, &EqProduct(UniformBits)),
        // max
        (Bitcoin::Max8, &EqProduct(UniformBits)),
        (Bitcoin::Max16, &EqProduct(UniformBits)),
        (Bitcoin::Max32, &EqProduct(UniformBits)),
        (Bitcoin::Max64, &EqProduct(UniformBits)),
        // median
        (Bitcoin::Median8, &UniformBits),
        (Bitcoin::Median16, &UniformBits),
        (Bitcoin::Median32, &UniformBits),
        (Bitcoin::Median64, &UniformBits),
        // div mod
        (Bitcoin::DivMod8, &EqProduct(UniformBits)),
        (Bitcoin::DivMod16, &EqProduct(UniformBits)),
        (Bitcoin::DivMod32, &EqProduct(UniformBits)),
        (Bitcoin::DivMod64, &EqProduct(UniformBits)),
        (Bitcoin::DivMod128_64, &GenericProduct(UniformBitsExact::<128>, UniformBitsExact::<64>)),
        // divide
        (Bitcoin::Divide8, &EqProduct(UniformBits)),
        (Bitcoin::Divide16, &EqProduct(UniformBits)),
        (Bitcoin::Divide32, &EqProduct(UniformBits)),
        (Bitcoin::Divide64, &EqProduct(UniformBits)),
        // modulo
        (Bitcoin::Modulo8, &EqProduct(UniformBits)),
        (Bitcoin::Modulo16, &EqProduct(UniformBits)),
        (Bitcoin::Modulo32, &EqProduct(UniformBits)),
        (Bitcoin::Modulo64, &EqProduct(UniformBits)),
        // divides
        (Bitcoin::Divides8, &EqProduct(UniformBits)),
        (Bitcoin::Divides16, &EqProduct(UniformBits)),
        (Bitcoin::Divides32, &EqProduct(UniformBits)),
        (Bitcoin::Divides64, &EqProduct(UniformBits)),

        // Hashes
        (Bitcoin::HashToCurve, &UniformBits),
        (Bitcoin::Sha256Iv, &input::Unit),
        (Bitcoin::Sha256Block, &UniformBits),
        (Bitcoin::Sha256Ctx8Init, &input::Unit),
        (Bitcoin::Sha256Ctx8Add1, &GenericProduct(Sha256Ctx, UniformBits)),
        (Bitcoin::Sha256Ctx8Add2, &GenericProduct(Sha256Ctx, UniformBits)),
        (Bitcoin::Sha256Ctx8Add4, &GenericProduct(Sha256Ctx, UniformBits)),
        (Bitcoin::Sha256Ctx8Add8, &GenericProduct(Sha256Ctx, UniformBits)),
        (Bitcoin::Sha256Ctx8Add16, &GenericProduct(Sha256Ctx, UniformBits)),
        (Bitcoin::Sha256Ctx8Add32, &GenericProduct(Sha256Ctx, UniformBits)),
        (Bitcoin::Sha256Ctx8Add64, &GenericProduct(Sha256Ctx, UniformBits)),
        (Bitcoin::Sha256Ctx8Add128, &GenericProduct(Sha256Ctx, UniformBits)),
        (Bitcoin::Sha256Ctx8Add256, &GenericProduct(Sha256Ctx, UniformBits)),
        (Bitcoin::Sha256Ctx8Add512, &GenericProduct(Sha256Ctx, UniformBits)),
        (Bitcoin::Sha256Ctx8AddBuffer511, &GenericProduct(Sha256Ctx, UniformBits)),
        (Bitcoin::Sha256Ctx8Finalize, &Sha256Ctx),
        (Bitcoin::Swu, &UniformBits),
        // Jets for secp FE
        (Bitcoin::FeNormalize, &input::Fe),
        (Bitcoin::FeNegate, &input::Fe),
        (Bitcoin::FeAdd, &EqProduct(input::Fe)),
        (Bitcoin::FeSquare, &input::Fe),
        (Bitcoin::FeMultiply, &EqProduct(input::Fe)),
        (Bitcoin::FeMultiplyBeta, &input::Fe),
        (Bitcoin::FeInvert, &input::Fe),
        (Bitcoin::FeSquareRoot, &input::Fe),
        (Bitcoin::FeIsZero, &input::Fe),
        (Bitcoin::FeIsOdd, &input::Fe),
        // Jets for secp scalars
        (Bitcoin::ScalarNormalize, &input::Scalar),
        (Bitcoin::ScalarNegate, &input::Scalar),
        (Bitcoin::ScalarAdd, &EqProduct(input::Scalar)),
        (Bitcoin::ScalarSquare, &input::Scalar),
        (Bitcoin::ScalarMultiply, &EqProduct(input::Scalar)),
        (Bitcoin::ScalarMultiplyLambda, &input::Scalar),
        (Bitcoin::ScalarInvert, &input::Scalar),
        (Bitcoin::ScalarIsZero, &input::Scalar),
        // Jets for secp gej points
        // FIXME we should have specific samplers for GEJ pairs and GEJ-GE pairs
        // which relate the points in algebraic ways (being negatives, multiples
        // of lambda away from each other)
        (Bitcoin::GejInfinity, &input::Unit),
        (Bitcoin::GejRescale, &GenericProduct(input::Gej, input::Fe)),
        (Bitcoin::GejNormalize, &input::Gej), 
        (Bitcoin::GejNegate, &input::Gej), 
        (Bitcoin::GeNegate, &input::Ge),
        (Bitcoin::GejDouble, &input::Gej), 
        (Bitcoin::GejAdd, &EqProduct(input::Gej)),
        (Bitcoin::GejGeAddEx, &GenericProduct(input::Gej, input::Ge)),
        (Bitcoin::GejGeAdd, &GenericProduct(input::Gej, input::Ge)),
        (Bitcoin::GejIsInfinity, &input::Gej), 
        (Bitcoin::GejEquiv, &GenericProduct(input::Gej, input::Gej)),
        (Bitcoin::GejGeEquiv, &GenericProduct(input::Gej, input::Ge)),
        (Bitcoin::GejXEquiv, &GenericProduct(input::Fe, input::Gej)),
        (Bitcoin::GejYIsOdd, &input::Gej), 
        (Bitcoin::GejIsOnCurve, &input::Gej), 
        // Other jets
        (Bitcoin::GeIsOnCurve, &input::Ge),
        (Bitcoin::Scale, &GenericProduct(input::Scalar, input::Gej)),
        (Bitcoin::Generate, &input::Scalar),
        (Bitcoin::LinearCombination1, &GenericProduct(GenericProduct(input::Scalar, input::Gej), input::Scalar)),
        (Bitcoin::LinearVerify1, &GenericProduct(GenericProduct(GenericProduct(input::Scalar, input::Ge), input::Scalar), input::Ge)),
        (Bitcoin::Decompress, &input::Point),
        (Bitcoin::PointVerify1, &GenericProduct(GenericProduct(GenericProduct(input::Scalar, input::Point), input::Scalar), input::Point)),
        (Bitcoin::BuildTaptweak, &GenericProduct(input::Fe, input::Scalar)),
        // Signature jets
        (Bitcoin::CheckSigVerify, &input::CheckSigSignature),
        (Bitcoin::Bip0340Verify, &input::Bip340Signature),

        // Timelock parsing jets
        (Bitcoin::ParseLock, &UniformBits), // all values take same time
        (Bitcoin::ParseSequence, &PrefixBit(UniformBits)), // top bit treated specially
    ];
    for (jet, sample) in arr {
        count += 1;
        jet_checker.record(jet);
        println!(
            "[{:3}/{:3}] For {} we have {} distributions",
            count, jet_checker.n_jets(), jet, sample.n_distributions(),
        );

        let (src_ty, tgt_ty) = jet_arrow(jet);

        let mut group = c.benchmark_group(jet.to_string());
        let env = BitcoinEnvSampling::null().env();
        if is_heavy_jet(jet) {
            group.measurement_time(std::time::Duration::from_secs(5));
        };
        for dist in 0..sample.n_distributions() {
            let params = JetParams::for_sample(dist, sample);
            // Assumption: `buffer.write` is non-negligible
            for i in 0..5 {
                let bench_name = format!("{}_{}", sample.distribution_name(dist), i);
                group.bench_with_input(bench_name, &params,|b, params| {
                    b.iter_batched(
                        || {
                            let mut buffer = JetBuffer::new(&src_ty, &tgt_ty, params);
                            let (src, dst) = buffer.write(&src_ty, params, &mut rng);
                            (dst, src, &env, buffer)
                        },
                        |(mut dst, src, env, _buffer)| jet.c_jet_ptr()(&mut dst, src, env.c_tx_env()),
                        BatchSize::SmallInput,
                    )
                });
            }
        }
        group.finish();
    }

    // SIGHALL jets chapter jets with unit src type
    let jets = [
        // The jets below just read data from pre-computed cache which is fixed
        // regardless of the input data. That is, we just read hashed values
        // from the cache and return them.
        (Bitcoin::SigAllHash, BitcoinBenchEnvType::Random),
        (Bitcoin::TxHash, BitcoinBenchEnvType::Random),
        (Bitcoin::TapEnvHash, BitcoinBenchEnvType::Random),
        (Bitcoin::InputsHash, BitcoinBenchEnvType::Random),
        (Bitcoin::OutputsHash, BitcoinBenchEnvType::Random),
        (Bitcoin::InputUtxosHash, BitcoinBenchEnvType::Random),
        (Bitcoin::OutputValuesHash, BitcoinBenchEnvType::Random),
        (Bitcoin::OutputScriptsHash, BitcoinBenchEnvType::Random),
        (Bitcoin::InputOutpointsHash, BitcoinBenchEnvType::Random),
        (Bitcoin::InputSequencesHash, BitcoinBenchEnvType::Random),
        (Bitcoin::InputAnnexesHash, BitcoinBenchEnvType::Random),
        (Bitcoin::InputScriptSigsHash, BitcoinBenchEnvType::Random),
        (Bitcoin::InputValuesHash, BitcoinBenchEnvType::Random),
        (Bitcoin::InputScriptsHash, BitcoinBenchEnvType::Random),
        (Bitcoin::TapleafHash, BitcoinBenchEnvType::Random),
        (Bitcoin::TappathHash, BitcoinBenchEnvType::Random),
        (Bitcoin::TotalInputValue, BitcoinBenchEnvType::Random),
        (Bitcoin::TotalOutputValue, BitcoinBenchEnvType::Random),
        (Bitcoin::Fee, BitcoinBenchEnvType::Random),
        //
        // ------------------------------------
        // Jets with no environment required. But no custom sampling
        (Bitcoin::BuildTapleafSimplicity, BitcoinBenchEnvType::None),
        (Bitcoin::BuildTapbranch, BitcoinBenchEnvType::None),
        (Bitcoin::TapdataInit, BitcoinBenchEnvType::None),
        // ------------------------------------
        // Timelock jets
        // No need to specially consider issuances or pegins
        (Bitcoin::CheckLockHeight, BitcoinBenchEnvType::Random),
        (Bitcoin::CheckLockTime, BitcoinBenchEnvType::Random),
        (Bitcoin::CheckLockDistance, BitcoinBenchEnvType::Random),
        (Bitcoin::CheckLockDuration, BitcoinBenchEnvType::Random),
        (Bitcoin::TxLockHeight, BitcoinBenchEnvType::Random),
        (Bitcoin::TxLockTime, BitcoinBenchEnvType::Random),
        (Bitcoin::TxLockDistance, BitcoinBenchEnvType::Random),
        (Bitcoin::TxLockDuration, BitcoinBenchEnvType::Random),
        (Bitcoin::TxIsFinal, BitcoinBenchEnvType::Random),
        // Jets with tx introspection
        // Nothing special about issuances or pegins
        (Bitcoin::ScriptCMR, BitcoinBenchEnvType::Random),
        (Bitcoin::InternalKey, BitcoinBenchEnvType::Random),
        (Bitcoin::CurrentIndex, BitcoinBenchEnvType::Random),
        (Bitcoin::NumInputs, BitcoinBenchEnvType::Random),
        (Bitcoin::NumOutputs, BitcoinBenchEnvType::Random),
        (Bitcoin::LockTime, BitcoinBenchEnvType::Random),
        (Bitcoin::TransactionId, BitcoinBenchEnvType::Random),
        // // -----------------------------------------
        // Current Input
        // Each jet has worst case dependent on whether it is pegin or issuance
        // or none
        (Bitcoin::CurrentPrevOutpoint, BitcoinBenchEnvType::Random),
        (Bitcoin::CurrentValue, BitcoinBenchEnvType::Random),
        (Bitcoin::CurrentScriptHash, BitcoinBenchEnvType::Random),
        (Bitcoin::CurrentSequence, BitcoinBenchEnvType::Random),
        // Annex note: We explicitly add annex in inputs
        (Bitcoin::CurrentAnnexHash, BitcoinBenchEnvType::Random),
        (Bitcoin::CurrentScriptSigHash, BitcoinBenchEnvType::Random),
        // -----------------------------------------
        // General tx jets
        (Bitcoin::TapleafVersion, BitcoinBenchEnvType::None),
        (Bitcoin::Version, BitcoinBenchEnvType::None),
    ];

    // Bitcoin environment jets
    for (jet, env_sampler) in jets {
        jet_checker.record(jet);

        let (src_ty, tgt_ty) = jet_arrow(jet);
        let env = env_sampler.env();

        let mut group = c.benchmark_group(jet.to_string());
        for i in 0..NUM_RANDOM_SAMPLES {
            let params = JetParams::with_rand_aligns(InputSampling::Random);
            let name = format!("{}", i);
            group.bench_with_input(&name, &params,|b, params| {
                b.iter_batched(
                    || {
                        let mut buffer = JetBuffer::new(&src_ty, &tgt_ty, params);
                        let (src, dst) = buffer.write(&src_ty, params, &mut rng);
                        (dst, src, buffer)
                    },
                    |(mut dst, src, _buffer)| jet.c_jet_ptr()(&mut dst, src, env.c_tx_env()),
                    BatchSize::SmallInput,
                )
            });
        }
        group.finish();
    }

    // Input to outpoint hash jet
    fn outpoint_hash() -> Value {
        let ctx8 = SimplicityCtx8::with_len(511).value();
        let outpoint = bitcoin::OutPoint::sample().value();
        Value::product(ctx8, outpoint)
    }

    fn annex_hash() -> Value {
        let ctx8 = SimplicityCtx8::with_len(511).value();
        let annex = if rand::random() {
            Value::some(Value::u256(rand::random::<[u8; 32]>()))
        } else {
            Value::none(Final::u256())
        };
        Value::product(ctx8, annex)
    }
    let arr: [(Bitcoin, Arc<dyn Fn() -> Value>); 2] = [
        (Bitcoin::OutpointHash, Arc::new(&outpoint_hash)),
        (Bitcoin::AnnexHash, Arc::new(annex_hash)),
    ];

    for (jet, inp_fn) in arr {
        jet_checker.record(jet);

        let (src_ty, tgt_ty) = jet_arrow(jet);
        let env = BitcoinEnvSampling::null().env();

        let mut group = c.benchmark_group(jet.to_string());
        for i in 0..NUM_RANDOM_SAMPLES {
            let params = JetParams::with_rand_aligns(InputSampling::Custom(inp_fn.clone()));
            let name = format!("{}", i);
            group.bench_with_input(&name, &params, |b, params| {
                b.iter_batched(
                    || {
                        // Bitcoin sighash chapter jets with non-unit src type
                        let mut buffer = JetBuffer::new(&src_ty, &tgt_ty, params);
                        let (src, dst) = buffer.write(&src_ty, params, &mut rng);
                        (dst, src, buffer)
                    },
                    |(mut dst, src, _buffer)| jet.c_jet_ptr()(&mut dst, src, env.c_tx_env()),
                    BatchSize::SmallInput,
                )
            });
        }
        group.finish()
    }

    // Operations that use tx input or output index.
    fn index_value(bound: u32) -> Value {
        let v = rand::random::<u32>() % bound;
        Value::u32(v)
    }

    #[allow(clippy::enum_variant_names)]
    enum Index {
        // Select the input index 0. This is where we do pegin/issuance/annex etc.
        InputIdx0,
        // Select random input
        Input,
        // Select random output
        Output,
        // Markle branch index
        MarkleBranchIndex,
    }
    // Jets that operate on index
    // arr contains a tuple of three things.
    // arr[i].0 = jet
    // arr[i].1 = The input index to select. Two choices: (input, output)
    // arr[i].2 = The environment type to use.
    //
    // For jets that depend on current index, we have made the environment such that
    // index 0 would be input with pegin/issuance/annex etc.
    // For jets that merely introspect,
    let arr = [
        // // Transaction chapter jets with output index
        (Bitcoin::OutputValue, Index::Output, BitcoinBenchEnvType::Random),
        (Bitcoin::OutputHash, Index::Output, BitcoinBenchEnvType::Random),
        (Bitcoin::OutputScriptHash, Index::Output, BitcoinBenchEnvType::Random),
        // // Transaction chapter jets with input index
        (Bitcoin::InputPrevOutpoint, Index::Input, BitcoinBenchEnvType::Random),
        (Bitcoin::InputValue, Index::Input, BitcoinBenchEnvType::Random),
        (Bitcoin::InputHash, Index::Input, BitcoinBenchEnvType::Random),
        (Bitcoin::InputScriptHash, Index::Input, BitcoinBenchEnvType::Random),
        (Bitcoin::InputSequence, Index::Input, BitcoinBenchEnvType::Random),
        (Bitcoin::InputUtxoHash, Index::Input, BitcoinBenchEnvType::Random),
        (Bitcoin::InputAnnexHash, Index::InputIdx0, BitcoinBenchEnvType::Annex),
        (Bitcoin::InputScriptSigHash, Index::Input, BitcoinBenchEnvType::Random),
        (Bitcoin::Tappath, Index::MarkleBranchIndex, BitcoinBenchEnvType::Random),
    ];

    for (jet, index, env_type) in arr {
        jet_checker.record(jet);

        let (src_ty, tgt_ty) = jet_arrow(jet);
        let env = env_type.env();
        let mut group = c.benchmark_group(jet.to_string());

        for i in 0..NUM_RANDOM_SAMPLES {
            // We always select the current input because this is where we
            // are doing issuances/pegins/etc.
            let v = match index {
                Index::InputIdx0 => index_value(1),
                Index::Input => index_value(NUM_TX_INPUTS as u32),
                Index::Output => index_value(NUM_TX_OUTPUTS as u32), // any output
                Index::MarkleBranchIndex => Value::u8(0), // 0 index
            };
            let params = JetParams::with_rand_aligns(InputSampling::Fixed(v));
            let name = format!("{}", i);
            group.bench_with_input(&name, &params, |b, params| {
                b.iter_batched(
                    || {
                        let mut buffer = JetBuffer::new(&src_ty, &tgt_ty, params);
                        let (src, dst) = buffer.write(&src_ty, params, &mut rng);
                        (dst, src, buffer)
                    },
                    |(mut dst, src, _buffer)| jet.c_jet_ptr()(&mut dst, src, env.c_tx_env()),
                    BatchSize::SmallInput,
                )
            });
        }
    }

    jet_checker.check_all_covered();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        // For simpler benchmarks, we don't need to run for long
        // We care most about secp jets
        .measurement_time(std::time::Duration::from_millis(50))
        .warm_up_time(std::time::Duration::from_millis(50))
        // .sample_size(100)
        // .nresamples(10_000)
        .plotting_backend(criterion::PlottingBackend::None)
        .without_plots();
    targets = bench,
}
criterion_main!(benches);
