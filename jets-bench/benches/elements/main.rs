use std::sync::Arc;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use elements::confidential;
use rand::rngs::ThreadRng;
use simplicity::elements;
use simplicity::jet::elements::ElementsEnv;
use simplicity::jet::{Elements, Jet};
use simplicity::types;
use simplicity::types::Final;
use simplicity::Value;
use simplicity_bench::input::{
    self, EqProduct, GenericProduct, InputSample, PrefixBit, Sha256Ctx, UniformBits,
    DivMod12864Input,
};
use simplicity_bench::{
    genesis_pegin, BenchSample, ElementsEnvSampling, InputSampling, JetBuffer, JetParams, SimplicityCtx8,
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
enum ElementsBenchEnvType {
    /// None
    None,
    /// Random env, we don't care about issuances, pegins or conf data
    /// These jets use pre-cached data and don't care about how the data
    /// was constructed
    Random,
    /// Jets have worst case on issuance inputs
    Issuance,
    /// Jets have worst case on pegin inputs
    Pegin,
    /// Annex inputs
    Annex,
    /// Jets have worst case on fee outputs
    AllFeeOutputs,
}

impl ElementsBenchEnvType {
    fn env(&self) -> ElementsEnv<Arc<elements::Transaction>> {
        let mut env_sampler = match self {
            ElementsBenchEnvType::None => ElementsEnvSampling::null(),
            ElementsBenchEnvType::Random
            | ElementsBenchEnvType::Annex
            | ElementsBenchEnvType::AllFeeOutputs => {
                let selector = rand::random::<usize>() % 4;
                ElementsEnvSampling::random(selector)
            }
            ElementsBenchEnvType::Issuance => ElementsEnvSampling::issuance(),
            ElementsBenchEnvType::Pegin => ElementsEnvSampling::pegin(),
        };
        if *self == ElementsBenchEnvType::AllFeeOutputs {
            env_sampler = env_sampler.all_fee_outputs();
        }
        env_sampler = env_sampler
            .n_inputs(NUM_TX_INPUTS)
            .n_outputs(NUM_TX_OUTPUTS);
        env_sampler.env()
    }
}

fn jet_arrow(jet: Elements) -> (Arc<types::Final>, Arc<types::Final>) {
    let src_ty = jet.source_ty().to_final();
    let tgt_ty = jet.target_ty().to_final();
    (src_ty, tgt_ty)
}

// Separate out heavy jets to run them more times in our benchmark.
fn is_heavy_jet(jet: Elements) -> bool {
    // Hashes
    match jet {
        Elements::HashToCurve |
        Elements::Sha256Iv |
        Elements::Sha256Block |
        Elements::Sha256Ctx8Init |
        Elements::Sha256Ctx8Add1 |
        Elements::Sha256Ctx8Add2 |
        Elements::Sha256Ctx8Add4 |
        Elements::Sha256Ctx8Add8 |
        Elements::Sha256Ctx8Add16 |
        Elements::Sha256Ctx8Add32 |
        Elements::Sha256Ctx8Add64 |
        Elements::Sha256Ctx8Add128 |
        Elements::Sha256Ctx8Add256 |
        Elements::Sha256Ctx8Add512 |
        Elements::Sha256Ctx8AddBuffer511 |
        Elements::Sha256Ctx8Finalize |
        Elements::Swu |
        // Jets for secp FE
        Elements::FeNormalize |
        Elements::FeNegate |
        Elements::FeAdd |
        Elements::FeSquare |
        Elements::FeMultiply |
        Elements::FeMultiplyBeta |
        Elements::FeInvert |
        Elements::FeSquareRoot |
        Elements::FeIsZero |
        Elements::FeIsOdd |
        // Jets for secp scalars
        Elements::ScalarNormalize |
        Elements::ScalarNegate |
        Elements::ScalarAdd |
        Elements::ScalarSquare |
        Elements::ScalarMultiply |
        Elements::ScalarMultiplyLambda |
        Elements::ScalarInvert |
        Elements::ScalarIsZero |
        // Jets for secp gej points
        Elements::GejInfinity |
        Elements::GejRescale |
        Elements::GejNormalize |
        Elements::GejNegate |
        Elements::GeNegate |
        Elements::GejDouble |
        Elements::GejAdd |
        Elements::GejGeAddEx |
        Elements::GejGeAdd |
        Elements::GejIsInfinity |
        Elements::GejEquiv |
        Elements::GejGeEquiv |
        Elements::GejXEquiv |
        Elements::GejYIsOdd |
        Elements::GejIsOnCurve |
        // Other jets
        Elements::GeIsOnCurve |
        Elements::Scale |
        Elements::Generate |
        Elements::LinearCombination1 |
        Elements::LinearVerify1 |
        Elements::Decompress |
        Elements::PointVerify1 |
        // Signature jets
        Elements::CheckSigVerify |
        Elements::Bip0340Verify  => true,
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

    let arr: [(Elements, &dyn InputSample); 367] = [
        // Bit logics
        (Elements::Verify, &UniformBits),
        // low
        (Elements::Low1, &input::Unit),
        (Elements::Low8, &input::Unit),
        (Elements::Low16, &input::Unit),
        (Elements::Low32, &input::Unit),
        (Elements::Low64, &input::Unit),
        // high
        (Elements::High1, &input::Unit),
        (Elements::High8, &input::Unit),
        (Elements::High16, &input::Unit),
        (Elements::High32, &input::Unit),
        (Elements::High64, &input::Unit),
        // complement
        (Elements::Complement1, &UniformBits),
        (Elements::Complement8, &UniformBits),
        (Elements::Complement16, &UniformBits),
        (Elements::Complement32, &UniformBits),
        (Elements::Complement64, &UniformBits),
        // and
        (Elements::And1, &EqProduct(UniformBits)),
        (Elements::And8, &EqProduct(UniformBits)),
        (Elements::And16, &EqProduct(UniformBits)),
        (Elements::And32, &EqProduct(UniformBits)),
        (Elements::And64, &EqProduct(UniformBits)),
        // or
        (Elements::Or1, &EqProduct(UniformBits)),
        (Elements::Or8, &EqProduct(UniformBits)),
        (Elements::Or16, &EqProduct(UniformBits)),
        (Elements::Or32, &EqProduct(UniformBits)),
        (Elements::Or64, &EqProduct(UniformBits)),
        // xor
        (Elements::Xor1, &EqProduct(UniformBits)),
        (Elements::Xor8, &EqProduct(UniformBits)),
        (Elements::Xor16, &EqProduct(UniformBits)),
        (Elements::Xor32, &EqProduct(UniformBits)),
        (Elements::Xor64, &EqProduct(UniformBits)),
        // maj
        (Elements::Maj1, &UniformBits),
        (Elements::Maj8, &UniformBits),
        (Elements::Maj16, &UniformBits),
        (Elements::Maj32, &UniformBits),
        (Elements::Maj64, &UniformBits),
        // xor xor
        (Elements::XorXor1, &UniformBits),
        (Elements::XorXor8, &UniformBits),
        (Elements::XorXor16, &UniformBits),
        (Elements::XorXor32, &UniformBits),
        (Elements::XorXor64, &UniformBits),
        // ch
        (Elements::Ch1, &UniformBits),
        (Elements::Ch8, &UniformBits),
        (Elements::Ch16, &UniformBits),
        (Elements::Ch32, &UniformBits),
        (Elements::Ch64, &UniformBits),
        // left shift
        (Elements::LeftShift8, &UniformBits),
        (Elements::LeftShift16, &UniformBits),
        (Elements::LeftShift32, &UniformBits),
        (Elements::LeftShift64, &UniformBits),
        (Elements::LeftShiftWith8, &UniformBits),
        (Elements::LeftShiftWith16, &UniformBits),
        (Elements::LeftShiftWith32, &UniformBits),
        (Elements::LeftShiftWith64, &UniformBits),
        // right shift
        (Elements::RightShift8, &UniformBits),
        (Elements::RightShift16, &UniformBits),
        (Elements::RightShift32, &UniformBits),
        (Elements::RightShift64, &UniformBits),
        (Elements::RightShiftWith8, &UniformBits),
        (Elements::RightShiftWith16, &UniformBits),
        (Elements::RightShiftWith32, &UniformBits),
        (Elements::RightShiftWith64, &UniformBits),
        // full left shift
        (Elements::FullLeftShift8_1, &UniformBits),
        (Elements::FullLeftShift8_2, &UniformBits),
        (Elements::FullLeftShift8_4, &UniformBits),
        (Elements::FullLeftShift16_1, &UniformBits),
        (Elements::FullLeftShift16_2, &UniformBits),
        (Elements::FullLeftShift16_4, &UniformBits),
        (Elements::FullLeftShift16_8, &UniformBits),
        (Elements::FullLeftShift32_1, &UniformBits),
        (Elements::FullLeftShift32_2, &UniformBits),
        (Elements::FullLeftShift32_4, &UniformBits),
        (Elements::FullLeftShift32_8, &UniformBits),
        (Elements::FullLeftShift32_16, &UniformBits),
        (Elements::FullLeftShift64_1, &UniformBits),
        (Elements::FullLeftShift64_2, &UniformBits),
        (Elements::FullLeftShift64_4, &UniformBits),
        (Elements::FullLeftShift64_8, &UniformBits),
        (Elements::FullLeftShift64_16, &UniformBits),
        (Elements::FullLeftShift64_32, &UniformBits),
        // full right shift
        (Elements::FullRightShift8_1, &UniformBits),
        (Elements::FullRightShift8_2, &UniformBits),
        (Elements::FullRightShift8_4, &UniformBits),
        (Elements::FullRightShift16_1, &UniformBits),
        (Elements::FullRightShift16_2, &UniformBits),
        (Elements::FullRightShift16_4, &UniformBits),
        (Elements::FullRightShift16_8, &UniformBits),
        (Elements::FullRightShift32_1, &UniformBits),
        (Elements::FullRightShift32_2, &UniformBits),
        (Elements::FullRightShift32_4, &UniformBits),
        (Elements::FullRightShift32_8, &UniformBits),
        (Elements::FullRightShift32_16, &UniformBits),
        (Elements::FullRightShift64_1, &UniformBits),
        (Elements::FullRightShift64_2, &UniformBits),
        (Elements::FullRightShift64_4, &UniformBits),
        (Elements::FullRightShift64_8, &UniformBits),
        (Elements::FullRightShift64_16, &UniformBits),
        (Elements::FullRightShift64_32, &UniformBits),
        // left rotate
        (Elements::LeftRotate8, &UniformBits),
        (Elements::LeftRotate16, &UniformBits),
        (Elements::LeftRotate32, &UniformBits),
        (Elements::LeftRotate64, &UniformBits),
        // right rotate
        (Elements::RightRotate8, &UniformBits),
        (Elements::RightRotate16, &UniformBits),
        (Elements::RightRotate32, &UniformBits),
        (Elements::RightRotate64, &UniformBits),
        // left extend
        (Elements::LeftExtend1_8, &UniformBits),
        (Elements::LeftExtend1_16, &UniformBits),
        (Elements::LeftExtend1_32, &UniformBits),
        (Elements::LeftExtend1_64, &UniformBits),
        (Elements::LeftExtend8_16, &UniformBits),
        (Elements::LeftExtend8_32, &UniformBits),
        (Elements::LeftExtend8_64, &UniformBits),
        (Elements::LeftExtend16_32, &UniformBits),
        (Elements::LeftExtend16_64, &UniformBits),
        (Elements::LeftExtend32_64, &UniformBits),
        // right extend
        // no right-extend for 1-bit values
        (Elements::RightExtend8_16, &UniformBits),
        (Elements::RightExtend8_32, &UniformBits),
        (Elements::RightExtend8_64, &UniformBits),
        (Elements::RightExtend16_32, &UniformBits),
        (Elements::RightExtend16_64, &UniformBits),
        (Elements::RightExtend32_64, &UniformBits),
        // left pad
        (Elements::LeftPadLow1_8, &UniformBits),
        (Elements::LeftPadLow1_16, &UniformBits),
        (Elements::LeftPadLow1_32, &UniformBits),
        (Elements::LeftPadLow1_64, &UniformBits),
        (Elements::LeftPadLow8_16, &UniformBits),
        (Elements::LeftPadLow8_32, &UniformBits),
        (Elements::LeftPadLow8_64, &UniformBits),
        (Elements::LeftPadLow16_32, &UniformBits),
        (Elements::LeftPadLow16_64, &UniformBits),
        (Elements::LeftPadLow32_64, &UniformBits),
        (Elements::LeftPadHigh1_8, &UniformBits),
        (Elements::LeftPadHigh1_16, &UniformBits),
        (Elements::LeftPadHigh1_32, &UniformBits),
        (Elements::LeftPadHigh1_64, &UniformBits),
        (Elements::LeftPadHigh8_16, &UniformBits),
        (Elements::LeftPadHigh8_32, &UniformBits),
        (Elements::LeftPadHigh8_64, &UniformBits),
        (Elements::LeftPadHigh16_32, &UniformBits),
        (Elements::LeftPadHigh16_64, &UniformBits),
        (Elements::LeftPadHigh32_64, &UniformBits),
        // right pad
        (Elements::RightPadLow1_8, &UniformBits),
        (Elements::RightPadLow1_16, &UniformBits),
        (Elements::RightPadLow1_32, &UniformBits),
        (Elements::RightPadLow1_64, &UniformBits),
        (Elements::RightPadLow8_16, &UniformBits),
        (Elements::RightPadLow8_32, &UniformBits),
        (Elements::RightPadLow8_64, &UniformBits),
        (Elements::RightPadLow16_32, &UniformBits),
        (Elements::RightPadLow16_64, &UniformBits),
        (Elements::RightPadLow32_64, &UniformBits),
        (Elements::RightPadHigh1_8, &UniformBits),
        (Elements::RightPadHigh1_16, &UniformBits),
        (Elements::RightPadHigh1_32, &UniformBits),
        (Elements::RightPadHigh1_64, &UniformBits),
        (Elements::RightPadHigh8_16, &UniformBits),
        (Elements::RightPadHigh8_32, &UniformBits),
        (Elements::RightPadHigh8_64, &UniformBits),
        (Elements::RightPadHigh16_32, &UniformBits),
        (Elements::RightPadHigh16_64, &UniformBits),
        (Elements::RightPadHigh32_64, &UniformBits),
        // leftmost
        (Elements::Leftmost8_1, &UniformBits),
        (Elements::Leftmost8_2, &UniformBits),
        (Elements::Leftmost8_4, &UniformBits),
        (Elements::Leftmost16_1, &UniformBits),
        (Elements::Leftmost16_2, &UniformBits),
        (Elements::Leftmost16_4, &UniformBits),
        (Elements::Leftmost16_8, &UniformBits),
        (Elements::Leftmost32_1, &UniformBits),
        (Elements::Leftmost32_2, &UniformBits),
        (Elements::Leftmost32_4, &UniformBits),
        (Elements::Leftmost32_8, &UniformBits),
        (Elements::Leftmost32_16, &UniformBits),
        (Elements::Leftmost64_1, &UniformBits),
        (Elements::Leftmost64_2, &UniformBits),
        (Elements::Leftmost64_4, &UniformBits),
        (Elements::Leftmost64_8, &UniformBits),
        (Elements::Leftmost64_16, &UniformBits),
        (Elements::Leftmost64_32, &UniformBits),
        // rightmost
        (Elements::Rightmost8_1, &UniformBits),
        (Elements::Rightmost8_2, &UniformBits),
        (Elements::Rightmost8_4, &UniformBits),
        (Elements::Rightmost16_1, &UniformBits),
        (Elements::Rightmost16_2, &UniformBits),
        (Elements::Rightmost16_4, &UniformBits),
        (Elements::Rightmost16_8, &UniformBits),
        (Elements::Rightmost32_1, &UniformBits),
        (Elements::Rightmost32_2, &UniformBits),
        (Elements::Rightmost32_4, &UniformBits),
        (Elements::Rightmost32_8, &UniformBits),
        (Elements::Rightmost32_16, &UniformBits),
        (Elements::Rightmost64_1, &UniformBits),
        (Elements::Rightmost64_2, &UniformBits),
        (Elements::Rightmost64_4, &UniformBits),
        (Elements::Rightmost64_8, &UniformBits),
        (Elements::Rightmost64_16, &UniformBits),
        (Elements::Rightmost64_32, &UniformBits),
        // some
        (Elements::Some1, &UniformBits),
        (Elements::Some8, &UniformBits),
        (Elements::Some16, &UniformBits),
        (Elements::Some32, &UniformBits),
        (Elements::Some64, &UniformBits),
        // all
        (Elements::All8, &UniformBits),
        (Elements::All16, &UniformBits),
        (Elements::All32, &UniformBits),
        (Elements::All64, &UniformBits),
        // one
        (Elements::One8, &input::Unit),
        (Elements::One16, &input::Unit),
        (Elements::One32, &input::Unit),
        (Elements::One64, &input::Unit),
        // eq
        (Elements::Eq1, &EqProduct(UniformBits)),
        (Elements::Eq8, &EqProduct(UniformBits)),
        (Elements::Eq16, &EqProduct(UniformBits)),
        (Elements::Eq32, &EqProduct(UniformBits)),
        (Elements::Eq64, &EqProduct(UniformBits)),
        (Elements::Eq256, &EqProduct(UniformBits)),
        // Arithmetic
        // add
        (Elements::Add8, &EqProduct(UniformBits)),
        (Elements::Add16, &EqProduct(UniformBits)),
        (Elements::Add32, &EqProduct(UniformBits)),
        (Elements::Add64, &EqProduct(UniformBits)),
        // full add
        (Elements::FullAdd8, &PrefixBit(EqProduct(UniformBits))),
        (Elements::FullAdd16, &PrefixBit(EqProduct(UniformBits))),
        (Elements::FullAdd32, &PrefixBit(EqProduct(UniformBits))),
        (Elements::FullAdd64, &PrefixBit(EqProduct(UniformBits))),
        // full increment
        (Elements::FullIncrement8, &PrefixBit(UniformBits)),
        (Elements::FullIncrement16, &PrefixBit(UniformBits)),
        (Elements::FullIncrement32, &PrefixBit(UniformBits)),
        (Elements::FullIncrement64, &PrefixBit(UniformBits)),
        // increment
        (Elements::Increment8, &UniformBits),
        (Elements::Increment16, &UniformBits),
        (Elements::Increment32, &UniformBits),
        (Elements::Increment64, &UniformBits),
        // subtract
        (Elements::Subtract8, &EqProduct(UniformBits)),
        (Elements::Subtract16, &EqProduct(UniformBits)),
        (Elements::Subtract32, &EqProduct(UniformBits)),
        (Elements::Subtract64, &EqProduct(UniformBits)),
        // full subtract
        (Elements::FullSubtract8, &PrefixBit(EqProduct(UniformBits))),
        (Elements::FullSubtract16, &PrefixBit(EqProduct(UniformBits))),
        (Elements::FullSubtract32, &PrefixBit(EqProduct(UniformBits))),
        (Elements::FullSubtract64, &PrefixBit(EqProduct(UniformBits))),
        // negate
        (Elements::Negate8, &UniformBits),
        (Elements::Negate16, &UniformBits),
        (Elements::Negate32, &UniformBits),
        (Elements::Negate64, &UniformBits),
        // full decrement
        (Elements::FullDecrement8, &PrefixBit(UniformBits)),
        (Elements::FullDecrement16, &PrefixBit(UniformBits)),
        (Elements::FullDecrement32, &PrefixBit(UniformBits)),
        (Elements::FullDecrement64, &PrefixBit(UniformBits)),
        // decrement
        (Elements::Decrement8, &UniformBits),
        (Elements::Decrement16, &UniformBits),
        (Elements::Decrement32, &UniformBits),
        (Elements::Decrement64, &UniformBits),
        // multiply
        (Elements::Multiply8, &EqProduct(UniformBits)),
        (Elements::Multiply16, &EqProduct(UniformBits)),
        (Elements::Multiply32, &EqProduct(UniformBits)),
        (Elements::Multiply64, &EqProduct(UniformBits)),
        // full multiply
        (Elements::FullMultiply8, &EqProduct(UniformBits)),
        (Elements::FullMultiply16, &EqProduct(UniformBits)),
        (Elements::FullMultiply32, &EqProduct(UniformBits)),
        (Elements::FullMultiply64, &EqProduct(UniformBits)),
        // is zero
        (Elements::IsZero8, &UniformBits),
        (Elements::IsZero16, &UniformBits),
        (Elements::IsZero32, &UniformBits),
        (Elements::IsZero64, &UniformBits),
        // is one
        (Elements::IsOne8, &UniformBits),
        (Elements::IsOne16, &UniformBits),
        (Elements::IsOne32, &UniformBits),
        (Elements::IsOne64, &UniformBits),
        // le
        (Elements::Le8, &EqProduct(UniformBits)),
        (Elements::Le16, &EqProduct(UniformBits)),
        (Elements::Le32, &EqProduct(UniformBits)),
        (Elements::Le64, &EqProduct(UniformBits)),
        // lt
        (Elements::Lt8, &EqProduct(UniformBits)),
        (Elements::Lt16, &EqProduct(UniformBits)),
        (Elements::Lt32, &EqProduct(UniformBits)),
        (Elements::Lt64, &EqProduct(UniformBits)),
        // min
        (Elements::Min8, &EqProduct(UniformBits)),
        (Elements::Min16, &EqProduct(UniformBits)),
        (Elements::Min32, &EqProduct(UniformBits)),
        (Elements::Min64, &EqProduct(UniformBits)),
        // max
        (Elements::Max8, &EqProduct(UniformBits)),
        (Elements::Max16, &EqProduct(UniformBits)),
        (Elements::Max32, &EqProduct(UniformBits)),
        (Elements::Max64, &EqProduct(UniformBits)),
        // median
        (Elements::Median8, &UniformBits),
        (Elements::Median16, &UniformBits),
        (Elements::Median32, &UniformBits),
        (Elements::Median64, &UniformBits),
        // div mod
        (Elements::DivMod8, &EqProduct(UniformBits)),
        (Elements::DivMod16, &EqProduct(UniformBits)),
        (Elements::DivMod32, &EqProduct(UniformBits)),
        (Elements::DivMod64, &EqProduct(UniformBits)),
        (Elements::DivMod128_64, &DivMod12864Input),
        // divide
        (Elements::Divide8, &EqProduct(UniformBits)),
        (Elements::Divide16, &EqProduct(UniformBits)),
        (Elements::Divide32, &EqProduct(UniformBits)),
        (Elements::Divide64, &EqProduct(UniformBits)),
        // modulo
        (Elements::Modulo8, &EqProduct(UniformBits)),
        (Elements::Modulo16, &EqProduct(UniformBits)),
        (Elements::Modulo32, &EqProduct(UniformBits)),
        (Elements::Modulo64, &EqProduct(UniformBits)),
        // divides
        (Elements::Divides8, &EqProduct(UniformBits)),
        (Elements::Divides16, &EqProduct(UniformBits)),
        (Elements::Divides32, &EqProduct(UniformBits)),
        (Elements::Divides64, &EqProduct(UniformBits)),

        // Hashes
        (Elements::HashToCurve, &UniformBits),
        (Elements::Sha256Iv, &input::Unit),
        (Elements::Sha256Block, &UniformBits),
        (Elements::Sha256Ctx8Init, &input::Unit),
        (Elements::Sha256Ctx8Add1, &GenericProduct(Sha256Ctx, UniformBits)),
        (Elements::Sha256Ctx8Add2, &GenericProduct(Sha256Ctx, UniformBits)),
        (Elements::Sha256Ctx8Add4, &GenericProduct(Sha256Ctx, UniformBits)),
        (Elements::Sha256Ctx8Add8, &GenericProduct(Sha256Ctx, UniformBits)),
        (Elements::Sha256Ctx8Add16, &GenericProduct(Sha256Ctx, UniformBits)),
        (Elements::Sha256Ctx8Add32, &GenericProduct(Sha256Ctx, UniformBits)),
        (Elements::Sha256Ctx8Add64, &GenericProduct(Sha256Ctx, UniformBits)),
        (Elements::Sha256Ctx8Add128, &GenericProduct(Sha256Ctx, UniformBits)),
        (Elements::Sha256Ctx8Add256, &GenericProduct(Sha256Ctx, UniformBits)),
        (Elements::Sha256Ctx8Add512, &GenericProduct(Sha256Ctx, UniformBits)),
        (Elements::Sha256Ctx8AddBuffer511, &GenericProduct(Sha256Ctx, UniformBits)),
        (Elements::Sha256Ctx8Finalize, &Sha256Ctx),
        (Elements::Swu, &UniformBits),
        // Jets for secp FE
        (Elements::FeNormalize, &input::Fe),
        (Elements::FeNegate, &input::Fe),
        (Elements::FeAdd, &EqProduct(input::Fe)),
        (Elements::FeSquare, &input::Fe),
        (Elements::FeMultiply, &EqProduct(input::Fe)),
        (Elements::FeMultiplyBeta, &input::Fe),
        (Elements::FeInvert, &input::Fe),
        (Elements::FeSquareRoot, &input::Fe),
        (Elements::FeIsZero, &input::Fe),
        (Elements::FeIsOdd, &input::Fe),
        // Jets for secp scalars
        (Elements::ScalarNormalize, &input::Scalar),
        (Elements::ScalarNegate, &input::Scalar),
        (Elements::ScalarAdd, &EqProduct(input::Scalar)),
        (Elements::ScalarSquare, &input::Scalar),
        (Elements::ScalarMultiply, &EqProduct(input::Scalar)),
        (Elements::ScalarMultiplyLambda, &input::Scalar),
        (Elements::ScalarInvert, &input::Scalar),
        (Elements::ScalarIsZero, &input::Scalar),
        // Jets for secp gej points
        // FIXME we should have specific samplers for GEJ pairs and GEJ-GE pairs
        // which relate the points in algebraic ways (being negatives, multiples
        // of lambda away from each other)
        (Elements::GejInfinity, &input::Unit),
        (Elements::GejRescale, &GenericProduct(input::Gej, input::Fe)),
        (Elements::GejNormalize, &input::Gej), 
        (Elements::GejNegate, &input::Gej), 
        (Elements::GeNegate, &input::Ge),
        (Elements::GejDouble, &input::Gej), 
        (Elements::GejAdd, &EqProduct(input::Gej)),
        (Elements::GejGeAddEx, &GenericProduct(input::Gej, input::Ge)),
        (Elements::GejGeAdd, &GenericProduct(input::Gej, input::Ge)),
        (Elements::GejIsInfinity, &input::Gej), 
        (Elements::GejEquiv, &GenericProduct(input::Gej, input::Gej)),
        (Elements::GejGeEquiv, &GenericProduct(input::Gej, input::Ge)),
        (Elements::GejXEquiv, &GenericProduct(input::Fe, input::Gej)),
        (Elements::GejYIsOdd, &input::Gej), 
        (Elements::GejIsOnCurve, &input::Gej), 
        // Other jets
        (Elements::GeIsOnCurve, &input::Ge),
        (Elements::Scale, &GenericProduct(input::Scalar, input::Gej)),
        (Elements::Generate, &input::Scalar),
        (Elements::LinearCombination1, &GenericProduct(GenericProduct(input::Scalar, input::Gej), input::Scalar)),
        (Elements::LinearVerify1, &GenericProduct(GenericProduct(GenericProduct(input::Scalar, input::Ge), input::Scalar), input::Ge)),
        (Elements::Decompress, &input::Point),
        (Elements::PointVerify1, &GenericProduct(GenericProduct(GenericProduct(input::Scalar, input::Point), input::Scalar), input::Point)),
        // Signature jets
        (Elements::CheckSigVerify, &input::CheckSigSignature),
        (Elements::Bip0340Verify, &input::Bip340Signature),

        // Timelock parsing jets
        (Elements::ParseLock, &UniformBits), // all values take same time
        (Elements::ParseSequence, &PrefixBit(UniformBits)), // top bit treated specially
    ];
    for (jet, sample) in arr {
        count += 1;
        jet_checker.record(jet);
        println!(
            "[{:3}/{:3}] For {} we have {} distributions",
            count, jet_checker.n_jets(), jet, sample.n_distributions(),
        );

        let (src_ty, tgt_ty) = jet_arrow(jet);

        let mut group = c.benchmark_group(format!("elements_{jet}"));
        let env = ElementsEnvSampling::null().env();
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
        (Elements::SigAllHash, ElementsBenchEnvType::Random),
        (Elements::TxHash, ElementsBenchEnvType::Random),
        (Elements::TapEnvHash, ElementsBenchEnvType::Random),
        (Elements::InputsHash, ElementsBenchEnvType::Random),
        (Elements::OutputsHash, ElementsBenchEnvType::Random),
        (Elements::IssuancesHash, ElementsBenchEnvType::Random),
        (Elements::InputUtxosHash, ElementsBenchEnvType::Random),
        (Elements::OutputAmountsHash, ElementsBenchEnvType::Random),
        (Elements::OutputScriptsHash, ElementsBenchEnvType::Random),
        (Elements::OutputNoncesHash, ElementsBenchEnvType::Random),
        (Elements::OutputRangeProofsHash, ElementsBenchEnvType::Random),
        (Elements::OutputSurjectionProofsHash, ElementsBenchEnvType::Random),
        (Elements::InputOutpointsHash, ElementsBenchEnvType::Random),
        (Elements::InputSequencesHash, ElementsBenchEnvType::Random),
        (Elements::InputAnnexesHash, ElementsBenchEnvType::Random),
        (Elements::InputScriptSigsHash, ElementsBenchEnvType::Random),
        (Elements::IssuanceAssetAmountsHash, ElementsBenchEnvType::Random),
        (Elements::IssuanceTokenAmountsHash, ElementsBenchEnvType::Random),
        (Elements::IssuanceRangeProofsHash, ElementsBenchEnvType::Random),
        (Elements::IssuanceBlindingEntropyHash, ElementsBenchEnvType::Random),
        (Elements::InputAmountsHash, ElementsBenchEnvType::Random),
        (Elements::InputScriptsHash, ElementsBenchEnvType::Random),
        (Elements::TapleafHash, ElementsBenchEnvType::Random),
        (Elements::TappathHash, ElementsBenchEnvType::Random),
        //
        // ------------------------------------
        // Jets with no environment required. But no custom sampling
        (Elements::LbtcAsset, ElementsBenchEnvType::None),
        (Elements::BuildTapleafSimplicity, ElementsBenchEnvType::None),
        (Elements::BuildTapbranch, ElementsBenchEnvType::None),
        // ------------------------------------
        // Timelock jets
        // No need to specially consider issuances or pegins
        (Elements::CheckLockHeight, ElementsBenchEnvType::Random),
        (Elements::CheckLockTime, ElementsBenchEnvType::Random),
        (Elements::CheckLockDistance, ElementsBenchEnvType::Random),
        (Elements::CheckLockDuration, ElementsBenchEnvType::Random),
        (Elements::TxLockHeight, ElementsBenchEnvType::Random),
        (Elements::TxLockTime, ElementsBenchEnvType::Random),
        (Elements::TxLockDistance, ElementsBenchEnvType::Random),
        (Elements::TxLockDuration, ElementsBenchEnvType::Random),
        (Elements::TxIsFinal, ElementsBenchEnvType::Random),
        // -----------------------------------------
        // Jets with no environment required, but no custom sampling required
        // These jets just perform some calculation
        (Elements::CalculateIssuanceEntropy, ElementsBenchEnvType::None),
        (Elements::CalculateAsset, ElementsBenchEnvType::None),
        (Elements::CalculateExplicitToken, ElementsBenchEnvType::None),
        (Elements::CalculateConfidentialToken, ElementsBenchEnvType::None),
        // Jets with tx introspection
        // Nothing special about issuances or pegins
        (Elements::ScriptCMR, ElementsBenchEnvType::Random),
        (Elements::InternalKey, ElementsBenchEnvType::Random),
        (Elements::CurrentIndex, ElementsBenchEnvType::Random),
        (Elements::NumInputs, ElementsBenchEnvType::Random),
        (Elements::NumOutputs, ElementsBenchEnvType::Random),
        (Elements::LockTime, ElementsBenchEnvType::Random),
        (Elements::TransactionId, ElementsBenchEnvType::Random),
        (Elements::TotalFee, ElementsBenchEnvType::AllFeeOutputs),
        // // -----------------------------------------
        // Current Input
        // Each jet has worst case dependent on whether it is pegin or issuance
        // or none
        (Elements::CurrentPegin, ElementsBenchEnvType::Pegin),
        (Elements::CurrentPrevOutpoint, ElementsBenchEnvType::Random),
        (Elements::CurrentAsset, ElementsBenchEnvType::Random),
        (Elements::CurrentAmount, ElementsBenchEnvType::Random),
        (Elements::CurrentScriptHash, ElementsBenchEnvType::Random),
        (Elements::CurrentSequence, ElementsBenchEnvType::Random),
        // Annex note: We explicitly add annex in inputs
        (Elements::CurrentAnnexHash, ElementsBenchEnvType::Random),
        (Elements::CurrentScriptSigHash, ElementsBenchEnvType::Random),
        (Elements::CurrentReissuanceBlinding, ElementsBenchEnvType::Issuance),
        (Elements::CurrentNewIssuanceContract, ElementsBenchEnvType::Issuance),
        (Elements::CurrentReissuanceEntropy, ElementsBenchEnvType::Issuance),
        (Elements::CurrentIssuanceAssetAmount, ElementsBenchEnvType::Issuance),
        (Elements::CurrentIssuanceTokenAmount, ElementsBenchEnvType::Issuance),
        (Elements::CurrentIssuanceAssetProof, ElementsBenchEnvType::Issuance),
        (Elements::CurrentIssuanceTokenProof, ElementsBenchEnvType::Issuance),
        // -----------------------------------------
        // General tx jets
        (Elements::TapleafVersion, ElementsBenchEnvType::None),
        (Elements::Version, ElementsBenchEnvType::None),
        (Elements::GenesisBlockHash, ElementsBenchEnvType::None),
    ];

    // Elements environment jets
    for (jet, env_sampler) in jets {
        jet_checker.record(jet);

        let (src_ty, tgt_ty) = jet_arrow(jet);
        let env = env_sampler.env();

        let mut group = c.benchmark_group(format!("elements_{jet}"));
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
        let genesis_pegin = genesis_pegin();
        let outpoint = elements::OutPoint::sample().value();
        Value::product(ctx8, Value::product(genesis_pegin, outpoint))
    }

    fn asset_amount_hash() -> Value {
        let ctx8 = SimplicityCtx8::with_len(511).value();
        let asset = confidential::Asset::sample().value();
        let amount = confidential::Value::sample().value();
        Value::product(ctx8, Value::product(asset, amount))
    }

    fn nonce_hash() -> Value {
        let ctx8 = SimplicityCtx8::with_len(511).value();
        let nonce = confidential::Nonce::sample().value();
        Value::product(ctx8, nonce)
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
    let arr: [(Elements, Arc<dyn Fn() -> Value>); 4] = [
        (Elements::OutpointHash, Arc::new(&outpoint_hash)),
        (Elements::AssetAmountHash, Arc::new(&asset_amount_hash)),
        (Elements::NonceHash, Arc::new(nonce_hash)),
        (Elements::AnnexHash, Arc::new(annex_hash)),
    ];

    for (jet, inp_fn) in arr {
        jet_checker.record(jet);

        let (src_ty, tgt_ty) = jet_arrow(jet);
        let env = ElementsEnvSampling::null().env();

        let mut group = c.benchmark_group(format!("elements_{jet}"));
        for i in 0..NUM_RANDOM_SAMPLES {
            let params = JetParams::with_rand_aligns(InputSampling::Custom(inp_fn.clone()));
            let name = format!("{}", i);
            group.bench_with_input(&name, &params, |b, params| {
                b.iter_batched(
                    || {
                        // Elements sighash chapter jets with non-unit src type
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
        // Issuance chapter jets with index
        (Elements::Issuance, Index::InputIdx0, ElementsBenchEnvType::Issuance),
        (Elements::IssuanceAsset, Index::InputIdx0, ElementsBenchEnvType::Issuance),
        (Elements::IssuanceToken, Index::InputIdx0, ElementsBenchEnvType::Issuance),
        (Elements::IssuanceEntropy, Index::InputIdx0, ElementsBenchEnvType::Issuance),
        // // Transaction chapter jets with output index
        (Elements::OutputAsset, Index::Output, ElementsBenchEnvType::Random),
        (Elements::OutputAmount, Index::Output, ElementsBenchEnvType::Random),
        (Elements::OutputNonce, Index::Output, ElementsBenchEnvType::Random),
        (Elements::OutputHash, Index::Output, ElementsBenchEnvType::Random),
        (Elements::OutputScriptHash, Index::Output, ElementsBenchEnvType::Random),
        (Elements::OutputNullDatum, Index::Output, ElementsBenchEnvType::Random), // I Don't know what this is, and how to test this, presumably related to how pegouts work
        (Elements::OutputSurjectionProof, Index::Output, ElementsBenchEnvType::Random),
        (Elements::OutputRangeProof, Index::Output, ElementsBenchEnvType::Random),
        (Elements::OutputIsFee, Index::Output, ElementsBenchEnvType::AllFeeOutputs), // slowest will be for fee outputs 
        // // Transaction chapter jets with input index
        (Elements::InputPegin, Index::InputIdx0, ElementsBenchEnvType::Pegin),
        (Elements::InputPrevOutpoint, Index::Input, ElementsBenchEnvType::Random),
        (Elements::InputAsset, Index::Input, ElementsBenchEnvType::Random),
        (Elements::InputAmount, Index::Input, ElementsBenchEnvType::Random),
        (Elements::InputHash, Index::Input, ElementsBenchEnvType::Random),
        (Elements::InputScriptHash, Index::Input, ElementsBenchEnvType::Random),
        (Elements::InputSequence, Index::Input, ElementsBenchEnvType::Random),
        (Elements::InputUtxoHash, Index::Input, ElementsBenchEnvType::Random),
        (Elements::InputAnnexHash, Index::InputIdx0, ElementsBenchEnvType::Annex),
        (Elements::InputScriptSigHash, Index::Input, ElementsBenchEnvType::Random),
        (Elements::ReissuanceBlinding, Index::InputIdx0, ElementsBenchEnvType::Issuance),
        (Elements::NewIssuanceContract, Index::InputIdx0, ElementsBenchEnvType::Issuance),
        (Elements::ReissuanceEntropy, Index::InputIdx0, ElementsBenchEnvType::Issuance),
        (Elements::IssuanceAssetAmount, Index::InputIdx0, ElementsBenchEnvType::Issuance),
        (Elements::IssuanceTokenAmount, Index::InputIdx0, ElementsBenchEnvType::Issuance),
        (Elements::IssuanceAssetProof, Index::InputIdx0, ElementsBenchEnvType::Issuance),
        (Elements::IssuanceTokenProof, Index::InputIdx0, ElementsBenchEnvType::Issuance),
        (Elements::IssuanceHash, Index::InputIdx0, ElementsBenchEnvType::Issuance),
        (Elements::Tappath, Index::MarkleBranchIndex, ElementsBenchEnvType::Random),
    ];

    for (jet, index, env_type) in arr {
        jet_checker.record(jet);

        let (src_ty, tgt_ty) = jet_arrow(jet);
        let env = env_type.env();
        let mut group = c.benchmark_group(format!("elements_{jet}"));

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
