/* This file has been automatically generated. */

use crate::jet::type_name::TypeName;
use crate::jet::Jet;
use crate::merkle::cmr::Cmr;
use crate::decode_bits;
use crate::{decode, BitIter, BitWriter};
use crate::analysis::Cost;
use hashes::sha256::Midstate;
use simplicity_sys::CFrameItem;
use std::io::Write;
use std::{borrow::Borrow, fmt, str};
use crate::jet::bitcoin::BitcoinEnv;
use simplicity_sys::bitcoin::CTxEnv;

/// The Bitcoin jet family.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Bitcoin {
    Add16,
    Add32,
    Add64,
    Add8,
    All16,
    All32,
    All64,
    All8,
    And1,
    And16,
    And32,
    And64,
    And8,
    AnnexHash,
    Bip0340Verify,
    BuildTapbranch,
    BuildTapleafSimplicity,
    BuildTaptweak,
    Ch1,
    Ch16,
    Ch32,
    Ch64,
    Ch8,
    CheckLockDistance,
    CheckLockDuration,
    CheckLockHeight,
    CheckLockTime,
    CheckSigVerify,
    Complement1,
    Complement16,
    Complement32,
    Complement64,
    Complement8,
    CurrentAnnexHash,
    CurrentIndex,
    CurrentPrevOutpoint,
    CurrentScriptHash,
    CurrentScriptSigHash,
    CurrentSequence,
    CurrentValue,
    Decompress,
    Decrement16,
    Decrement32,
    Decrement64,
    Decrement8,
    DivMod128_64,
    DivMod16,
    DivMod32,
    DivMod64,
    DivMod8,
    Divide16,
    Divide32,
    Divide64,
    Divide8,
    Divides16,
    Divides32,
    Divides64,
    Divides8,
    Eq1,
    Eq16,
    Eq256,
    Eq32,
    Eq64,
    Eq8,
    FeAdd,
    FeInvert,
    FeIsOdd,
    FeIsZero,
    FeMultiply,
    FeMultiplyBeta,
    FeNegate,
    FeNormalize,
    FeSquare,
    FeSquareRoot,
    Fee,
    FullAdd16,
    FullAdd32,
    FullAdd64,
    FullAdd8,
    FullDecrement16,
    FullDecrement32,
    FullDecrement64,
    FullDecrement8,
    FullIncrement16,
    FullIncrement32,
    FullIncrement64,
    FullIncrement8,
    FullLeftShift16_1,
    FullLeftShift16_2,
    FullLeftShift16_4,
    FullLeftShift16_8,
    FullLeftShift32_1,
    FullLeftShift32_16,
    FullLeftShift32_2,
    FullLeftShift32_4,
    FullLeftShift32_8,
    FullLeftShift64_1,
    FullLeftShift64_16,
    FullLeftShift64_2,
    FullLeftShift64_32,
    FullLeftShift64_4,
    FullLeftShift64_8,
    FullLeftShift8_1,
    FullLeftShift8_2,
    FullLeftShift8_4,
    FullMultiply16,
    FullMultiply32,
    FullMultiply64,
    FullMultiply8,
    FullRightShift16_1,
    FullRightShift16_2,
    FullRightShift16_4,
    FullRightShift16_8,
    FullRightShift32_1,
    FullRightShift32_16,
    FullRightShift32_2,
    FullRightShift32_4,
    FullRightShift32_8,
    FullRightShift64_1,
    FullRightShift64_16,
    FullRightShift64_2,
    FullRightShift64_32,
    FullRightShift64_4,
    FullRightShift64_8,
    FullRightShift8_1,
    FullRightShift8_2,
    FullRightShift8_4,
    FullSubtract16,
    FullSubtract32,
    FullSubtract64,
    FullSubtract8,
    GeIsOnCurve,
    GeNegate,
    GejAdd,
    GejDouble,
    GejEquiv,
    GejGeAdd,
    GejGeAddEx,
    GejGeEquiv,
    GejInfinity,
    GejIsInfinity,
    GejIsOnCurve,
    GejNegate,
    GejNormalize,
    GejRescale,
    GejXEquiv,
    GejYIsOdd,
    Generate,
    HashToCurve,
    High1,
    High16,
    High32,
    High64,
    High8,
    Increment16,
    Increment32,
    Increment64,
    Increment8,
    InputAnnexHash,
    InputAnnexesHash,
    InputHash,
    InputOutpointsHash,
    InputPrevOutpoint,
    InputScriptHash,
    InputScriptSigHash,
    InputScriptSigsHash,
    InputScriptsHash,
    InputSequence,
    InputSequencesHash,
    InputUtxoHash,
    InputUtxosHash,
    InputValue,
    InputValuesHash,
    InputsHash,
    InternalKey,
    IsOne16,
    IsOne32,
    IsOne64,
    IsOne8,
    IsZero16,
    IsZero32,
    IsZero64,
    IsZero8,
    Le16,
    Le32,
    Le64,
    Le8,
    LeftExtend16_32,
    LeftExtend16_64,
    LeftExtend1_16,
    LeftExtend1_32,
    LeftExtend1_64,
    LeftExtend1_8,
    LeftExtend32_64,
    LeftExtend8_16,
    LeftExtend8_32,
    LeftExtend8_64,
    LeftPadHigh16_32,
    LeftPadHigh16_64,
    LeftPadHigh1_16,
    LeftPadHigh1_32,
    LeftPadHigh1_64,
    LeftPadHigh1_8,
    LeftPadHigh32_64,
    LeftPadHigh8_16,
    LeftPadHigh8_32,
    LeftPadHigh8_64,
    LeftPadLow16_32,
    LeftPadLow16_64,
    LeftPadLow1_16,
    LeftPadLow1_32,
    LeftPadLow1_64,
    LeftPadLow1_8,
    LeftPadLow32_64,
    LeftPadLow8_16,
    LeftPadLow8_32,
    LeftPadLow8_64,
    LeftRotate16,
    LeftRotate32,
    LeftRotate64,
    LeftRotate8,
    LeftShift16,
    LeftShift32,
    LeftShift64,
    LeftShift8,
    LeftShiftWith16,
    LeftShiftWith32,
    LeftShiftWith64,
    LeftShiftWith8,
    Leftmost16_1,
    Leftmost16_2,
    Leftmost16_4,
    Leftmost16_8,
    Leftmost32_1,
    Leftmost32_16,
    Leftmost32_2,
    Leftmost32_4,
    Leftmost32_8,
    Leftmost64_1,
    Leftmost64_16,
    Leftmost64_2,
    Leftmost64_32,
    Leftmost64_4,
    Leftmost64_8,
    Leftmost8_1,
    Leftmost8_2,
    Leftmost8_4,
    LinearCombination1,
    LinearVerify1,
    LockTime,
    Low1,
    Low16,
    Low32,
    Low64,
    Low8,
    Lt16,
    Lt32,
    Lt64,
    Lt8,
    Maj1,
    Maj16,
    Maj32,
    Maj64,
    Maj8,
    Max16,
    Max32,
    Max64,
    Max8,
    Median16,
    Median32,
    Median64,
    Median8,
    Min16,
    Min32,
    Min64,
    Min8,
    Modulo16,
    Modulo32,
    Modulo64,
    Modulo8,
    Multiply16,
    Multiply32,
    Multiply64,
    Multiply8,
    Negate16,
    Negate32,
    Negate64,
    Negate8,
    NumInputs,
    NumOutputs,
    One16,
    One32,
    One64,
    One8,
    Or1,
    Or16,
    Or32,
    Or64,
    Or8,
    OutpointHash,
    OutputHash,
    OutputScriptHash,
    OutputScriptsHash,
    OutputValue,
    OutputValuesHash,
    OutputsHash,
    ParseLock,
    ParseSequence,
    PointVerify1,
    RightExtend16_32,
    RightExtend16_64,
    RightExtend32_64,
    RightExtend8_16,
    RightExtend8_32,
    RightExtend8_64,
    RightPadHigh16_32,
    RightPadHigh16_64,
    RightPadHigh1_16,
    RightPadHigh1_32,
    RightPadHigh1_64,
    RightPadHigh1_8,
    RightPadHigh32_64,
    RightPadHigh8_16,
    RightPadHigh8_32,
    RightPadHigh8_64,
    RightPadLow16_32,
    RightPadLow16_64,
    RightPadLow1_16,
    RightPadLow1_32,
    RightPadLow1_64,
    RightPadLow1_8,
    RightPadLow32_64,
    RightPadLow8_16,
    RightPadLow8_32,
    RightPadLow8_64,
    RightRotate16,
    RightRotate32,
    RightRotate64,
    RightRotate8,
    RightShift16,
    RightShift32,
    RightShift64,
    RightShift8,
    RightShiftWith16,
    RightShiftWith32,
    RightShiftWith64,
    RightShiftWith8,
    Rightmost16_1,
    Rightmost16_2,
    Rightmost16_4,
    Rightmost16_8,
    Rightmost32_1,
    Rightmost32_16,
    Rightmost32_2,
    Rightmost32_4,
    Rightmost32_8,
    Rightmost64_1,
    Rightmost64_16,
    Rightmost64_2,
    Rightmost64_32,
    Rightmost64_4,
    Rightmost64_8,
    Rightmost8_1,
    Rightmost8_2,
    Rightmost8_4,
    ScalarAdd,
    ScalarInvert,
    ScalarIsZero,
    ScalarMultiply,
    ScalarMultiplyLambda,
    ScalarNegate,
    ScalarNormalize,
    ScalarSquare,
    Scale,
    ScriptCMR,
    Sha256Block,
    Sha256Ctx8Add1,
    Sha256Ctx8Add128,
    Sha256Ctx8Add16,
    Sha256Ctx8Add2,
    Sha256Ctx8Add256,
    Sha256Ctx8Add32,
    Sha256Ctx8Add4,
    Sha256Ctx8Add512,
    Sha256Ctx8Add64,
    Sha256Ctx8Add8,
    Sha256Ctx8AddBuffer511,
    Sha256Ctx8Finalize,
    Sha256Ctx8Init,
    Sha256Iv,
    SigAllHash,
    Some1,
    Some16,
    Some32,
    Some64,
    Some8,
    Subtract16,
    Subtract32,
    Subtract64,
    Subtract8,
    Swu,
    TapEnvHash,
    TapdataInit,
    TapleafHash,
    TapleafVersion,
    Tappath,
    TappathHash,
    TotalInputValue,
    TotalOutputValue,
    TransactionId,
    TxHash,
    TxIsFinal,
    TxLockDistance,
    TxLockDuration,
    TxLockHeight,
    TxLockTime,
    Verify,
    Version,
    Xor1,
    Xor16,
    Xor32,
    Xor64,
    Xor8,
    XorXor1,
    XorXor16,
    XorXor32,
    XorXor64,
    XorXor8,
}

impl Bitcoin {
    /// Array of all Bitcoin jets.
    pub const ALL: [Self; 428] = [
        Self::Add16,
        Self::Add32,
        Self::Add64,
        Self::Add8,
        Self::All16,
        Self::All32,
        Self::All64,
        Self::All8,
        Self::And1,
        Self::And16,
        Self::And32,
        Self::And64,
        Self::And8,
        Self::AnnexHash,
        Self::Bip0340Verify,
        Self::BuildTapbranch,
        Self::BuildTapleafSimplicity,
        Self::BuildTaptweak,
        Self::Ch1,
        Self::Ch16,
        Self::Ch32,
        Self::Ch64,
        Self::Ch8,
        Self::CheckLockDistance,
        Self::CheckLockDuration,
        Self::CheckLockHeight,
        Self::CheckLockTime,
        Self::CheckSigVerify,
        Self::Complement1,
        Self::Complement16,
        Self::Complement32,
        Self::Complement64,
        Self::Complement8,
        Self::CurrentAnnexHash,
        Self::CurrentIndex,
        Self::CurrentPrevOutpoint,
        Self::CurrentScriptHash,
        Self::CurrentScriptSigHash,
        Self::CurrentSequence,
        Self::CurrentValue,
        Self::Decompress,
        Self::Decrement16,
        Self::Decrement32,
        Self::Decrement64,
        Self::Decrement8,
        Self::DivMod128_64,
        Self::DivMod16,
        Self::DivMod32,
        Self::DivMod64,
        Self::DivMod8,
        Self::Divide16,
        Self::Divide32,
        Self::Divide64,
        Self::Divide8,
        Self::Divides16,
        Self::Divides32,
        Self::Divides64,
        Self::Divides8,
        Self::Eq1,
        Self::Eq16,
        Self::Eq256,
        Self::Eq32,
        Self::Eq64,
        Self::Eq8,
        Self::FeAdd,
        Self::FeInvert,
        Self::FeIsOdd,
        Self::FeIsZero,
        Self::FeMultiply,
        Self::FeMultiplyBeta,
        Self::FeNegate,
        Self::FeNormalize,
        Self::FeSquare,
        Self::FeSquareRoot,
        Self::Fee,
        Self::FullAdd16,
        Self::FullAdd32,
        Self::FullAdd64,
        Self::FullAdd8,
        Self::FullDecrement16,
        Self::FullDecrement32,
        Self::FullDecrement64,
        Self::FullDecrement8,
        Self::FullIncrement16,
        Self::FullIncrement32,
        Self::FullIncrement64,
        Self::FullIncrement8,
        Self::FullLeftShift16_1,
        Self::FullLeftShift16_2,
        Self::FullLeftShift16_4,
        Self::FullLeftShift16_8,
        Self::FullLeftShift32_1,
        Self::FullLeftShift32_16,
        Self::FullLeftShift32_2,
        Self::FullLeftShift32_4,
        Self::FullLeftShift32_8,
        Self::FullLeftShift64_1,
        Self::FullLeftShift64_16,
        Self::FullLeftShift64_2,
        Self::FullLeftShift64_32,
        Self::FullLeftShift64_4,
        Self::FullLeftShift64_8,
        Self::FullLeftShift8_1,
        Self::FullLeftShift8_2,
        Self::FullLeftShift8_4,
        Self::FullMultiply16,
        Self::FullMultiply32,
        Self::FullMultiply64,
        Self::FullMultiply8,
        Self::FullRightShift16_1,
        Self::FullRightShift16_2,
        Self::FullRightShift16_4,
        Self::FullRightShift16_8,
        Self::FullRightShift32_1,
        Self::FullRightShift32_16,
        Self::FullRightShift32_2,
        Self::FullRightShift32_4,
        Self::FullRightShift32_8,
        Self::FullRightShift64_1,
        Self::FullRightShift64_16,
        Self::FullRightShift64_2,
        Self::FullRightShift64_32,
        Self::FullRightShift64_4,
        Self::FullRightShift64_8,
        Self::FullRightShift8_1,
        Self::FullRightShift8_2,
        Self::FullRightShift8_4,
        Self::FullSubtract16,
        Self::FullSubtract32,
        Self::FullSubtract64,
        Self::FullSubtract8,
        Self::GeIsOnCurve,
        Self::GeNegate,
        Self::GejAdd,
        Self::GejDouble,
        Self::GejEquiv,
        Self::GejGeAdd,
        Self::GejGeAddEx,
        Self::GejGeEquiv,
        Self::GejInfinity,
        Self::GejIsInfinity,
        Self::GejIsOnCurve,
        Self::GejNegate,
        Self::GejNormalize,
        Self::GejRescale,
        Self::GejXEquiv,
        Self::GejYIsOdd,
        Self::Generate,
        Self::HashToCurve,
        Self::High1,
        Self::High16,
        Self::High32,
        Self::High64,
        Self::High8,
        Self::Increment16,
        Self::Increment32,
        Self::Increment64,
        Self::Increment8,
        Self::InputAnnexHash,
        Self::InputAnnexesHash,
        Self::InputHash,
        Self::InputOutpointsHash,
        Self::InputPrevOutpoint,
        Self::InputScriptHash,
        Self::InputScriptSigHash,
        Self::InputScriptSigsHash,
        Self::InputScriptsHash,
        Self::InputSequence,
        Self::InputSequencesHash,
        Self::InputUtxoHash,
        Self::InputUtxosHash,
        Self::InputValue,
        Self::InputValuesHash,
        Self::InputsHash,
        Self::InternalKey,
        Self::IsOne16,
        Self::IsOne32,
        Self::IsOne64,
        Self::IsOne8,
        Self::IsZero16,
        Self::IsZero32,
        Self::IsZero64,
        Self::IsZero8,
        Self::Le16,
        Self::Le32,
        Self::Le64,
        Self::Le8,
        Self::LeftExtend16_32,
        Self::LeftExtend16_64,
        Self::LeftExtend1_16,
        Self::LeftExtend1_32,
        Self::LeftExtend1_64,
        Self::LeftExtend1_8,
        Self::LeftExtend32_64,
        Self::LeftExtend8_16,
        Self::LeftExtend8_32,
        Self::LeftExtend8_64,
        Self::LeftPadHigh16_32,
        Self::LeftPadHigh16_64,
        Self::LeftPadHigh1_16,
        Self::LeftPadHigh1_32,
        Self::LeftPadHigh1_64,
        Self::LeftPadHigh1_8,
        Self::LeftPadHigh32_64,
        Self::LeftPadHigh8_16,
        Self::LeftPadHigh8_32,
        Self::LeftPadHigh8_64,
        Self::LeftPadLow16_32,
        Self::LeftPadLow16_64,
        Self::LeftPadLow1_16,
        Self::LeftPadLow1_32,
        Self::LeftPadLow1_64,
        Self::LeftPadLow1_8,
        Self::LeftPadLow32_64,
        Self::LeftPadLow8_16,
        Self::LeftPadLow8_32,
        Self::LeftPadLow8_64,
        Self::LeftRotate16,
        Self::LeftRotate32,
        Self::LeftRotate64,
        Self::LeftRotate8,
        Self::LeftShift16,
        Self::LeftShift32,
        Self::LeftShift64,
        Self::LeftShift8,
        Self::LeftShiftWith16,
        Self::LeftShiftWith32,
        Self::LeftShiftWith64,
        Self::LeftShiftWith8,
        Self::Leftmost16_1,
        Self::Leftmost16_2,
        Self::Leftmost16_4,
        Self::Leftmost16_8,
        Self::Leftmost32_1,
        Self::Leftmost32_16,
        Self::Leftmost32_2,
        Self::Leftmost32_4,
        Self::Leftmost32_8,
        Self::Leftmost64_1,
        Self::Leftmost64_16,
        Self::Leftmost64_2,
        Self::Leftmost64_32,
        Self::Leftmost64_4,
        Self::Leftmost64_8,
        Self::Leftmost8_1,
        Self::Leftmost8_2,
        Self::Leftmost8_4,
        Self::LinearCombination1,
        Self::LinearVerify1,
        Self::LockTime,
        Self::Low1,
        Self::Low16,
        Self::Low32,
        Self::Low64,
        Self::Low8,
        Self::Lt16,
        Self::Lt32,
        Self::Lt64,
        Self::Lt8,
        Self::Maj1,
        Self::Maj16,
        Self::Maj32,
        Self::Maj64,
        Self::Maj8,
        Self::Max16,
        Self::Max32,
        Self::Max64,
        Self::Max8,
        Self::Median16,
        Self::Median32,
        Self::Median64,
        Self::Median8,
        Self::Min16,
        Self::Min32,
        Self::Min64,
        Self::Min8,
        Self::Modulo16,
        Self::Modulo32,
        Self::Modulo64,
        Self::Modulo8,
        Self::Multiply16,
        Self::Multiply32,
        Self::Multiply64,
        Self::Multiply8,
        Self::Negate16,
        Self::Negate32,
        Self::Negate64,
        Self::Negate8,
        Self::NumInputs,
        Self::NumOutputs,
        Self::One16,
        Self::One32,
        Self::One64,
        Self::One8,
        Self::Or1,
        Self::Or16,
        Self::Or32,
        Self::Or64,
        Self::Or8,
        Self::OutpointHash,
        Self::OutputHash,
        Self::OutputScriptHash,
        Self::OutputScriptsHash,
        Self::OutputValue,
        Self::OutputValuesHash,
        Self::OutputsHash,
        Self::ParseLock,
        Self::ParseSequence,
        Self::PointVerify1,
        Self::RightExtend16_32,
        Self::RightExtend16_64,
        Self::RightExtend32_64,
        Self::RightExtend8_16,
        Self::RightExtend8_32,
        Self::RightExtend8_64,
        Self::RightPadHigh16_32,
        Self::RightPadHigh16_64,
        Self::RightPadHigh1_16,
        Self::RightPadHigh1_32,
        Self::RightPadHigh1_64,
        Self::RightPadHigh1_8,
        Self::RightPadHigh32_64,
        Self::RightPadHigh8_16,
        Self::RightPadHigh8_32,
        Self::RightPadHigh8_64,
        Self::RightPadLow16_32,
        Self::RightPadLow16_64,
        Self::RightPadLow1_16,
        Self::RightPadLow1_32,
        Self::RightPadLow1_64,
        Self::RightPadLow1_8,
        Self::RightPadLow32_64,
        Self::RightPadLow8_16,
        Self::RightPadLow8_32,
        Self::RightPadLow8_64,
        Self::RightRotate16,
        Self::RightRotate32,
        Self::RightRotate64,
        Self::RightRotate8,
        Self::RightShift16,
        Self::RightShift32,
        Self::RightShift64,
        Self::RightShift8,
        Self::RightShiftWith16,
        Self::RightShiftWith32,
        Self::RightShiftWith64,
        Self::RightShiftWith8,
        Self::Rightmost16_1,
        Self::Rightmost16_2,
        Self::Rightmost16_4,
        Self::Rightmost16_8,
        Self::Rightmost32_1,
        Self::Rightmost32_16,
        Self::Rightmost32_2,
        Self::Rightmost32_4,
        Self::Rightmost32_8,
        Self::Rightmost64_1,
        Self::Rightmost64_16,
        Self::Rightmost64_2,
        Self::Rightmost64_32,
        Self::Rightmost64_4,
        Self::Rightmost64_8,
        Self::Rightmost8_1,
        Self::Rightmost8_2,
        Self::Rightmost8_4,
        Self::ScalarAdd,
        Self::ScalarInvert,
        Self::ScalarIsZero,
        Self::ScalarMultiply,
        Self::ScalarMultiplyLambda,
        Self::ScalarNegate,
        Self::ScalarNormalize,
        Self::ScalarSquare,
        Self::Scale,
        Self::ScriptCMR,
        Self::Sha256Block,
        Self::Sha256Ctx8Add1,
        Self::Sha256Ctx8Add128,
        Self::Sha256Ctx8Add16,
        Self::Sha256Ctx8Add2,
        Self::Sha256Ctx8Add256,
        Self::Sha256Ctx8Add32,
        Self::Sha256Ctx8Add4,
        Self::Sha256Ctx8Add512,
        Self::Sha256Ctx8Add64,
        Self::Sha256Ctx8Add8,
        Self::Sha256Ctx8AddBuffer511,
        Self::Sha256Ctx8Finalize,
        Self::Sha256Ctx8Init,
        Self::Sha256Iv,
        Self::SigAllHash,
        Self::Some1,
        Self::Some16,
        Self::Some32,
        Self::Some64,
        Self::Some8,
        Self::Subtract16,
        Self::Subtract32,
        Self::Subtract64,
        Self::Subtract8,
        Self::Swu,
        Self::TapEnvHash,
        Self::TapdataInit,
        Self::TapleafHash,
        Self::TapleafVersion,
        Self::Tappath,
        Self::TappathHash,
        Self::TotalInputValue,
        Self::TotalOutputValue,
        Self::TransactionId,
        Self::TxHash,
        Self::TxIsFinal,
        Self::TxLockDistance,
        Self::TxLockDuration,
        Self::TxLockHeight,
        Self::TxLockTime,
        Self::Verify,
        Self::Version,
        Self::Xor1,
        Self::Xor16,
        Self::Xor32,
        Self::Xor64,
        Self::Xor8,
        Self::XorXor1,
        Self::XorXor16,
        Self::XorXor32,
        Self::XorXor64,
        Self::XorXor8,
    ];
}

impl Jet for Bitcoin {

    type Transaction = bitcoin::Transaction;
    type Environment<T> = BitcoinEnv<T> where T: Borrow<Self::Transaction>;
    type CJetEnvironment = CTxEnv;

    fn c_jet_env<T>(env: &Self::Environment<T>) -> &Self::CJetEnvironment
        where T: Borrow<Self::Transaction>
    {
        env.c_tx_env()
    }

    fn cmr(&self) -> Cmr {
        unimplemented!("Bitcoin jet CMRs weights have not yet been implemented.")
    }

    fn source_ty(&self) -> TypeName {
        let name: &'static [u8] = match self {
            Bitcoin::Add16 => b"i",
            Bitcoin::Add32 => b"l",
            Bitcoin::Add64 => b"*ll",
            Bitcoin::Add8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::All16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::All32 => b"i",
            Bitcoin::All64 => b"l",
            Bitcoin::All8 => b"***22*22**22*22",
            Bitcoin::And1 => b"*22",
            Bitcoin::And16 => b"i",
            Bitcoin::And32 => b"l",
            Bitcoin::And64 => b"*ll",
            Bitcoin::And8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::AnnexHash => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh+1h",
            Bitcoin::Bip0340Verify => b"**hh*hh",
            Bitcoin::BuildTapbranch => b"*hh",
            Bitcoin::BuildTapleafSimplicity => b"h",
            Bitcoin::BuildTaptweak => b"*hh",
            Bitcoin::Ch1 => b"*2*22",
            Bitcoin::Ch16 => b"*****22*22**22*22***22*22**22*22i",
            Bitcoin::Ch32 => b"*il",
            Bitcoin::Ch64 => b"*l*ll",
            Bitcoin::Ch8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Bitcoin::CheckLockDistance => b"****22*22**22*22***22*22**22*22",
            Bitcoin::CheckLockDuration => b"****22*22**22*22***22*22**22*22",
            Bitcoin::CheckLockHeight => b"i",
            Bitcoin::CheckLockTime => b"i",
            Bitcoin::CheckSigVerify => b"**h*hh*hh",
            Bitcoin::Complement1 => b"2",
            Bitcoin::Complement16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Complement32 => b"i",
            Bitcoin::Complement64 => b"l",
            Bitcoin::Complement8 => b"***22*22**22*22",
            Bitcoin::CurrentAnnexHash => b"1",
            Bitcoin::CurrentIndex => b"1",
            Bitcoin::CurrentPrevOutpoint => b"1",
            Bitcoin::CurrentScriptHash => b"1",
            Bitcoin::CurrentScriptSigHash => b"1",
            Bitcoin::CurrentSequence => b"1",
            Bitcoin::CurrentValue => b"1",
            Bitcoin::Decompress => b"*2h",
            Bitcoin::Decrement16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Decrement32 => b"i",
            Bitcoin::Decrement64 => b"l",
            Bitcoin::Decrement8 => b"***22*22**22*22",
            Bitcoin::DivMod128_64 => b"**lll",
            Bitcoin::DivMod16 => b"i",
            Bitcoin::DivMod32 => b"l",
            Bitcoin::DivMod64 => b"*ll",
            Bitcoin::DivMod8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Divide16 => b"i",
            Bitcoin::Divide32 => b"l",
            Bitcoin::Divide64 => b"*ll",
            Bitcoin::Divide8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Divides16 => b"i",
            Bitcoin::Divides32 => b"l",
            Bitcoin::Divides64 => b"*ll",
            Bitcoin::Divides8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Eq1 => b"*22",
            Bitcoin::Eq16 => b"i",
            Bitcoin::Eq256 => b"*hh",
            Bitcoin::Eq32 => b"l",
            Bitcoin::Eq64 => b"*ll",
            Bitcoin::Eq8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::FeAdd => b"*hh",
            Bitcoin::FeInvert => b"h",
            Bitcoin::FeIsOdd => b"h",
            Bitcoin::FeIsZero => b"h",
            Bitcoin::FeMultiply => b"*hh",
            Bitcoin::FeMultiplyBeta => b"h",
            Bitcoin::FeNegate => b"h",
            Bitcoin::FeNormalize => b"h",
            Bitcoin::FeSquare => b"h",
            Bitcoin::FeSquareRoot => b"h",
            Bitcoin::Fee => b"1",
            Bitcoin::FullAdd16 => b"*2i",
            Bitcoin::FullAdd32 => b"*2l",
            Bitcoin::FullAdd64 => b"*2*ll",
            Bitcoin::FullAdd8 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::FullDecrement16 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::FullDecrement32 => b"*2i",
            Bitcoin::FullDecrement64 => b"*2l",
            Bitcoin::FullDecrement8 => b"*2***22*22**22*22",
            Bitcoin::FullIncrement16 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::FullIncrement32 => b"*2i",
            Bitcoin::FullIncrement64 => b"*2l",
            Bitcoin::FullIncrement8 => b"*2***22*22**22*22",
            Bitcoin::FullLeftShift16_1 => b"*****22*22**22*22***22*22**22*222",
            Bitcoin::FullLeftShift16_2 => b"*****22*22**22*22***22*22**22*22*22",
            Bitcoin::FullLeftShift16_4 => b"*****22*22**22*22***22*22**22*22**22*22",
            Bitcoin::FullLeftShift16_8 => b"*****22*22**22*22***22*22**22*22***22*22**22*22",
            Bitcoin::FullLeftShift32_1 => b"*i2",
            Bitcoin::FullLeftShift32_16 => b"*i****22*22**22*22***22*22**22*22",
            Bitcoin::FullLeftShift32_2 => b"*i*22",
            Bitcoin::FullLeftShift32_4 => b"*i**22*22",
            Bitcoin::FullLeftShift32_8 => b"*i***22*22**22*22",
            Bitcoin::FullLeftShift64_1 => b"*l2",
            Bitcoin::FullLeftShift64_16 => b"*l****22*22**22*22***22*22**22*22",
            Bitcoin::FullLeftShift64_2 => b"*l*22",
            Bitcoin::FullLeftShift64_32 => b"*li",
            Bitcoin::FullLeftShift64_4 => b"*l**22*22",
            Bitcoin::FullLeftShift64_8 => b"*l***22*22**22*22",
            Bitcoin::FullLeftShift8_1 => b"****22*22**22*222",
            Bitcoin::FullLeftShift8_2 => b"****22*22**22*22*22",
            Bitcoin::FullLeftShift8_4 => b"****22*22**22*22**22*22",
            Bitcoin::FullMultiply16 => b"l",
            Bitcoin::FullMultiply32 => b"*ll",
            Bitcoin::FullMultiply64 => b"h",
            Bitcoin::FullMultiply8 => b"i",
            Bitcoin::FullRightShift16_1 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::FullRightShift16_2 => b"**22****22*22**22*22***22*22**22*22",
            Bitcoin::FullRightShift16_4 => b"***22*22****22*22**22*22***22*22**22*22",
            Bitcoin::FullRightShift16_8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Bitcoin::FullRightShift32_1 => b"*2i",
            Bitcoin::FullRightShift32_16 => b"*****22*22**22*22***22*22**22*22i",
            Bitcoin::FullRightShift32_2 => b"**22i",
            Bitcoin::FullRightShift32_4 => b"***22*22i",
            Bitcoin::FullRightShift32_8 => b"****22*22**22*22i",
            Bitcoin::FullRightShift64_1 => b"*2l",
            Bitcoin::FullRightShift64_16 => b"*****22*22**22*22***22*22**22*22l",
            Bitcoin::FullRightShift64_2 => b"**22l",
            Bitcoin::FullRightShift64_32 => b"*il",
            Bitcoin::FullRightShift64_4 => b"***22*22l",
            Bitcoin::FullRightShift64_8 => b"****22*22**22*22l",
            Bitcoin::FullRightShift8_1 => b"*2***22*22**22*22",
            Bitcoin::FullRightShift8_2 => b"**22***22*22**22*22",
            Bitcoin::FullRightShift8_4 => b"***22*22***22*22**22*22",
            Bitcoin::FullSubtract16 => b"*2i",
            Bitcoin::FullSubtract32 => b"*2l",
            Bitcoin::FullSubtract64 => b"*2*ll",
            Bitcoin::FullSubtract8 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::GeIsOnCurve => b"*hh",
            Bitcoin::GeNegate => b"*hh",
            Bitcoin::GejAdd => b"***hhh**hhh",
            Bitcoin::GejDouble => b"**hhh",
            Bitcoin::GejEquiv => b"***hhh**hhh",
            Bitcoin::GejGeAdd => b"***hhh*hh",
            Bitcoin::GejGeAddEx => b"***hhh*hh",
            Bitcoin::GejGeEquiv => b"***hhh*hh",
            Bitcoin::GejInfinity => b"1",
            Bitcoin::GejIsInfinity => b"**hhh",
            Bitcoin::GejIsOnCurve => b"**hhh",
            Bitcoin::GejNegate => b"**hhh",
            Bitcoin::GejNormalize => b"**hhh",
            Bitcoin::GejRescale => b"***hhhh",
            Bitcoin::GejXEquiv => b"*h**hhh",
            Bitcoin::GejYIsOdd => b"**hhh",
            Bitcoin::Generate => b"h",
            Bitcoin::HashToCurve => b"h",
            Bitcoin::High1 => b"1",
            Bitcoin::High16 => b"1",
            Bitcoin::High32 => b"1",
            Bitcoin::High64 => b"1",
            Bitcoin::High8 => b"1",
            Bitcoin::Increment16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Increment32 => b"i",
            Bitcoin::Increment64 => b"l",
            Bitcoin::Increment8 => b"***22*22**22*22",
            Bitcoin::InputAnnexHash => b"i",
            Bitcoin::InputAnnexesHash => b"1",
            Bitcoin::InputHash => b"i",
            Bitcoin::InputOutpointsHash => b"1",
            Bitcoin::InputPrevOutpoint => b"i",
            Bitcoin::InputScriptHash => b"i",
            Bitcoin::InputScriptSigHash => b"i",
            Bitcoin::InputScriptSigsHash => b"1",
            Bitcoin::InputScriptsHash => b"1",
            Bitcoin::InputSequence => b"i",
            Bitcoin::InputSequencesHash => b"1",
            Bitcoin::InputUtxoHash => b"i",
            Bitcoin::InputUtxosHash => b"1",
            Bitcoin::InputValue => b"i",
            Bitcoin::InputValuesHash => b"1",
            Bitcoin::InputsHash => b"1",
            Bitcoin::InternalKey => b"1",
            Bitcoin::IsOne16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::IsOne32 => b"i",
            Bitcoin::IsOne64 => b"l",
            Bitcoin::IsOne8 => b"***22*22**22*22",
            Bitcoin::IsZero16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::IsZero32 => b"i",
            Bitcoin::IsZero64 => b"l",
            Bitcoin::IsZero8 => b"***22*22**22*22",
            Bitcoin::Le16 => b"i",
            Bitcoin::Le32 => b"l",
            Bitcoin::Le64 => b"*ll",
            Bitcoin::Le8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftExtend16_32 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftExtend16_64 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftExtend1_16 => b"2",
            Bitcoin::LeftExtend1_32 => b"2",
            Bitcoin::LeftExtend1_64 => b"2",
            Bitcoin::LeftExtend1_8 => b"2",
            Bitcoin::LeftExtend32_64 => b"i",
            Bitcoin::LeftExtend8_16 => b"***22*22**22*22",
            Bitcoin::LeftExtend8_32 => b"***22*22**22*22",
            Bitcoin::LeftExtend8_64 => b"***22*22**22*22",
            Bitcoin::LeftPadHigh16_32 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftPadHigh16_64 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftPadHigh1_16 => b"2",
            Bitcoin::LeftPadHigh1_32 => b"2",
            Bitcoin::LeftPadHigh1_64 => b"2",
            Bitcoin::LeftPadHigh1_8 => b"2",
            Bitcoin::LeftPadHigh32_64 => b"i",
            Bitcoin::LeftPadHigh8_16 => b"***22*22**22*22",
            Bitcoin::LeftPadHigh8_32 => b"***22*22**22*22",
            Bitcoin::LeftPadHigh8_64 => b"***22*22**22*22",
            Bitcoin::LeftPadLow16_32 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftPadLow16_64 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftPadLow1_16 => b"2",
            Bitcoin::LeftPadLow1_32 => b"2",
            Bitcoin::LeftPadLow1_64 => b"2",
            Bitcoin::LeftPadLow1_8 => b"2",
            Bitcoin::LeftPadLow32_64 => b"i",
            Bitcoin::LeftPadLow8_16 => b"***22*22**22*22",
            Bitcoin::LeftPadLow8_32 => b"***22*22**22*22",
            Bitcoin::LeftPadLow8_64 => b"***22*22**22*22",
            Bitcoin::LeftRotate16 => b"***22*22****22*22**22*22***22*22**22*22",
            Bitcoin::LeftRotate32 => b"****22*22**22*22i",
            Bitcoin::LeftRotate64 => b"****22*22**22*22l",
            Bitcoin::LeftRotate8 => b"***22*22***22*22**22*22",
            Bitcoin::LeftShift16 => b"***22*22****22*22**22*22***22*22**22*22",
            Bitcoin::LeftShift32 => b"****22*22**22*22i",
            Bitcoin::LeftShift64 => b"****22*22**22*22l",
            Bitcoin::LeftShift8 => b"***22*22***22*22**22*22",
            Bitcoin::LeftShiftWith16 => b"*2***22*22****22*22**22*22***22*22**22*22",
            Bitcoin::LeftShiftWith32 => b"*2****22*22**22*22i",
            Bitcoin::LeftShiftWith64 => b"*2****22*22**22*22l",
            Bitcoin::LeftShiftWith8 => b"*2***22*22***22*22**22*22",
            Bitcoin::Leftmost16_1 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Leftmost16_2 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Leftmost16_4 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Leftmost16_8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Leftmost32_1 => b"i",
            Bitcoin::Leftmost32_16 => b"i",
            Bitcoin::Leftmost32_2 => b"i",
            Bitcoin::Leftmost32_4 => b"i",
            Bitcoin::Leftmost32_8 => b"i",
            Bitcoin::Leftmost64_1 => b"l",
            Bitcoin::Leftmost64_16 => b"l",
            Bitcoin::Leftmost64_2 => b"l",
            Bitcoin::Leftmost64_32 => b"l",
            Bitcoin::Leftmost64_4 => b"l",
            Bitcoin::Leftmost64_8 => b"l",
            Bitcoin::Leftmost8_1 => b"***22*22**22*22",
            Bitcoin::Leftmost8_2 => b"***22*22**22*22",
            Bitcoin::Leftmost8_4 => b"***22*22**22*22",
            Bitcoin::LinearCombination1 => b"**h**hhhh",
            Bitcoin::LinearVerify1 => b"***h*hhh*hh",
            Bitcoin::LockTime => b"1",
            Bitcoin::Low1 => b"1",
            Bitcoin::Low16 => b"1",
            Bitcoin::Low32 => b"1",
            Bitcoin::Low64 => b"1",
            Bitcoin::Low8 => b"1",
            Bitcoin::Lt16 => b"i",
            Bitcoin::Lt32 => b"l",
            Bitcoin::Lt64 => b"*ll",
            Bitcoin::Lt8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Maj1 => b"*2*22",
            Bitcoin::Maj16 => b"*****22*22**22*22***22*22**22*22i",
            Bitcoin::Maj32 => b"*il",
            Bitcoin::Maj64 => b"*l*ll",
            Bitcoin::Maj8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Bitcoin::Max16 => b"i",
            Bitcoin::Max32 => b"l",
            Bitcoin::Max64 => b"*ll",
            Bitcoin::Max8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Median16 => b"*****22*22**22*22***22*22**22*22i",
            Bitcoin::Median32 => b"*il",
            Bitcoin::Median64 => b"*l*ll",
            Bitcoin::Median8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Bitcoin::Min16 => b"i",
            Bitcoin::Min32 => b"l",
            Bitcoin::Min64 => b"*ll",
            Bitcoin::Min8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Modulo16 => b"i",
            Bitcoin::Modulo32 => b"l",
            Bitcoin::Modulo64 => b"*ll",
            Bitcoin::Modulo8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Multiply16 => b"i",
            Bitcoin::Multiply32 => b"l",
            Bitcoin::Multiply64 => b"*ll",
            Bitcoin::Multiply8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Negate16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Negate32 => b"i",
            Bitcoin::Negate64 => b"l",
            Bitcoin::Negate8 => b"***22*22**22*22",
            Bitcoin::NumInputs => b"1",
            Bitcoin::NumOutputs => b"1",
            Bitcoin::One16 => b"1",
            Bitcoin::One32 => b"1",
            Bitcoin::One64 => b"1",
            Bitcoin::One8 => b"1",
            Bitcoin::Or1 => b"*22",
            Bitcoin::Or16 => b"i",
            Bitcoin::Or32 => b"l",
            Bitcoin::Or64 => b"*ll",
            Bitcoin::Or8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::OutpointHash => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh*hi",
            Bitcoin::OutputHash => b"i",
            Bitcoin::OutputScriptHash => b"i",
            Bitcoin::OutputScriptsHash => b"1",
            Bitcoin::OutputValue => b"i",
            Bitcoin::OutputValuesHash => b"1",
            Bitcoin::OutputsHash => b"1",
            Bitcoin::ParseLock => b"i",
            Bitcoin::ParseSequence => b"i",
            Bitcoin::PointVerify1 => b"***h*2hh*2h",
            Bitcoin::RightExtend16_32 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::RightExtend16_64 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::RightExtend32_64 => b"i",
            Bitcoin::RightExtend8_16 => b"***22*22**22*22",
            Bitcoin::RightExtend8_32 => b"***22*22**22*22",
            Bitcoin::RightExtend8_64 => b"***22*22**22*22",
            Bitcoin::RightPadHigh16_32 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::RightPadHigh16_64 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::RightPadHigh1_16 => b"2",
            Bitcoin::RightPadHigh1_32 => b"2",
            Bitcoin::RightPadHigh1_64 => b"2",
            Bitcoin::RightPadHigh1_8 => b"2",
            Bitcoin::RightPadHigh32_64 => b"i",
            Bitcoin::RightPadHigh8_16 => b"***22*22**22*22",
            Bitcoin::RightPadHigh8_32 => b"***22*22**22*22",
            Bitcoin::RightPadHigh8_64 => b"***22*22**22*22",
            Bitcoin::RightPadLow16_32 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::RightPadLow16_64 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::RightPadLow1_16 => b"2",
            Bitcoin::RightPadLow1_32 => b"2",
            Bitcoin::RightPadLow1_64 => b"2",
            Bitcoin::RightPadLow1_8 => b"2",
            Bitcoin::RightPadLow32_64 => b"i",
            Bitcoin::RightPadLow8_16 => b"***22*22**22*22",
            Bitcoin::RightPadLow8_32 => b"***22*22**22*22",
            Bitcoin::RightPadLow8_64 => b"***22*22**22*22",
            Bitcoin::RightRotate16 => b"***22*22****22*22**22*22***22*22**22*22",
            Bitcoin::RightRotate32 => b"****22*22**22*22i",
            Bitcoin::RightRotate64 => b"****22*22**22*22l",
            Bitcoin::RightRotate8 => b"***22*22***22*22**22*22",
            Bitcoin::RightShift16 => b"***22*22****22*22**22*22***22*22**22*22",
            Bitcoin::RightShift32 => b"****22*22**22*22i",
            Bitcoin::RightShift64 => b"****22*22**22*22l",
            Bitcoin::RightShift8 => b"***22*22***22*22**22*22",
            Bitcoin::RightShiftWith16 => b"*2***22*22****22*22**22*22***22*22**22*22",
            Bitcoin::RightShiftWith32 => b"*2****22*22**22*22i",
            Bitcoin::RightShiftWith64 => b"*2****22*22**22*22l",
            Bitcoin::RightShiftWith8 => b"*2***22*22***22*22**22*22",
            Bitcoin::Rightmost16_1 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Rightmost16_2 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Rightmost16_4 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Rightmost16_8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Rightmost32_1 => b"i",
            Bitcoin::Rightmost32_16 => b"i",
            Bitcoin::Rightmost32_2 => b"i",
            Bitcoin::Rightmost32_4 => b"i",
            Bitcoin::Rightmost32_8 => b"i",
            Bitcoin::Rightmost64_1 => b"l",
            Bitcoin::Rightmost64_16 => b"l",
            Bitcoin::Rightmost64_2 => b"l",
            Bitcoin::Rightmost64_32 => b"l",
            Bitcoin::Rightmost64_4 => b"l",
            Bitcoin::Rightmost64_8 => b"l",
            Bitcoin::Rightmost8_1 => b"***22*22**22*22",
            Bitcoin::Rightmost8_2 => b"***22*22**22*22",
            Bitcoin::Rightmost8_4 => b"***22*22**22*22",
            Bitcoin::ScalarAdd => b"*hh",
            Bitcoin::ScalarInvert => b"h",
            Bitcoin::ScalarIsZero => b"h",
            Bitcoin::ScalarMultiply => b"*hh",
            Bitcoin::ScalarMultiplyLambda => b"h",
            Bitcoin::ScalarNegate => b"h",
            Bitcoin::ScalarNormalize => b"h",
            Bitcoin::ScalarSquare => b"h",
            Bitcoin::Scale => b"*h**hhh",
            Bitcoin::ScriptCMR => b"1",
            Bitcoin::Sha256Block => b"*h*hh",
            Bitcoin::Sha256Ctx8Add1 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh***22*22**22*22",
            Bitcoin::Sha256Ctx8Add128 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh**hh*hh",
            Bitcoin::Sha256Ctx8Add16 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh*ll",
            Bitcoin::Sha256Ctx8Add2 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh****22*22**22*22***22*22**22*22",
            Bitcoin::Sha256Ctx8Add256 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh***hh*hh**hh*hh",
            Bitcoin::Sha256Ctx8Add32 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lhh",
            Bitcoin::Sha256Ctx8Add4 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lhi",
            Bitcoin::Sha256Ctx8Add512 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh****hh*hh**hh*hh***hh*hh**hh*hh",
            Bitcoin::Sha256Ctx8Add64 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh*hh",
            Bitcoin::Sha256Ctx8Add8 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lhl",
            Bitcoin::Sha256Ctx8AddBuffer511 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh*+1***hh*hh**hh*hh*+1**hh*hh*+1*hh*+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22",
            Bitcoin::Sha256Ctx8Finalize => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::Sha256Ctx8Init => b"1",
            Bitcoin::Sha256Iv => b"1",
            Bitcoin::SigAllHash => b"1",
            Bitcoin::Some1 => b"2",
            Bitcoin::Some16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Some32 => b"i",
            Bitcoin::Some64 => b"l",
            Bitcoin::Some8 => b"***22*22**22*22",
            Bitcoin::Subtract16 => b"i",
            Bitcoin::Subtract32 => b"l",
            Bitcoin::Subtract64 => b"*ll",
            Bitcoin::Subtract8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Swu => b"h",
            Bitcoin::TapEnvHash => b"1",
            Bitcoin::TapdataInit => b"1",
            Bitcoin::TapleafHash => b"1",
            Bitcoin::TapleafVersion => b"1",
            Bitcoin::Tappath => b"***22*22**22*22",
            Bitcoin::TappathHash => b"1",
            Bitcoin::TotalInputValue => b"1",
            Bitcoin::TotalOutputValue => b"1",
            Bitcoin::TransactionId => b"1",
            Bitcoin::TxHash => b"1",
            Bitcoin::TxIsFinal => b"1",
            Bitcoin::TxLockDistance => b"1",
            Bitcoin::TxLockDuration => b"1",
            Bitcoin::TxLockHeight => b"1",
            Bitcoin::TxLockTime => b"1",
            Bitcoin::Verify => b"2",
            Bitcoin::Version => b"1",
            Bitcoin::Xor1 => b"*22",
            Bitcoin::Xor16 => b"i",
            Bitcoin::Xor32 => b"l",
            Bitcoin::Xor64 => b"*ll",
            Bitcoin::Xor8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::XorXor1 => b"*2*22",
            Bitcoin::XorXor16 => b"*****22*22**22*22***22*22**22*22i",
            Bitcoin::XorXor32 => b"*il",
            Bitcoin::XorXor64 => b"*l*ll",
            Bitcoin::XorXor8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
        };

        TypeName(name)
    }

    fn target_ty(&self) -> TypeName {
        let name: &'static [u8] = match self {
            Bitcoin::Add16 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::Add32 => b"*2i",
            Bitcoin::Add64 => b"*2l",
            Bitcoin::Add8 => b"*2***22*22**22*22",
            Bitcoin::All16 => b"2",
            Bitcoin::All32 => b"2",
            Bitcoin::All64 => b"2",
            Bitcoin::All8 => b"2",
            Bitcoin::And1 => b"2",
            Bitcoin::And16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::And32 => b"i",
            Bitcoin::And64 => b"l",
            Bitcoin::And8 => b"***22*22**22*22",
            Bitcoin::AnnexHash => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::Bip0340Verify => b"1",
            Bitcoin::BuildTapbranch => b"h",
            Bitcoin::BuildTapleafSimplicity => b"h",
            Bitcoin::BuildTaptweak => b"h",
            Bitcoin::Ch1 => b"2",
            Bitcoin::Ch16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Ch32 => b"i",
            Bitcoin::Ch64 => b"l",
            Bitcoin::Ch8 => b"***22*22**22*22",
            Bitcoin::CheckLockDistance => b"1",
            Bitcoin::CheckLockDuration => b"1",
            Bitcoin::CheckLockHeight => b"1",
            Bitcoin::CheckLockTime => b"1",
            Bitcoin::CheckSigVerify => b"1",
            Bitcoin::Complement1 => b"2",
            Bitcoin::Complement16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Complement32 => b"i",
            Bitcoin::Complement64 => b"l",
            Bitcoin::Complement8 => b"***22*22**22*22",
            Bitcoin::CurrentAnnexHash => b"+1h",
            Bitcoin::CurrentIndex => b"i",
            Bitcoin::CurrentPrevOutpoint => b"*hi",
            Bitcoin::CurrentScriptHash => b"h",
            Bitcoin::CurrentScriptSigHash => b"h",
            Bitcoin::CurrentSequence => b"i",
            Bitcoin::CurrentValue => b"l",
            Bitcoin::Decompress => b"+1*hh",
            Bitcoin::Decrement16 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::Decrement32 => b"*2i",
            Bitcoin::Decrement64 => b"*2l",
            Bitcoin::Decrement8 => b"*2***22*22**22*22",
            Bitcoin::DivMod128_64 => b"*ll",
            Bitcoin::DivMod16 => b"i",
            Bitcoin::DivMod32 => b"l",
            Bitcoin::DivMod64 => b"*ll",
            Bitcoin::DivMod8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Divide16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Divide32 => b"i",
            Bitcoin::Divide64 => b"l",
            Bitcoin::Divide8 => b"***22*22**22*22",
            Bitcoin::Divides16 => b"2",
            Bitcoin::Divides32 => b"2",
            Bitcoin::Divides64 => b"2",
            Bitcoin::Divides8 => b"2",
            Bitcoin::Eq1 => b"2",
            Bitcoin::Eq16 => b"2",
            Bitcoin::Eq256 => b"2",
            Bitcoin::Eq32 => b"2",
            Bitcoin::Eq64 => b"2",
            Bitcoin::Eq8 => b"2",
            Bitcoin::FeAdd => b"h",
            Bitcoin::FeInvert => b"h",
            Bitcoin::FeIsOdd => b"2",
            Bitcoin::FeIsZero => b"2",
            Bitcoin::FeMultiply => b"h",
            Bitcoin::FeMultiplyBeta => b"h",
            Bitcoin::FeNegate => b"h",
            Bitcoin::FeNormalize => b"h",
            Bitcoin::FeSquare => b"h",
            Bitcoin::FeSquareRoot => b"+1h",
            Bitcoin::Fee => b"l",
            Bitcoin::FullAdd16 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::FullAdd32 => b"*2i",
            Bitcoin::FullAdd64 => b"*2l",
            Bitcoin::FullAdd8 => b"*2***22*22**22*22",
            Bitcoin::FullDecrement16 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::FullDecrement32 => b"*2i",
            Bitcoin::FullDecrement64 => b"*2l",
            Bitcoin::FullDecrement8 => b"*2***22*22**22*22",
            Bitcoin::FullIncrement16 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::FullIncrement32 => b"*2i",
            Bitcoin::FullIncrement64 => b"*2l",
            Bitcoin::FullIncrement8 => b"*2***22*22**22*22",
            Bitcoin::FullLeftShift16_1 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::FullLeftShift16_2 => b"**22****22*22**22*22***22*22**22*22",
            Bitcoin::FullLeftShift16_4 => b"***22*22****22*22**22*22***22*22**22*22",
            Bitcoin::FullLeftShift16_8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Bitcoin::FullLeftShift32_1 => b"*2i",
            Bitcoin::FullLeftShift32_16 => b"*****22*22**22*22***22*22**22*22i",
            Bitcoin::FullLeftShift32_2 => b"**22i",
            Bitcoin::FullLeftShift32_4 => b"***22*22i",
            Bitcoin::FullLeftShift32_8 => b"****22*22**22*22i",
            Bitcoin::FullLeftShift64_1 => b"*2l",
            Bitcoin::FullLeftShift64_16 => b"*****22*22**22*22***22*22**22*22l",
            Bitcoin::FullLeftShift64_2 => b"**22l",
            Bitcoin::FullLeftShift64_32 => b"*il",
            Bitcoin::FullLeftShift64_4 => b"***22*22l",
            Bitcoin::FullLeftShift64_8 => b"****22*22**22*22l",
            Bitcoin::FullLeftShift8_1 => b"*2***22*22**22*22",
            Bitcoin::FullLeftShift8_2 => b"**22***22*22**22*22",
            Bitcoin::FullLeftShift8_4 => b"***22*22***22*22**22*22",
            Bitcoin::FullMultiply16 => b"i",
            Bitcoin::FullMultiply32 => b"l",
            Bitcoin::FullMultiply64 => b"*ll",
            Bitcoin::FullMultiply8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::FullRightShift16_1 => b"*****22*22**22*22***22*22**22*222",
            Bitcoin::FullRightShift16_2 => b"*****22*22**22*22***22*22**22*22*22",
            Bitcoin::FullRightShift16_4 => b"*****22*22**22*22***22*22**22*22**22*22",
            Bitcoin::FullRightShift16_8 => b"*****22*22**22*22***22*22**22*22***22*22**22*22",
            Bitcoin::FullRightShift32_1 => b"*i2",
            Bitcoin::FullRightShift32_16 => b"*i****22*22**22*22***22*22**22*22",
            Bitcoin::FullRightShift32_2 => b"*i*22",
            Bitcoin::FullRightShift32_4 => b"*i**22*22",
            Bitcoin::FullRightShift32_8 => b"*i***22*22**22*22",
            Bitcoin::FullRightShift64_1 => b"*l2",
            Bitcoin::FullRightShift64_16 => b"*l****22*22**22*22***22*22**22*22",
            Bitcoin::FullRightShift64_2 => b"*l*22",
            Bitcoin::FullRightShift64_32 => b"*li",
            Bitcoin::FullRightShift64_4 => b"*l**22*22",
            Bitcoin::FullRightShift64_8 => b"*l***22*22**22*22",
            Bitcoin::FullRightShift8_1 => b"****22*22**22*222",
            Bitcoin::FullRightShift8_2 => b"****22*22**22*22*22",
            Bitcoin::FullRightShift8_4 => b"****22*22**22*22**22*22",
            Bitcoin::FullSubtract16 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::FullSubtract32 => b"*2i",
            Bitcoin::FullSubtract64 => b"*2l",
            Bitcoin::FullSubtract8 => b"*2***22*22**22*22",
            Bitcoin::GeIsOnCurve => b"2",
            Bitcoin::GeNegate => b"*hh",
            Bitcoin::GejAdd => b"**hhh",
            Bitcoin::GejDouble => b"**hhh",
            Bitcoin::GejEquiv => b"2",
            Bitcoin::GejGeAdd => b"**hhh",
            Bitcoin::GejGeAddEx => b"*h**hhh",
            Bitcoin::GejGeEquiv => b"2",
            Bitcoin::GejInfinity => b"**hhh",
            Bitcoin::GejIsInfinity => b"2",
            Bitcoin::GejIsOnCurve => b"2",
            Bitcoin::GejNegate => b"**hhh",
            Bitcoin::GejNormalize => b"+1*hh",
            Bitcoin::GejRescale => b"**hhh",
            Bitcoin::GejXEquiv => b"2",
            Bitcoin::GejYIsOdd => b"2",
            Bitcoin::Generate => b"**hhh",
            Bitcoin::HashToCurve => b"*hh",
            Bitcoin::High1 => b"2",
            Bitcoin::High16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::High32 => b"i",
            Bitcoin::High64 => b"l",
            Bitcoin::High8 => b"***22*22**22*22",
            Bitcoin::Increment16 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::Increment32 => b"*2i",
            Bitcoin::Increment64 => b"*2l",
            Bitcoin::Increment8 => b"*2***22*22**22*22",
            Bitcoin::InputAnnexHash => b"+1+1h",
            Bitcoin::InputAnnexesHash => b"h",
            Bitcoin::InputHash => b"+1h",
            Bitcoin::InputOutpointsHash => b"h",
            Bitcoin::InputPrevOutpoint => b"+1*hi",
            Bitcoin::InputScriptHash => b"+1h",
            Bitcoin::InputScriptSigHash => b"+1h",
            Bitcoin::InputScriptSigsHash => b"h",
            Bitcoin::InputScriptsHash => b"h",
            Bitcoin::InputSequence => b"+1i",
            Bitcoin::InputSequencesHash => b"h",
            Bitcoin::InputUtxoHash => b"+1h",
            Bitcoin::InputUtxosHash => b"h",
            Bitcoin::InputValue => b"+1l",
            Bitcoin::InputValuesHash => b"h",
            Bitcoin::InputsHash => b"h",
            Bitcoin::InternalKey => b"h",
            Bitcoin::IsOne16 => b"2",
            Bitcoin::IsOne32 => b"2",
            Bitcoin::IsOne64 => b"2",
            Bitcoin::IsOne8 => b"2",
            Bitcoin::IsZero16 => b"2",
            Bitcoin::IsZero32 => b"2",
            Bitcoin::IsZero64 => b"2",
            Bitcoin::IsZero8 => b"2",
            Bitcoin::Le16 => b"2",
            Bitcoin::Le32 => b"2",
            Bitcoin::Le64 => b"2",
            Bitcoin::Le8 => b"2",
            Bitcoin::LeftExtend16_32 => b"i",
            Bitcoin::LeftExtend16_64 => b"l",
            Bitcoin::LeftExtend1_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftExtend1_32 => b"i",
            Bitcoin::LeftExtend1_64 => b"l",
            Bitcoin::LeftExtend1_8 => b"***22*22**22*22",
            Bitcoin::LeftExtend32_64 => b"l",
            Bitcoin::LeftExtend8_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftExtend8_32 => b"i",
            Bitcoin::LeftExtend8_64 => b"l",
            Bitcoin::LeftPadHigh16_32 => b"i",
            Bitcoin::LeftPadHigh16_64 => b"l",
            Bitcoin::LeftPadHigh1_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftPadHigh1_32 => b"i",
            Bitcoin::LeftPadHigh1_64 => b"l",
            Bitcoin::LeftPadHigh1_8 => b"***22*22**22*22",
            Bitcoin::LeftPadHigh32_64 => b"l",
            Bitcoin::LeftPadHigh8_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftPadHigh8_32 => b"i",
            Bitcoin::LeftPadHigh8_64 => b"l",
            Bitcoin::LeftPadLow16_32 => b"i",
            Bitcoin::LeftPadLow16_64 => b"l",
            Bitcoin::LeftPadLow1_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftPadLow1_32 => b"i",
            Bitcoin::LeftPadLow1_64 => b"l",
            Bitcoin::LeftPadLow1_8 => b"***22*22**22*22",
            Bitcoin::LeftPadLow32_64 => b"l",
            Bitcoin::LeftPadLow8_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftPadLow8_32 => b"i",
            Bitcoin::LeftPadLow8_64 => b"l",
            Bitcoin::LeftRotate16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftRotate32 => b"i",
            Bitcoin::LeftRotate64 => b"l",
            Bitcoin::LeftRotate8 => b"***22*22**22*22",
            Bitcoin::LeftShift16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftShift32 => b"i",
            Bitcoin::LeftShift64 => b"l",
            Bitcoin::LeftShift8 => b"***22*22**22*22",
            Bitcoin::LeftShiftWith16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftShiftWith32 => b"i",
            Bitcoin::LeftShiftWith64 => b"l",
            Bitcoin::LeftShiftWith8 => b"***22*22**22*22",
            Bitcoin::Leftmost16_1 => b"2",
            Bitcoin::Leftmost16_2 => b"*22",
            Bitcoin::Leftmost16_4 => b"**22*22",
            Bitcoin::Leftmost16_8 => b"***22*22**22*22",
            Bitcoin::Leftmost32_1 => b"2",
            Bitcoin::Leftmost32_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Leftmost32_2 => b"*22",
            Bitcoin::Leftmost32_4 => b"**22*22",
            Bitcoin::Leftmost32_8 => b"***22*22**22*22",
            Bitcoin::Leftmost64_1 => b"2",
            Bitcoin::Leftmost64_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Leftmost64_2 => b"*22",
            Bitcoin::Leftmost64_32 => b"i",
            Bitcoin::Leftmost64_4 => b"**22*22",
            Bitcoin::Leftmost64_8 => b"***22*22**22*22",
            Bitcoin::Leftmost8_1 => b"2",
            Bitcoin::Leftmost8_2 => b"*22",
            Bitcoin::Leftmost8_4 => b"**22*22",
            Bitcoin::LinearCombination1 => b"**hhh",
            Bitcoin::LinearVerify1 => b"1",
            Bitcoin::LockTime => b"i",
            Bitcoin::Low1 => b"2",
            Bitcoin::Low16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Low32 => b"i",
            Bitcoin::Low64 => b"l",
            Bitcoin::Low8 => b"***22*22**22*22",
            Bitcoin::Lt16 => b"2",
            Bitcoin::Lt32 => b"2",
            Bitcoin::Lt64 => b"2",
            Bitcoin::Lt8 => b"2",
            Bitcoin::Maj1 => b"2",
            Bitcoin::Maj16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Maj32 => b"i",
            Bitcoin::Maj64 => b"l",
            Bitcoin::Maj8 => b"***22*22**22*22",
            Bitcoin::Max16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Max32 => b"i",
            Bitcoin::Max64 => b"l",
            Bitcoin::Max8 => b"***22*22**22*22",
            Bitcoin::Median16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Median32 => b"i",
            Bitcoin::Median64 => b"l",
            Bitcoin::Median8 => b"***22*22**22*22",
            Bitcoin::Min16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Min32 => b"i",
            Bitcoin::Min64 => b"l",
            Bitcoin::Min8 => b"***22*22**22*22",
            Bitcoin::Modulo16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Modulo32 => b"i",
            Bitcoin::Modulo64 => b"l",
            Bitcoin::Modulo8 => b"***22*22**22*22",
            Bitcoin::Multiply16 => b"i",
            Bitcoin::Multiply32 => b"l",
            Bitcoin::Multiply64 => b"*ll",
            Bitcoin::Multiply8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Negate16 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::Negate32 => b"*2i",
            Bitcoin::Negate64 => b"*2l",
            Bitcoin::Negate8 => b"*2***22*22**22*22",
            Bitcoin::NumInputs => b"i",
            Bitcoin::NumOutputs => b"i",
            Bitcoin::One16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::One32 => b"i",
            Bitcoin::One64 => b"l",
            Bitcoin::One8 => b"***22*22**22*22",
            Bitcoin::Or1 => b"2",
            Bitcoin::Or16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Or32 => b"i",
            Bitcoin::Or64 => b"l",
            Bitcoin::Or8 => b"***22*22**22*22",
            Bitcoin::OutpointHash => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::OutputHash => b"+1h",
            Bitcoin::OutputScriptHash => b"+1h",
            Bitcoin::OutputScriptsHash => b"h",
            Bitcoin::OutputValue => b"+1l",
            Bitcoin::OutputValuesHash => b"h",
            Bitcoin::OutputsHash => b"h",
            Bitcoin::ParseLock => b"+ii",
            Bitcoin::ParseSequence => b"+1+****22*22**22*22***22*22**22*22****22*22**22*22***22*22**22*22",
            Bitcoin::PointVerify1 => b"1",
            Bitcoin::RightExtend16_32 => b"i",
            Bitcoin::RightExtend16_64 => b"l",
            Bitcoin::RightExtend32_64 => b"l",
            Bitcoin::RightExtend8_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::RightExtend8_32 => b"i",
            Bitcoin::RightExtend8_64 => b"l",
            Bitcoin::RightPadHigh16_32 => b"i",
            Bitcoin::RightPadHigh16_64 => b"l",
            Bitcoin::RightPadHigh1_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::RightPadHigh1_32 => b"i",
            Bitcoin::RightPadHigh1_64 => b"l",
            Bitcoin::RightPadHigh1_8 => b"***22*22**22*22",
            Bitcoin::RightPadHigh32_64 => b"l",
            Bitcoin::RightPadHigh8_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::RightPadHigh8_32 => b"i",
            Bitcoin::RightPadHigh8_64 => b"l",
            Bitcoin::RightPadLow16_32 => b"i",
            Bitcoin::RightPadLow16_64 => b"l",
            Bitcoin::RightPadLow1_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::RightPadLow1_32 => b"i",
            Bitcoin::RightPadLow1_64 => b"l",
            Bitcoin::RightPadLow1_8 => b"***22*22**22*22",
            Bitcoin::RightPadLow32_64 => b"l",
            Bitcoin::RightPadLow8_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::RightPadLow8_32 => b"i",
            Bitcoin::RightPadLow8_64 => b"l",
            Bitcoin::RightRotate16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::RightRotate32 => b"i",
            Bitcoin::RightRotate64 => b"l",
            Bitcoin::RightRotate8 => b"***22*22**22*22",
            Bitcoin::RightShift16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::RightShift32 => b"i",
            Bitcoin::RightShift64 => b"l",
            Bitcoin::RightShift8 => b"***22*22**22*22",
            Bitcoin::RightShiftWith16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::RightShiftWith32 => b"i",
            Bitcoin::RightShiftWith64 => b"l",
            Bitcoin::RightShiftWith8 => b"***22*22**22*22",
            Bitcoin::Rightmost16_1 => b"2",
            Bitcoin::Rightmost16_2 => b"*22",
            Bitcoin::Rightmost16_4 => b"**22*22",
            Bitcoin::Rightmost16_8 => b"***22*22**22*22",
            Bitcoin::Rightmost32_1 => b"2",
            Bitcoin::Rightmost32_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Rightmost32_2 => b"*22",
            Bitcoin::Rightmost32_4 => b"**22*22",
            Bitcoin::Rightmost32_8 => b"***22*22**22*22",
            Bitcoin::Rightmost64_1 => b"2",
            Bitcoin::Rightmost64_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Rightmost64_2 => b"*22",
            Bitcoin::Rightmost64_32 => b"i",
            Bitcoin::Rightmost64_4 => b"**22*22",
            Bitcoin::Rightmost64_8 => b"***22*22**22*22",
            Bitcoin::Rightmost8_1 => b"2",
            Bitcoin::Rightmost8_2 => b"*22",
            Bitcoin::Rightmost8_4 => b"**22*22",
            Bitcoin::ScalarAdd => b"h",
            Bitcoin::ScalarInvert => b"h",
            Bitcoin::ScalarIsZero => b"2",
            Bitcoin::ScalarMultiply => b"h",
            Bitcoin::ScalarMultiplyLambda => b"h",
            Bitcoin::ScalarNegate => b"h",
            Bitcoin::ScalarNormalize => b"h",
            Bitcoin::ScalarSquare => b"h",
            Bitcoin::Scale => b"**hhh",
            Bitcoin::ScriptCMR => b"h",
            Bitcoin::Sha256Block => b"h",
            Bitcoin::Sha256Ctx8Add1 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::Sha256Ctx8Add128 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::Sha256Ctx8Add16 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::Sha256Ctx8Add2 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::Sha256Ctx8Add256 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::Sha256Ctx8Add32 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::Sha256Ctx8Add4 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::Sha256Ctx8Add512 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::Sha256Ctx8Add64 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::Sha256Ctx8Add8 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::Sha256Ctx8AddBuffer511 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::Sha256Ctx8Finalize => b"h",
            Bitcoin::Sha256Ctx8Init => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::Sha256Iv => b"h",
            Bitcoin::SigAllHash => b"h",
            Bitcoin::Some1 => b"2",
            Bitcoin::Some16 => b"2",
            Bitcoin::Some32 => b"2",
            Bitcoin::Some64 => b"2",
            Bitcoin::Some8 => b"2",
            Bitcoin::Subtract16 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::Subtract32 => b"*2i",
            Bitcoin::Subtract64 => b"*2l",
            Bitcoin::Subtract8 => b"*2***22*22**22*22",
            Bitcoin::Swu => b"*hh",
            Bitcoin::TapEnvHash => b"h",
            Bitcoin::TapdataInit => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::TapleafHash => b"h",
            Bitcoin::TapleafVersion => b"***22*22**22*22",
            Bitcoin::Tappath => b"+1h",
            Bitcoin::TappathHash => b"h",
            Bitcoin::TotalInputValue => b"l",
            Bitcoin::TotalOutputValue => b"l",
            Bitcoin::TransactionId => b"h",
            Bitcoin::TxHash => b"h",
            Bitcoin::TxIsFinal => b"2",
            Bitcoin::TxLockDistance => b"****22*22**22*22***22*22**22*22",
            Bitcoin::TxLockDuration => b"****22*22**22*22***22*22**22*22",
            Bitcoin::TxLockHeight => b"i",
            Bitcoin::TxLockTime => b"i",
            Bitcoin::Verify => b"1",
            Bitcoin::Version => b"i",
            Bitcoin::Xor1 => b"2",
            Bitcoin::Xor16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Xor32 => b"i",
            Bitcoin::Xor64 => b"l",
            Bitcoin::Xor8 => b"***22*22**22*22",
            Bitcoin::XorXor1 => b"2",
            Bitcoin::XorXor16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::XorXor32 => b"i",
            Bitcoin::XorXor64 => b"l",
            Bitcoin::XorXor8 => b"***22*22**22*22",
        };

        TypeName(name)
    }

    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> std::io::Result<usize> {
        let (n, len) = match self {
            Bitcoin::Verify => (0, 3),
            Bitcoin::Low1 => (8, 6),
            Bitcoin::Low8 => (37, 8),
            Bitcoin::Low16 => (304, 11),
            Bitcoin::Low32 => (305, 11),
            Bitcoin::Low64 => (306, 11),
            Bitcoin::High1 => (10, 6),
            Bitcoin::High8 => (45, 8),
            Bitcoin::High16 => (368, 11),
            Bitcoin::High32 => (369, 11),
            Bitcoin::High64 => (370, 11),
            Bitcoin::Complement1 => (96, 9),
            Bitcoin::Complement8 => (389, 11),
            Bitcoin::Complement16 => (3120, 14),
            Bitcoin::Complement32 => (3121, 14),
            Bitcoin::Complement64 => (3122, 14),
            Bitcoin::And1 => (98, 9),
            Bitcoin::And8 => (397, 11),
            Bitcoin::And16 => (3184, 14),
            Bitcoin::And32 => (3185, 14),
            Bitcoin::And64 => (3186, 14),
            Bitcoin::Or1 => (100, 9),
            Bitcoin::Or8 => (405, 11),
            Bitcoin::Or16 => (3248, 14),
            Bitcoin::Or32 => (3249, 14),
            Bitcoin::Or64 => (3250, 14),
            Bitcoin::Xor1 => (102, 9),
            Bitcoin::Xor8 => (413, 11),
            Bitcoin::Xor16 => (3312, 14),
            Bitcoin::Xor32 => (3313, 14),
            Bitcoin::Xor64 => (3314, 14),
            Bitcoin::Maj1 => (208, 10),
            Bitcoin::Maj8 => (837, 12),
            Bitcoin::Maj16 => (6704, 15),
            Bitcoin::Maj32 => (6705, 15),
            Bitcoin::Maj64 => (6706, 15),
            Bitcoin::XorXor1 => (210, 10),
            Bitcoin::XorXor8 => (845, 12),
            Bitcoin::XorXor16 => (6768, 15),
            Bitcoin::XorXor32 => (6769, 15),
            Bitcoin::XorXor64 => (6770, 15),
            Bitcoin::Ch1 => (212, 10),
            Bitcoin::Ch8 => (853, 12),
            Bitcoin::Ch16 => (6832, 15),
            Bitcoin::Ch32 => (6833, 15),
            Bitcoin::Ch64 => (6834, 15),
            Bitcoin::Some1 => (214, 10),
            Bitcoin::Some8 => (861, 12),
            Bitcoin::Some16 => (6896, 15),
            Bitcoin::Some32 => (6897, 15),
            Bitcoin::Some64 => (6898, 15),
            Bitcoin::All8 => (869, 12),
            Bitcoin::All16 => (6960, 15),
            Bitcoin::All32 => (6961, 15),
            Bitcoin::All64 => (6962, 15),
            Bitcoin::Eq1 => (218, 10),
            Bitcoin::Eq8 => (877, 12),
            Bitcoin::Eq16 => (7024, 15),
            Bitcoin::Eq32 => (7025, 15),
            Bitcoin::Eq64 => (7026, 15),
            Bitcoin::Eq256 => (14056, 16),
            Bitcoin::FullLeftShift8_1 => (1765, 13),
            Bitcoin::FullLeftShift16_1 => (14128, 16),
            Bitcoin::FullLeftShift32_1 => (14129, 16),
            Bitcoin::FullLeftShift64_1 => (14130, 16),
            Bitcoin::FullLeftShift8_2 => (7076, 15),
            Bitcoin::FullLeftShift16_2 => (7077, 15),
            Bitcoin::FullLeftShift32_2 => (56624, 18),
            Bitcoin::FullLeftShift64_2 => (56625, 18),
            Bitcoin::FullLeftShift8_4 => (1770, 13),
            Bitcoin::FullLeftShift16_4 => (7084, 15),
            Bitcoin::FullLeftShift32_4 => (7085, 15),
            Bitcoin::FullLeftShift64_4 => (56688, 18),
            Bitcoin::FullLeftShift16_8 => (14176, 16),
            Bitcoin::FullLeftShift32_8 => (56708, 18),
            Bitcoin::FullLeftShift64_8 => (56709, 18),
            Bitcoin::FullLeftShift32_16 => (14178, 16),
            Bitcoin::FullLeftShift64_16 => (56716, 18),
            Bitcoin::FullLeftShift64_32 => (14180, 16),
            Bitcoin::FullRightShift8_1 => (1781, 13),
            Bitcoin::FullRightShift16_1 => (14256, 16),
            Bitcoin::FullRightShift32_1 => (14257, 16),
            Bitcoin::FullRightShift64_1 => (14258, 16),
            Bitcoin::FullRightShift8_2 => (7140, 15),
            Bitcoin::FullRightShift16_2 => (7141, 15),
            Bitcoin::FullRightShift32_2 => (57136, 18),
            Bitcoin::FullRightShift64_2 => (57137, 18),
            Bitcoin::FullRightShift8_4 => (1786, 13),
            Bitcoin::FullRightShift16_4 => (7148, 15),
            Bitcoin::FullRightShift32_4 => (7149, 15),
            Bitcoin::FullRightShift64_4 => (57200, 18),
            Bitcoin::FullRightShift16_8 => (14304, 16),
            Bitcoin::FullRightShift32_8 => (57220, 18),
            Bitcoin::FullRightShift64_8 => (57221, 18),
            Bitcoin::FullRightShift32_16 => (14306, 16),
            Bitcoin::FullRightShift64_16 => (57228, 18),
            Bitcoin::FullRightShift64_32 => (14308, 16),
            Bitcoin::Leftmost8_1 => (28677, 17),
            Bitcoin::Leftmost16_1 => (229424, 20),
            Bitcoin::Leftmost32_1 => (229425, 20),
            Bitcoin::Leftmost64_1 => (229426, 20),
            Bitcoin::Leftmost8_2 => (114724, 19),
            Bitcoin::Leftmost16_2 => (114725, 19),
            Bitcoin::Leftmost32_2 => (917808, 22),
            Bitcoin::Leftmost64_2 => (917809, 22),
            Bitcoin::Leftmost8_4 => (28682, 17),
            Bitcoin::Leftmost16_4 => (114732, 19),
            Bitcoin::Leftmost32_4 => (114733, 19),
            Bitcoin::Leftmost64_4 => (917872, 22),
            Bitcoin::Leftmost16_8 => (229472, 20),
            Bitcoin::Leftmost32_8 => (917892, 22),
            Bitcoin::Leftmost64_8 => (917893, 22),
            Bitcoin::Leftmost32_16 => (229474, 20),
            Bitcoin::Leftmost64_16 => (917900, 22),
            Bitcoin::Leftmost64_32 => (229476, 20),
            Bitcoin::Rightmost8_1 => (28693, 17),
            Bitcoin::Rightmost16_1 => (229552, 20),
            Bitcoin::Rightmost32_1 => (229553, 20),
            Bitcoin::Rightmost64_1 => (229554, 20),
            Bitcoin::Rightmost8_2 => (114788, 19),
            Bitcoin::Rightmost16_2 => (114789, 19),
            Bitcoin::Rightmost32_2 => (918320, 22),
            Bitcoin::Rightmost64_2 => (918321, 22),
            Bitcoin::Rightmost8_4 => (28698, 17),
            Bitcoin::Rightmost16_4 => (114796, 19),
            Bitcoin::Rightmost32_4 => (114797, 19),
            Bitcoin::Rightmost64_4 => (918384, 22),
            Bitcoin::Rightmost16_8 => (229600, 20),
            Bitcoin::Rightmost32_8 => (918404, 22),
            Bitcoin::Rightmost64_8 => (918405, 22),
            Bitcoin::Rightmost32_16 => (229602, 20),
            Bitcoin::Rightmost64_16 => (918412, 22),
            Bitcoin::Rightmost64_32 => (229604, 20),
            Bitcoin::LeftPadLow1_8 => (28709, 17),
            Bitcoin::LeftPadLow1_16 => (229680, 20),
            Bitcoin::LeftPadLow1_32 => (229681, 20),
            Bitcoin::LeftPadLow1_64 => (229682, 20),
            Bitcoin::LeftPadLow8_16 => (229728, 20),
            Bitcoin::LeftPadLow8_32 => (918916, 22),
            Bitcoin::LeftPadLow8_64 => (918917, 22),
            Bitcoin::LeftPadLow16_32 => (229730, 20),
            Bitcoin::LeftPadLow16_64 => (918924, 22),
            Bitcoin::LeftPadLow32_64 => (229732, 20),
            Bitcoin::LeftPadHigh1_8 => (28725, 17),
            Bitcoin::LeftPadHigh1_16 => (229808, 20),
            Bitcoin::LeftPadHigh1_32 => (229809, 20),
            Bitcoin::LeftPadHigh1_64 => (229810, 20),
            Bitcoin::LeftPadHigh8_16 => (229856, 20),
            Bitcoin::LeftPadHigh8_32 => (919428, 22),
            Bitcoin::LeftPadHigh8_64 => (919429, 22),
            Bitcoin::LeftPadHigh16_32 => (229858, 20),
            Bitcoin::LeftPadHigh16_64 => (919436, 22),
            Bitcoin::LeftPadHigh32_64 => (229860, 20),
            Bitcoin::LeftExtend1_8 => (28741, 17),
            Bitcoin::LeftExtend1_16 => (229936, 20),
            Bitcoin::LeftExtend1_32 => (229937, 20),
            Bitcoin::LeftExtend1_64 => (229938, 20),
            Bitcoin::LeftExtend8_16 => (229984, 20),
            Bitcoin::LeftExtend8_32 => (919940, 22),
            Bitcoin::LeftExtend8_64 => (919941, 22),
            Bitcoin::LeftExtend16_32 => (229986, 20),
            Bitcoin::LeftExtend16_64 => (919948, 22),
            Bitcoin::LeftExtend32_64 => (229988, 20),
            Bitcoin::RightPadLow1_8 => (28757, 17),
            Bitcoin::RightPadLow1_16 => (230064, 20),
            Bitcoin::RightPadLow1_32 => (230065, 20),
            Bitcoin::RightPadLow1_64 => (230066, 20),
            Bitcoin::RightPadLow8_16 => (230112, 20),
            Bitcoin::RightPadLow8_32 => (920452, 22),
            Bitcoin::RightPadLow8_64 => (920453, 22),
            Bitcoin::RightPadLow16_32 => (230114, 20),
            Bitcoin::RightPadLow16_64 => (920460, 22),
            Bitcoin::RightPadLow32_64 => (230116, 20),
            Bitcoin::RightPadHigh1_8 => (28773, 17),
            Bitcoin::RightPadHigh1_16 => (230192, 20),
            Bitcoin::RightPadHigh1_32 => (230193, 20),
            Bitcoin::RightPadHigh1_64 => (230194, 20),
            Bitcoin::RightPadHigh8_16 => (230240, 20),
            Bitcoin::RightPadHigh8_32 => (920964, 22),
            Bitcoin::RightPadHigh8_64 => (920965, 22),
            Bitcoin::RightPadHigh16_32 => (230242, 20),
            Bitcoin::RightPadHigh16_64 => (920972, 22),
            Bitcoin::RightPadHigh32_64 => (230244, 20),
            Bitcoin::RightExtend8_16 => (230368, 20),
            Bitcoin::RightExtend8_32 => (921476, 22),
            Bitcoin::RightExtend8_64 => (921477, 22),
            Bitcoin::RightExtend16_32 => (230370, 20),
            Bitcoin::RightExtend16_64 => (921484, 22),
            Bitcoin::RightExtend32_64 => (230372, 20),
            Bitcoin::LeftShiftWith8 => (14405, 16),
            Bitcoin::LeftShiftWith16 => (115248, 19),
            Bitcoin::LeftShiftWith32 => (115249, 19),
            Bitcoin::LeftShiftWith64 => (115250, 19),
            Bitcoin::RightShiftWith8 => (14413, 16),
            Bitcoin::RightShiftWith16 => (115312, 19),
            Bitcoin::RightShiftWith32 => (115313, 19),
            Bitcoin::RightShiftWith64 => (115314, 19),
            Bitcoin::LeftShift8 => (14421, 16),
            Bitcoin::LeftShift16 => (115376, 19),
            Bitcoin::LeftShift32 => (115377, 19),
            Bitcoin::LeftShift64 => (115378, 19),
            Bitcoin::RightShift8 => (14429, 16),
            Bitcoin::RightShift16 => (115440, 19),
            Bitcoin::RightShift32 => (115441, 19),
            Bitcoin::RightShift64 => (115442, 19),
            Bitcoin::LeftRotate8 => (14437, 16),
            Bitcoin::LeftRotate16 => (115504, 19),
            Bitcoin::LeftRotate32 => (115505, 19),
            Bitcoin::LeftRotate64 => (115506, 19),
            Bitcoin::RightRotate8 => (14445, 16),
            Bitcoin::RightRotate16 => (115568, 19),
            Bitcoin::RightRotate32 => (115569, 19),
            Bitcoin::RightRotate64 => (115570, 19),
            Bitcoin::One8 => (69, 8),
            Bitcoin::One16 => (560, 11),
            Bitcoin::One32 => (561, 11),
            Bitcoin::One64 => (562, 11),
            Bitcoin::FullAdd8 => (293, 10),
            Bitcoin::FullAdd16 => (2352, 13),
            Bitcoin::FullAdd32 => (2353, 13),
            Bitcoin::FullAdd64 => (2354, 13),
            Bitcoin::Add8 => (301, 10),
            Bitcoin::Add16 => (2416, 13),
            Bitcoin::Add32 => (2417, 13),
            Bitcoin::Add64 => (2418, 13),
            Bitcoin::FullIncrement8 => (2437, 13),
            Bitcoin::FullIncrement16 => (19504, 16),
            Bitcoin::FullIncrement32 => (19505, 16),
            Bitcoin::FullIncrement64 => (19506, 16),
            Bitcoin::Increment8 => (2445, 13),
            Bitcoin::Increment16 => (19568, 16),
            Bitcoin::Increment32 => (19569, 16),
            Bitcoin::Increment64 => (19570, 16),
            Bitcoin::FullSubtract8 => (2461, 13),
            Bitcoin::FullSubtract16 => (19696, 16),
            Bitcoin::FullSubtract32 => (19697, 16),
            Bitcoin::FullSubtract64 => (19698, 16),
            Bitcoin::Subtract8 => (4933, 14),
            Bitcoin::Subtract16 => (39472, 17),
            Bitcoin::Subtract32 => (39473, 17),
            Bitcoin::Subtract64 => (39474, 17),
            Bitcoin::Negate8 => (4941, 14),
            Bitcoin::Negate16 => (39536, 17),
            Bitcoin::Negate32 => (39537, 17),
            Bitcoin::Negate64 => (39538, 17),
            Bitcoin::FullDecrement8 => (4949, 14),
            Bitcoin::FullDecrement16 => (39600, 17),
            Bitcoin::FullDecrement32 => (39601, 17),
            Bitcoin::FullDecrement64 => (39602, 17),
            Bitcoin::Decrement8 => (4957, 14),
            Bitcoin::Decrement16 => (39664, 17),
            Bitcoin::Decrement32 => (39665, 17),
            Bitcoin::Decrement64 => (39666, 17),
            Bitcoin::FullMultiply8 => (4965, 14),
            Bitcoin::FullMultiply16 => (39728, 17),
            Bitcoin::FullMultiply32 => (39729, 17),
            Bitcoin::FullMultiply64 => (39730, 17),
            Bitcoin::Multiply8 => (4973, 14),
            Bitcoin::Multiply16 => (39792, 17),
            Bitcoin::Multiply32 => (39793, 17),
            Bitcoin::Multiply64 => (39794, 17),
            Bitcoin::IsZero8 => (4981, 14),
            Bitcoin::IsZero16 => (39856, 17),
            Bitcoin::IsZero32 => (39857, 17),
            Bitcoin::IsZero64 => (39858, 17),
            Bitcoin::IsOne8 => (4989, 14),
            Bitcoin::IsOne16 => (39920, 17),
            Bitcoin::IsOne32 => (39921, 17),
            Bitcoin::IsOne64 => (39922, 17),
            Bitcoin::Le8 => (79877, 18),
            Bitcoin::Le16 => (639024, 21),
            Bitcoin::Le32 => (639025, 21),
            Bitcoin::Le64 => (639026, 21),
            Bitcoin::Lt8 => (79885, 18),
            Bitcoin::Lt16 => (639088, 21),
            Bitcoin::Lt32 => (639089, 21),
            Bitcoin::Lt64 => (639090, 21),
            Bitcoin::Min8 => (79893, 18),
            Bitcoin::Min16 => (639152, 21),
            Bitcoin::Min32 => (639153, 21),
            Bitcoin::Min64 => (639154, 21),
            Bitcoin::Max8 => (79901, 18),
            Bitcoin::Max16 => (639216, 21),
            Bitcoin::Max32 => (639217, 21),
            Bitcoin::Max64 => (639218, 21),
            Bitcoin::Median8 => (79909, 18),
            Bitcoin::Median16 => (639280, 21),
            Bitcoin::Median32 => (639281, 21),
            Bitcoin::Median64 => (639282, 21),
            Bitcoin::DivMod128_64 => (639346, 21),
            Bitcoin::DivMod8 => (79925, 18),
            Bitcoin::DivMod16 => (639408, 21),
            Bitcoin::DivMod32 => (639409, 21),
            Bitcoin::DivMod64 => (639410, 21),
            Bitcoin::Divide8 => (79933, 18),
            Bitcoin::Divide16 => (639472, 21),
            Bitcoin::Divide32 => (639473, 21),
            Bitcoin::Divide64 => (639474, 21),
            Bitcoin::Modulo8 => (79941, 18),
            Bitcoin::Modulo16 => (639536, 21),
            Bitcoin::Modulo32 => (639537, 21),
            Bitcoin::Modulo64 => (639538, 21),
            Bitcoin::Divides8 => (79949, 18),
            Bitcoin::Divides16 => (639600, 21),
            Bitcoin::Divides32 => (639601, 21),
            Bitcoin::Divides64 => (639602, 21),
            Bitcoin::Sha256Block => (20, 6),
            Bitcoin::Sha256Iv => (84, 8),
            Bitcoin::Sha256Ctx8Add1 => (170, 9),
            Bitcoin::Sha256Ctx8Add2 => (684, 11),
            Bitcoin::Sha256Ctx8Add4 => (685, 11),
            Bitcoin::Sha256Ctx8Add8 => (5488, 14),
            Bitcoin::Sha256Ctx8Add16 => (5489, 14),
            Bitcoin::Sha256Ctx8Add32 => (5490, 14),
            Bitcoin::Sha256Ctx8Add64 => (5491, 14),
            Bitcoin::Sha256Ctx8Add128 => (10984, 15),
            Bitcoin::Sha256Ctx8Add256 => (10985, 15),
            Bitcoin::Sha256Ctx8Add512 => (10986, 15),
            Bitcoin::Sha256Ctx8AddBuffer511 => (688, 11),
            Bitcoin::Sha256Ctx8Finalize => (689, 11),
            Bitcoin::Sha256Ctx8Init => (690, 11),
            Bitcoin::PointVerify1 => (192, 9),
            Bitcoin::Decompress => (388, 10),
            Bitcoin::LinearVerify1 => (778, 11),
            Bitcoin::LinearCombination1 => (6240, 14),
            Bitcoin::Scale => (3121, 13),
            Bitcoin::Generate => (3122, 13),
            Bitcoin::GejInfinity => (3123, 13),
            Bitcoin::GejNormalize => (6248, 14),
            Bitcoin::GejNegate => (6249, 14),
            Bitcoin::GeNegate => (6250, 14),
            Bitcoin::GejDouble => (6251, 14),
            Bitcoin::GejAdd => (6252, 14),
            Bitcoin::GejGeAddEx => (6253, 14),
            Bitcoin::GejGeAdd => (6254, 14),
            Bitcoin::GejRescale => (6255, 14),
            Bitcoin::GejIsInfinity => (100096, 18),
            Bitcoin::GejEquiv => (100097, 18),
            Bitcoin::GejGeEquiv => (100098, 18),
            Bitcoin::GejXEquiv => (100099, 18),
            Bitcoin::GejYIsOdd => (100100, 18),
            Bitcoin::GejIsOnCurve => (100101, 18),
            Bitcoin::GeIsOnCurve => (100102, 18),
            Bitcoin::ScalarNormalize => (100103, 18),
            Bitcoin::ScalarNegate => (100104, 18),
            Bitcoin::ScalarAdd => (100105, 18),
            Bitcoin::ScalarSquare => (100106, 18),
            Bitcoin::ScalarMultiply => (100107, 18),
            Bitcoin::ScalarMultiplyLambda => (100108, 18),
            Bitcoin::ScalarInvert => (100109, 18),
            Bitcoin::ScalarIsZero => (100110, 18),
            Bitcoin::FeNormalize => (200227, 19),
            Bitcoin::FeNegate => (200228, 19),
            Bitcoin::FeAdd => (200229, 19),
            Bitcoin::FeSquare => (200230, 19),
            Bitcoin::FeMultiply => (200231, 19),
            Bitcoin::FeMultiplyBeta => (200232, 19),
            Bitcoin::FeInvert => (200233, 19),
            Bitcoin::FeSquareRoot => (200234, 19),
            Bitcoin::FeIsZero => (200235, 19),
            Bitcoin::FeIsOdd => (200236, 19),
            Bitcoin::HashToCurve => (200238, 19),
            Bitcoin::Swu => (200239, 19),
            Bitcoin::CheckSigVerify => (98, 8),
            Bitcoin::Bip0340Verify => (396, 10),
            Bitcoin::ParseLock => (102, 8),
            Bitcoin::ParseSequence => (412, 10),
            Bitcoin::TapdataInit => (413, 10),
            Bitcoin::SigAllHash => (4, 3),
            Bitcoin::TxHash => (20, 5),
            Bitcoin::TapEnvHash => (21, 5),
            Bitcoin::OutputsHash => (176, 8),
            Bitcoin::InputsHash => (177, 8),
            Bitcoin::InputUtxosHash => (178, 8),
            Bitcoin::OutputHash => (179, 8),
            Bitcoin::OutputValuesHash => (360, 9),
            Bitcoin::OutputScriptsHash => (361, 9),
            Bitcoin::InputHash => (362, 9),
            Bitcoin::InputOutpointsHash => (363, 9),
            Bitcoin::InputSequencesHash => (364, 9),
            Bitcoin::InputAnnexesHash => (365, 9),
            Bitcoin::InputScriptSigsHash => (366, 9),
            Bitcoin::InputUtxoHash => (367, 9),
            Bitcoin::InputValuesHash => (5888, 13),
            Bitcoin::InputScriptsHash => (5889, 13),
            Bitcoin::TapleafHash => (5890, 13),
            Bitcoin::TappathHash => (5891, 13),
            Bitcoin::OutpointHash => (5892, 13),
            Bitcoin::AnnexHash => (5893, 13),
            Bitcoin::BuildTapleafSimplicity => (5894, 13),
            Bitcoin::BuildTapbranch => (5895, 13),
            Bitcoin::BuildTaptweak => (5896, 13),
            Bitcoin::CheckLockHeight => (24, 5),
            Bitcoin::CheckLockTime => (100, 7),
            Bitcoin::CheckLockDistance => (101, 7),
            Bitcoin::CheckLockDuration => (816, 10),
            Bitcoin::TxLockHeight => (817, 10),
            Bitcoin::TxLockTime => (818, 10),
            Bitcoin::TxLockDistance => (819, 10),
            Bitcoin::TxLockDuration => (1640, 11),
            Bitcoin::TxIsFinal => (1641, 11),
            Bitcoin::ScriptCMR => (26, 5),
            Bitcoin::InternalKey => (108, 7),
            Bitcoin::CurrentIndex => (109, 7),
            Bitcoin::NumInputs => (880, 10),
            Bitcoin::NumOutputs => (881, 10),
            Bitcoin::LockTime => (882, 10),
            Bitcoin::Fee => (883, 10),
            Bitcoin::OutputValue => (1768, 11),
            Bitcoin::OutputScriptHash => (1769, 11),
            Bitcoin::TotalOutputValue => (1770, 11),
            Bitcoin::CurrentPrevOutpoint => (1771, 11),
            Bitcoin::CurrentValue => (1772, 11),
            Bitcoin::CurrentScriptHash => (1773, 11),
            Bitcoin::CurrentSequence => (1774, 11),
            Bitcoin::CurrentAnnexHash => (1775, 11),
            Bitcoin::CurrentScriptSigHash => (28416, 15),
            Bitcoin::InputPrevOutpoint => (28417, 15),
            Bitcoin::InputValue => (28418, 15),
            Bitcoin::InputScriptHash => (28419, 15),
            Bitcoin::InputSequence => (28420, 15),
            Bitcoin::InputAnnexHash => (28421, 15),
            Bitcoin::InputScriptSigHash => (28422, 15),
            Bitcoin::TotalInputValue => (28423, 15),
            Bitcoin::TapleafVersion => (28424, 15),
            Bitcoin::Tappath => (28425, 15),
            Bitcoin::Version => (28426, 15),
            Bitcoin::TransactionId => (28427, 15),
        };

        w.write_bits_be(n, len)
    }

    fn decode<I: Iterator<Item = u8>>(bits: &mut BitIter<I>) -> Result<Self, decode::Error> {
        decode_bits!(bits, {
            0 => {
                0 => {
                    0 => {Bitcoin::Verify},
                    1 => {
                        0 => {
                            0 => {
                                0 => {Bitcoin::Low1},
                                1 => {
                                    0 => {
                                        0 => {},
                                        1 => {Bitcoin::Low8}
                                    },
                                    1 => {
                                        0 => {
                                            0 => {
                                                0 => {
                                                    0 => {Bitcoin::Low16},
                                                    1 => {Bitcoin::Low32}
                                                },
                                                1 => {
                                                    0 => {Bitcoin::Low64},
                                                    1 => {}
                                                }
                                            },
                                            1 => {}
                                        },
                                        1 => {}
                                    }
                                }
                            },
                            1 => {
                                0 => {Bitcoin::High1},
                                1 => {
                                    0 => {
                                        0 => {},
                                        1 => {Bitcoin::High8}
                                    },
                                    1 => {
                                        0 => {
                                            0 => {
                                                0 => {
                                                    0 => {Bitcoin::High16},
                                                    1 => {Bitcoin::High32}
                                                },
                                                1 => {
                                                    0 => {Bitcoin::High64},
                                                    1 => {}
                                                }
                                            },
                                            1 => {}
                                        },
                                        1 => {}
                                    }
                                }
                            }
                        },
                        1 => {
                            0 => {
                                0 => {
                                    0 => {
                                        0 => {
                                            0 => {Bitcoin::Complement1},
                                            1 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {Bitcoin::Complement8}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Bitcoin::Complement16},
                                                                1 => {Bitcoin::Complement32}
                                                            },
                                                            1 => {
                                                                0 => {Bitcoin::Complement64},
                                                                1 => {}
                                                            }
                                                        },
                                                        1 => {}
                                                    },
                                                    1 => {}
                                                }
                                            }
                                        },
                                        1 => {
                                            0 => {Bitcoin::And1},
                                            1 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {Bitcoin::And8}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Bitcoin::And16},
                                                                1 => {Bitcoin::And32}
                                                            },
                                                            1 => {
                                                                0 => {Bitcoin::And64},
                                                                1 => {}
                                                            }
                                                        },
                                                        1 => {}
                                                    },
                                                    1 => {}
                                                }
                                            }
                                        }
                                    },
                                    1 => {
                                        0 => {
                                            0 => {Bitcoin::Or1},
                                            1 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {Bitcoin::Or8}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Bitcoin::Or16},
                                                                1 => {Bitcoin::Or32}
                                                            },
                                                            1 => {
                                                                0 => {Bitcoin::Or64},
                                                                1 => {}
                                                            }
                                                        },
                                                        1 => {}
                                                    },
                                                    1 => {}
                                                }
                                            }
                                        },
                                        1 => {
                                            0 => {Bitcoin::Xor1},
                                            1 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {Bitcoin::Xor8}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Bitcoin::Xor16},
                                                                1 => {Bitcoin::Xor32}
                                                            },
                                                            1 => {
                                                                0 => {Bitcoin::Xor64},
                                                                1 => {}
                                                            }
                                                        },
                                                        1 => {}
                                                    },
                                                    1 => {}
                                                }
                                            }
                                        }
                                    }
                                },
                                1 => {
                                    0 => {
                                        0 => {
                                            0 => {
                                                0 => {Bitcoin::Maj1},
                                                1 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {Bitcoin::Maj8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Bitcoin::Maj16},
                                                                    1 => {Bitcoin::Maj32}
                                                                },
                                                                1 => {
                                                                    0 => {Bitcoin::Maj64},
                                                                    1 => {}
                                                                }
                                                            },
                                                            1 => {}
                                                        },
                                                        1 => {}
                                                    }
                                                }
                                            },
                                            1 => {
                                                0 => {Bitcoin::XorXor1},
                                                1 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {Bitcoin::XorXor8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Bitcoin::XorXor16},
                                                                    1 => {Bitcoin::XorXor32}
                                                                },
                                                                1 => {
                                                                    0 => {Bitcoin::XorXor64},
                                                                    1 => {}
                                                                }
                                                            },
                                                            1 => {}
                                                        },
                                                        1 => {}
                                                    }
                                                }
                                            }
                                        },
                                        1 => {
                                            0 => {
                                                0 => {Bitcoin::Ch1},
                                                1 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {Bitcoin::Ch8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Bitcoin::Ch16},
                                                                    1 => {Bitcoin::Ch32}
                                                                },
                                                                1 => {
                                                                    0 => {Bitcoin::Ch64},
                                                                    1 => {}
                                                                }
                                                            },
                                                            1 => {}
                                                        },
                                                        1 => {}
                                                    }
                                                }
                                            },
                                            1 => {
                                                0 => {Bitcoin::Some1},
                                                1 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {Bitcoin::Some8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Bitcoin::Some16},
                                                                    1 => {Bitcoin::Some32}
                                                                },
                                                                1 => {
                                                                    0 => {Bitcoin::Some64},
                                                                    1 => {}
                                                                }
                                                            },
                                                            1 => {}
                                                        },
                                                        1 => {}
                                                    }
                                                }
                                            }
                                        }
                                    },
                                    1 => {
                                        0 => {
                                            0 => {
                                                0 => {},
                                                1 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {Bitcoin::All8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Bitcoin::All16},
                                                                    1 => {Bitcoin::All32}
                                                                },
                                                                1 => {
                                                                    0 => {Bitcoin::All64},
                                                                    1 => {}
                                                                }
                                                            },
                                                            1 => {}
                                                        },
                                                        1 => {}
                                                    }
                                                }
                                            },
                                            1 => {
                                                0 => {Bitcoin::Eq1},
                                                1 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {Bitcoin::Eq8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Bitcoin::Eq16},
                                                                    1 => {Bitcoin::Eq32}
                                                                },
                                                                1 => {
                                                                    0 => {Bitcoin::Eq64},
                                                                    1 => {}
                                                                }
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Bitcoin::Eq256},
                                                                        1 => {}
                                                                    },
                                                                    1 => {}
                                                                },
                                                                1 => {}
                                                            }
                                                        },
                                                        1 => {}
                                                    }
                                                }
                                            }
                                        },
                                        1 => {
                                            0 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {
                                                        0 => {
                                                            0 => {},
                                                            1 => {Bitcoin::FullLeftShift8_1}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Bitcoin::FullLeftShift16_1},
                                                                        1 => {Bitcoin::FullLeftShift32_1}
                                                                    },
                                                                    1 => {
                                                                        0 => {Bitcoin::FullLeftShift64_1},
                                                                        1 => {}
                                                                    }
                                                                },
                                                                1 => {}
                                                            },
                                                            1 => {}
                                                        }
                                                    }
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {},
                                                            1 => {
                                                                0 => {
                                                                    0 => {Bitcoin::FullLeftShift8_2},
                                                                    1 => {Bitcoin::FullLeftShift16_2}
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Bitcoin::FullLeftShift32_2},
                                                                                1 => {Bitcoin::FullLeftShift64_2}
                                                                            },
                                                                            1 => {}
                                                                        },
                                                                        1 => {}
                                                                    },
                                                                    1 => {}
                                                                }
                                                            }
                                                        },
                                                        1 => {
                                                            0 => {Bitcoin::FullLeftShift8_4},
                                                            1 => {
                                                                0 => {
                                                                    0 => {Bitcoin::FullLeftShift16_4},
                                                                    1 => {Bitcoin::FullLeftShift32_4}
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Bitcoin::FullLeftShift64_4},
                                                                                1 => {}
                                                                            },
                                                                            1 => {}
                                                                        },
                                                                        1 => {}
                                                                    },
                                                                    1 => {}
                                                                }
                                                            }
                                                        }
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Bitcoin::FullLeftShift16_8},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Bitcoin::FullLeftShift32_8},
                                                                                1 => {Bitcoin::FullLeftShift64_8}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {Bitcoin::FullLeftShift32_16},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Bitcoin::FullLeftShift64_16},
                                                                                1 => {}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {Bitcoin::FullLeftShift64_32},
                                                                        1 => {}
                                                                    },
                                                                    1 => {}
                                                                }
                                                            },
                                                            1 => {}
                                                        },
                                                        1 => {}
                                                    }
                                                }
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {
                                                        0 => {
                                                            0 => {},
                                                            1 => {Bitcoin::FullRightShift8_1}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Bitcoin::FullRightShift16_1},
                                                                        1 => {Bitcoin::FullRightShift32_1}
                                                                    },
                                                                    1 => {
                                                                        0 => {Bitcoin::FullRightShift64_1},
                                                                        1 => {}
                                                                    }
                                                                },
                                                                1 => {}
                                                            },
                                                            1 => {}
                                                        }
                                                    }
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {},
                                                            1 => {
                                                                0 => {
                                                                    0 => {Bitcoin::FullRightShift8_2},
                                                                    1 => {Bitcoin::FullRightShift16_2}
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Bitcoin::FullRightShift32_2},
                                                                                1 => {Bitcoin::FullRightShift64_2}
                                                                            },
                                                                            1 => {}
                                                                        },
                                                                        1 => {}
                                                                    },
                                                                    1 => {}
                                                                }
                                                            }
                                                        },
                                                        1 => {
                                                            0 => {Bitcoin::FullRightShift8_4},
                                                            1 => {
                                                                0 => {
                                                                    0 => {Bitcoin::FullRightShift16_4},
                                                                    1 => {Bitcoin::FullRightShift32_4}
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Bitcoin::FullRightShift64_4},
                                                                                1 => {}
                                                                            },
                                                                            1 => {}
                                                                        },
                                                                        1 => {}
                                                                    },
                                                                    1 => {}
                                                                }
                                                            }
                                                        }
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Bitcoin::FullRightShift16_8},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Bitcoin::FullRightShift32_8},
                                                                                1 => {Bitcoin::FullRightShift64_8}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {Bitcoin::FullRightShift32_16},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Bitcoin::FullRightShift64_16},
                                                                                1 => {}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {Bitcoin::FullRightShift64_32},
                                                                        1 => {}
                                                                    },
                                                                    1 => {}
                                                                }
                                                            },
                                                            1 => {}
                                                        },
                                                        1 => {}
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            1 => {
                                0 => {
                                    0 => {
                                        0 => {
                                            0 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {},
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {},
                                                                            1 => {Bitcoin::Leftmost8_1}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::Leftmost16_1},
                                                                                        1 => {Bitcoin::Leftmost32_1}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::Leftmost64_1},
                                                                                        1 => {}
                                                                                    }
                                                                                },
                                                                                1 => {}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {},
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Bitcoin::Leftmost8_2},
                                                                                    1 => {Bitcoin::Leftmost16_2}
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::Leftmost32_2},
                                                                                                1 => {Bitcoin::Leftmost64_2}
                                                                                            },
                                                                                            1 => {}
                                                                                        },
                                                                                        1 => {}
                                                                                    },
                                                                                    1 => {}
                                                                                }
                                                                            }
                                                                        },
                                                                        1 => {
                                                                            0 => {Bitcoin::Leftmost8_4},
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Bitcoin::Leftmost16_4},
                                                                                    1 => {Bitcoin::Leftmost32_4}
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::Leftmost64_4},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        },
                                                                                        1 => {}
                                                                                    },
                                                                                    1 => {}
                                                                                }
                                                                            }
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::Leftmost16_8},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::Leftmost32_8},
                                                                                                1 => {Bitcoin::Leftmost64_8}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::Leftmost32_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::Leftmost64_16},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::Leftmost64_32},
                                                                                        1 => {}
                                                                                    },
                                                                                    1 => {}
                                                                                }
                                                                            },
                                                                            1 => {}
                                                                        },
                                                                        1 => {}
                                                                    }
                                                                }
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {},
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {},
                                                                            1 => {Bitcoin::Rightmost8_1}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::Rightmost16_1},
                                                                                        1 => {Bitcoin::Rightmost32_1}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::Rightmost64_1},
                                                                                        1 => {}
                                                                                    }
                                                                                },
                                                                                1 => {}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {},
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Bitcoin::Rightmost8_2},
                                                                                    1 => {Bitcoin::Rightmost16_2}
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::Rightmost32_2},
                                                                                                1 => {Bitcoin::Rightmost64_2}
                                                                                            },
                                                                                            1 => {}
                                                                                        },
                                                                                        1 => {}
                                                                                    },
                                                                                    1 => {}
                                                                                }
                                                                            }
                                                                        },
                                                                        1 => {
                                                                            0 => {Bitcoin::Rightmost8_4},
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Bitcoin::Rightmost16_4},
                                                                                    1 => {Bitcoin::Rightmost32_4}
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::Rightmost64_4},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        },
                                                                                        1 => {}
                                                                                    },
                                                                                    1 => {}
                                                                                }
                                                                            }
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::Rightmost16_8},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::Rightmost32_8},
                                                                                                1 => {Bitcoin::Rightmost64_8}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::Rightmost32_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::Rightmost64_16},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::Rightmost64_32},
                                                                                        1 => {}
                                                                                    },
                                                                                    1 => {}
                                                                                }
                                                                            },
                                                                            1 => {}
                                                                        },
                                                                        1 => {}
                                                                    }
                                                                }
                                                            }
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {},
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {},
                                                                            1 => {Bitcoin::LeftPadLow1_8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::LeftPadLow1_16},
                                                                                        1 => {Bitcoin::LeftPadLow1_32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::LeftPadLow1_64},
                                                                                        1 => {}
                                                                                    }
                                                                                },
                                                                                1 => {}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {},
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::LeftPadLow8_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::LeftPadLow8_32},
                                                                                                1 => {Bitcoin::LeftPadLow8_64}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::LeftPadLow16_32},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::LeftPadLow16_64},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::LeftPadLow32_64},
                                                                                        1 => {}
                                                                                    },
                                                                                    1 => {}
                                                                                }
                                                                            },
                                                                            1 => {}
                                                                        },
                                                                        1 => {}
                                                                    }
                                                                }
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {},
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {},
                                                                            1 => {Bitcoin::LeftPadHigh1_8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::LeftPadHigh1_16},
                                                                                        1 => {Bitcoin::LeftPadHigh1_32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::LeftPadHigh1_64},
                                                                                        1 => {}
                                                                                    }
                                                                                },
                                                                                1 => {}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {},
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::LeftPadHigh8_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::LeftPadHigh8_32},
                                                                                                1 => {Bitcoin::LeftPadHigh8_64}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::LeftPadHigh16_32},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::LeftPadHigh16_64},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::LeftPadHigh32_64},
                                                                                        1 => {}
                                                                                    },
                                                                                    1 => {}
                                                                                }
                                                                            },
                                                                            1 => {}
                                                                        },
                                                                        1 => {}
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {},
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {},
                                                                            1 => {Bitcoin::LeftExtend1_8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::LeftExtend1_16},
                                                                                        1 => {Bitcoin::LeftExtend1_32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::LeftExtend1_64},
                                                                                        1 => {}
                                                                                    }
                                                                                },
                                                                                1 => {}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {},
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::LeftExtend8_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::LeftExtend8_32},
                                                                                                1 => {Bitcoin::LeftExtend8_64}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::LeftExtend16_32},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::LeftExtend16_64},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::LeftExtend32_64},
                                                                                        1 => {}
                                                                                    },
                                                                                    1 => {}
                                                                                }
                                                                            },
                                                                            1 => {}
                                                                        },
                                                                        1 => {}
                                                                    }
                                                                }
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {},
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {},
                                                                            1 => {Bitcoin::RightPadLow1_8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::RightPadLow1_16},
                                                                                        1 => {Bitcoin::RightPadLow1_32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::RightPadLow1_64},
                                                                                        1 => {}
                                                                                    }
                                                                                },
                                                                                1 => {}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {},
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::RightPadLow8_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::RightPadLow8_32},
                                                                                                1 => {Bitcoin::RightPadLow8_64}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::RightPadLow16_32},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::RightPadLow16_64},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::RightPadLow32_64},
                                                                                        1 => {}
                                                                                    },
                                                                                    1 => {}
                                                                                }
                                                                            },
                                                                            1 => {}
                                                                        },
                                                                        1 => {}
                                                                    }
                                                                }
                                                            }
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {},
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {},
                                                                            1 => {Bitcoin::RightPadHigh1_8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::RightPadHigh1_16},
                                                                                        1 => {Bitcoin::RightPadHigh1_32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::RightPadHigh1_64},
                                                                                        1 => {}
                                                                                    }
                                                                                },
                                                                                1 => {}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {},
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::RightPadHigh8_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::RightPadHigh8_32},
                                                                                                1 => {Bitcoin::RightPadHigh8_64}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::RightPadHigh16_32},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::RightPadHigh16_64},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::RightPadHigh32_64},
                                                                                        1 => {}
                                                                                    },
                                                                                    1 => {}
                                                                                }
                                                                            },
                                                                            1 => {}
                                                                        },
                                                                        1 => {}
                                                                    }
                                                                }
                                                            },
                                                            1 => {
                                                                0 => {},
                                                                1 => {
                                                                    0 => {},
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::RightExtend8_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::RightExtend8_32},
                                                                                                1 => {Bitcoin::RightExtend8_64}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::RightExtend16_32},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::RightExtend16_64},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::RightExtend32_64},
                                                                                        1 => {}
                                                                                    },
                                                                                    1 => {}
                                                                                }
                                                                            },
                                                                            1 => {}
                                                                        },
                                                                        1 => {}
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {},
                                                                1 => {
                                                                    0 => {
                                                                        0 => {},
                                                                        1 => {Bitcoin::LeftShiftWith8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Bitcoin::LeftShiftWith16},
                                                                                    1 => {Bitcoin::LeftShiftWith32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Bitcoin::LeftShiftWith64},
                                                                                    1 => {}
                                                                                }
                                                                            },
                                                                            1 => {}
                                                                        },
                                                                        1 => {}
                                                                    }
                                                                }
                                                            },
                                                            1 => {
                                                                0 => {},
                                                                1 => {
                                                                    0 => {
                                                                        0 => {},
                                                                        1 => {Bitcoin::RightShiftWith8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Bitcoin::RightShiftWith16},
                                                                                    1 => {Bitcoin::RightShiftWith32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Bitcoin::RightShiftWith64},
                                                                                    1 => {}
                                                                                }
                                                                            },
                                                                            1 => {}
                                                                        },
                                                                        1 => {}
                                                                    }
                                                                }
                                                            }
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {},
                                                                1 => {
                                                                    0 => {
                                                                        0 => {},
                                                                        1 => {Bitcoin::LeftShift8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Bitcoin::LeftShift16},
                                                                                    1 => {Bitcoin::LeftShift32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Bitcoin::LeftShift64},
                                                                                    1 => {}
                                                                                }
                                                                            },
                                                                            1 => {}
                                                                        },
                                                                        1 => {}
                                                                    }
                                                                }
                                                            },
                                                            1 => {
                                                                0 => {},
                                                                1 => {
                                                                    0 => {
                                                                        0 => {},
                                                                        1 => {Bitcoin::RightShift8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Bitcoin::RightShift16},
                                                                                    1 => {Bitcoin::RightShift32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Bitcoin::RightShift64},
                                                                                    1 => {}
                                                                                }
                                                                            },
                                                                            1 => {}
                                                                        },
                                                                        1 => {}
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {},
                                                                1 => {
                                                                    0 => {
                                                                        0 => {},
                                                                        1 => {Bitcoin::LeftRotate8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Bitcoin::LeftRotate16},
                                                                                    1 => {Bitcoin::LeftRotate32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Bitcoin::LeftRotate64},
                                                                                    1 => {}
                                                                                }
                                                                            },
                                                                            1 => {}
                                                                        },
                                                                        1 => {}
                                                                    }
                                                                }
                                                            },
                                                            1 => {
                                                                0 => {},
                                                                1 => {
                                                                    0 => {
                                                                        0 => {},
                                                                        1 => {Bitcoin::RightRotate8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Bitcoin::RightRotate16},
                                                                                    1 => {Bitcoin::RightRotate32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Bitcoin::RightRotate64},
                                                                                    1 => {}
                                                                                }
                                                                            },
                                                                            1 => {}
                                                                        },
                                                                        1 => {}
                                                                    }
                                                                }
                                                            }
                                                        },
                                                        1 => {}
                                                    }
                                                }
                                            },
                                            1 => {}
                                        },
                                        1 => {}
                                    },
                                    1 => {}
                                },
                                1 => {}
                            }
                        }
                    }
                },
                1 => {
                    0 => {
                        0 => {
                            0 => {
                                0 => {},
                                1 => {
                                    0 => {
                                        0 => {},
                                        1 => {Bitcoin::One8}
                                    },
                                    1 => {
                                        0 => {
                                            0 => {
                                                0 => {
                                                    0 => {Bitcoin::One16},
                                                    1 => {Bitcoin::One32}
                                                },
                                                1 => {
                                                    0 => {Bitcoin::One64},
                                                    1 => {}
                                                }
                                            },
                                            1 => {}
                                        },
                                        1 => {}
                                    }
                                }
                            },
                            1 => {
                                0 => {
                                    0 => {
                                        0 => {},
                                        1 => {
                                            0 => {
                                                0 => {},
                                                1 => {Bitcoin::FullAdd8}
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Bitcoin::FullAdd16},
                                                            1 => {Bitcoin::FullAdd32}
                                                        },
                                                        1 => {
                                                            0 => {Bitcoin::FullAdd64},
                                                            1 => {}
                                                        }
                                                    },
                                                    1 => {}
                                                },
                                                1 => {}
                                            }
                                        }
                                    },
                                    1 => {
                                        0 => {},
                                        1 => {
                                            0 => {
                                                0 => {},
                                                1 => {Bitcoin::Add8}
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Bitcoin::Add16},
                                                            1 => {Bitcoin::Add32}
                                                        },
                                                        1 => {
                                                            0 => {Bitcoin::Add64},
                                                            1 => {}
                                                        }
                                                    },
                                                    1 => {}
                                                },
                                                1 => {}
                                            }
                                        }
                                    }
                                },
                                1 => {
                                    0 => {
                                        0 => {
                                            0 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {
                                                        0 => {
                                                            0 => {},
                                                            1 => {Bitcoin::FullIncrement8}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Bitcoin::FullIncrement16},
                                                                        1 => {Bitcoin::FullIncrement32}
                                                                    },
                                                                    1 => {
                                                                        0 => {Bitcoin::FullIncrement64},
                                                                        1 => {}
                                                                    }
                                                                },
                                                                1 => {}
                                                            },
                                                            1 => {}
                                                        }
                                                    }
                                                },
                                                1 => {
                                                    0 => {},
                                                    1 => {
                                                        0 => {
                                                            0 => {},
                                                            1 => {Bitcoin::Increment8}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Bitcoin::Increment16},
                                                                        1 => {Bitcoin::Increment32}
                                                                    },
                                                                    1 => {
                                                                        0 => {Bitcoin::Increment64},
                                                                        1 => {}
                                                                    }
                                                                },
                                                                1 => {}
                                                            },
                                                            1 => {}
                                                        }
                                                    }
                                                }
                                            },
                                            1 => {
                                                0 => {},
                                                1 => {
                                                    0 => {},
                                                    1 => {
                                                        0 => {
                                                            0 => {},
                                                            1 => {Bitcoin::FullSubtract8}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Bitcoin::FullSubtract16},
                                                                        1 => {Bitcoin::FullSubtract32}
                                                                    },
                                                                    1 => {
                                                                        0 => {Bitcoin::FullSubtract64},
                                                                        1 => {}
                                                                    }
                                                                },
                                                                1 => {}
                                                            },
                                                            1 => {}
                                                        }
                                                    }
                                                }
                                            }
                                        },
                                        1 => {
                                            0 => {
                                                0 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {
                                                            0 => {
                                                                0 => {},
                                                                1 => {Bitcoin::Subtract8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Bitcoin::Subtract16},
                                                                            1 => {Bitcoin::Subtract32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Bitcoin::Subtract64},
                                                                            1 => {}
                                                                        }
                                                                    },
                                                                    1 => {}
                                                                },
                                                                1 => {}
                                                            }
                                                        }
                                                    },
                                                    1 => {
                                                        0 => {},
                                                        1 => {
                                                            0 => {
                                                                0 => {},
                                                                1 => {Bitcoin::Negate8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Bitcoin::Negate16},
                                                                            1 => {Bitcoin::Negate32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Bitcoin::Negate64},
                                                                            1 => {}
                                                                        }
                                                                    },
                                                                    1 => {}
                                                                },
                                                                1 => {}
                                                            }
                                                        }
                                                    }
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {
                                                            0 => {
                                                                0 => {},
                                                                1 => {Bitcoin::FullDecrement8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Bitcoin::FullDecrement16},
                                                                            1 => {Bitcoin::FullDecrement32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Bitcoin::FullDecrement64},
                                                                            1 => {}
                                                                        }
                                                                    },
                                                                    1 => {}
                                                                },
                                                                1 => {}
                                                            }
                                                        }
                                                    },
                                                    1 => {
                                                        0 => {},
                                                        1 => {
                                                            0 => {
                                                                0 => {},
                                                                1 => {Bitcoin::Decrement8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Bitcoin::Decrement16},
                                                                            1 => {Bitcoin::Decrement32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Bitcoin::Decrement64},
                                                                            1 => {}
                                                                        }
                                                                    },
                                                                    1 => {}
                                                                },
                                                                1 => {}
                                                            }
                                                        }
                                                    }
                                                }
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {
                                                            0 => {
                                                                0 => {},
                                                                1 => {Bitcoin::FullMultiply8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Bitcoin::FullMultiply16},
                                                                            1 => {Bitcoin::FullMultiply32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Bitcoin::FullMultiply64},
                                                                            1 => {}
                                                                        }
                                                                    },
                                                                    1 => {}
                                                                },
                                                                1 => {}
                                                            }
                                                        }
                                                    },
                                                    1 => {
                                                        0 => {},
                                                        1 => {
                                                            0 => {
                                                                0 => {},
                                                                1 => {Bitcoin::Multiply8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Bitcoin::Multiply16},
                                                                            1 => {Bitcoin::Multiply32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Bitcoin::Multiply64},
                                                                            1 => {}
                                                                        }
                                                                    },
                                                                    1 => {}
                                                                },
                                                                1 => {}
                                                            }
                                                        }
                                                    }
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {
                                                            0 => {
                                                                0 => {},
                                                                1 => {Bitcoin::IsZero8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Bitcoin::IsZero16},
                                                                            1 => {Bitcoin::IsZero32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Bitcoin::IsZero64},
                                                                            1 => {}
                                                                        }
                                                                    },
                                                                    1 => {}
                                                                },
                                                                1 => {}
                                                            }
                                                        }
                                                    },
                                                    1 => {
                                                        0 => {},
                                                        1 => {
                                                            0 => {
                                                                0 => {},
                                                                1 => {Bitcoin::IsOne8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Bitcoin::IsOne16},
                                                                            1 => {Bitcoin::IsOne32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Bitcoin::IsOne64},
                                                                            1 => {}
                                                                        }
                                                                    },
                                                                    1 => {}
                                                                },
                                                                1 => {}
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    },
                                    1 => {
                                        0 => {
                                            0 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {},
                                                                                1 => {Bitcoin::Le8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Bitcoin::Le16},
                                                                                            1 => {Bitcoin::Le32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Bitcoin::Le64},
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {}
                                                                                },
                                                                                1 => {}
                                                                            }
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {},
                                                                                1 => {Bitcoin::Lt8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Bitcoin::Lt16},
                                                                                            1 => {Bitcoin::Lt32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Bitcoin::Lt64},
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {}
                                                                                },
                                                                                1 => {}
                                                                            }
                                                                        }
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {},
                                                                                1 => {Bitcoin::Min8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Bitcoin::Min16},
                                                                                            1 => {Bitcoin::Min32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Bitcoin::Min64},
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {}
                                                                                },
                                                                                1 => {}
                                                                            }
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {},
                                                                                1 => {Bitcoin::Max8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Bitcoin::Max16},
                                                                                            1 => {Bitcoin::Max32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Bitcoin::Max64},
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {}
                                                                                },
                                                                                1 => {}
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {},
                                                                                1 => {Bitcoin::Median8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Bitcoin::Median16},
                                                                                            1 => {Bitcoin::Median32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Bitcoin::Median64},
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {}
                                                                                },
                                                                                1 => {}
                                                                            }
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {},
                                                                        1 => {
                                                                            0 => {},
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {},
                                                                                        1 => {
                                                                                            0 => {Bitcoin::DivMod128_64},
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {}
                                                                                },
                                                                                1 => {}
                                                                            }
                                                                        }
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {},
                                                                                1 => {Bitcoin::DivMod8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Bitcoin::DivMod16},
                                                                                            1 => {Bitcoin::DivMod32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Bitcoin::DivMod64},
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {}
                                                                                },
                                                                                1 => {}
                                                                            }
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {},
                                                                                1 => {Bitcoin::Divide8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Bitcoin::Divide16},
                                                                                            1 => {Bitcoin::Divide32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Bitcoin::Divide64},
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {}
                                                                                },
                                                                                1 => {}
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {},
                                                                                1 => {Bitcoin::Modulo8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Bitcoin::Modulo16},
                                                                                            1 => {Bitcoin::Modulo32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Bitcoin::Modulo64},
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {}
                                                                                },
                                                                                1 => {}
                                                                            }
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {},
                                                                                1 => {Bitcoin::Divides8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Bitcoin::Divides16},
                                                                                            1 => {Bitcoin::Divides32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Bitcoin::Divides64},
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {}
                                                                                },
                                                                                1 => {}
                                                                            }
                                                                        }
                                                                    }
                                                                },
                                                                1 => {}
                                                            },
                                                            1 => {}
                                                        }
                                                    },
                                                    1 => {}
                                                },
                                                1 => {}
                                            },
                                            1 => {}
                                        },
                                        1 => {}
                                    }
                                }
                            }
                        },
                        1 => {
                            0 => {
                                0 => {Bitcoin::Sha256Block},
                                1 => {
                                    0 => {
                                        0 => {Bitcoin::Sha256Iv},
                                        1 => {
                                            0 => {Bitcoin::Sha256Ctx8Add1},
                                            1 => {
                                                0 => {
                                                    0 => {Bitcoin::Sha256Ctx8Add2},
                                                    1 => {Bitcoin::Sha256Ctx8Add4}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Bitcoin::Sha256Ctx8Add8},
                                                                1 => {Bitcoin::Sha256Ctx8Add16}
                                                            },
                                                            1 => {
                                                                0 => {Bitcoin::Sha256Ctx8Add32},
                                                                1 => {Bitcoin::Sha256Ctx8Add64}
                                                            }
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Bitcoin::Sha256Ctx8Add128},
                                                                    1 => {Bitcoin::Sha256Ctx8Add256}
                                                                },
                                                                1 => {
                                                                    0 => {Bitcoin::Sha256Ctx8Add512},
                                                                    1 => {}
                                                                }
                                                            },
                                                            1 => {}
                                                        }
                                                    },
                                                    1 => {}
                                                }
                                            }
                                        }
                                    },
                                    1 => {
                                        0 => {
                                            0 => {
                                                0 => {
                                                    0 => {Bitcoin::Sha256Ctx8AddBuffer511},
                                                    1 => {Bitcoin::Sha256Ctx8Finalize}
                                                },
                                                1 => {
                                                    0 => {Bitcoin::Sha256Ctx8Init},
                                                    1 => {}
                                                }
                                            },
                                            1 => {}
                                        },
                                        1 => {}
                                    }
                                }
                            },
                            1 => {}
                        }
                    },
                    1 => {
                        0 => {
                            0 => {
                                0 => {
                                    0 => {
                                        0 => {
                                            0 => {Bitcoin::PointVerify1},
                                            1 => {}
                                        },
                                        1 => {
                                            0 => {
                                                0 => {Bitcoin::Decompress},
                                                1 => {
                                                    0 => {Bitcoin::LinearVerify1},
                                                    1 => {}
                                                }
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Bitcoin::LinearCombination1},
                                                                1 => {}
                                                            },
                                                            1 => {Bitcoin::Scale}
                                                        },
                                                        1 => {
                                                            0 => {Bitcoin::Generate},
                                                            1 => {Bitcoin::GejInfinity}
                                                        }
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Bitcoin::GejNormalize},
                                                                1 => {Bitcoin::GejNegate}
                                                            },
                                                            1 => {
                                                                0 => {Bitcoin::GeNegate},
                                                                1 => {Bitcoin::GejDouble}
                                                            }
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {Bitcoin::GejAdd},
                                                                1 => {Bitcoin::GejGeAddEx}
                                                            },
                                                            1 => {
                                                                0 => {Bitcoin::GejGeAdd},
                                                                1 => {Bitcoin::GejRescale}
                                                            }
                                                        }
                                                    }
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Bitcoin::GejIsInfinity},
                                                                                1 => {Bitcoin::GejEquiv}
                                                                            },
                                                                            1 => {
                                                                                0 => {Bitcoin::GejGeEquiv},
                                                                                1 => {Bitcoin::GejXEquiv}
                                                                            }
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Bitcoin::GejYIsOdd},
                                                                                1 => {Bitcoin::GejIsOnCurve}
                                                                            },
                                                                            1 => {
                                                                                0 => {Bitcoin::GeIsOnCurve},
                                                                                1 => {Bitcoin::ScalarNormalize}
                                                                            }
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Bitcoin::ScalarNegate},
                                                                                1 => {Bitcoin::ScalarAdd}
                                                                            },
                                                                            1 => {
                                                                                0 => {Bitcoin::ScalarSquare},
                                                                                1 => {Bitcoin::ScalarMultiply}
                                                                            }
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Bitcoin::ScalarMultiplyLambda},
                                                                                1 => {Bitcoin::ScalarInvert}
                                                                            },
                                                                            1 => {
                                                                                0 => {Bitcoin::ScalarIsZero},
                                                                                1 => {}
                                                                            }
                                                                        }
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {},
                                                                                1 => {
                                                                                    0 => {},
                                                                                    1 => {Bitcoin::FeNormalize}
                                                                                }
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Bitcoin::FeNegate},
                                                                                    1 => {Bitcoin::FeAdd}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Bitcoin::FeSquare},
                                                                                    1 => {Bitcoin::FeMultiply}
                                                                                }
                                                                            }
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Bitcoin::FeMultiplyBeta},
                                                                                    1 => {Bitcoin::FeInvert}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Bitcoin::FeSquareRoot},
                                                                                    1 => {Bitcoin::FeIsZero}
                                                                                }
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Bitcoin::FeIsOdd},
                                                                                    1 => {}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Bitcoin::HashToCurve},
                                                                                    1 => {Bitcoin::Swu}
                                                                                }
                                                                            }
                                                                        }
                                                                    },
                                                                    1 => {}
                                                                }
                                                            },
                                                            1 => {}
                                                        },
                                                        1 => {}
                                                    },
                                                    1 => {}
                                                }
                                            }
                                        }
                                    },
                                    1 => {
                                        0 => {Bitcoin::CheckSigVerify},
                                        1 => {
                                            0 => {
                                                0 => {Bitcoin::Bip0340Verify},
                                                1 => {}
                                            },
                                            1 => {}
                                        }
                                    }
                                },
                                1 => {
                                    0 => {},
                                    1 => {
                                        0 => {Bitcoin::ParseLock},
                                        1 => {
                                            0 => {
                                                0 => {Bitcoin::ParseSequence},
                                                1 => {Bitcoin::TapdataInit}
                                            },
                                            1 => {}
                                        }
                                    }
                                }
                            },
                            1 => {}
                        },
                        1 => {}
                    }
                }
            },
            1 => {
                0 => {
                    0 => {Bitcoin::SigAllHash},
                    1 => {
                        0 => {
                            0 => {Bitcoin::TxHash},
                            1 => {Bitcoin::TapEnvHash}
                        },
                        1 => {
                            0 => {
                                0 => {
                                    0 => {
                                        0 => {Bitcoin::OutputsHash},
                                        1 => {Bitcoin::InputsHash}
                                    },
                                    1 => {
                                        0 => {Bitcoin::InputUtxosHash},
                                        1 => {Bitcoin::OutputHash}
                                    }
                                },
                                1 => {
                                    0 => {
                                        0 => {
                                            0 => {Bitcoin::OutputValuesHash},
                                            1 => {Bitcoin::OutputScriptsHash}
                                        },
                                        1 => {
                                            0 => {Bitcoin::InputHash},
                                            1 => {Bitcoin::InputOutpointsHash}
                                        }
                                    },
                                    1 => {
                                        0 => {
                                            0 => {Bitcoin::InputSequencesHash},
                                            1 => {Bitcoin::InputAnnexesHash}
                                        },
                                        1 => {
                                            0 => {Bitcoin::InputScriptSigsHash},
                                            1 => {Bitcoin::InputUtxoHash}
                                        }
                                    }
                                }
                            },
                            1 => {
                                0 => {
                                    0 => {
                                        0 => {
                                            0 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Bitcoin::InputValuesHash},
                                                            1 => {Bitcoin::InputScriptsHash}
                                                        },
                                                        1 => {
                                                            0 => {Bitcoin::TapleafHash},
                                                            1 => {Bitcoin::TappathHash}
                                                        }
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {Bitcoin::OutpointHash},
                                                            1 => {Bitcoin::AnnexHash}
                                                        },
                                                        1 => {
                                                            0 => {Bitcoin::BuildTapleafSimplicity},
                                                            1 => {Bitcoin::BuildTapbranch}
                                                        }
                                                    }
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Bitcoin::BuildTaptweak},
                                                            1 => {}
                                                        },
                                                        1 => {}
                                                    },
                                                    1 => {}
                                                }
                                            },
                                            1 => {}
                                        },
                                        1 => {}
                                    },
                                    1 => {}
                                },
                                1 => {}
                            }
                        }
                    }
                },
                1 => {
                    0 => {
                        0 => {
                            0 => {Bitcoin::CheckLockHeight},
                            1 => {
                                0 => {
                                    0 => {Bitcoin::CheckLockTime},
                                    1 => {Bitcoin::CheckLockDistance}
                                },
                                1 => {
                                    0 => {
                                        0 => {
                                            0 => {
                                                0 => {Bitcoin::CheckLockDuration},
                                                1 => {Bitcoin::TxLockHeight}
                                            },
                                            1 => {
                                                0 => {Bitcoin::TxLockTime},
                                                1 => {Bitcoin::TxLockDistance}
                                            }
                                        },
                                        1 => {
                                            0 => {
                                                0 => {
                                                    0 => {Bitcoin::TxLockDuration},
                                                    1 => {Bitcoin::TxIsFinal}
                                                },
                                                1 => {}
                                            },
                                            1 => {}
                                        }
                                    },
                                    1 => {}
                                }
                            }
                        },
                        1 => {
                            0 => {Bitcoin::ScriptCMR},
                            1 => {
                                0 => {
                                    0 => {Bitcoin::InternalKey},
                                    1 => {Bitcoin::CurrentIndex}
                                },
                                1 => {
                                    0 => {
                                        0 => {
                                            0 => {
                                                0 => {Bitcoin::NumInputs},
                                                1 => {Bitcoin::NumOutputs}
                                            },
                                            1 => {
                                                0 => {Bitcoin::LockTime},
                                                1 => {Bitcoin::Fee}
                                            }
                                        },
                                        1 => {
                                            0 => {
                                                0 => {
                                                    0 => {Bitcoin::OutputValue},
                                                    1 => {Bitcoin::OutputScriptHash}
                                                },
                                                1 => {
                                                    0 => {Bitcoin::TotalOutputValue},
                                                    1 => {Bitcoin::CurrentPrevOutpoint}
                                                }
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {Bitcoin::CurrentValue},
                                                    1 => {Bitcoin::CurrentScriptHash}
                                                },
                                                1 => {
                                                    0 => {Bitcoin::CurrentSequence},
                                                    1 => {Bitcoin::CurrentAnnexHash}
                                                }
                                            }
                                        }
                                    },
                                    1 => {
                                        0 => {
                                            0 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Bitcoin::CurrentScriptSigHash},
                                                                    1 => {Bitcoin::InputPrevOutpoint}
                                                                },
                                                                1 => {
                                                                    0 => {Bitcoin::InputValue},
                                                                    1 => {Bitcoin::InputScriptHash}
                                                                }
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {Bitcoin::InputSequence},
                                                                    1 => {Bitcoin::InputAnnexHash}
                                                                },
                                                                1 => {
                                                                    0 => {Bitcoin::InputScriptSigHash},
                                                                    1 => {Bitcoin::TotalInputValue}
                                                                }
                                                            }
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Bitcoin::TapleafVersion},
                                                                    1 => {Bitcoin::Tappath}
                                                                },
                                                                1 => {
                                                                    0 => {Bitcoin::Version},
                                                                    1 => {Bitcoin::TransactionId}
                                                                }
                                                            },
                                                            1 => {}
                                                        }
                                                    },
                                                    1 => {}
                                                },
                                                1 => {}
                                            },
                                            1 => {}
                                        },
                                        1 => {}
                                    }
                                }
                            }
                        }
                    },
                    1 => {}
                }
            }
        })
    }

    fn c_jet_ptr(&self) -> &dyn Fn(&mut CFrameItem, CFrameItem, &Self::CJetEnvironment) -> bool {
        match self {
            Bitcoin::Add16 => &simplicity_sys::c_jets::jets_wrapper::add_16,
            Bitcoin::Add32 => &simplicity_sys::c_jets::jets_wrapper::add_32,
            Bitcoin::Add64 => &simplicity_sys::c_jets::jets_wrapper::add_64,
            Bitcoin::Add8 => &simplicity_sys::c_jets::jets_wrapper::add_8,
            Bitcoin::All16 => &simplicity_sys::c_jets::jets_wrapper::all_16,
            Bitcoin::All32 => &simplicity_sys::c_jets::jets_wrapper::all_32,
            Bitcoin::All64 => &simplicity_sys::c_jets::jets_wrapper::all_64,
            Bitcoin::All8 => &simplicity_sys::c_jets::jets_wrapper::all_8,
            Bitcoin::And1 => &simplicity_sys::c_jets::jets_wrapper::and_1,
            Bitcoin::And16 => &simplicity_sys::c_jets::jets_wrapper::and_16,
            Bitcoin::And32 => &simplicity_sys::c_jets::jets_wrapper::and_32,
            Bitcoin::And64 => &simplicity_sys::c_jets::jets_wrapper::and_64,
            Bitcoin::And8 => &simplicity_sys::c_jets::jets_wrapper::and_8,
            Bitcoin::AnnexHash => &simplicity_sys::c_jets::jets_wrapper::bitcoin_annex_hash,
            Bitcoin::Bip0340Verify => &simplicity_sys::c_jets::jets_wrapper::bip_0340_verify,
            Bitcoin::BuildTapbranch => &simplicity_sys::c_jets::jets_wrapper::bitcoin_build_tapbranch,
            Bitcoin::BuildTapleafSimplicity => &simplicity_sys::c_jets::jets_wrapper::bitcoin_build_tapleaf_simplicity,
            Bitcoin::BuildTaptweak => &simplicity_sys::c_jets::jets_wrapper::bitcoin_build_taptweak,
            Bitcoin::Ch1 => &simplicity_sys::c_jets::jets_wrapper::ch_1,
            Bitcoin::Ch16 => &simplicity_sys::c_jets::jets_wrapper::ch_16,
            Bitcoin::Ch32 => &simplicity_sys::c_jets::jets_wrapper::ch_32,
            Bitcoin::Ch64 => &simplicity_sys::c_jets::jets_wrapper::ch_64,
            Bitcoin::Ch8 => &simplicity_sys::c_jets::jets_wrapper::ch_8,
            Bitcoin::CheckLockDistance => &simplicity_sys::c_jets::jets_wrapper::bitcoin_check_lock_distance,
            Bitcoin::CheckLockDuration => &simplicity_sys::c_jets::jets_wrapper::bitcoin_check_lock_duration,
            Bitcoin::CheckLockHeight => &simplicity_sys::c_jets::jets_wrapper::bitcoin_check_lock_height,
            Bitcoin::CheckLockTime => &simplicity_sys::c_jets::jets_wrapper::bitcoin_check_lock_time,
            Bitcoin::CheckSigVerify => &simplicity_sys::c_jets::jets_wrapper::check_sig_verify,
            Bitcoin::Complement1 => &simplicity_sys::c_jets::jets_wrapper::complement_1,
            Bitcoin::Complement16 => &simplicity_sys::c_jets::jets_wrapper::complement_16,
            Bitcoin::Complement32 => &simplicity_sys::c_jets::jets_wrapper::complement_32,
            Bitcoin::Complement64 => &simplicity_sys::c_jets::jets_wrapper::complement_64,
            Bitcoin::Complement8 => &simplicity_sys::c_jets::jets_wrapper::complement_8,
            Bitcoin::CurrentAnnexHash => &simplicity_sys::c_jets::jets_wrapper::bitcoin_current_annex_hash,
            Bitcoin::CurrentIndex => &simplicity_sys::c_jets::jets_wrapper::bitcoin_current_index,
            Bitcoin::CurrentPrevOutpoint => &simplicity_sys::c_jets::jets_wrapper::bitcoin_current_prev_outpoint,
            Bitcoin::CurrentScriptHash => &simplicity_sys::c_jets::jets_wrapper::bitcoin_current_script_hash,
            Bitcoin::CurrentScriptSigHash => &simplicity_sys::c_jets::jets_wrapper::bitcoin_current_script_sig_hash,
            Bitcoin::CurrentSequence => &simplicity_sys::c_jets::jets_wrapper::bitcoin_current_sequence,
            Bitcoin::CurrentValue => &simplicity_sys::c_jets::jets_wrapper::bitcoin_current_value,
            Bitcoin::Decompress => &simplicity_sys::c_jets::jets_wrapper::decompress,
            Bitcoin::Decrement16 => &simplicity_sys::c_jets::jets_wrapper::decrement_16,
            Bitcoin::Decrement32 => &simplicity_sys::c_jets::jets_wrapper::decrement_32,
            Bitcoin::Decrement64 => &simplicity_sys::c_jets::jets_wrapper::decrement_64,
            Bitcoin::Decrement8 => &simplicity_sys::c_jets::jets_wrapper::decrement_8,
            Bitcoin::DivMod128_64 => &simplicity_sys::c_jets::jets_wrapper::div_mod_128_64,
            Bitcoin::DivMod16 => &simplicity_sys::c_jets::jets_wrapper::div_mod_16,
            Bitcoin::DivMod32 => &simplicity_sys::c_jets::jets_wrapper::div_mod_32,
            Bitcoin::DivMod64 => &simplicity_sys::c_jets::jets_wrapper::div_mod_64,
            Bitcoin::DivMod8 => &simplicity_sys::c_jets::jets_wrapper::div_mod_8,
            Bitcoin::Divide16 => &simplicity_sys::c_jets::jets_wrapper::divide_16,
            Bitcoin::Divide32 => &simplicity_sys::c_jets::jets_wrapper::divide_32,
            Bitcoin::Divide64 => &simplicity_sys::c_jets::jets_wrapper::divide_64,
            Bitcoin::Divide8 => &simplicity_sys::c_jets::jets_wrapper::divide_8,
            Bitcoin::Divides16 => &simplicity_sys::c_jets::jets_wrapper::divides_16,
            Bitcoin::Divides32 => &simplicity_sys::c_jets::jets_wrapper::divides_32,
            Bitcoin::Divides64 => &simplicity_sys::c_jets::jets_wrapper::divides_64,
            Bitcoin::Divides8 => &simplicity_sys::c_jets::jets_wrapper::divides_8,
            Bitcoin::Eq1 => &simplicity_sys::c_jets::jets_wrapper::eq_1,
            Bitcoin::Eq16 => &simplicity_sys::c_jets::jets_wrapper::eq_16,
            Bitcoin::Eq256 => &simplicity_sys::c_jets::jets_wrapper::eq_256,
            Bitcoin::Eq32 => &simplicity_sys::c_jets::jets_wrapper::eq_32,
            Bitcoin::Eq64 => &simplicity_sys::c_jets::jets_wrapper::eq_64,
            Bitcoin::Eq8 => &simplicity_sys::c_jets::jets_wrapper::eq_8,
            Bitcoin::FeAdd => &simplicity_sys::c_jets::jets_wrapper::fe_add,
            Bitcoin::FeInvert => &simplicity_sys::c_jets::jets_wrapper::fe_invert,
            Bitcoin::FeIsOdd => &simplicity_sys::c_jets::jets_wrapper::fe_is_odd,
            Bitcoin::FeIsZero => &simplicity_sys::c_jets::jets_wrapper::fe_is_zero,
            Bitcoin::FeMultiply => &simplicity_sys::c_jets::jets_wrapper::fe_multiply,
            Bitcoin::FeMultiplyBeta => &simplicity_sys::c_jets::jets_wrapper::fe_multiply_beta,
            Bitcoin::FeNegate => &simplicity_sys::c_jets::jets_wrapper::fe_negate,
            Bitcoin::FeNormalize => &simplicity_sys::c_jets::jets_wrapper::fe_normalize,
            Bitcoin::FeSquare => &simplicity_sys::c_jets::jets_wrapper::fe_square,
            Bitcoin::FeSquareRoot => &simplicity_sys::c_jets::jets_wrapper::fe_square_root,
            Bitcoin::Fee => &simplicity_sys::c_jets::jets_wrapper::bitcoin_fee,
            Bitcoin::FullAdd16 => &simplicity_sys::c_jets::jets_wrapper::full_add_16,
            Bitcoin::FullAdd32 => &simplicity_sys::c_jets::jets_wrapper::full_add_32,
            Bitcoin::FullAdd64 => &simplicity_sys::c_jets::jets_wrapper::full_add_64,
            Bitcoin::FullAdd8 => &simplicity_sys::c_jets::jets_wrapper::full_add_8,
            Bitcoin::FullDecrement16 => &simplicity_sys::c_jets::jets_wrapper::full_decrement_16,
            Bitcoin::FullDecrement32 => &simplicity_sys::c_jets::jets_wrapper::full_decrement_32,
            Bitcoin::FullDecrement64 => &simplicity_sys::c_jets::jets_wrapper::full_decrement_64,
            Bitcoin::FullDecrement8 => &simplicity_sys::c_jets::jets_wrapper::full_decrement_8,
            Bitcoin::FullIncrement16 => &simplicity_sys::c_jets::jets_wrapper::full_increment_16,
            Bitcoin::FullIncrement32 => &simplicity_sys::c_jets::jets_wrapper::full_increment_32,
            Bitcoin::FullIncrement64 => &simplicity_sys::c_jets::jets_wrapper::full_increment_64,
            Bitcoin::FullIncrement8 => &simplicity_sys::c_jets::jets_wrapper::full_increment_8,
            Bitcoin::FullLeftShift16_1 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_16_1,
            Bitcoin::FullLeftShift16_2 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_16_2,
            Bitcoin::FullLeftShift16_4 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_16_4,
            Bitcoin::FullLeftShift16_8 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_16_8,
            Bitcoin::FullLeftShift32_1 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_1,
            Bitcoin::FullLeftShift32_16 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_16,
            Bitcoin::FullLeftShift32_2 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_2,
            Bitcoin::FullLeftShift32_4 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_4,
            Bitcoin::FullLeftShift32_8 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_8,
            Bitcoin::FullLeftShift64_1 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_1,
            Bitcoin::FullLeftShift64_16 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_16,
            Bitcoin::FullLeftShift64_2 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_2,
            Bitcoin::FullLeftShift64_32 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_32,
            Bitcoin::FullLeftShift64_4 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_4,
            Bitcoin::FullLeftShift64_8 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_8,
            Bitcoin::FullLeftShift8_1 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_8_1,
            Bitcoin::FullLeftShift8_2 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_8_2,
            Bitcoin::FullLeftShift8_4 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_8_4,
            Bitcoin::FullMultiply16 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_16,
            Bitcoin::FullMultiply32 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_32,
            Bitcoin::FullMultiply64 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_64,
            Bitcoin::FullMultiply8 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_8,
            Bitcoin::FullRightShift16_1 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_16_1,
            Bitcoin::FullRightShift16_2 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_16_2,
            Bitcoin::FullRightShift16_4 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_16_4,
            Bitcoin::FullRightShift16_8 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_16_8,
            Bitcoin::FullRightShift32_1 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_1,
            Bitcoin::FullRightShift32_16 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_16,
            Bitcoin::FullRightShift32_2 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_2,
            Bitcoin::FullRightShift32_4 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_4,
            Bitcoin::FullRightShift32_8 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_8,
            Bitcoin::FullRightShift64_1 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_1,
            Bitcoin::FullRightShift64_16 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_16,
            Bitcoin::FullRightShift64_2 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_2,
            Bitcoin::FullRightShift64_32 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_32,
            Bitcoin::FullRightShift64_4 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_4,
            Bitcoin::FullRightShift64_8 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_8,
            Bitcoin::FullRightShift8_1 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_8_1,
            Bitcoin::FullRightShift8_2 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_8_2,
            Bitcoin::FullRightShift8_4 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_8_4,
            Bitcoin::FullSubtract16 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_16,
            Bitcoin::FullSubtract32 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_32,
            Bitcoin::FullSubtract64 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_64,
            Bitcoin::FullSubtract8 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_8,
            Bitcoin::GeIsOnCurve => &simplicity_sys::c_jets::jets_wrapper::ge_is_on_curve,
            Bitcoin::GeNegate => &simplicity_sys::c_jets::jets_wrapper::ge_negate,
            Bitcoin::GejAdd => &simplicity_sys::c_jets::jets_wrapper::gej_add,
            Bitcoin::GejDouble => &simplicity_sys::c_jets::jets_wrapper::gej_double,
            Bitcoin::GejEquiv => &simplicity_sys::c_jets::jets_wrapper::gej_equiv,
            Bitcoin::GejGeAdd => &simplicity_sys::c_jets::jets_wrapper::gej_ge_add,
            Bitcoin::GejGeAddEx => &simplicity_sys::c_jets::jets_wrapper::gej_ge_add_ex,
            Bitcoin::GejGeEquiv => &simplicity_sys::c_jets::jets_wrapper::gej_ge_equiv,
            Bitcoin::GejInfinity => &simplicity_sys::c_jets::jets_wrapper::gej_infinity,
            Bitcoin::GejIsInfinity => &simplicity_sys::c_jets::jets_wrapper::gej_is_infinity,
            Bitcoin::GejIsOnCurve => &simplicity_sys::c_jets::jets_wrapper::gej_is_on_curve,
            Bitcoin::GejNegate => &simplicity_sys::c_jets::jets_wrapper::gej_negate,
            Bitcoin::GejNormalize => &simplicity_sys::c_jets::jets_wrapper::gej_normalize,
            Bitcoin::GejRescale => &simplicity_sys::c_jets::jets_wrapper::gej_rescale,
            Bitcoin::GejXEquiv => &simplicity_sys::c_jets::jets_wrapper::gej_x_equiv,
            Bitcoin::GejYIsOdd => &simplicity_sys::c_jets::jets_wrapper::gej_y_is_odd,
            Bitcoin::Generate => &simplicity_sys::c_jets::jets_wrapper::generate,
            Bitcoin::HashToCurve => &simplicity_sys::c_jets::jets_wrapper::hash_to_curve,
            Bitcoin::High1 => &simplicity_sys::c_jets::jets_wrapper::high_1,
            Bitcoin::High16 => &simplicity_sys::c_jets::jets_wrapper::high_16,
            Bitcoin::High32 => &simplicity_sys::c_jets::jets_wrapper::high_32,
            Bitcoin::High64 => &simplicity_sys::c_jets::jets_wrapper::high_64,
            Bitcoin::High8 => &simplicity_sys::c_jets::jets_wrapper::high_8,
            Bitcoin::Increment16 => &simplicity_sys::c_jets::jets_wrapper::increment_16,
            Bitcoin::Increment32 => &simplicity_sys::c_jets::jets_wrapper::increment_32,
            Bitcoin::Increment64 => &simplicity_sys::c_jets::jets_wrapper::increment_64,
            Bitcoin::Increment8 => &simplicity_sys::c_jets::jets_wrapper::increment_8,
            Bitcoin::InputAnnexHash => &simplicity_sys::c_jets::jets_wrapper::bitcoin_input_annex_hash,
            Bitcoin::InputAnnexesHash => &simplicity_sys::c_jets::jets_wrapper::bitcoin_input_annexes_hash,
            Bitcoin::InputHash => &simplicity_sys::c_jets::jets_wrapper::bitcoin_input_hash,
            Bitcoin::InputOutpointsHash => &simplicity_sys::c_jets::jets_wrapper::bitcoin_input_outpoints_hash,
            Bitcoin::InputPrevOutpoint => &simplicity_sys::c_jets::jets_wrapper::bitcoin_input_prev_outpoint,
            Bitcoin::InputScriptHash => &simplicity_sys::c_jets::jets_wrapper::bitcoin_input_script_hash,
            Bitcoin::InputScriptSigHash => &simplicity_sys::c_jets::jets_wrapper::bitcoin_input_script_sig_hash,
            Bitcoin::InputScriptSigsHash => &simplicity_sys::c_jets::jets_wrapper::bitcoin_input_script_sigs_hash,
            Bitcoin::InputScriptsHash => &simplicity_sys::c_jets::jets_wrapper::bitcoin_input_scripts_hash,
            Bitcoin::InputSequence => &simplicity_sys::c_jets::jets_wrapper::bitcoin_input_sequence,
            Bitcoin::InputSequencesHash => &simplicity_sys::c_jets::jets_wrapper::bitcoin_input_sequences_hash,
            Bitcoin::InputUtxoHash => &simplicity_sys::c_jets::jets_wrapper::bitcoin_input_utxo_hash,
            Bitcoin::InputUtxosHash => &simplicity_sys::c_jets::jets_wrapper::bitcoin_input_utxos_hash,
            Bitcoin::InputValue => &simplicity_sys::c_jets::jets_wrapper::bitcoin_input_value,
            Bitcoin::InputValuesHash => &simplicity_sys::c_jets::jets_wrapper::bitcoin_input_values_hash,
            Bitcoin::InputsHash => &simplicity_sys::c_jets::jets_wrapper::bitcoin_inputs_hash,
            Bitcoin::InternalKey => &simplicity_sys::c_jets::jets_wrapper::bitcoin_internal_key,
            Bitcoin::IsOne16 => &simplicity_sys::c_jets::jets_wrapper::is_one_16,
            Bitcoin::IsOne32 => &simplicity_sys::c_jets::jets_wrapper::is_one_32,
            Bitcoin::IsOne64 => &simplicity_sys::c_jets::jets_wrapper::is_one_64,
            Bitcoin::IsOne8 => &simplicity_sys::c_jets::jets_wrapper::is_one_8,
            Bitcoin::IsZero16 => &simplicity_sys::c_jets::jets_wrapper::is_zero_16,
            Bitcoin::IsZero32 => &simplicity_sys::c_jets::jets_wrapper::is_zero_32,
            Bitcoin::IsZero64 => &simplicity_sys::c_jets::jets_wrapper::is_zero_64,
            Bitcoin::IsZero8 => &simplicity_sys::c_jets::jets_wrapper::is_zero_8,
            Bitcoin::Le16 => &simplicity_sys::c_jets::jets_wrapper::le_16,
            Bitcoin::Le32 => &simplicity_sys::c_jets::jets_wrapper::le_32,
            Bitcoin::Le64 => &simplicity_sys::c_jets::jets_wrapper::le_64,
            Bitcoin::Le8 => &simplicity_sys::c_jets::jets_wrapper::le_8,
            Bitcoin::LeftExtend16_32 => &simplicity_sys::c_jets::jets_wrapper::left_extend_16_32,
            Bitcoin::LeftExtend16_64 => &simplicity_sys::c_jets::jets_wrapper::left_extend_16_64,
            Bitcoin::LeftExtend1_16 => &simplicity_sys::c_jets::jets_wrapper::left_extend_1_16,
            Bitcoin::LeftExtend1_32 => &simplicity_sys::c_jets::jets_wrapper::left_extend_1_32,
            Bitcoin::LeftExtend1_64 => &simplicity_sys::c_jets::jets_wrapper::left_extend_1_64,
            Bitcoin::LeftExtend1_8 => &simplicity_sys::c_jets::jets_wrapper::left_extend_1_8,
            Bitcoin::LeftExtend32_64 => &simplicity_sys::c_jets::jets_wrapper::left_extend_32_64,
            Bitcoin::LeftExtend8_16 => &simplicity_sys::c_jets::jets_wrapper::left_extend_8_16,
            Bitcoin::LeftExtend8_32 => &simplicity_sys::c_jets::jets_wrapper::left_extend_8_32,
            Bitcoin::LeftExtend8_64 => &simplicity_sys::c_jets::jets_wrapper::left_extend_8_64,
            Bitcoin::LeftPadHigh16_32 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_16_32,
            Bitcoin::LeftPadHigh16_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_16_64,
            Bitcoin::LeftPadHigh1_16 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_1_16,
            Bitcoin::LeftPadHigh1_32 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_1_32,
            Bitcoin::LeftPadHigh1_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_1_64,
            Bitcoin::LeftPadHigh1_8 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_1_8,
            Bitcoin::LeftPadHigh32_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_32_64,
            Bitcoin::LeftPadHigh8_16 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_8_16,
            Bitcoin::LeftPadHigh8_32 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_8_32,
            Bitcoin::LeftPadHigh8_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_8_64,
            Bitcoin::LeftPadLow16_32 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_16_32,
            Bitcoin::LeftPadLow16_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_16_64,
            Bitcoin::LeftPadLow1_16 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_1_16,
            Bitcoin::LeftPadLow1_32 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_1_32,
            Bitcoin::LeftPadLow1_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_1_64,
            Bitcoin::LeftPadLow1_8 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_1_8,
            Bitcoin::LeftPadLow32_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_32_64,
            Bitcoin::LeftPadLow8_16 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_8_16,
            Bitcoin::LeftPadLow8_32 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_8_32,
            Bitcoin::LeftPadLow8_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_8_64,
            Bitcoin::LeftRotate16 => &simplicity_sys::c_jets::jets_wrapper::left_rotate_16,
            Bitcoin::LeftRotate32 => &simplicity_sys::c_jets::jets_wrapper::left_rotate_32,
            Bitcoin::LeftRotate64 => &simplicity_sys::c_jets::jets_wrapper::left_rotate_64,
            Bitcoin::LeftRotate8 => &simplicity_sys::c_jets::jets_wrapper::left_rotate_8,
            Bitcoin::LeftShift16 => &simplicity_sys::c_jets::jets_wrapper::left_shift_16,
            Bitcoin::LeftShift32 => &simplicity_sys::c_jets::jets_wrapper::left_shift_32,
            Bitcoin::LeftShift64 => &simplicity_sys::c_jets::jets_wrapper::left_shift_64,
            Bitcoin::LeftShift8 => &simplicity_sys::c_jets::jets_wrapper::left_shift_8,
            Bitcoin::LeftShiftWith16 => &simplicity_sys::c_jets::jets_wrapper::left_shift_with_16,
            Bitcoin::LeftShiftWith32 => &simplicity_sys::c_jets::jets_wrapper::left_shift_with_32,
            Bitcoin::LeftShiftWith64 => &simplicity_sys::c_jets::jets_wrapper::left_shift_with_64,
            Bitcoin::LeftShiftWith8 => &simplicity_sys::c_jets::jets_wrapper::left_shift_with_8,
            Bitcoin::Leftmost16_1 => &simplicity_sys::c_jets::jets_wrapper::leftmost_16_1,
            Bitcoin::Leftmost16_2 => &simplicity_sys::c_jets::jets_wrapper::leftmost_16_2,
            Bitcoin::Leftmost16_4 => &simplicity_sys::c_jets::jets_wrapper::leftmost_16_4,
            Bitcoin::Leftmost16_8 => &simplicity_sys::c_jets::jets_wrapper::leftmost_16_8,
            Bitcoin::Leftmost32_1 => &simplicity_sys::c_jets::jets_wrapper::leftmost_32_1,
            Bitcoin::Leftmost32_16 => &simplicity_sys::c_jets::jets_wrapper::leftmost_32_16,
            Bitcoin::Leftmost32_2 => &simplicity_sys::c_jets::jets_wrapper::leftmost_32_2,
            Bitcoin::Leftmost32_4 => &simplicity_sys::c_jets::jets_wrapper::leftmost_32_4,
            Bitcoin::Leftmost32_8 => &simplicity_sys::c_jets::jets_wrapper::leftmost_32_8,
            Bitcoin::Leftmost64_1 => &simplicity_sys::c_jets::jets_wrapper::leftmost_64_1,
            Bitcoin::Leftmost64_16 => &simplicity_sys::c_jets::jets_wrapper::leftmost_64_16,
            Bitcoin::Leftmost64_2 => &simplicity_sys::c_jets::jets_wrapper::leftmost_64_2,
            Bitcoin::Leftmost64_32 => &simplicity_sys::c_jets::jets_wrapper::leftmost_64_32,
            Bitcoin::Leftmost64_4 => &simplicity_sys::c_jets::jets_wrapper::leftmost_64_4,
            Bitcoin::Leftmost64_8 => &simplicity_sys::c_jets::jets_wrapper::leftmost_64_8,
            Bitcoin::Leftmost8_1 => &simplicity_sys::c_jets::jets_wrapper::leftmost_8_1,
            Bitcoin::Leftmost8_2 => &simplicity_sys::c_jets::jets_wrapper::leftmost_8_2,
            Bitcoin::Leftmost8_4 => &simplicity_sys::c_jets::jets_wrapper::leftmost_8_4,
            Bitcoin::LinearCombination1 => &simplicity_sys::c_jets::jets_wrapper::linear_combination_1,
            Bitcoin::LinearVerify1 => &simplicity_sys::c_jets::jets_wrapper::linear_verify_1,
            Bitcoin::LockTime => &simplicity_sys::c_jets::jets_wrapper::bitcoin_lock_time,
            Bitcoin::Low1 => &simplicity_sys::c_jets::jets_wrapper::low_1,
            Bitcoin::Low16 => &simplicity_sys::c_jets::jets_wrapper::low_16,
            Bitcoin::Low32 => &simplicity_sys::c_jets::jets_wrapper::low_32,
            Bitcoin::Low64 => &simplicity_sys::c_jets::jets_wrapper::low_64,
            Bitcoin::Low8 => &simplicity_sys::c_jets::jets_wrapper::low_8,
            Bitcoin::Lt16 => &simplicity_sys::c_jets::jets_wrapper::lt_16,
            Bitcoin::Lt32 => &simplicity_sys::c_jets::jets_wrapper::lt_32,
            Bitcoin::Lt64 => &simplicity_sys::c_jets::jets_wrapper::lt_64,
            Bitcoin::Lt8 => &simplicity_sys::c_jets::jets_wrapper::lt_8,
            Bitcoin::Maj1 => &simplicity_sys::c_jets::jets_wrapper::maj_1,
            Bitcoin::Maj16 => &simplicity_sys::c_jets::jets_wrapper::maj_16,
            Bitcoin::Maj32 => &simplicity_sys::c_jets::jets_wrapper::maj_32,
            Bitcoin::Maj64 => &simplicity_sys::c_jets::jets_wrapper::maj_64,
            Bitcoin::Maj8 => &simplicity_sys::c_jets::jets_wrapper::maj_8,
            Bitcoin::Max16 => &simplicity_sys::c_jets::jets_wrapper::max_16,
            Bitcoin::Max32 => &simplicity_sys::c_jets::jets_wrapper::max_32,
            Bitcoin::Max64 => &simplicity_sys::c_jets::jets_wrapper::max_64,
            Bitcoin::Max8 => &simplicity_sys::c_jets::jets_wrapper::max_8,
            Bitcoin::Median16 => &simplicity_sys::c_jets::jets_wrapper::median_16,
            Bitcoin::Median32 => &simplicity_sys::c_jets::jets_wrapper::median_32,
            Bitcoin::Median64 => &simplicity_sys::c_jets::jets_wrapper::median_64,
            Bitcoin::Median8 => &simplicity_sys::c_jets::jets_wrapper::median_8,
            Bitcoin::Min16 => &simplicity_sys::c_jets::jets_wrapper::min_16,
            Bitcoin::Min32 => &simplicity_sys::c_jets::jets_wrapper::min_32,
            Bitcoin::Min64 => &simplicity_sys::c_jets::jets_wrapper::min_64,
            Bitcoin::Min8 => &simplicity_sys::c_jets::jets_wrapper::min_8,
            Bitcoin::Modulo16 => &simplicity_sys::c_jets::jets_wrapper::modulo_16,
            Bitcoin::Modulo32 => &simplicity_sys::c_jets::jets_wrapper::modulo_32,
            Bitcoin::Modulo64 => &simplicity_sys::c_jets::jets_wrapper::modulo_64,
            Bitcoin::Modulo8 => &simplicity_sys::c_jets::jets_wrapper::modulo_8,
            Bitcoin::Multiply16 => &simplicity_sys::c_jets::jets_wrapper::multiply_16,
            Bitcoin::Multiply32 => &simplicity_sys::c_jets::jets_wrapper::multiply_32,
            Bitcoin::Multiply64 => &simplicity_sys::c_jets::jets_wrapper::multiply_64,
            Bitcoin::Multiply8 => &simplicity_sys::c_jets::jets_wrapper::multiply_8,
            Bitcoin::Negate16 => &simplicity_sys::c_jets::jets_wrapper::negate_16,
            Bitcoin::Negate32 => &simplicity_sys::c_jets::jets_wrapper::negate_32,
            Bitcoin::Negate64 => &simplicity_sys::c_jets::jets_wrapper::negate_64,
            Bitcoin::Negate8 => &simplicity_sys::c_jets::jets_wrapper::negate_8,
            Bitcoin::NumInputs => &simplicity_sys::c_jets::jets_wrapper::bitcoin_num_inputs,
            Bitcoin::NumOutputs => &simplicity_sys::c_jets::jets_wrapper::bitcoin_num_outputs,
            Bitcoin::One16 => &simplicity_sys::c_jets::jets_wrapper::one_16,
            Bitcoin::One32 => &simplicity_sys::c_jets::jets_wrapper::one_32,
            Bitcoin::One64 => &simplicity_sys::c_jets::jets_wrapper::one_64,
            Bitcoin::One8 => &simplicity_sys::c_jets::jets_wrapper::one_8,
            Bitcoin::Or1 => &simplicity_sys::c_jets::jets_wrapper::or_1,
            Bitcoin::Or16 => &simplicity_sys::c_jets::jets_wrapper::or_16,
            Bitcoin::Or32 => &simplicity_sys::c_jets::jets_wrapper::or_32,
            Bitcoin::Or64 => &simplicity_sys::c_jets::jets_wrapper::or_64,
            Bitcoin::Or8 => &simplicity_sys::c_jets::jets_wrapper::or_8,
            Bitcoin::OutpointHash => &simplicity_sys::c_jets::jets_wrapper::bitcoin_outpoint_hash,
            Bitcoin::OutputHash => &simplicity_sys::c_jets::jets_wrapper::bitcoin_output_hash,
            Bitcoin::OutputScriptHash => &simplicity_sys::c_jets::jets_wrapper::bitcoin_output_script_hash,
            Bitcoin::OutputScriptsHash => &simplicity_sys::c_jets::jets_wrapper::bitcoin_output_scripts_hash,
            Bitcoin::OutputValue => &simplicity_sys::c_jets::jets_wrapper::bitcoin_output_value,
            Bitcoin::OutputValuesHash => &simplicity_sys::c_jets::jets_wrapper::bitcoin_output_values_hash,
            Bitcoin::OutputsHash => &simplicity_sys::c_jets::jets_wrapper::bitcoin_outputs_hash,
            Bitcoin::ParseLock => &simplicity_sys::c_jets::jets_wrapper::parse_lock,
            Bitcoin::ParseSequence => &simplicity_sys::c_jets::jets_wrapper::parse_sequence,
            Bitcoin::PointVerify1 => &simplicity_sys::c_jets::jets_wrapper::point_verify_1,
            Bitcoin::RightExtend16_32 => &simplicity_sys::c_jets::jets_wrapper::right_extend_16_32,
            Bitcoin::RightExtend16_64 => &simplicity_sys::c_jets::jets_wrapper::right_extend_16_64,
            Bitcoin::RightExtend32_64 => &simplicity_sys::c_jets::jets_wrapper::right_extend_32_64,
            Bitcoin::RightExtend8_16 => &simplicity_sys::c_jets::jets_wrapper::right_extend_8_16,
            Bitcoin::RightExtend8_32 => &simplicity_sys::c_jets::jets_wrapper::right_extend_8_32,
            Bitcoin::RightExtend8_64 => &simplicity_sys::c_jets::jets_wrapper::right_extend_8_64,
            Bitcoin::RightPadHigh16_32 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_16_32,
            Bitcoin::RightPadHigh16_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_16_64,
            Bitcoin::RightPadHigh1_16 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_1_16,
            Bitcoin::RightPadHigh1_32 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_1_32,
            Bitcoin::RightPadHigh1_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_1_64,
            Bitcoin::RightPadHigh1_8 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_1_8,
            Bitcoin::RightPadHigh32_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_32_64,
            Bitcoin::RightPadHigh8_16 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_8_16,
            Bitcoin::RightPadHigh8_32 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_8_32,
            Bitcoin::RightPadHigh8_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_8_64,
            Bitcoin::RightPadLow16_32 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_16_32,
            Bitcoin::RightPadLow16_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_16_64,
            Bitcoin::RightPadLow1_16 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_1_16,
            Bitcoin::RightPadLow1_32 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_1_32,
            Bitcoin::RightPadLow1_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_1_64,
            Bitcoin::RightPadLow1_8 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_1_8,
            Bitcoin::RightPadLow32_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_32_64,
            Bitcoin::RightPadLow8_16 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_8_16,
            Bitcoin::RightPadLow8_32 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_8_32,
            Bitcoin::RightPadLow8_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_8_64,
            Bitcoin::RightRotate16 => &simplicity_sys::c_jets::jets_wrapper::right_rotate_16,
            Bitcoin::RightRotate32 => &simplicity_sys::c_jets::jets_wrapper::right_rotate_32,
            Bitcoin::RightRotate64 => &simplicity_sys::c_jets::jets_wrapper::right_rotate_64,
            Bitcoin::RightRotate8 => &simplicity_sys::c_jets::jets_wrapper::right_rotate_8,
            Bitcoin::RightShift16 => &simplicity_sys::c_jets::jets_wrapper::right_shift_16,
            Bitcoin::RightShift32 => &simplicity_sys::c_jets::jets_wrapper::right_shift_32,
            Bitcoin::RightShift64 => &simplicity_sys::c_jets::jets_wrapper::right_shift_64,
            Bitcoin::RightShift8 => &simplicity_sys::c_jets::jets_wrapper::right_shift_8,
            Bitcoin::RightShiftWith16 => &simplicity_sys::c_jets::jets_wrapper::right_shift_with_16,
            Bitcoin::RightShiftWith32 => &simplicity_sys::c_jets::jets_wrapper::right_shift_with_32,
            Bitcoin::RightShiftWith64 => &simplicity_sys::c_jets::jets_wrapper::right_shift_with_64,
            Bitcoin::RightShiftWith8 => &simplicity_sys::c_jets::jets_wrapper::right_shift_with_8,
            Bitcoin::Rightmost16_1 => &simplicity_sys::c_jets::jets_wrapper::rightmost_16_1,
            Bitcoin::Rightmost16_2 => &simplicity_sys::c_jets::jets_wrapper::rightmost_16_2,
            Bitcoin::Rightmost16_4 => &simplicity_sys::c_jets::jets_wrapper::rightmost_16_4,
            Bitcoin::Rightmost16_8 => &simplicity_sys::c_jets::jets_wrapper::rightmost_16_8,
            Bitcoin::Rightmost32_1 => &simplicity_sys::c_jets::jets_wrapper::rightmost_32_1,
            Bitcoin::Rightmost32_16 => &simplicity_sys::c_jets::jets_wrapper::rightmost_32_16,
            Bitcoin::Rightmost32_2 => &simplicity_sys::c_jets::jets_wrapper::rightmost_32_2,
            Bitcoin::Rightmost32_4 => &simplicity_sys::c_jets::jets_wrapper::rightmost_32_4,
            Bitcoin::Rightmost32_8 => &simplicity_sys::c_jets::jets_wrapper::rightmost_32_8,
            Bitcoin::Rightmost64_1 => &simplicity_sys::c_jets::jets_wrapper::rightmost_64_1,
            Bitcoin::Rightmost64_16 => &simplicity_sys::c_jets::jets_wrapper::rightmost_64_16,
            Bitcoin::Rightmost64_2 => &simplicity_sys::c_jets::jets_wrapper::rightmost_64_2,
            Bitcoin::Rightmost64_32 => &simplicity_sys::c_jets::jets_wrapper::rightmost_64_32,
            Bitcoin::Rightmost64_4 => &simplicity_sys::c_jets::jets_wrapper::rightmost_64_4,
            Bitcoin::Rightmost64_8 => &simplicity_sys::c_jets::jets_wrapper::rightmost_64_8,
            Bitcoin::Rightmost8_1 => &simplicity_sys::c_jets::jets_wrapper::rightmost_8_1,
            Bitcoin::Rightmost8_2 => &simplicity_sys::c_jets::jets_wrapper::rightmost_8_2,
            Bitcoin::Rightmost8_4 => &simplicity_sys::c_jets::jets_wrapper::rightmost_8_4,
            Bitcoin::ScalarAdd => &simplicity_sys::c_jets::jets_wrapper::scalar_add,
            Bitcoin::ScalarInvert => &simplicity_sys::c_jets::jets_wrapper::scalar_invert,
            Bitcoin::ScalarIsZero => &simplicity_sys::c_jets::jets_wrapper::scalar_is_zero,
            Bitcoin::ScalarMultiply => &simplicity_sys::c_jets::jets_wrapper::scalar_multiply,
            Bitcoin::ScalarMultiplyLambda => &simplicity_sys::c_jets::jets_wrapper::scalar_multiply_lambda,
            Bitcoin::ScalarNegate => &simplicity_sys::c_jets::jets_wrapper::scalar_negate,
            Bitcoin::ScalarNormalize => &simplicity_sys::c_jets::jets_wrapper::scalar_normalize,
            Bitcoin::ScalarSquare => &simplicity_sys::c_jets::jets_wrapper::scalar_square,
            Bitcoin::Scale => &simplicity_sys::c_jets::jets_wrapper::scale,
            Bitcoin::ScriptCMR => &simplicity_sys::c_jets::jets_wrapper::bitcoin_script_cmr,
            Bitcoin::Sha256Block => &simplicity_sys::c_jets::jets_wrapper::sha_256_block,
            Bitcoin::Sha256Ctx8Add1 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_1,
            Bitcoin::Sha256Ctx8Add128 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_128,
            Bitcoin::Sha256Ctx8Add16 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_16,
            Bitcoin::Sha256Ctx8Add2 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_2,
            Bitcoin::Sha256Ctx8Add256 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_256,
            Bitcoin::Sha256Ctx8Add32 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_32,
            Bitcoin::Sha256Ctx8Add4 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_4,
            Bitcoin::Sha256Ctx8Add512 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_512,
            Bitcoin::Sha256Ctx8Add64 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_64,
            Bitcoin::Sha256Ctx8Add8 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_8,
            Bitcoin::Sha256Ctx8AddBuffer511 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_buffer_511,
            Bitcoin::Sha256Ctx8Finalize => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_finalize,
            Bitcoin::Sha256Ctx8Init => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_init,
            Bitcoin::Sha256Iv => &simplicity_sys::c_jets::jets_wrapper::sha_256_iv,
            Bitcoin::SigAllHash => &simplicity_sys::c_jets::jets_wrapper::bitcoin_sig_all_hash,
            Bitcoin::Some1 => &simplicity_sys::c_jets::jets_wrapper::some_1,
            Bitcoin::Some16 => &simplicity_sys::c_jets::jets_wrapper::some_16,
            Bitcoin::Some32 => &simplicity_sys::c_jets::jets_wrapper::some_32,
            Bitcoin::Some64 => &simplicity_sys::c_jets::jets_wrapper::some_64,
            Bitcoin::Some8 => &simplicity_sys::c_jets::jets_wrapper::some_8,
            Bitcoin::Subtract16 => &simplicity_sys::c_jets::jets_wrapper::subtract_16,
            Bitcoin::Subtract32 => &simplicity_sys::c_jets::jets_wrapper::subtract_32,
            Bitcoin::Subtract64 => &simplicity_sys::c_jets::jets_wrapper::subtract_64,
            Bitcoin::Subtract8 => &simplicity_sys::c_jets::jets_wrapper::subtract_8,
            Bitcoin::Swu => &simplicity_sys::c_jets::jets_wrapper::swu,
            Bitcoin::TapEnvHash => &simplicity_sys::c_jets::jets_wrapper::bitcoin_tap_env_hash,
            Bitcoin::TapdataInit => &simplicity_sys::c_jets::jets_wrapper::tapdata_init,
            Bitcoin::TapleafHash => &simplicity_sys::c_jets::jets_wrapper::bitcoin_tapleaf_hash,
            Bitcoin::TapleafVersion => &simplicity_sys::c_jets::jets_wrapper::bitcoin_tapleaf_version,
            Bitcoin::Tappath => &simplicity_sys::c_jets::jets_wrapper::bitcoin_tappath,
            Bitcoin::TappathHash => &simplicity_sys::c_jets::jets_wrapper::bitcoin_tappath_hash,
            Bitcoin::TotalInputValue => &simplicity_sys::c_jets::jets_wrapper::bitcoin_total_input_value,
            Bitcoin::TotalOutputValue => &simplicity_sys::c_jets::jets_wrapper::bitcoin_total_output_value,
            Bitcoin::TransactionId => &simplicity_sys::c_jets::jets_wrapper::bitcoin_transaction_id,
            Bitcoin::TxHash => &simplicity_sys::c_jets::jets_wrapper::bitcoin_tx_hash,
            Bitcoin::TxIsFinal => &simplicity_sys::c_jets::jets_wrapper::bitcoin_tx_is_final,
            Bitcoin::TxLockDistance => &simplicity_sys::c_jets::jets_wrapper::bitcoin_tx_lock_distance,
            Bitcoin::TxLockDuration => &simplicity_sys::c_jets::jets_wrapper::bitcoin_tx_lock_duration,
            Bitcoin::TxLockHeight => &simplicity_sys::c_jets::jets_wrapper::bitcoin_tx_lock_height,
            Bitcoin::TxLockTime => &simplicity_sys::c_jets::jets_wrapper::bitcoin_tx_lock_time,
            Bitcoin::Verify => &simplicity_sys::c_jets::jets_wrapper::verify,
            Bitcoin::Version => &simplicity_sys::c_jets::jets_wrapper::bitcoin_version,
            Bitcoin::Xor1 => &simplicity_sys::c_jets::jets_wrapper::xor_1,
            Bitcoin::Xor16 => &simplicity_sys::c_jets::jets_wrapper::xor_16,
            Bitcoin::Xor32 => &simplicity_sys::c_jets::jets_wrapper::xor_32,
            Bitcoin::Xor64 => &simplicity_sys::c_jets::jets_wrapper::xor_64,
            Bitcoin::Xor8 => &simplicity_sys::c_jets::jets_wrapper::xor_8,
            Bitcoin::XorXor1 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_1,
            Bitcoin::XorXor16 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_16,
            Bitcoin::XorXor32 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_32,
            Bitcoin::XorXor64 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_64,
            Bitcoin::XorXor8 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_8,
        }
    }

    fn cost(&self) -> Cost {
        unimplemented!("Unspecified cost of Bitcoin jets")
    }
}

impl fmt::Display for Bitcoin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Bitcoin::Add16 => f.write_str("add_16"),
            Bitcoin::Add32 => f.write_str("add_32"),
            Bitcoin::Add64 => f.write_str("add_64"),
            Bitcoin::Add8 => f.write_str("add_8"),
            Bitcoin::All16 => f.write_str("all_16"),
            Bitcoin::All32 => f.write_str("all_32"),
            Bitcoin::All64 => f.write_str("all_64"),
            Bitcoin::All8 => f.write_str("all_8"),
            Bitcoin::And1 => f.write_str("and_1"),
            Bitcoin::And16 => f.write_str("and_16"),
            Bitcoin::And32 => f.write_str("and_32"),
            Bitcoin::And64 => f.write_str("and_64"),
            Bitcoin::And8 => f.write_str("and_8"),
            Bitcoin::AnnexHash => f.write_str("annex_hash"),
            Bitcoin::Bip0340Verify => f.write_str("bip_0340_verify"),
            Bitcoin::BuildTapbranch => f.write_str("build_tapbranch"),
            Bitcoin::BuildTapleafSimplicity => f.write_str("build_tapleaf_simplicity"),
            Bitcoin::BuildTaptweak => f.write_str("build_taptweak"),
            Bitcoin::Ch1 => f.write_str("ch_1"),
            Bitcoin::Ch16 => f.write_str("ch_16"),
            Bitcoin::Ch32 => f.write_str("ch_32"),
            Bitcoin::Ch64 => f.write_str("ch_64"),
            Bitcoin::Ch8 => f.write_str("ch_8"),
            Bitcoin::CheckLockDistance => f.write_str("check_lock_distance"),
            Bitcoin::CheckLockDuration => f.write_str("check_lock_duration"),
            Bitcoin::CheckLockHeight => f.write_str("check_lock_height"),
            Bitcoin::CheckLockTime => f.write_str("check_lock_time"),
            Bitcoin::CheckSigVerify => f.write_str("check_sig_verify"),
            Bitcoin::Complement1 => f.write_str("complement_1"),
            Bitcoin::Complement16 => f.write_str("complement_16"),
            Bitcoin::Complement32 => f.write_str("complement_32"),
            Bitcoin::Complement64 => f.write_str("complement_64"),
            Bitcoin::Complement8 => f.write_str("complement_8"),
            Bitcoin::CurrentAnnexHash => f.write_str("current_annex_hash"),
            Bitcoin::CurrentIndex => f.write_str("current_index"),
            Bitcoin::CurrentPrevOutpoint => f.write_str("current_prev_outpoint"),
            Bitcoin::CurrentScriptHash => f.write_str("current_script_hash"),
            Bitcoin::CurrentScriptSigHash => f.write_str("current_script_sig_hash"),
            Bitcoin::CurrentSequence => f.write_str("current_sequence"),
            Bitcoin::CurrentValue => f.write_str("current_value"),
            Bitcoin::Decompress => f.write_str("decompress"),
            Bitcoin::Decrement16 => f.write_str("decrement_16"),
            Bitcoin::Decrement32 => f.write_str("decrement_32"),
            Bitcoin::Decrement64 => f.write_str("decrement_64"),
            Bitcoin::Decrement8 => f.write_str("decrement_8"),
            Bitcoin::DivMod128_64 => f.write_str("div_mod_128_64"),
            Bitcoin::DivMod16 => f.write_str("div_mod_16"),
            Bitcoin::DivMod32 => f.write_str("div_mod_32"),
            Bitcoin::DivMod64 => f.write_str("div_mod_64"),
            Bitcoin::DivMod8 => f.write_str("div_mod_8"),
            Bitcoin::Divide16 => f.write_str("divide_16"),
            Bitcoin::Divide32 => f.write_str("divide_32"),
            Bitcoin::Divide64 => f.write_str("divide_64"),
            Bitcoin::Divide8 => f.write_str("divide_8"),
            Bitcoin::Divides16 => f.write_str("divides_16"),
            Bitcoin::Divides32 => f.write_str("divides_32"),
            Bitcoin::Divides64 => f.write_str("divides_64"),
            Bitcoin::Divides8 => f.write_str("divides_8"),
            Bitcoin::Eq1 => f.write_str("eq_1"),
            Bitcoin::Eq16 => f.write_str("eq_16"),
            Bitcoin::Eq256 => f.write_str("eq_256"),
            Bitcoin::Eq32 => f.write_str("eq_32"),
            Bitcoin::Eq64 => f.write_str("eq_64"),
            Bitcoin::Eq8 => f.write_str("eq_8"),
            Bitcoin::FeAdd => f.write_str("fe_add"),
            Bitcoin::FeInvert => f.write_str("fe_invert"),
            Bitcoin::FeIsOdd => f.write_str("fe_is_odd"),
            Bitcoin::FeIsZero => f.write_str("fe_is_zero"),
            Bitcoin::FeMultiply => f.write_str("fe_multiply"),
            Bitcoin::FeMultiplyBeta => f.write_str("fe_multiply_beta"),
            Bitcoin::FeNegate => f.write_str("fe_negate"),
            Bitcoin::FeNormalize => f.write_str("fe_normalize"),
            Bitcoin::FeSquare => f.write_str("fe_square"),
            Bitcoin::FeSquareRoot => f.write_str("fe_square_root"),
            Bitcoin::Fee => f.write_str("fee"),
            Bitcoin::FullAdd16 => f.write_str("full_add_16"),
            Bitcoin::FullAdd32 => f.write_str("full_add_32"),
            Bitcoin::FullAdd64 => f.write_str("full_add_64"),
            Bitcoin::FullAdd8 => f.write_str("full_add_8"),
            Bitcoin::FullDecrement16 => f.write_str("full_decrement_16"),
            Bitcoin::FullDecrement32 => f.write_str("full_decrement_32"),
            Bitcoin::FullDecrement64 => f.write_str("full_decrement_64"),
            Bitcoin::FullDecrement8 => f.write_str("full_decrement_8"),
            Bitcoin::FullIncrement16 => f.write_str("full_increment_16"),
            Bitcoin::FullIncrement32 => f.write_str("full_increment_32"),
            Bitcoin::FullIncrement64 => f.write_str("full_increment_64"),
            Bitcoin::FullIncrement8 => f.write_str("full_increment_8"),
            Bitcoin::FullLeftShift16_1 => f.write_str("full_left_shift_16_1"),
            Bitcoin::FullLeftShift16_2 => f.write_str("full_left_shift_16_2"),
            Bitcoin::FullLeftShift16_4 => f.write_str("full_left_shift_16_4"),
            Bitcoin::FullLeftShift16_8 => f.write_str("full_left_shift_16_8"),
            Bitcoin::FullLeftShift32_1 => f.write_str("full_left_shift_32_1"),
            Bitcoin::FullLeftShift32_16 => f.write_str("full_left_shift_32_16"),
            Bitcoin::FullLeftShift32_2 => f.write_str("full_left_shift_32_2"),
            Bitcoin::FullLeftShift32_4 => f.write_str("full_left_shift_32_4"),
            Bitcoin::FullLeftShift32_8 => f.write_str("full_left_shift_32_8"),
            Bitcoin::FullLeftShift64_1 => f.write_str("full_left_shift_64_1"),
            Bitcoin::FullLeftShift64_16 => f.write_str("full_left_shift_64_16"),
            Bitcoin::FullLeftShift64_2 => f.write_str("full_left_shift_64_2"),
            Bitcoin::FullLeftShift64_32 => f.write_str("full_left_shift_64_32"),
            Bitcoin::FullLeftShift64_4 => f.write_str("full_left_shift_64_4"),
            Bitcoin::FullLeftShift64_8 => f.write_str("full_left_shift_64_8"),
            Bitcoin::FullLeftShift8_1 => f.write_str("full_left_shift_8_1"),
            Bitcoin::FullLeftShift8_2 => f.write_str("full_left_shift_8_2"),
            Bitcoin::FullLeftShift8_4 => f.write_str("full_left_shift_8_4"),
            Bitcoin::FullMultiply16 => f.write_str("full_multiply_16"),
            Bitcoin::FullMultiply32 => f.write_str("full_multiply_32"),
            Bitcoin::FullMultiply64 => f.write_str("full_multiply_64"),
            Bitcoin::FullMultiply8 => f.write_str("full_multiply_8"),
            Bitcoin::FullRightShift16_1 => f.write_str("full_right_shift_16_1"),
            Bitcoin::FullRightShift16_2 => f.write_str("full_right_shift_16_2"),
            Bitcoin::FullRightShift16_4 => f.write_str("full_right_shift_16_4"),
            Bitcoin::FullRightShift16_8 => f.write_str("full_right_shift_16_8"),
            Bitcoin::FullRightShift32_1 => f.write_str("full_right_shift_32_1"),
            Bitcoin::FullRightShift32_16 => f.write_str("full_right_shift_32_16"),
            Bitcoin::FullRightShift32_2 => f.write_str("full_right_shift_32_2"),
            Bitcoin::FullRightShift32_4 => f.write_str("full_right_shift_32_4"),
            Bitcoin::FullRightShift32_8 => f.write_str("full_right_shift_32_8"),
            Bitcoin::FullRightShift64_1 => f.write_str("full_right_shift_64_1"),
            Bitcoin::FullRightShift64_16 => f.write_str("full_right_shift_64_16"),
            Bitcoin::FullRightShift64_2 => f.write_str("full_right_shift_64_2"),
            Bitcoin::FullRightShift64_32 => f.write_str("full_right_shift_64_32"),
            Bitcoin::FullRightShift64_4 => f.write_str("full_right_shift_64_4"),
            Bitcoin::FullRightShift64_8 => f.write_str("full_right_shift_64_8"),
            Bitcoin::FullRightShift8_1 => f.write_str("full_right_shift_8_1"),
            Bitcoin::FullRightShift8_2 => f.write_str("full_right_shift_8_2"),
            Bitcoin::FullRightShift8_4 => f.write_str("full_right_shift_8_4"),
            Bitcoin::FullSubtract16 => f.write_str("full_subtract_16"),
            Bitcoin::FullSubtract32 => f.write_str("full_subtract_32"),
            Bitcoin::FullSubtract64 => f.write_str("full_subtract_64"),
            Bitcoin::FullSubtract8 => f.write_str("full_subtract_8"),
            Bitcoin::GeIsOnCurve => f.write_str("ge_is_on_curve"),
            Bitcoin::GeNegate => f.write_str("ge_negate"),
            Bitcoin::GejAdd => f.write_str("gej_add"),
            Bitcoin::GejDouble => f.write_str("gej_double"),
            Bitcoin::GejEquiv => f.write_str("gej_equiv"),
            Bitcoin::GejGeAdd => f.write_str("gej_ge_add"),
            Bitcoin::GejGeAddEx => f.write_str("gej_ge_add_ex"),
            Bitcoin::GejGeEquiv => f.write_str("gej_ge_equiv"),
            Bitcoin::GejInfinity => f.write_str("gej_infinity"),
            Bitcoin::GejIsInfinity => f.write_str("gej_is_infinity"),
            Bitcoin::GejIsOnCurve => f.write_str("gej_is_on_curve"),
            Bitcoin::GejNegate => f.write_str("gej_negate"),
            Bitcoin::GejNormalize => f.write_str("gej_normalize"),
            Bitcoin::GejRescale => f.write_str("gej_rescale"),
            Bitcoin::GejXEquiv => f.write_str("gej_x_equiv"),
            Bitcoin::GejYIsOdd => f.write_str("gej_y_is_odd"),
            Bitcoin::Generate => f.write_str("generate"),
            Bitcoin::HashToCurve => f.write_str("hash_to_curve"),
            Bitcoin::High1 => f.write_str("high_1"),
            Bitcoin::High16 => f.write_str("high_16"),
            Bitcoin::High32 => f.write_str("high_32"),
            Bitcoin::High64 => f.write_str("high_64"),
            Bitcoin::High8 => f.write_str("high_8"),
            Bitcoin::Increment16 => f.write_str("increment_16"),
            Bitcoin::Increment32 => f.write_str("increment_32"),
            Bitcoin::Increment64 => f.write_str("increment_64"),
            Bitcoin::Increment8 => f.write_str("increment_8"),
            Bitcoin::InputAnnexHash => f.write_str("input_annex_hash"),
            Bitcoin::InputAnnexesHash => f.write_str("input_annexes_hash"),
            Bitcoin::InputHash => f.write_str("input_hash"),
            Bitcoin::InputOutpointsHash => f.write_str("input_outpoints_hash"),
            Bitcoin::InputPrevOutpoint => f.write_str("input_prev_outpoint"),
            Bitcoin::InputScriptHash => f.write_str("input_script_hash"),
            Bitcoin::InputScriptSigHash => f.write_str("input_script_sig_hash"),
            Bitcoin::InputScriptSigsHash => f.write_str("input_script_sigs_hash"),
            Bitcoin::InputScriptsHash => f.write_str("input_scripts_hash"),
            Bitcoin::InputSequence => f.write_str("input_sequence"),
            Bitcoin::InputSequencesHash => f.write_str("input_sequences_hash"),
            Bitcoin::InputUtxoHash => f.write_str("input_utxo_hash"),
            Bitcoin::InputUtxosHash => f.write_str("input_utxos_hash"),
            Bitcoin::InputValue => f.write_str("input_value"),
            Bitcoin::InputValuesHash => f.write_str("input_values_hash"),
            Bitcoin::InputsHash => f.write_str("inputs_hash"),
            Bitcoin::InternalKey => f.write_str("internal_key"),
            Bitcoin::IsOne16 => f.write_str("is_one_16"),
            Bitcoin::IsOne32 => f.write_str("is_one_32"),
            Bitcoin::IsOne64 => f.write_str("is_one_64"),
            Bitcoin::IsOne8 => f.write_str("is_one_8"),
            Bitcoin::IsZero16 => f.write_str("is_zero_16"),
            Bitcoin::IsZero32 => f.write_str("is_zero_32"),
            Bitcoin::IsZero64 => f.write_str("is_zero_64"),
            Bitcoin::IsZero8 => f.write_str("is_zero_8"),
            Bitcoin::Le16 => f.write_str("le_16"),
            Bitcoin::Le32 => f.write_str("le_32"),
            Bitcoin::Le64 => f.write_str("le_64"),
            Bitcoin::Le8 => f.write_str("le_8"),
            Bitcoin::LeftExtend16_32 => f.write_str("left_extend_16_32"),
            Bitcoin::LeftExtend16_64 => f.write_str("left_extend_16_64"),
            Bitcoin::LeftExtend1_16 => f.write_str("left_extend_1_16"),
            Bitcoin::LeftExtend1_32 => f.write_str("left_extend_1_32"),
            Bitcoin::LeftExtend1_64 => f.write_str("left_extend_1_64"),
            Bitcoin::LeftExtend1_8 => f.write_str("left_extend_1_8"),
            Bitcoin::LeftExtend32_64 => f.write_str("left_extend_32_64"),
            Bitcoin::LeftExtend8_16 => f.write_str("left_extend_8_16"),
            Bitcoin::LeftExtend8_32 => f.write_str("left_extend_8_32"),
            Bitcoin::LeftExtend8_64 => f.write_str("left_extend_8_64"),
            Bitcoin::LeftPadHigh16_32 => f.write_str("left_pad_high_16_32"),
            Bitcoin::LeftPadHigh16_64 => f.write_str("left_pad_high_16_64"),
            Bitcoin::LeftPadHigh1_16 => f.write_str("left_pad_high_1_16"),
            Bitcoin::LeftPadHigh1_32 => f.write_str("left_pad_high_1_32"),
            Bitcoin::LeftPadHigh1_64 => f.write_str("left_pad_high_1_64"),
            Bitcoin::LeftPadHigh1_8 => f.write_str("left_pad_high_1_8"),
            Bitcoin::LeftPadHigh32_64 => f.write_str("left_pad_high_32_64"),
            Bitcoin::LeftPadHigh8_16 => f.write_str("left_pad_high_8_16"),
            Bitcoin::LeftPadHigh8_32 => f.write_str("left_pad_high_8_32"),
            Bitcoin::LeftPadHigh8_64 => f.write_str("left_pad_high_8_64"),
            Bitcoin::LeftPadLow16_32 => f.write_str("left_pad_low_16_32"),
            Bitcoin::LeftPadLow16_64 => f.write_str("left_pad_low_16_64"),
            Bitcoin::LeftPadLow1_16 => f.write_str("left_pad_low_1_16"),
            Bitcoin::LeftPadLow1_32 => f.write_str("left_pad_low_1_32"),
            Bitcoin::LeftPadLow1_64 => f.write_str("left_pad_low_1_64"),
            Bitcoin::LeftPadLow1_8 => f.write_str("left_pad_low_1_8"),
            Bitcoin::LeftPadLow32_64 => f.write_str("left_pad_low_32_64"),
            Bitcoin::LeftPadLow8_16 => f.write_str("left_pad_low_8_16"),
            Bitcoin::LeftPadLow8_32 => f.write_str("left_pad_low_8_32"),
            Bitcoin::LeftPadLow8_64 => f.write_str("left_pad_low_8_64"),
            Bitcoin::LeftRotate16 => f.write_str("left_rotate_16"),
            Bitcoin::LeftRotate32 => f.write_str("left_rotate_32"),
            Bitcoin::LeftRotate64 => f.write_str("left_rotate_64"),
            Bitcoin::LeftRotate8 => f.write_str("left_rotate_8"),
            Bitcoin::LeftShift16 => f.write_str("left_shift_16"),
            Bitcoin::LeftShift32 => f.write_str("left_shift_32"),
            Bitcoin::LeftShift64 => f.write_str("left_shift_64"),
            Bitcoin::LeftShift8 => f.write_str("left_shift_8"),
            Bitcoin::LeftShiftWith16 => f.write_str("left_shift_with_16"),
            Bitcoin::LeftShiftWith32 => f.write_str("left_shift_with_32"),
            Bitcoin::LeftShiftWith64 => f.write_str("left_shift_with_64"),
            Bitcoin::LeftShiftWith8 => f.write_str("left_shift_with_8"),
            Bitcoin::Leftmost16_1 => f.write_str("leftmost_16_1"),
            Bitcoin::Leftmost16_2 => f.write_str("leftmost_16_2"),
            Bitcoin::Leftmost16_4 => f.write_str("leftmost_16_4"),
            Bitcoin::Leftmost16_8 => f.write_str("leftmost_16_8"),
            Bitcoin::Leftmost32_1 => f.write_str("leftmost_32_1"),
            Bitcoin::Leftmost32_16 => f.write_str("leftmost_32_16"),
            Bitcoin::Leftmost32_2 => f.write_str("leftmost_32_2"),
            Bitcoin::Leftmost32_4 => f.write_str("leftmost_32_4"),
            Bitcoin::Leftmost32_8 => f.write_str("leftmost_32_8"),
            Bitcoin::Leftmost64_1 => f.write_str("leftmost_64_1"),
            Bitcoin::Leftmost64_16 => f.write_str("leftmost_64_16"),
            Bitcoin::Leftmost64_2 => f.write_str("leftmost_64_2"),
            Bitcoin::Leftmost64_32 => f.write_str("leftmost_64_32"),
            Bitcoin::Leftmost64_4 => f.write_str("leftmost_64_4"),
            Bitcoin::Leftmost64_8 => f.write_str("leftmost_64_8"),
            Bitcoin::Leftmost8_1 => f.write_str("leftmost_8_1"),
            Bitcoin::Leftmost8_2 => f.write_str("leftmost_8_2"),
            Bitcoin::Leftmost8_4 => f.write_str("leftmost_8_4"),
            Bitcoin::LinearCombination1 => f.write_str("linear_combination_1"),
            Bitcoin::LinearVerify1 => f.write_str("linear_verify_1"),
            Bitcoin::LockTime => f.write_str("lock_time"),
            Bitcoin::Low1 => f.write_str("low_1"),
            Bitcoin::Low16 => f.write_str("low_16"),
            Bitcoin::Low32 => f.write_str("low_32"),
            Bitcoin::Low64 => f.write_str("low_64"),
            Bitcoin::Low8 => f.write_str("low_8"),
            Bitcoin::Lt16 => f.write_str("lt_16"),
            Bitcoin::Lt32 => f.write_str("lt_32"),
            Bitcoin::Lt64 => f.write_str("lt_64"),
            Bitcoin::Lt8 => f.write_str("lt_8"),
            Bitcoin::Maj1 => f.write_str("maj_1"),
            Bitcoin::Maj16 => f.write_str("maj_16"),
            Bitcoin::Maj32 => f.write_str("maj_32"),
            Bitcoin::Maj64 => f.write_str("maj_64"),
            Bitcoin::Maj8 => f.write_str("maj_8"),
            Bitcoin::Max16 => f.write_str("max_16"),
            Bitcoin::Max32 => f.write_str("max_32"),
            Bitcoin::Max64 => f.write_str("max_64"),
            Bitcoin::Max8 => f.write_str("max_8"),
            Bitcoin::Median16 => f.write_str("median_16"),
            Bitcoin::Median32 => f.write_str("median_32"),
            Bitcoin::Median64 => f.write_str("median_64"),
            Bitcoin::Median8 => f.write_str("median_8"),
            Bitcoin::Min16 => f.write_str("min_16"),
            Bitcoin::Min32 => f.write_str("min_32"),
            Bitcoin::Min64 => f.write_str("min_64"),
            Bitcoin::Min8 => f.write_str("min_8"),
            Bitcoin::Modulo16 => f.write_str("modulo_16"),
            Bitcoin::Modulo32 => f.write_str("modulo_32"),
            Bitcoin::Modulo64 => f.write_str("modulo_64"),
            Bitcoin::Modulo8 => f.write_str("modulo_8"),
            Bitcoin::Multiply16 => f.write_str("multiply_16"),
            Bitcoin::Multiply32 => f.write_str("multiply_32"),
            Bitcoin::Multiply64 => f.write_str("multiply_64"),
            Bitcoin::Multiply8 => f.write_str("multiply_8"),
            Bitcoin::Negate16 => f.write_str("negate_16"),
            Bitcoin::Negate32 => f.write_str("negate_32"),
            Bitcoin::Negate64 => f.write_str("negate_64"),
            Bitcoin::Negate8 => f.write_str("negate_8"),
            Bitcoin::NumInputs => f.write_str("num_inputs"),
            Bitcoin::NumOutputs => f.write_str("num_outputs"),
            Bitcoin::One16 => f.write_str("one_16"),
            Bitcoin::One32 => f.write_str("one_32"),
            Bitcoin::One64 => f.write_str("one_64"),
            Bitcoin::One8 => f.write_str("one_8"),
            Bitcoin::Or1 => f.write_str("or_1"),
            Bitcoin::Or16 => f.write_str("or_16"),
            Bitcoin::Or32 => f.write_str("or_32"),
            Bitcoin::Or64 => f.write_str("or_64"),
            Bitcoin::Or8 => f.write_str("or_8"),
            Bitcoin::OutpointHash => f.write_str("outpoint_hash"),
            Bitcoin::OutputHash => f.write_str("output_hash"),
            Bitcoin::OutputScriptHash => f.write_str("output_script_hash"),
            Bitcoin::OutputScriptsHash => f.write_str("output_scripts_hash"),
            Bitcoin::OutputValue => f.write_str("output_value"),
            Bitcoin::OutputValuesHash => f.write_str("output_values_hash"),
            Bitcoin::OutputsHash => f.write_str("outputs_hash"),
            Bitcoin::ParseLock => f.write_str("parse_lock"),
            Bitcoin::ParseSequence => f.write_str("parse_sequence"),
            Bitcoin::PointVerify1 => f.write_str("point_verify_1"),
            Bitcoin::RightExtend16_32 => f.write_str("right_extend_16_32"),
            Bitcoin::RightExtend16_64 => f.write_str("right_extend_16_64"),
            Bitcoin::RightExtend32_64 => f.write_str("right_extend_32_64"),
            Bitcoin::RightExtend8_16 => f.write_str("right_extend_8_16"),
            Bitcoin::RightExtend8_32 => f.write_str("right_extend_8_32"),
            Bitcoin::RightExtend8_64 => f.write_str("right_extend_8_64"),
            Bitcoin::RightPadHigh16_32 => f.write_str("right_pad_high_16_32"),
            Bitcoin::RightPadHigh16_64 => f.write_str("right_pad_high_16_64"),
            Bitcoin::RightPadHigh1_16 => f.write_str("right_pad_high_1_16"),
            Bitcoin::RightPadHigh1_32 => f.write_str("right_pad_high_1_32"),
            Bitcoin::RightPadHigh1_64 => f.write_str("right_pad_high_1_64"),
            Bitcoin::RightPadHigh1_8 => f.write_str("right_pad_high_1_8"),
            Bitcoin::RightPadHigh32_64 => f.write_str("right_pad_high_32_64"),
            Bitcoin::RightPadHigh8_16 => f.write_str("right_pad_high_8_16"),
            Bitcoin::RightPadHigh8_32 => f.write_str("right_pad_high_8_32"),
            Bitcoin::RightPadHigh8_64 => f.write_str("right_pad_high_8_64"),
            Bitcoin::RightPadLow16_32 => f.write_str("right_pad_low_16_32"),
            Bitcoin::RightPadLow16_64 => f.write_str("right_pad_low_16_64"),
            Bitcoin::RightPadLow1_16 => f.write_str("right_pad_low_1_16"),
            Bitcoin::RightPadLow1_32 => f.write_str("right_pad_low_1_32"),
            Bitcoin::RightPadLow1_64 => f.write_str("right_pad_low_1_64"),
            Bitcoin::RightPadLow1_8 => f.write_str("right_pad_low_1_8"),
            Bitcoin::RightPadLow32_64 => f.write_str("right_pad_low_32_64"),
            Bitcoin::RightPadLow8_16 => f.write_str("right_pad_low_8_16"),
            Bitcoin::RightPadLow8_32 => f.write_str("right_pad_low_8_32"),
            Bitcoin::RightPadLow8_64 => f.write_str("right_pad_low_8_64"),
            Bitcoin::RightRotate16 => f.write_str("right_rotate_16"),
            Bitcoin::RightRotate32 => f.write_str("right_rotate_32"),
            Bitcoin::RightRotate64 => f.write_str("right_rotate_64"),
            Bitcoin::RightRotate8 => f.write_str("right_rotate_8"),
            Bitcoin::RightShift16 => f.write_str("right_shift_16"),
            Bitcoin::RightShift32 => f.write_str("right_shift_32"),
            Bitcoin::RightShift64 => f.write_str("right_shift_64"),
            Bitcoin::RightShift8 => f.write_str("right_shift_8"),
            Bitcoin::RightShiftWith16 => f.write_str("right_shift_with_16"),
            Bitcoin::RightShiftWith32 => f.write_str("right_shift_with_32"),
            Bitcoin::RightShiftWith64 => f.write_str("right_shift_with_64"),
            Bitcoin::RightShiftWith8 => f.write_str("right_shift_with_8"),
            Bitcoin::Rightmost16_1 => f.write_str("rightmost_16_1"),
            Bitcoin::Rightmost16_2 => f.write_str("rightmost_16_2"),
            Bitcoin::Rightmost16_4 => f.write_str("rightmost_16_4"),
            Bitcoin::Rightmost16_8 => f.write_str("rightmost_16_8"),
            Bitcoin::Rightmost32_1 => f.write_str("rightmost_32_1"),
            Bitcoin::Rightmost32_16 => f.write_str("rightmost_32_16"),
            Bitcoin::Rightmost32_2 => f.write_str("rightmost_32_2"),
            Bitcoin::Rightmost32_4 => f.write_str("rightmost_32_4"),
            Bitcoin::Rightmost32_8 => f.write_str("rightmost_32_8"),
            Bitcoin::Rightmost64_1 => f.write_str("rightmost_64_1"),
            Bitcoin::Rightmost64_16 => f.write_str("rightmost_64_16"),
            Bitcoin::Rightmost64_2 => f.write_str("rightmost_64_2"),
            Bitcoin::Rightmost64_32 => f.write_str("rightmost_64_32"),
            Bitcoin::Rightmost64_4 => f.write_str("rightmost_64_4"),
            Bitcoin::Rightmost64_8 => f.write_str("rightmost_64_8"),
            Bitcoin::Rightmost8_1 => f.write_str("rightmost_8_1"),
            Bitcoin::Rightmost8_2 => f.write_str("rightmost_8_2"),
            Bitcoin::Rightmost8_4 => f.write_str("rightmost_8_4"),
            Bitcoin::ScalarAdd => f.write_str("scalar_add"),
            Bitcoin::ScalarInvert => f.write_str("scalar_invert"),
            Bitcoin::ScalarIsZero => f.write_str("scalar_is_zero"),
            Bitcoin::ScalarMultiply => f.write_str("scalar_multiply"),
            Bitcoin::ScalarMultiplyLambda => f.write_str("scalar_multiply_lambda"),
            Bitcoin::ScalarNegate => f.write_str("scalar_negate"),
            Bitcoin::ScalarNormalize => f.write_str("scalar_normalize"),
            Bitcoin::ScalarSquare => f.write_str("scalar_square"),
            Bitcoin::Scale => f.write_str("scale"),
            Bitcoin::ScriptCMR => f.write_str("script_cmr"),
            Bitcoin::Sha256Block => f.write_str("sha_256_block"),
            Bitcoin::Sha256Ctx8Add1 => f.write_str("sha_256_ctx_8_add_1"),
            Bitcoin::Sha256Ctx8Add128 => f.write_str("sha_256_ctx_8_add_128"),
            Bitcoin::Sha256Ctx8Add16 => f.write_str("sha_256_ctx_8_add_16"),
            Bitcoin::Sha256Ctx8Add2 => f.write_str("sha_256_ctx_8_add_2"),
            Bitcoin::Sha256Ctx8Add256 => f.write_str("sha_256_ctx_8_add_256"),
            Bitcoin::Sha256Ctx8Add32 => f.write_str("sha_256_ctx_8_add_32"),
            Bitcoin::Sha256Ctx8Add4 => f.write_str("sha_256_ctx_8_add_4"),
            Bitcoin::Sha256Ctx8Add512 => f.write_str("sha_256_ctx_8_add_512"),
            Bitcoin::Sha256Ctx8Add64 => f.write_str("sha_256_ctx_8_add_64"),
            Bitcoin::Sha256Ctx8Add8 => f.write_str("sha_256_ctx_8_add_8"),
            Bitcoin::Sha256Ctx8AddBuffer511 => f.write_str("sha_256_ctx_8_add_buffer_511"),
            Bitcoin::Sha256Ctx8Finalize => f.write_str("sha_256_ctx_8_finalize"),
            Bitcoin::Sha256Ctx8Init => f.write_str("sha_256_ctx_8_init"),
            Bitcoin::Sha256Iv => f.write_str("sha_256_iv"),
            Bitcoin::SigAllHash => f.write_str("sig_all_hash"),
            Bitcoin::Some1 => f.write_str("some_1"),
            Bitcoin::Some16 => f.write_str("some_16"),
            Bitcoin::Some32 => f.write_str("some_32"),
            Bitcoin::Some64 => f.write_str("some_64"),
            Bitcoin::Some8 => f.write_str("some_8"),
            Bitcoin::Subtract16 => f.write_str("subtract_16"),
            Bitcoin::Subtract32 => f.write_str("subtract_32"),
            Bitcoin::Subtract64 => f.write_str("subtract_64"),
            Bitcoin::Subtract8 => f.write_str("subtract_8"),
            Bitcoin::Swu => f.write_str("swu"),
            Bitcoin::TapEnvHash => f.write_str("tap_env_hash"),
            Bitcoin::TapdataInit => f.write_str("tapdata_init"),
            Bitcoin::TapleafHash => f.write_str("tapleaf_hash"),
            Bitcoin::TapleafVersion => f.write_str("tapleaf_version"),
            Bitcoin::Tappath => f.write_str("tappath"),
            Bitcoin::TappathHash => f.write_str("tappath_hash"),
            Bitcoin::TotalInputValue => f.write_str("total_input_value"),
            Bitcoin::TotalOutputValue => f.write_str("total_output_value"),
            Bitcoin::TransactionId => f.write_str("transaction_id"),
            Bitcoin::TxHash => f.write_str("tx_hash"),
            Bitcoin::TxIsFinal => f.write_str("tx_is_final"),
            Bitcoin::TxLockDistance => f.write_str("tx_lock_distance"),
            Bitcoin::TxLockDuration => f.write_str("tx_lock_duration"),
            Bitcoin::TxLockHeight => f.write_str("tx_lock_height"),
            Bitcoin::TxLockTime => f.write_str("tx_lock_time"),
            Bitcoin::Verify => f.write_str("verify"),
            Bitcoin::Version => f.write_str("version"),
            Bitcoin::Xor1 => f.write_str("xor_1"),
            Bitcoin::Xor16 => f.write_str("xor_16"),
            Bitcoin::Xor32 => f.write_str("xor_32"),
            Bitcoin::Xor64 => f.write_str("xor_64"),
            Bitcoin::Xor8 => f.write_str("xor_8"),
            Bitcoin::XorXor1 => f.write_str("xor_xor_1"),
            Bitcoin::XorXor16 => f.write_str("xor_xor_16"),
            Bitcoin::XorXor32 => f.write_str("xor_xor_32"),
            Bitcoin::XorXor64 => f.write_str("xor_xor_64"),
            Bitcoin::XorXor8 => f.write_str("xor_xor_8"),
        }
    }
}

impl str::FromStr for Bitcoin {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add_16" => Ok(Bitcoin::Add16),
            "add_32" => Ok(Bitcoin::Add32),
            "add_64" => Ok(Bitcoin::Add64),
            "add_8" => Ok(Bitcoin::Add8),
            "all_16" => Ok(Bitcoin::All16),
            "all_32" => Ok(Bitcoin::All32),
            "all_64" => Ok(Bitcoin::All64),
            "all_8" => Ok(Bitcoin::All8),
            "and_1" => Ok(Bitcoin::And1),
            "and_16" => Ok(Bitcoin::And16),
            "and_32" => Ok(Bitcoin::And32),
            "and_64" => Ok(Bitcoin::And64),
            "and_8" => Ok(Bitcoin::And8),
            "annex_hash" => Ok(Bitcoin::AnnexHash),
            "bip_0340_verify" => Ok(Bitcoin::Bip0340Verify),
            "build_tapbranch" => Ok(Bitcoin::BuildTapbranch),
            "build_tapleaf_simplicity" => Ok(Bitcoin::BuildTapleafSimplicity),
            "build_taptweak" => Ok(Bitcoin::BuildTaptweak),
            "ch_1" => Ok(Bitcoin::Ch1),
            "ch_16" => Ok(Bitcoin::Ch16),
            "ch_32" => Ok(Bitcoin::Ch32),
            "ch_64" => Ok(Bitcoin::Ch64),
            "ch_8" => Ok(Bitcoin::Ch8),
            "check_lock_distance" => Ok(Bitcoin::CheckLockDistance),
            "check_lock_duration" => Ok(Bitcoin::CheckLockDuration),
            "check_lock_height" => Ok(Bitcoin::CheckLockHeight),
            "check_lock_time" => Ok(Bitcoin::CheckLockTime),
            "check_sig_verify" => Ok(Bitcoin::CheckSigVerify),
            "complement_1" => Ok(Bitcoin::Complement1),
            "complement_16" => Ok(Bitcoin::Complement16),
            "complement_32" => Ok(Bitcoin::Complement32),
            "complement_64" => Ok(Bitcoin::Complement64),
            "complement_8" => Ok(Bitcoin::Complement8),
            "current_annex_hash" => Ok(Bitcoin::CurrentAnnexHash),
            "current_index" => Ok(Bitcoin::CurrentIndex),
            "current_prev_outpoint" => Ok(Bitcoin::CurrentPrevOutpoint),
            "current_script_hash" => Ok(Bitcoin::CurrentScriptHash),
            "current_script_sig_hash" => Ok(Bitcoin::CurrentScriptSigHash),
            "current_sequence" => Ok(Bitcoin::CurrentSequence),
            "current_value" => Ok(Bitcoin::CurrentValue),
            "decompress" => Ok(Bitcoin::Decompress),
            "decrement_16" => Ok(Bitcoin::Decrement16),
            "decrement_32" => Ok(Bitcoin::Decrement32),
            "decrement_64" => Ok(Bitcoin::Decrement64),
            "decrement_8" => Ok(Bitcoin::Decrement8),
            "div_mod_128_64" => Ok(Bitcoin::DivMod128_64),
            "div_mod_16" => Ok(Bitcoin::DivMod16),
            "div_mod_32" => Ok(Bitcoin::DivMod32),
            "div_mod_64" => Ok(Bitcoin::DivMod64),
            "div_mod_8" => Ok(Bitcoin::DivMod8),
            "divide_16" => Ok(Bitcoin::Divide16),
            "divide_32" => Ok(Bitcoin::Divide32),
            "divide_64" => Ok(Bitcoin::Divide64),
            "divide_8" => Ok(Bitcoin::Divide8),
            "divides_16" => Ok(Bitcoin::Divides16),
            "divides_32" => Ok(Bitcoin::Divides32),
            "divides_64" => Ok(Bitcoin::Divides64),
            "divides_8" => Ok(Bitcoin::Divides8),
            "eq_1" => Ok(Bitcoin::Eq1),
            "eq_16" => Ok(Bitcoin::Eq16),
            "eq_256" => Ok(Bitcoin::Eq256),
            "eq_32" => Ok(Bitcoin::Eq32),
            "eq_64" => Ok(Bitcoin::Eq64),
            "eq_8" => Ok(Bitcoin::Eq8),
            "fe_add" => Ok(Bitcoin::FeAdd),
            "fe_invert" => Ok(Bitcoin::FeInvert),
            "fe_is_odd" => Ok(Bitcoin::FeIsOdd),
            "fe_is_zero" => Ok(Bitcoin::FeIsZero),
            "fe_multiply" => Ok(Bitcoin::FeMultiply),
            "fe_multiply_beta" => Ok(Bitcoin::FeMultiplyBeta),
            "fe_negate" => Ok(Bitcoin::FeNegate),
            "fe_normalize" => Ok(Bitcoin::FeNormalize),
            "fe_square" => Ok(Bitcoin::FeSquare),
            "fe_square_root" => Ok(Bitcoin::FeSquareRoot),
            "fee" => Ok(Bitcoin::Fee),
            "full_add_16" => Ok(Bitcoin::FullAdd16),
            "full_add_32" => Ok(Bitcoin::FullAdd32),
            "full_add_64" => Ok(Bitcoin::FullAdd64),
            "full_add_8" => Ok(Bitcoin::FullAdd8),
            "full_decrement_16" => Ok(Bitcoin::FullDecrement16),
            "full_decrement_32" => Ok(Bitcoin::FullDecrement32),
            "full_decrement_64" => Ok(Bitcoin::FullDecrement64),
            "full_decrement_8" => Ok(Bitcoin::FullDecrement8),
            "full_increment_16" => Ok(Bitcoin::FullIncrement16),
            "full_increment_32" => Ok(Bitcoin::FullIncrement32),
            "full_increment_64" => Ok(Bitcoin::FullIncrement64),
            "full_increment_8" => Ok(Bitcoin::FullIncrement8),
            "full_left_shift_16_1" => Ok(Bitcoin::FullLeftShift16_1),
            "full_left_shift_16_2" => Ok(Bitcoin::FullLeftShift16_2),
            "full_left_shift_16_4" => Ok(Bitcoin::FullLeftShift16_4),
            "full_left_shift_16_8" => Ok(Bitcoin::FullLeftShift16_8),
            "full_left_shift_32_1" => Ok(Bitcoin::FullLeftShift32_1),
            "full_left_shift_32_16" => Ok(Bitcoin::FullLeftShift32_16),
            "full_left_shift_32_2" => Ok(Bitcoin::FullLeftShift32_2),
            "full_left_shift_32_4" => Ok(Bitcoin::FullLeftShift32_4),
            "full_left_shift_32_8" => Ok(Bitcoin::FullLeftShift32_8),
            "full_left_shift_64_1" => Ok(Bitcoin::FullLeftShift64_1),
            "full_left_shift_64_16" => Ok(Bitcoin::FullLeftShift64_16),
            "full_left_shift_64_2" => Ok(Bitcoin::FullLeftShift64_2),
            "full_left_shift_64_32" => Ok(Bitcoin::FullLeftShift64_32),
            "full_left_shift_64_4" => Ok(Bitcoin::FullLeftShift64_4),
            "full_left_shift_64_8" => Ok(Bitcoin::FullLeftShift64_8),
            "full_left_shift_8_1" => Ok(Bitcoin::FullLeftShift8_1),
            "full_left_shift_8_2" => Ok(Bitcoin::FullLeftShift8_2),
            "full_left_shift_8_4" => Ok(Bitcoin::FullLeftShift8_4),
            "full_multiply_16" => Ok(Bitcoin::FullMultiply16),
            "full_multiply_32" => Ok(Bitcoin::FullMultiply32),
            "full_multiply_64" => Ok(Bitcoin::FullMultiply64),
            "full_multiply_8" => Ok(Bitcoin::FullMultiply8),
            "full_right_shift_16_1" => Ok(Bitcoin::FullRightShift16_1),
            "full_right_shift_16_2" => Ok(Bitcoin::FullRightShift16_2),
            "full_right_shift_16_4" => Ok(Bitcoin::FullRightShift16_4),
            "full_right_shift_16_8" => Ok(Bitcoin::FullRightShift16_8),
            "full_right_shift_32_1" => Ok(Bitcoin::FullRightShift32_1),
            "full_right_shift_32_16" => Ok(Bitcoin::FullRightShift32_16),
            "full_right_shift_32_2" => Ok(Bitcoin::FullRightShift32_2),
            "full_right_shift_32_4" => Ok(Bitcoin::FullRightShift32_4),
            "full_right_shift_32_8" => Ok(Bitcoin::FullRightShift32_8),
            "full_right_shift_64_1" => Ok(Bitcoin::FullRightShift64_1),
            "full_right_shift_64_16" => Ok(Bitcoin::FullRightShift64_16),
            "full_right_shift_64_2" => Ok(Bitcoin::FullRightShift64_2),
            "full_right_shift_64_32" => Ok(Bitcoin::FullRightShift64_32),
            "full_right_shift_64_4" => Ok(Bitcoin::FullRightShift64_4),
            "full_right_shift_64_8" => Ok(Bitcoin::FullRightShift64_8),
            "full_right_shift_8_1" => Ok(Bitcoin::FullRightShift8_1),
            "full_right_shift_8_2" => Ok(Bitcoin::FullRightShift8_2),
            "full_right_shift_8_4" => Ok(Bitcoin::FullRightShift8_4),
            "full_subtract_16" => Ok(Bitcoin::FullSubtract16),
            "full_subtract_32" => Ok(Bitcoin::FullSubtract32),
            "full_subtract_64" => Ok(Bitcoin::FullSubtract64),
            "full_subtract_8" => Ok(Bitcoin::FullSubtract8),
            "ge_is_on_curve" => Ok(Bitcoin::GeIsOnCurve),
            "ge_negate" => Ok(Bitcoin::GeNegate),
            "gej_add" => Ok(Bitcoin::GejAdd),
            "gej_double" => Ok(Bitcoin::GejDouble),
            "gej_equiv" => Ok(Bitcoin::GejEquiv),
            "gej_ge_add" => Ok(Bitcoin::GejGeAdd),
            "gej_ge_add_ex" => Ok(Bitcoin::GejGeAddEx),
            "gej_ge_equiv" => Ok(Bitcoin::GejGeEquiv),
            "gej_infinity" => Ok(Bitcoin::GejInfinity),
            "gej_is_infinity" => Ok(Bitcoin::GejIsInfinity),
            "gej_is_on_curve" => Ok(Bitcoin::GejIsOnCurve),
            "gej_negate" => Ok(Bitcoin::GejNegate),
            "gej_normalize" => Ok(Bitcoin::GejNormalize),
            "gej_rescale" => Ok(Bitcoin::GejRescale),
            "gej_x_equiv" => Ok(Bitcoin::GejXEquiv),
            "gej_y_is_odd" => Ok(Bitcoin::GejYIsOdd),
            "generate" => Ok(Bitcoin::Generate),
            "hash_to_curve" => Ok(Bitcoin::HashToCurve),
            "high_1" => Ok(Bitcoin::High1),
            "high_16" => Ok(Bitcoin::High16),
            "high_32" => Ok(Bitcoin::High32),
            "high_64" => Ok(Bitcoin::High64),
            "high_8" => Ok(Bitcoin::High8),
            "increment_16" => Ok(Bitcoin::Increment16),
            "increment_32" => Ok(Bitcoin::Increment32),
            "increment_64" => Ok(Bitcoin::Increment64),
            "increment_8" => Ok(Bitcoin::Increment8),
            "input_annex_hash" => Ok(Bitcoin::InputAnnexHash),
            "input_annexes_hash" => Ok(Bitcoin::InputAnnexesHash),
            "input_hash" => Ok(Bitcoin::InputHash),
            "input_outpoints_hash" => Ok(Bitcoin::InputOutpointsHash),
            "input_prev_outpoint" => Ok(Bitcoin::InputPrevOutpoint),
            "input_script_hash" => Ok(Bitcoin::InputScriptHash),
            "input_script_sig_hash" => Ok(Bitcoin::InputScriptSigHash),
            "input_script_sigs_hash" => Ok(Bitcoin::InputScriptSigsHash),
            "input_scripts_hash" => Ok(Bitcoin::InputScriptsHash),
            "input_sequence" => Ok(Bitcoin::InputSequence),
            "input_sequences_hash" => Ok(Bitcoin::InputSequencesHash),
            "input_utxo_hash" => Ok(Bitcoin::InputUtxoHash),
            "input_utxos_hash" => Ok(Bitcoin::InputUtxosHash),
            "input_value" => Ok(Bitcoin::InputValue),
            "input_values_hash" => Ok(Bitcoin::InputValuesHash),
            "inputs_hash" => Ok(Bitcoin::InputsHash),
            "internal_key" => Ok(Bitcoin::InternalKey),
            "is_one_16" => Ok(Bitcoin::IsOne16),
            "is_one_32" => Ok(Bitcoin::IsOne32),
            "is_one_64" => Ok(Bitcoin::IsOne64),
            "is_one_8" => Ok(Bitcoin::IsOne8),
            "is_zero_16" => Ok(Bitcoin::IsZero16),
            "is_zero_32" => Ok(Bitcoin::IsZero32),
            "is_zero_64" => Ok(Bitcoin::IsZero64),
            "is_zero_8" => Ok(Bitcoin::IsZero8),
            "le_16" => Ok(Bitcoin::Le16),
            "le_32" => Ok(Bitcoin::Le32),
            "le_64" => Ok(Bitcoin::Le64),
            "le_8" => Ok(Bitcoin::Le8),
            "left_extend_16_32" => Ok(Bitcoin::LeftExtend16_32),
            "left_extend_16_64" => Ok(Bitcoin::LeftExtend16_64),
            "left_extend_1_16" => Ok(Bitcoin::LeftExtend1_16),
            "left_extend_1_32" => Ok(Bitcoin::LeftExtend1_32),
            "left_extend_1_64" => Ok(Bitcoin::LeftExtend1_64),
            "left_extend_1_8" => Ok(Bitcoin::LeftExtend1_8),
            "left_extend_32_64" => Ok(Bitcoin::LeftExtend32_64),
            "left_extend_8_16" => Ok(Bitcoin::LeftExtend8_16),
            "left_extend_8_32" => Ok(Bitcoin::LeftExtend8_32),
            "left_extend_8_64" => Ok(Bitcoin::LeftExtend8_64),
            "left_pad_high_16_32" => Ok(Bitcoin::LeftPadHigh16_32),
            "left_pad_high_16_64" => Ok(Bitcoin::LeftPadHigh16_64),
            "left_pad_high_1_16" => Ok(Bitcoin::LeftPadHigh1_16),
            "left_pad_high_1_32" => Ok(Bitcoin::LeftPadHigh1_32),
            "left_pad_high_1_64" => Ok(Bitcoin::LeftPadHigh1_64),
            "left_pad_high_1_8" => Ok(Bitcoin::LeftPadHigh1_8),
            "left_pad_high_32_64" => Ok(Bitcoin::LeftPadHigh32_64),
            "left_pad_high_8_16" => Ok(Bitcoin::LeftPadHigh8_16),
            "left_pad_high_8_32" => Ok(Bitcoin::LeftPadHigh8_32),
            "left_pad_high_8_64" => Ok(Bitcoin::LeftPadHigh8_64),
            "left_pad_low_16_32" => Ok(Bitcoin::LeftPadLow16_32),
            "left_pad_low_16_64" => Ok(Bitcoin::LeftPadLow16_64),
            "left_pad_low_1_16" => Ok(Bitcoin::LeftPadLow1_16),
            "left_pad_low_1_32" => Ok(Bitcoin::LeftPadLow1_32),
            "left_pad_low_1_64" => Ok(Bitcoin::LeftPadLow1_64),
            "left_pad_low_1_8" => Ok(Bitcoin::LeftPadLow1_8),
            "left_pad_low_32_64" => Ok(Bitcoin::LeftPadLow32_64),
            "left_pad_low_8_16" => Ok(Bitcoin::LeftPadLow8_16),
            "left_pad_low_8_32" => Ok(Bitcoin::LeftPadLow8_32),
            "left_pad_low_8_64" => Ok(Bitcoin::LeftPadLow8_64),
            "left_rotate_16" => Ok(Bitcoin::LeftRotate16),
            "left_rotate_32" => Ok(Bitcoin::LeftRotate32),
            "left_rotate_64" => Ok(Bitcoin::LeftRotate64),
            "left_rotate_8" => Ok(Bitcoin::LeftRotate8),
            "left_shift_16" => Ok(Bitcoin::LeftShift16),
            "left_shift_32" => Ok(Bitcoin::LeftShift32),
            "left_shift_64" => Ok(Bitcoin::LeftShift64),
            "left_shift_8" => Ok(Bitcoin::LeftShift8),
            "left_shift_with_16" => Ok(Bitcoin::LeftShiftWith16),
            "left_shift_with_32" => Ok(Bitcoin::LeftShiftWith32),
            "left_shift_with_64" => Ok(Bitcoin::LeftShiftWith64),
            "left_shift_with_8" => Ok(Bitcoin::LeftShiftWith8),
            "leftmost_16_1" => Ok(Bitcoin::Leftmost16_1),
            "leftmost_16_2" => Ok(Bitcoin::Leftmost16_2),
            "leftmost_16_4" => Ok(Bitcoin::Leftmost16_4),
            "leftmost_16_8" => Ok(Bitcoin::Leftmost16_8),
            "leftmost_32_1" => Ok(Bitcoin::Leftmost32_1),
            "leftmost_32_16" => Ok(Bitcoin::Leftmost32_16),
            "leftmost_32_2" => Ok(Bitcoin::Leftmost32_2),
            "leftmost_32_4" => Ok(Bitcoin::Leftmost32_4),
            "leftmost_32_8" => Ok(Bitcoin::Leftmost32_8),
            "leftmost_64_1" => Ok(Bitcoin::Leftmost64_1),
            "leftmost_64_16" => Ok(Bitcoin::Leftmost64_16),
            "leftmost_64_2" => Ok(Bitcoin::Leftmost64_2),
            "leftmost_64_32" => Ok(Bitcoin::Leftmost64_32),
            "leftmost_64_4" => Ok(Bitcoin::Leftmost64_4),
            "leftmost_64_8" => Ok(Bitcoin::Leftmost64_8),
            "leftmost_8_1" => Ok(Bitcoin::Leftmost8_1),
            "leftmost_8_2" => Ok(Bitcoin::Leftmost8_2),
            "leftmost_8_4" => Ok(Bitcoin::Leftmost8_4),
            "linear_combination_1" => Ok(Bitcoin::LinearCombination1),
            "linear_verify_1" => Ok(Bitcoin::LinearVerify1),
            "lock_time" => Ok(Bitcoin::LockTime),
            "low_1" => Ok(Bitcoin::Low1),
            "low_16" => Ok(Bitcoin::Low16),
            "low_32" => Ok(Bitcoin::Low32),
            "low_64" => Ok(Bitcoin::Low64),
            "low_8" => Ok(Bitcoin::Low8),
            "lt_16" => Ok(Bitcoin::Lt16),
            "lt_32" => Ok(Bitcoin::Lt32),
            "lt_64" => Ok(Bitcoin::Lt64),
            "lt_8" => Ok(Bitcoin::Lt8),
            "maj_1" => Ok(Bitcoin::Maj1),
            "maj_16" => Ok(Bitcoin::Maj16),
            "maj_32" => Ok(Bitcoin::Maj32),
            "maj_64" => Ok(Bitcoin::Maj64),
            "maj_8" => Ok(Bitcoin::Maj8),
            "max_16" => Ok(Bitcoin::Max16),
            "max_32" => Ok(Bitcoin::Max32),
            "max_64" => Ok(Bitcoin::Max64),
            "max_8" => Ok(Bitcoin::Max8),
            "median_16" => Ok(Bitcoin::Median16),
            "median_32" => Ok(Bitcoin::Median32),
            "median_64" => Ok(Bitcoin::Median64),
            "median_8" => Ok(Bitcoin::Median8),
            "min_16" => Ok(Bitcoin::Min16),
            "min_32" => Ok(Bitcoin::Min32),
            "min_64" => Ok(Bitcoin::Min64),
            "min_8" => Ok(Bitcoin::Min8),
            "modulo_16" => Ok(Bitcoin::Modulo16),
            "modulo_32" => Ok(Bitcoin::Modulo32),
            "modulo_64" => Ok(Bitcoin::Modulo64),
            "modulo_8" => Ok(Bitcoin::Modulo8),
            "multiply_16" => Ok(Bitcoin::Multiply16),
            "multiply_32" => Ok(Bitcoin::Multiply32),
            "multiply_64" => Ok(Bitcoin::Multiply64),
            "multiply_8" => Ok(Bitcoin::Multiply8),
            "negate_16" => Ok(Bitcoin::Negate16),
            "negate_32" => Ok(Bitcoin::Negate32),
            "negate_64" => Ok(Bitcoin::Negate64),
            "negate_8" => Ok(Bitcoin::Negate8),
            "num_inputs" => Ok(Bitcoin::NumInputs),
            "num_outputs" => Ok(Bitcoin::NumOutputs),
            "one_16" => Ok(Bitcoin::One16),
            "one_32" => Ok(Bitcoin::One32),
            "one_64" => Ok(Bitcoin::One64),
            "one_8" => Ok(Bitcoin::One8),
            "or_1" => Ok(Bitcoin::Or1),
            "or_16" => Ok(Bitcoin::Or16),
            "or_32" => Ok(Bitcoin::Or32),
            "or_64" => Ok(Bitcoin::Or64),
            "or_8" => Ok(Bitcoin::Or8),
            "outpoint_hash" => Ok(Bitcoin::OutpointHash),
            "output_hash" => Ok(Bitcoin::OutputHash),
            "output_script_hash" => Ok(Bitcoin::OutputScriptHash),
            "output_scripts_hash" => Ok(Bitcoin::OutputScriptsHash),
            "output_value" => Ok(Bitcoin::OutputValue),
            "output_values_hash" => Ok(Bitcoin::OutputValuesHash),
            "outputs_hash" => Ok(Bitcoin::OutputsHash),
            "parse_lock" => Ok(Bitcoin::ParseLock),
            "parse_sequence" => Ok(Bitcoin::ParseSequence),
            "point_verify_1" => Ok(Bitcoin::PointVerify1),
            "right_extend_16_32" => Ok(Bitcoin::RightExtend16_32),
            "right_extend_16_64" => Ok(Bitcoin::RightExtend16_64),
            "right_extend_32_64" => Ok(Bitcoin::RightExtend32_64),
            "right_extend_8_16" => Ok(Bitcoin::RightExtend8_16),
            "right_extend_8_32" => Ok(Bitcoin::RightExtend8_32),
            "right_extend_8_64" => Ok(Bitcoin::RightExtend8_64),
            "right_pad_high_16_32" => Ok(Bitcoin::RightPadHigh16_32),
            "right_pad_high_16_64" => Ok(Bitcoin::RightPadHigh16_64),
            "right_pad_high_1_16" => Ok(Bitcoin::RightPadHigh1_16),
            "right_pad_high_1_32" => Ok(Bitcoin::RightPadHigh1_32),
            "right_pad_high_1_64" => Ok(Bitcoin::RightPadHigh1_64),
            "right_pad_high_1_8" => Ok(Bitcoin::RightPadHigh1_8),
            "right_pad_high_32_64" => Ok(Bitcoin::RightPadHigh32_64),
            "right_pad_high_8_16" => Ok(Bitcoin::RightPadHigh8_16),
            "right_pad_high_8_32" => Ok(Bitcoin::RightPadHigh8_32),
            "right_pad_high_8_64" => Ok(Bitcoin::RightPadHigh8_64),
            "right_pad_low_16_32" => Ok(Bitcoin::RightPadLow16_32),
            "right_pad_low_16_64" => Ok(Bitcoin::RightPadLow16_64),
            "right_pad_low_1_16" => Ok(Bitcoin::RightPadLow1_16),
            "right_pad_low_1_32" => Ok(Bitcoin::RightPadLow1_32),
            "right_pad_low_1_64" => Ok(Bitcoin::RightPadLow1_64),
            "right_pad_low_1_8" => Ok(Bitcoin::RightPadLow1_8),
            "right_pad_low_32_64" => Ok(Bitcoin::RightPadLow32_64),
            "right_pad_low_8_16" => Ok(Bitcoin::RightPadLow8_16),
            "right_pad_low_8_32" => Ok(Bitcoin::RightPadLow8_32),
            "right_pad_low_8_64" => Ok(Bitcoin::RightPadLow8_64),
            "right_rotate_16" => Ok(Bitcoin::RightRotate16),
            "right_rotate_32" => Ok(Bitcoin::RightRotate32),
            "right_rotate_64" => Ok(Bitcoin::RightRotate64),
            "right_rotate_8" => Ok(Bitcoin::RightRotate8),
            "right_shift_16" => Ok(Bitcoin::RightShift16),
            "right_shift_32" => Ok(Bitcoin::RightShift32),
            "right_shift_64" => Ok(Bitcoin::RightShift64),
            "right_shift_8" => Ok(Bitcoin::RightShift8),
            "right_shift_with_16" => Ok(Bitcoin::RightShiftWith16),
            "right_shift_with_32" => Ok(Bitcoin::RightShiftWith32),
            "right_shift_with_64" => Ok(Bitcoin::RightShiftWith64),
            "right_shift_with_8" => Ok(Bitcoin::RightShiftWith8),
            "rightmost_16_1" => Ok(Bitcoin::Rightmost16_1),
            "rightmost_16_2" => Ok(Bitcoin::Rightmost16_2),
            "rightmost_16_4" => Ok(Bitcoin::Rightmost16_4),
            "rightmost_16_8" => Ok(Bitcoin::Rightmost16_8),
            "rightmost_32_1" => Ok(Bitcoin::Rightmost32_1),
            "rightmost_32_16" => Ok(Bitcoin::Rightmost32_16),
            "rightmost_32_2" => Ok(Bitcoin::Rightmost32_2),
            "rightmost_32_4" => Ok(Bitcoin::Rightmost32_4),
            "rightmost_32_8" => Ok(Bitcoin::Rightmost32_8),
            "rightmost_64_1" => Ok(Bitcoin::Rightmost64_1),
            "rightmost_64_16" => Ok(Bitcoin::Rightmost64_16),
            "rightmost_64_2" => Ok(Bitcoin::Rightmost64_2),
            "rightmost_64_32" => Ok(Bitcoin::Rightmost64_32),
            "rightmost_64_4" => Ok(Bitcoin::Rightmost64_4),
            "rightmost_64_8" => Ok(Bitcoin::Rightmost64_8),
            "rightmost_8_1" => Ok(Bitcoin::Rightmost8_1),
            "rightmost_8_2" => Ok(Bitcoin::Rightmost8_2),
            "rightmost_8_4" => Ok(Bitcoin::Rightmost8_4),
            "scalar_add" => Ok(Bitcoin::ScalarAdd),
            "scalar_invert" => Ok(Bitcoin::ScalarInvert),
            "scalar_is_zero" => Ok(Bitcoin::ScalarIsZero),
            "scalar_multiply" => Ok(Bitcoin::ScalarMultiply),
            "scalar_multiply_lambda" => Ok(Bitcoin::ScalarMultiplyLambda),
            "scalar_negate" => Ok(Bitcoin::ScalarNegate),
            "scalar_normalize" => Ok(Bitcoin::ScalarNormalize),
            "scalar_square" => Ok(Bitcoin::ScalarSquare),
            "scale" => Ok(Bitcoin::Scale),
            "script_cmr" => Ok(Bitcoin::ScriptCMR),
            "sha_256_block" => Ok(Bitcoin::Sha256Block),
            "sha_256_ctx_8_add_1" => Ok(Bitcoin::Sha256Ctx8Add1),
            "sha_256_ctx_8_add_128" => Ok(Bitcoin::Sha256Ctx8Add128),
            "sha_256_ctx_8_add_16" => Ok(Bitcoin::Sha256Ctx8Add16),
            "sha_256_ctx_8_add_2" => Ok(Bitcoin::Sha256Ctx8Add2),
            "sha_256_ctx_8_add_256" => Ok(Bitcoin::Sha256Ctx8Add256),
            "sha_256_ctx_8_add_32" => Ok(Bitcoin::Sha256Ctx8Add32),
            "sha_256_ctx_8_add_4" => Ok(Bitcoin::Sha256Ctx8Add4),
            "sha_256_ctx_8_add_512" => Ok(Bitcoin::Sha256Ctx8Add512),
            "sha_256_ctx_8_add_64" => Ok(Bitcoin::Sha256Ctx8Add64),
            "sha_256_ctx_8_add_8" => Ok(Bitcoin::Sha256Ctx8Add8),
            "sha_256_ctx_8_add_buffer_511" => Ok(Bitcoin::Sha256Ctx8AddBuffer511),
            "sha_256_ctx_8_finalize" => Ok(Bitcoin::Sha256Ctx8Finalize),
            "sha_256_ctx_8_init" => Ok(Bitcoin::Sha256Ctx8Init),
            "sha_256_iv" => Ok(Bitcoin::Sha256Iv),
            "sig_all_hash" => Ok(Bitcoin::SigAllHash),
            "some_1" => Ok(Bitcoin::Some1),
            "some_16" => Ok(Bitcoin::Some16),
            "some_32" => Ok(Bitcoin::Some32),
            "some_64" => Ok(Bitcoin::Some64),
            "some_8" => Ok(Bitcoin::Some8),
            "subtract_16" => Ok(Bitcoin::Subtract16),
            "subtract_32" => Ok(Bitcoin::Subtract32),
            "subtract_64" => Ok(Bitcoin::Subtract64),
            "subtract_8" => Ok(Bitcoin::Subtract8),
            "swu" => Ok(Bitcoin::Swu),
            "tap_env_hash" => Ok(Bitcoin::TapEnvHash),
            "tapdata_init" => Ok(Bitcoin::TapdataInit),
            "tapleaf_hash" => Ok(Bitcoin::TapleafHash),
            "tapleaf_version" => Ok(Bitcoin::TapleafVersion),
            "tappath" => Ok(Bitcoin::Tappath),
            "tappath_hash" => Ok(Bitcoin::TappathHash),
            "total_input_value" => Ok(Bitcoin::TotalInputValue),
            "total_output_value" => Ok(Bitcoin::TotalOutputValue),
            "transaction_id" => Ok(Bitcoin::TransactionId),
            "tx_hash" => Ok(Bitcoin::TxHash),
            "tx_is_final" => Ok(Bitcoin::TxIsFinal),
            "tx_lock_distance" => Ok(Bitcoin::TxLockDistance),
            "tx_lock_duration" => Ok(Bitcoin::TxLockDuration),
            "tx_lock_height" => Ok(Bitcoin::TxLockHeight),
            "tx_lock_time" => Ok(Bitcoin::TxLockTime),
            "verify" => Ok(Bitcoin::Verify),
            "version" => Ok(Bitcoin::Version),
            "xor_1" => Ok(Bitcoin::Xor1),
            "xor_16" => Ok(Bitcoin::Xor16),
            "xor_32" => Ok(Bitcoin::Xor32),
            "xor_64" => Ok(Bitcoin::Xor64),
            "xor_8" => Ok(Bitcoin::Xor8),
            "xor_xor_1" => Ok(Bitcoin::XorXor1),
            "xor_xor_16" => Ok(Bitcoin::XorXor16),
            "xor_xor_32" => Ok(Bitcoin::XorXor32),
            "xor_xor_64" => Ok(Bitcoin::XorXor64),
            "xor_xor_8" => Ok(Bitcoin::XorXor8),
            x => Err(crate::Error::InvalidJetName(x.to_owned())),
        }
    }
}
