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
use crate::jet::elements::ElementsEnv;
use simplicity_sys::elements::CTxEnv;

/// The Elements jet family.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Elements {
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
    AssetAmountHash,
    Bip0340Verify,
    BuildTapbranch,
    BuildTapleafSimplicity,
    BuildTaptweak,
    CalculateAsset,
    CalculateConfidentialToken,
    CalculateExplicitToken,
    CalculateIssuanceEntropy,
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
    CurrentAmount,
    CurrentAnnexHash,
    CurrentAsset,
    CurrentIndex,
    CurrentIssuanceAssetAmount,
    CurrentIssuanceAssetProof,
    CurrentIssuanceTokenAmount,
    CurrentIssuanceTokenProof,
    CurrentNewIssuanceContract,
    CurrentPegin,
    CurrentPrevOutpoint,
    CurrentReissuanceBlinding,
    CurrentReissuanceEntropy,
    CurrentScriptHash,
    CurrentScriptSigHash,
    CurrentSequence,
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
    GenesisBlockHash,
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
    InputAmount,
    InputAmountsHash,
    InputAnnexHash,
    InputAnnexesHash,
    InputAsset,
    InputHash,
    InputOutpointsHash,
    InputPegin,
    InputPrevOutpoint,
    InputScriptHash,
    InputScriptSigHash,
    InputScriptSigsHash,
    InputScriptsHash,
    InputSequence,
    InputSequencesHash,
    InputUtxoHash,
    InputUtxosHash,
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
    Issuance,
    IssuanceAsset,
    IssuanceAssetAmount,
    IssuanceAssetAmountsHash,
    IssuanceAssetProof,
    IssuanceBlindingEntropyHash,
    IssuanceEntropy,
    IssuanceHash,
    IssuanceRangeProofsHash,
    IssuanceToken,
    IssuanceTokenAmount,
    IssuanceTokenAmountsHash,
    IssuanceTokenProof,
    IssuancesHash,
    LbtcAsset,
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
    NewIssuanceContract,
    NonceHash,
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
    OutputAmount,
    OutputAmountsHash,
    OutputAsset,
    OutputHash,
    OutputIsFee,
    OutputNonce,
    OutputNoncesHash,
    OutputNullDatum,
    OutputRangeProof,
    OutputRangeProofsHash,
    OutputScriptHash,
    OutputScriptsHash,
    OutputSurjectionProof,
    OutputSurjectionProofsHash,
    OutputsHash,
    ParseLock,
    ParseSequence,
    PointVerify1,
    ReissuanceBlinding,
    ReissuanceEntropy,
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
    TotalFee,
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

impl Elements {
    /// Array of all Elements jets.
    pub const ALL: [Self; 471] = [
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
        Self::AssetAmountHash,
        Self::Bip0340Verify,
        Self::BuildTapbranch,
        Self::BuildTapleafSimplicity,
        Self::BuildTaptweak,
        Self::CalculateAsset,
        Self::CalculateConfidentialToken,
        Self::CalculateExplicitToken,
        Self::CalculateIssuanceEntropy,
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
        Self::CurrentAmount,
        Self::CurrentAnnexHash,
        Self::CurrentAsset,
        Self::CurrentIndex,
        Self::CurrentIssuanceAssetAmount,
        Self::CurrentIssuanceAssetProof,
        Self::CurrentIssuanceTokenAmount,
        Self::CurrentIssuanceTokenProof,
        Self::CurrentNewIssuanceContract,
        Self::CurrentPegin,
        Self::CurrentPrevOutpoint,
        Self::CurrentReissuanceBlinding,
        Self::CurrentReissuanceEntropy,
        Self::CurrentScriptHash,
        Self::CurrentScriptSigHash,
        Self::CurrentSequence,
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
        Self::GenesisBlockHash,
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
        Self::InputAmount,
        Self::InputAmountsHash,
        Self::InputAnnexHash,
        Self::InputAnnexesHash,
        Self::InputAsset,
        Self::InputHash,
        Self::InputOutpointsHash,
        Self::InputPegin,
        Self::InputPrevOutpoint,
        Self::InputScriptHash,
        Self::InputScriptSigHash,
        Self::InputScriptSigsHash,
        Self::InputScriptsHash,
        Self::InputSequence,
        Self::InputSequencesHash,
        Self::InputUtxoHash,
        Self::InputUtxosHash,
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
        Self::Issuance,
        Self::IssuanceAsset,
        Self::IssuanceAssetAmount,
        Self::IssuanceAssetAmountsHash,
        Self::IssuanceAssetProof,
        Self::IssuanceBlindingEntropyHash,
        Self::IssuanceEntropy,
        Self::IssuanceHash,
        Self::IssuanceRangeProofsHash,
        Self::IssuanceToken,
        Self::IssuanceTokenAmount,
        Self::IssuanceTokenAmountsHash,
        Self::IssuanceTokenProof,
        Self::IssuancesHash,
        Self::LbtcAsset,
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
        Self::NewIssuanceContract,
        Self::NonceHash,
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
        Self::OutputAmount,
        Self::OutputAmountsHash,
        Self::OutputAsset,
        Self::OutputHash,
        Self::OutputIsFee,
        Self::OutputNonce,
        Self::OutputNoncesHash,
        Self::OutputNullDatum,
        Self::OutputRangeProof,
        Self::OutputRangeProofsHash,
        Self::OutputScriptHash,
        Self::OutputScriptsHash,
        Self::OutputSurjectionProof,
        Self::OutputSurjectionProofsHash,
        Self::OutputsHash,
        Self::ParseLock,
        Self::ParseSequence,
        Self::PointVerify1,
        Self::ReissuanceBlinding,
        Self::ReissuanceEntropy,
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
        Self::TotalFee,
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

impl Jet for Elements {

    type Transaction = elements::Transaction;
    type Environment<T> = ElementsEnv<T> where T: Borrow<Self::Transaction>;
    type CJetEnvironment = CTxEnv;

    fn c_jet_env<T>(env: &Self::Environment<T>) -> &Self::CJetEnvironment
        where T: Borrow<Self::Transaction>
    {
        env.c_tx_env()
    }

    fn cmr(&self) -> Cmr {
        let bytes = match self {
            Elements::Add16 => [
                0x26, 0xae, 0x09, 0x94, 0xce, 0x8b, 0x77, 0x1a, 0xf7, 0xad, 0x28, 0x51, 0xb8, 0x3b,
                0x49, 0xa5, 0x95, 0x05, 0x36, 0x58, 0x9f, 0x67, 0xbd, 0x85, 0x59, 0x47, 0x04, 0x60,
                0x29, 0x75, 0x1c, 0x0d,
            ],
            Elements::Add32 => [
                0x3d, 0x76, 0x74, 0x46, 0x6e, 0xd6, 0x9e, 0x1d, 0xbe, 0xdc, 0xd4, 0x80, 0x57, 0xa9,
                0xe6, 0x28, 0x8c, 0x22, 0x25, 0x32, 0xfb, 0xc5, 0x04, 0x80, 0x49, 0x92, 0x8c, 0xfb,
                0x77, 0xf8, 0x29, 0xd9,
            ],
            Elements::Add64 => [
                0x9b, 0x56, 0xe6, 0x1e, 0xef, 0xe2, 0x80, 0x5c, 0xa8, 0x73, 0x96, 0xbd, 0xfb, 0x03,
                0xf5, 0xe1, 0xb1, 0x38, 0x5f, 0x7a, 0xc4, 0xbf, 0xf7, 0x68, 0x40, 0x26, 0xa0, 0x7c,
                0xf9, 0x7f, 0xb6, 0xf6,
            ],
            Elements::Add8 => [
                0xd7, 0x32, 0x8c, 0x09, 0x14, 0xee, 0x99, 0x9e, 0xfa, 0x0a, 0x6c, 0xb2, 0x6e, 0xb4,
                0x09, 0x12, 0xc2, 0x15, 0xc0, 0x62, 0xe5, 0x8a, 0x98, 0x1a, 0xe6, 0xb2, 0xe4, 0xa8,
                0x04, 0x74, 0xa1, 0xda,
            ],
            Elements::All16 => [
                0x16, 0xf0, 0xc9, 0x30, 0x7e, 0xb8, 0xf4, 0xc1, 0xfd, 0xd1, 0xba, 0xfa, 0xef, 0x28,
                0x79, 0x24, 0x29, 0x58, 0x49, 0x8e, 0x8f, 0x5b, 0x2e, 0x0d, 0x29, 0xf0, 0x65, 0x53,
                0xdc, 0x06, 0xa0, 0xbd,
            ],
            Elements::All32 => [
                0x0e, 0xb8, 0xb4, 0x0d, 0x29, 0x02, 0x17, 0x47, 0xee, 0xc4, 0x51, 0xd4, 0xe6, 0x63,
                0x58, 0x6a, 0x43, 0x6c, 0x2d, 0xb0, 0x93, 0x26, 0x75, 0xda, 0xf2, 0x16, 0x61, 0x23,
                0xbf, 0xe4, 0x52, 0xa6,
            ],
            Elements::All64 => [
                0xa6, 0x5c, 0x82, 0xd5, 0x3d, 0x38, 0x2e, 0xe2, 0x9a, 0xa8, 0x8b, 0x77, 0x18, 0xa9,
                0x7f, 0xbb, 0xce, 0x64, 0x75, 0xec, 0x32, 0xc4, 0xb4, 0xcd, 0x69, 0x08, 0xfd, 0xe4,
                0x5d, 0x81, 0xb6, 0x24,
            ],
            Elements::All8 => [
                0x1d, 0x3e, 0xc7, 0xfb, 0x6a, 0x07, 0x84, 0x7c, 0x92, 0xb8, 0xa9, 0x98, 0xe1, 0xf6,
                0xb4, 0x78, 0x31, 0x9d, 0x05, 0x0a, 0x38, 0x76, 0x42, 0xf4, 0x03, 0x2d, 0x2f, 0x7d,
                0x2e, 0x02, 0x7f, 0xcd,
            ],
            Elements::And1 => [
                0xb7, 0x73, 0xce, 0xfa, 0x41, 0x89, 0x57, 0xfe, 0xa7, 0xdf, 0xb4, 0x9c, 0x6c, 0x43,
                0xb3, 0xdb, 0xfa, 0x35, 0xfa, 0x3d, 0x80, 0xde, 0x8c, 0xfd, 0x4d, 0x70, 0xc0, 0x8d,
                0x94, 0x5f, 0x5f, 0xba,
            ],
            Elements::And16 => [
                0x57, 0xdd, 0x73, 0x0b, 0x1c, 0x8d, 0xdf, 0xf1, 0x3c, 0xae, 0x27, 0x69, 0x56, 0x2b,
                0xe0, 0xab, 0xc6, 0xca, 0x3b, 0xc8, 0x02, 0xda, 0x0a, 0xbb, 0xb7, 0xfc, 0x13, 0x8c,
                0xa4, 0x63, 0xda, 0x59,
            ],
            Elements::And32 => [
                0x75, 0x3e, 0x33, 0x2d, 0xdf, 0xa0, 0x96, 0xf0, 0x83, 0x99, 0xff, 0xaa, 0x7e, 0xc4,
                0xda, 0x40, 0x35, 0xbc, 0xba, 0xa1, 0x42, 0xe6, 0xe3, 0x8d, 0x4c, 0xb6, 0x07, 0xce,
                0x1f, 0x0b, 0x05, 0x1d,
            ],
            Elements::And64 => [
                0xf1, 0xad, 0x5e, 0x6c, 0x63, 0xee, 0x5c, 0x89, 0x0b, 0x0f, 0x2e, 0x71, 0x15, 0x61,
                0xb9, 0x05, 0x31, 0x64, 0x87, 0xac, 0x40, 0x44, 0xdd, 0x23, 0x0c, 0xf6, 0xa7, 0x36,
                0xf8, 0x1b, 0xd4, 0xf3,
            ],
            Elements::And8 => [
                0xac, 0x82, 0x8b, 0x72, 0x4c, 0x5f, 0x53, 0x40, 0xb5, 0x1e, 0x76, 0xe7, 0xb6, 0xe8,
                0xb2, 0x3a, 0xea, 0xb7, 0x53, 0x3f, 0xd8, 0xc0, 0x91, 0xae, 0x2a, 0x51, 0x55, 0x30,
                0xae, 0x7a, 0xb2, 0x00,
            ],
            Elements::AnnexHash => [
                0xeb, 0xa1, 0xb7, 0xfb, 0x2e, 0xa8, 0xd5, 0x7a, 0xf3, 0xb6, 0x27, 0x3e, 0x51, 0x82,
                0xfb, 0x99, 0xdb, 0x17, 0x08, 0xb2, 0xd0, 0xaf, 0xb9, 0xc6, 0xad, 0x03, 0x58, 0xcc,
                0x3f, 0x64, 0x7d, 0xc8,
            ],
            Elements::AssetAmountHash => [
                0xa8, 0x64, 0xd1, 0xf1, 0xf8, 0x39, 0x50, 0xb6, 0x31, 0x96, 0xfe, 0x88, 0xba, 0xec,
                0x2e, 0x42, 0x28, 0x95, 0x32, 0x30, 0xbe, 0x45, 0xc8, 0x8c, 0x4e, 0x0b, 0x99, 0x17,
                0x8b, 0x33, 0x8f, 0x07,
            ],
            Elements::Bip0340Verify => [
                0xc9, 0xc4, 0x5a, 0x8a, 0xec, 0x86, 0x59, 0x14, 0x3b, 0xfe, 0x2a, 0xf6, 0xea, 0xd4,
                0x8d, 0x4e, 0x05, 0x42, 0x45, 0x3a, 0xca, 0xe8, 0x4b, 0x9b, 0xbb, 0x97, 0x65, 0x6b,
                0x67, 0x0b, 0xdf, 0xdd,
            ],
            Elements::BuildTapbranch => [
                0xcb, 0xec, 0xf9, 0xbc, 0xe1, 0x72, 0xc5, 0x0f, 0x58, 0x59, 0x51, 0xdf, 0xf0, 0xe0,
                0x52, 0x3d, 0xb1, 0x09, 0xe5, 0x70, 0x25, 0xec, 0x37, 0xde, 0x2c, 0x3a, 0x74, 0xd4,
                0xa6, 0x73, 0xf2, 0x25,
            ],
            Elements::BuildTapleafSimplicity => [
                0x73, 0x6e, 0x9f, 0xce, 0xfb, 0x5b, 0x87, 0x3e, 0x09, 0xf2, 0x68, 0x1e, 0x74, 0xe4,
                0x9f, 0x5a, 0x10, 0x56, 0xf9, 0x0a, 0xf5, 0x2a, 0xb3, 0xec, 0x5b, 0x19, 0x77, 0x27,
                0xd3, 0x98, 0x34, 0xbe,
            ],
            Elements::BuildTaptweak => [
                0x38, 0x74, 0x1f, 0x80, 0xa2, 0xbf, 0x10, 0xf8, 0xf8, 0x72, 0x30, 0x77, 0xc6, 0x74,
                0x1c, 0xbe, 0xae, 0x2d, 0xca, 0xc8, 0x57, 0x90, 0x1b, 0x81, 0x37, 0x25, 0x80, 0x6f,
                0x21, 0x89, 0x8e, 0xe3,
            ],
            Elements::CalculateAsset => [
                0x67, 0x6f, 0x98, 0x8e, 0x54, 0x72, 0x51, 0xe7, 0xa0, 0x9e, 0xed, 0xe1, 0x92, 0x0a,
                0x9c, 0x5d, 0x35, 0xe7, 0x57, 0x59, 0xe6, 0xd8, 0xd4, 0x9b, 0x58, 0x0c, 0xeb, 0x3e,
                0x14, 0x6b, 0xcf, 0x7e,
            ],
            Elements::CalculateConfidentialToken => [
                0x8e, 0x40, 0x29, 0x05, 0x60, 0x74, 0x33, 0x98, 0x40, 0xc0, 0xe7, 0x00, 0x9b, 0x66,
                0xf3, 0x82, 0xbb, 0x98, 0xd7, 0xdd, 0xe7, 0x19, 0xdd, 0x2c, 0x9f, 0x2f, 0xd3, 0xac,
                0xd9, 0x32, 0x5a, 0xc6,
            ],
            Elements::CalculateExplicitToken => [
                0x87, 0x52, 0x28, 0x02, 0x0b, 0x45, 0x6c, 0x99, 0x8a, 0xad, 0x3a, 0xc5, 0xc4, 0x83,
                0x6f, 0x71, 0x94, 0x45, 0xdc, 0xbe, 0x6b, 0xd2, 0x74, 0xe8, 0xca, 0x14, 0x5d, 0x82,
                0x79, 0x7e, 0x1b, 0x92,
            ],
            Elements::CalculateIssuanceEntropy => [
                0xa6, 0xe7, 0xc4, 0x34, 0xae, 0xe5, 0xa7, 0x33, 0x0c, 0x1f, 0xc2, 0x46, 0xf7, 0x1d,
                0xcf, 0x05, 0x1c, 0xe5, 0x4b, 0xde, 0xe7, 0xb8, 0xe6, 0x39, 0x57, 0xbe, 0x18, 0xc2,
                0xd7, 0x51, 0xc6, 0x53,
            ],
            Elements::Ch1 => [
                0xb8, 0x41, 0xb8, 0x57, 0xa4, 0xaa, 0x50, 0xea, 0xca, 0x27, 0xa2, 0x6f, 0x74, 0x42,
                0xfc, 0xbf, 0xe9, 0x54, 0x67, 0x7a, 0xe6, 0xd4, 0x55, 0xf6, 0x05, 0x65, 0x49, 0x89,
                0xe3, 0x5a, 0xeb, 0x13,
            ],
            Elements::Ch16 => [
                0x9c, 0xff, 0x11, 0xa0, 0x9b, 0x60, 0x41, 0xe5, 0xf2, 0x63, 0x9a, 0xe4, 0xc0, 0x65,
                0xa1, 0x8f, 0xc6, 0x75, 0xdb, 0x2f, 0xbd, 0x98, 0x54, 0x08, 0xe2, 0x8f, 0x02, 0x7a,
                0x99, 0x11, 0x0e, 0x11,
            ],
            Elements::Ch32 => [
                0x07, 0x1c, 0xef, 0x80, 0x39, 0xc7, 0x9f, 0x71, 0x31, 0xcd, 0x6a, 0x5f, 0xe4, 0x93,
                0xdc, 0x26, 0x8f, 0x9d, 0xb5, 0x8f, 0x7b, 0x20, 0xa8, 0x55, 0x55, 0xe2, 0x97, 0xbd,
                0xd2, 0x16, 0xcf, 0x40,
            ],
            Elements::Ch64 => [
                0xd5, 0x55, 0xd2, 0x19, 0x63, 0xb0, 0x19, 0x2f, 0xc9, 0x72, 0x14, 0xb6, 0x3d, 0xc1,
                0xc3, 0xaf, 0x75, 0x8b, 0x29, 0x11, 0x58, 0xf0, 0xe1, 0xa3, 0xbc, 0xfd, 0xea, 0x67,
                0x9c, 0x66, 0x6d, 0xa6,
            ],
            Elements::Ch8 => [
                0x35, 0x3f, 0x63, 0xb0, 0xf8, 0xcb, 0x54, 0xf5, 0xae, 0x65, 0x75, 0xaf, 0x8c, 0xa2,
                0x24, 0x2c, 0xee, 0xe9, 0xf2, 0x7a, 0x84, 0x18, 0x6e, 0xb8, 0x0e, 0x62, 0x0d, 0x5e,
                0x2e, 0x85, 0x48, 0xec,
            ],
            Elements::CheckLockDistance => [
                0x7f, 0x78, 0xc7, 0xa7, 0x7a, 0x25, 0xad, 0xa2, 0x23, 0x26, 0x7d, 0x23, 0x9a, 0x59,
                0x22, 0xf7, 0x64, 0xb8, 0xac, 0x0c, 0x2f, 0xce, 0xf6, 0x8e, 0xb9, 0x3c, 0x0d, 0x92,
                0xda, 0x4a, 0xf5, 0x15,
            ],
            Elements::CheckLockDuration => [
                0x73, 0xda, 0xc8, 0xe2, 0x5d, 0x87, 0xea, 0xf3, 0x82, 0xc2, 0xa7, 0x72, 0x06, 0xad,
                0x38, 0xb9, 0x38, 0x43, 0x61, 0xe7, 0xd0, 0xdc, 0x87, 0xc0, 0xfa, 0x7a, 0xf7, 0xea,
                0x52, 0x45, 0x97, 0xb7,
            ],
            Elements::CheckLockHeight => [
                0x9e, 0x78, 0x98, 0xd0, 0x37, 0x62, 0x71, 0x34, 0xd2, 0xbd, 0x70, 0xc7, 0xfc, 0xa9,
                0xcb, 0xa4, 0x5e, 0xaf, 0x26, 0x7d, 0x4d, 0x09, 0xad, 0x50, 0xa9, 0xef, 0x71, 0x7a,
                0x8f, 0x27, 0x49, 0xdb,
            ],
            Elements::CheckLockTime => [
                0x68, 0x67, 0x3d, 0x12, 0xe2, 0x73, 0x2f, 0xaa, 0x1d, 0x39, 0xe2, 0x13, 0x6b, 0x14,
                0x06, 0xaf, 0xa0, 0x98, 0xa8, 0x4c, 0x96, 0xe8, 0xd6, 0x05, 0x02, 0xa2, 0xdd, 0x61,
                0xc5, 0x95, 0x70, 0xbb,
            ],
            Elements::CheckSigVerify => [
                0xb5, 0x80, 0x15, 0x54, 0x6d, 0x28, 0x52, 0x66, 0x5d, 0xd2, 0x1b, 0xf1, 0x12, 0x66,
                0x26, 0x70, 0x20, 0xfa, 0x5e, 0x27, 0x50, 0x01, 0xdd, 0x46, 0x18, 0xfa, 0x41, 0x56,
                0x25, 0x95, 0x2e, 0x68,
            ],
            Elements::Complement1 => [
                0xed, 0x74, 0xee, 0xb8, 0x3a, 0x00, 0xc7, 0x13, 0xcc, 0x14, 0xf3, 0x3e, 0xfe, 0x55,
                0x33, 0x83, 0xcd, 0x04, 0x11, 0xcc, 0x30, 0x20, 0xfd, 0x89, 0x27, 0x93, 0x16, 0x67,
                0x5d, 0x91, 0x0e, 0x66,
            ],
            Elements::Complement16 => [
                0x61, 0xfd, 0xd9, 0x04, 0xa4, 0xae, 0xb7, 0xeb, 0x76, 0x84, 0xaf, 0x61, 0x8e, 0x25,
                0xaa, 0xe9, 0x07, 0xcd, 0x1d, 0xb0, 0xf6, 0x2d, 0x97, 0x03, 0xc5, 0xb8, 0x54, 0xe1,
                0x66, 0x3c, 0xac, 0x9f,
            ],
            Elements::Complement32 => [
                0xfe, 0xb0, 0x2c, 0xc3, 0x6e, 0x19, 0x5b, 0x46, 0x2a, 0xe5, 0x04, 0xa9, 0x12, 0xda,
                0xdf, 0xe6, 0x6a, 0xd4, 0x7f, 0x23, 0xa0, 0xcb, 0x3b, 0xae, 0xa2, 0x1d, 0x31, 0xaa,
                0xa0, 0xce, 0x10, 0x1d,
            ],
            Elements::Complement64 => [
                0x45, 0x07, 0x2d, 0x5a, 0xa0, 0xe5, 0xc3, 0x7c, 0x9e, 0x52, 0x1d, 0xcc, 0x92, 0xe8,
                0xf3, 0x9a, 0x5f, 0x75, 0xe7, 0xd9, 0x28, 0x67, 0x0a, 0xca, 0xb7, 0x9c, 0xd8, 0xc8,
                0xb5, 0xb5, 0x9e, 0x1a,
            ],
            Elements::Complement8 => [
                0x69, 0x16, 0xb2, 0x8f, 0xb5, 0x74, 0xd9, 0xc9, 0x08, 0xa3, 0xf3, 0x3f, 0x74, 0xbf,
                0x06, 0xf7, 0xed, 0x93, 0x72, 0x54, 0x24, 0x7f, 0x9e, 0xfc, 0x26, 0x03, 0xd7, 0x17,
                0x1d, 0xd4, 0x97, 0xbe,
            ],
            Elements::CurrentAmount => [
                0x9e, 0x7b, 0xc9, 0x6d, 0x06, 0xc1, 0xf4, 0xa4, 0xd2, 0xbd, 0x43, 0x33, 0xd5, 0x56,
                0x43, 0xcb, 0xa6, 0x86, 0x3d, 0x3a, 0x33, 0x8e, 0x31, 0xdc, 0x0d, 0x17, 0x95, 0x58,
                0x41, 0x56, 0xea, 0x69,
            ],
            Elements::CurrentAnnexHash => [
                0x5f, 0x68, 0xbd, 0xaf, 0xd4, 0x17, 0xfe, 0xbb, 0x10, 0xde, 0xe1, 0x6b, 0x1d, 0x47,
                0xc2, 0x1f, 0xa9, 0x7e, 0xfc, 0x91, 0x91, 0x57, 0xc7, 0xf7, 0xbc, 0x54, 0xa6, 0x1d,
                0xc4, 0xf9, 0xf7, 0xd6,
            ],
            Elements::CurrentAsset => [
                0x03, 0xc8, 0x44, 0x59, 0xa5, 0xec, 0xdf, 0xdb, 0x46, 0xe3, 0xd3, 0x8e, 0xcb, 0x79,
                0xff, 0x22, 0x24, 0x3f, 0xdc, 0xf7, 0x0d, 0x80, 0xda, 0x1b, 0xa4, 0x56, 0x99, 0xe7,
                0x00, 0x8e, 0xec, 0xfe,
            ],
            Elements::CurrentIndex => [
                0x15, 0xe1, 0x05, 0x1f, 0xf2, 0x3f, 0x85, 0x1c, 0x19, 0x13, 0x1f, 0x0d, 0xe6, 0xed,
                0xc4, 0x88, 0x23, 0x76, 0xa2, 0x57, 0x90, 0xdb, 0xd9, 0x10, 0x28, 0x24, 0xaa, 0x22,
                0xa8, 0x89, 0xae, 0x84,
            ],
            Elements::CurrentIssuanceAssetAmount => [
                0xdc, 0x41, 0x0e, 0xc6, 0x99, 0x46, 0xf0, 0x8b, 0x9c, 0x4a, 0x40, 0xcf, 0x38, 0x81,
                0x9c, 0xbb, 0x01, 0x4e, 0x98, 0xfc, 0xe0, 0xa9, 0xb1, 0x96, 0x51, 0x45, 0x57, 0x47,
                0x7f, 0xe1, 0xc2, 0xa7,
            ],
            Elements::CurrentIssuanceAssetProof => [
                0xe5, 0xc1, 0xc6, 0xf7, 0xea, 0x02, 0x96, 0x14, 0x8e, 0x76, 0xf8, 0xb1, 0x79, 0xd4,
                0x15, 0x19, 0x82, 0x73, 0xdc, 0xc8, 0xe1, 0xfd, 0x89, 0x1f, 0x83, 0xef, 0x2b, 0xdf,
                0xb0, 0x69, 0x29, 0x36,
            ],
            Elements::CurrentIssuanceTokenAmount => [
                0xe3, 0xea, 0x9c, 0x61, 0x70, 0x95, 0x5f, 0x27, 0xb0, 0x1b, 0x12, 0x0c, 0xd6, 0x3a,
                0x36, 0x08, 0x50, 0xcc, 0x70, 0x0f, 0x47, 0x25, 0x19, 0x3b, 0xdf, 0xf8, 0x8e, 0x01,
                0xdf, 0xb0, 0x4e, 0xad,
            ],
            Elements::CurrentIssuanceTokenProof => [
                0x92, 0xe7, 0xd9, 0xdd, 0xbf, 0x75, 0x6e, 0x3e, 0x62, 0x2a, 0x44, 0xd4, 0xd0, 0x0e,
                0x84, 0x23, 0x6b, 0xcc, 0xe4, 0x2f, 0x16, 0xf3, 0x20, 0x0e, 0x8b, 0x89, 0xc0, 0x3c,
                0x44, 0x1e, 0x45, 0x8a,
            ],
            Elements::CurrentNewIssuanceContract => [
                0x23, 0x78, 0x15, 0x67, 0x5e, 0x9e, 0x9c, 0xf4, 0xd8, 0x54, 0x85, 0x60, 0xc7, 0x61,
                0xe3, 0xba, 0x7f, 0x64, 0xdc, 0xf2, 0x5c, 0x99, 0x52, 0x1b, 0xe9, 0xd8, 0x57, 0xe4,
                0xea, 0x41, 0xd8, 0x68,
            ],
            Elements::CurrentPegin => [
                0x36, 0x9a, 0xd5, 0xf6, 0xd7, 0x60, 0x48, 0x1c, 0xb3, 0x53, 0xa9, 0xba, 0xcf, 0x90,
                0x0f, 0xf1, 0x62, 0xb7, 0x60, 0x40, 0xc0, 0x7a, 0xd0, 0xa8, 0x3e, 0x52, 0xfc, 0xce,
                0xf1, 0x29, 0x57, 0x90,
            ],
            Elements::CurrentPrevOutpoint => [
                0xa1, 0xec, 0x90, 0x9c, 0x81, 0xe5, 0x08, 0xe8, 0x6e, 0x13, 0x61, 0xba, 0x82, 0xe5,
                0xfb, 0x47, 0xa2, 0xc7, 0x6b, 0xff, 0xba, 0x73, 0x21, 0xa6, 0xfc, 0x13, 0xcb, 0x12,
                0xaf, 0x7f, 0x69, 0x7c,
            ],
            Elements::CurrentReissuanceBlinding => [
                0xb7, 0x15, 0x50, 0xc8, 0xf6, 0x2e, 0x7a, 0xac, 0xff, 0x13, 0xe8, 0xd3, 0xc0, 0xae,
                0x05, 0x4e, 0xf0, 0x36, 0x40, 0x9d, 0x76, 0xc0, 0x65, 0x79, 0xc0, 0x26, 0xad, 0x68,
                0xa1, 0xb5, 0x61, 0xf8,
            ],
            Elements::CurrentReissuanceEntropy => [
                0x8b, 0x27, 0x15, 0x32, 0x7a, 0x0a, 0xc6, 0xb0, 0xd1, 0x8a, 0x2e, 0x51, 0xfb, 0xda,
                0x46, 0xc4, 0xb1, 0x18, 0xee, 0x06, 0x37, 0xd1, 0x14, 0xae, 0x3e, 0x58, 0xa2, 0x3b,
                0x45, 0xd6, 0x7f, 0xf8,
            ],
            Elements::CurrentScriptHash => [
                0xbf, 0xaf, 0x85, 0x84, 0x43, 0xce, 0xc8, 0x33, 0x7e, 0x37, 0x83, 0x9d, 0xc4, 0x11,
                0x35, 0x02, 0x71, 0x84, 0x58, 0x0c, 0x89, 0x21, 0x57, 0xce, 0x73, 0x04, 0x18, 0xc0,
                0x8d, 0x5e, 0xd8, 0x38,
            ],
            Elements::CurrentScriptSigHash => [
                0x8b, 0xaa, 0x9d, 0xf8, 0x54, 0xfe, 0x33, 0xee, 0xea, 0x66, 0x19, 0x5c, 0x97, 0xce,
                0x18, 0xed, 0x70, 0x1e, 0x10, 0x7e, 0xea, 0x7e, 0x02, 0xfb, 0x26, 0x03, 0xb2, 0xc1,
                0x21, 0x57, 0x50, 0x66,
            ],
            Elements::CurrentSequence => [
                0x0f, 0x42, 0x24, 0xf9, 0xaa, 0x86, 0x44, 0xf5, 0xc0, 0xf7, 0x91, 0x0f, 0x17, 0x39,
                0x04, 0x1a, 0xec, 0x8e, 0x7a, 0xf7, 0x7b, 0x5f, 0x38, 0xcf, 0xdc, 0xa0, 0xfd, 0xf4,
                0x82, 0x67, 0xa6, 0xc3,
            ],
            Elements::Decompress => [
                0x13, 0x97, 0x33, 0x17, 0xd5, 0x87, 0x41, 0x8e, 0xf3, 0x06, 0x36, 0x31, 0xa6, 0xed,
                0xb0, 0xac, 0xfa, 0x1c, 0xbe, 0x49, 0x83, 0xd7, 0x57, 0x4b, 0x1b, 0x30, 0x5f, 0x96,
                0x61, 0xc0, 0x48, 0xcb,
            ],
            Elements::Decrement16 => [
                0xe3, 0x4d, 0xb1, 0x18, 0x79, 0x27, 0x2b, 0x32, 0x7a, 0x3b, 0xd0, 0x34, 0xc0, 0xf6,
                0x1e, 0xf6, 0x0a, 0x2b, 0xe9, 0x6f, 0xdf, 0xe0, 0xb2, 0xd5, 0x7f, 0xfe, 0x39, 0xce,
                0x71, 0x4c, 0x78, 0xfb,
            ],
            Elements::Decrement32 => [
                0x01, 0x9e, 0xad, 0x5a, 0x73, 0x05, 0x60, 0x6d, 0xc9, 0x50, 0xfb, 0x55, 0x47, 0x6d,
                0x09, 0xc1, 0x7d, 0x66, 0xf5, 0x70, 0xda, 0xb5, 0x10, 0xb9, 0x0d, 0x2a, 0x27, 0xe2,
                0x26, 0x65, 0x99, 0xcf,
            ],
            Elements::Decrement64 => [
                0x34, 0x75, 0x2c, 0xf4, 0xe1, 0xd0, 0xa4, 0x31, 0xf0, 0x17, 0xa6, 0x8b, 0xeb, 0xfa,
                0xb7, 0x41, 0xbb, 0xc8, 0x8a, 0xff, 0xb5, 0x7c, 0xc0, 0xb3, 0x02, 0x5c, 0xcf, 0xdd,
                0x67, 0x62, 0x2f, 0x35,
            ],
            Elements::Decrement8 => [
                0x28, 0x92, 0xce, 0xb3, 0xb6, 0xec, 0x53, 0x25, 0xd0, 0xc1, 0xb9, 0xf5, 0x20, 0x42,
                0x5e, 0x4b, 0x05, 0xc2, 0xe1, 0xf4, 0x37, 0xe0, 0xb3, 0xf5, 0x81, 0xf4, 0x1b, 0x9d,
                0x0f, 0x7d, 0xff, 0x4d,
            ],
            Elements::DivMod128_64 => [
                0x22, 0x96, 0xb7, 0x0f, 0x60, 0x0e, 0x8a, 0x21, 0x4a, 0xd0, 0x70, 0xb2, 0x19, 0x4a,
                0x67, 0x7d, 0x30, 0x51, 0xbc, 0x1c, 0x49, 0x01, 0x83, 0x97, 0x5f, 0x2a, 0x1d, 0x3e,
                0x0c, 0xad, 0xe3, 0x78,
            ],
            Elements::DivMod16 => [
                0x64, 0x8f, 0xab, 0x86, 0x43, 0x74, 0x84, 0x6a, 0xbf, 0x4f, 0x9d, 0x9d, 0xef, 0xe2,
                0x75, 0x61, 0x4d, 0x33, 0xf4, 0x82, 0x9c, 0x36, 0xa4, 0x7e, 0xcb, 0x53, 0xd7, 0xbf,
                0xb6, 0x05, 0x48, 0x5f,
            ],
            Elements::DivMod32 => [
                0xbd, 0x3d, 0x4d, 0x55, 0x2d, 0x7b, 0x34, 0x7b, 0xd8, 0xa4, 0x4e, 0x3e, 0xe2, 0x24,
                0xc8, 0x46, 0xbe, 0x23, 0x0f, 0xf6, 0xe2, 0x04, 0x4d, 0xdb, 0x97, 0xf4, 0x8e, 0x27,
                0xd2, 0x0c, 0x42, 0x25,
            ],
            Elements::DivMod64 => [
                0xfa, 0x6b, 0xad, 0x6a, 0x95, 0xe2, 0xab, 0xa4, 0x30, 0x5b, 0xfe, 0x91, 0xcc, 0x47,
                0xac, 0xc3, 0xd9, 0x9b, 0x92, 0xe6, 0x75, 0xe6, 0x9d, 0x3b, 0x37, 0xbb, 0x09, 0x13,
                0x3d, 0x39, 0x0d, 0x0f,
            ],
            Elements::DivMod8 => [
                0x48, 0xcd, 0x50, 0x1b, 0xb2, 0xaa, 0x2a, 0xca, 0xe0, 0x14, 0xfe, 0x20, 0x8b, 0xb9,
                0x94, 0x1d, 0x07, 0xa9, 0xbf, 0xfe, 0x1a, 0xd6, 0xcd, 0x3d, 0x36, 0xfc, 0x6b, 0x08,
                0x60, 0xf6, 0xeb, 0xa7,
            ],
            Elements::Divide16 => [
                0x47, 0x0b, 0x01, 0xa5, 0x7c, 0x4f, 0x9d, 0x8f, 0x99, 0x7f, 0xcd, 0xe0, 0x06, 0x19,
                0x16, 0x11, 0xdd, 0xa4, 0xc9, 0x8b, 0xa2, 0xa5, 0xf1, 0xda, 0x13, 0x4a, 0xe4, 0xc2,
                0x2d, 0x52, 0xe9, 0x20,
            ],
            Elements::Divide32 => [
                0xab, 0x03, 0xac, 0xd8, 0x93, 0x61, 0x0c, 0x3c, 0x65, 0x82, 0xe7, 0xf7, 0xfb, 0xe5,
                0xe7, 0x56, 0x25, 0x74, 0xa7, 0xb2, 0x66, 0x46, 0xf1, 0xc2, 0xfd, 0xc6, 0xe7, 0x6e,
                0x44, 0x5a, 0x77, 0xa1,
            ],
            Elements::Divide64 => [
                0xeb, 0xfc, 0x56, 0xfb, 0xb8, 0xa4, 0x7e, 0x73, 0xff, 0xab, 0xb7, 0xea, 0x22, 0x8a,
                0xc7, 0x84, 0x37, 0xbe, 0x82, 0x0e, 0xdd, 0xfa, 0x47, 0x81, 0x4c, 0xce, 0xbd, 0x26,
                0x1b, 0xd8, 0xcf, 0xff,
            ],
            Elements::Divide8 => [
                0x2c, 0xcf, 0xbc, 0x7c, 0x02, 0xbf, 0x4d, 0x53, 0x04, 0x93, 0xbb, 0x22, 0x86, 0x7a,
                0x95, 0x1d, 0x8a, 0xe9, 0x13, 0x12, 0x66, 0x87, 0x59, 0x72, 0x84, 0xe9, 0xbb, 0xb3,
                0xe1, 0xe7, 0xe3, 0x49,
            ],
            Elements::Divides16 => [
                0x5f, 0xc3, 0xac, 0x38, 0x4d, 0x5f, 0x45, 0x40, 0x41, 0x56, 0x97, 0x1a, 0x76, 0x8d,
                0x93, 0xbc, 0x06, 0x4b, 0xc1, 0x7c, 0x15, 0xa3, 0x7c, 0x27, 0x01, 0x9d, 0xde, 0xef,
                0x17, 0x04, 0x6d, 0xd4,
            ],
            Elements::Divides32 => [
                0xcc, 0x45, 0xb4, 0x05, 0x24, 0x64, 0x38, 0xf7, 0x65, 0x74, 0x0b, 0x4f, 0xb0, 0xa3,
                0x4d, 0xc8, 0x1b, 0x34, 0x78, 0x01, 0x98, 0x86, 0x3b, 0x0f, 0xb1, 0x86, 0x44, 0x6a,
                0xdf, 0xbb, 0x09, 0xde,
            ],
            Elements::Divides64 => [
                0xdc, 0x47, 0x3b, 0xfd, 0xec, 0x30, 0xab, 0x98, 0xd4, 0x8c, 0xd0, 0x88, 0x84, 0xef,
                0x4f, 0xff, 0xef, 0x3d, 0x4b, 0x16, 0xad, 0x5c, 0x37, 0x11, 0x2a, 0x20, 0x35, 0xb9,
                0x9b, 0xb7, 0x74, 0x58,
            ],
            Elements::Divides8 => [
                0x0b, 0x55, 0x02, 0xac, 0x4f, 0x21, 0xf2, 0x30, 0xa0, 0x9c, 0xcf, 0xaf, 0xfa, 0xac,
                0x77, 0xa7, 0xc4, 0x1b, 0x2b, 0xf3, 0x0b, 0x14, 0x68, 0x48, 0x1e, 0x4d, 0xfb, 0x98,
                0xb6, 0x18, 0x7a, 0x0d,
            ],
            Elements::Eq1 => [
                0x60, 0x7f, 0x6b, 0x8f, 0x5d, 0x25, 0xb8, 0x0e, 0x05, 0xa2, 0xbf, 0x79, 0xd6, 0x2e,
                0x87, 0x07, 0x99, 0x52, 0x2c, 0xc3, 0xe3, 0x9c, 0xe9, 0x62, 0x57, 0x45, 0x52, 0x93,
                0xf9, 0xb2, 0xb2, 0xed,
            ],
            Elements::Eq16 => [
                0xc9, 0x96, 0xe4, 0x2b, 0x97, 0x9a, 0xbc, 0x53, 0x0c, 0xc2, 0x71, 0x63, 0x66, 0x71,
                0xe9, 0x20, 0x54, 0x87, 0x6a, 0x1e, 0xca, 0xed, 0x14, 0x33, 0xfd, 0x61, 0x9a, 0x25,
                0xfe, 0x6d, 0x03, 0xad,
            ],
            Elements::Eq256 => [
                0x77, 0x8d, 0x15, 0x06, 0xc7, 0x35, 0xd2, 0x77, 0x6b, 0x95, 0x0f, 0xac, 0xef, 0xc1,
                0x59, 0xb6, 0x78, 0xde, 0xc0, 0x38, 0x28, 0xcf, 0x02, 0x73, 0xee, 0xea, 0x64, 0xa9,
                0xda, 0x98, 0xc1, 0x2c,
            ],
            Elements::Eq32 => [
                0x66, 0xd3, 0x89, 0x03, 0xe7, 0x3b, 0x1a, 0x13, 0x20, 0xc6, 0x8a, 0x4a, 0x39, 0x70,
                0xd7, 0x1f, 0x94, 0xba, 0x9e, 0x2b, 0x15, 0x16, 0x83, 0x99, 0x43, 0xfb, 0x15, 0xe4,
                0x4e, 0xbf, 0x57, 0xfb,
            ],
            Elements::Eq64 => [
                0xd6, 0xa6, 0x66, 0xb4, 0xe0, 0xf9, 0xf5, 0x75, 0x50, 0x8d, 0xbf, 0x3b, 0x31, 0xce,
                0xea, 0x68, 0x39, 0x3c, 0x7d, 0xb2, 0xe9, 0x8b, 0xc5, 0x92, 0xfd, 0xd2, 0x6f, 0xae,
                0x83, 0x7a, 0x0b, 0x87,
            ],
            Elements::Eq8 => [
                0x99, 0x78, 0x7b, 0xa2, 0x67, 0x2d, 0xd0, 0xeb, 0x4d, 0x7d, 0x2e, 0xa9, 0x94, 0x49,
                0xde, 0x8f, 0x79, 0x8e, 0x7c, 0xb1, 0x81, 0xa5, 0xe1, 0x66, 0xe1, 0xa5, 0x3f, 0x98,
                0x02, 0xb6, 0x20, 0x64,
            ],
            Elements::FeAdd => [
                0xb0, 0x59, 0x3e, 0x18, 0x7e, 0xe7, 0x33, 0x3c, 0x47, 0xa0, 0x54, 0x67, 0xdf, 0x66,
                0xd5, 0x82, 0x0a, 0x6f, 0x5b, 0xef, 0x91, 0x4a, 0x4b, 0x76, 0xe5, 0xd1, 0x63, 0x31,
                0x4b, 0x5e, 0xf2, 0x0e,
            ],
            Elements::FeInvert => [
                0x34, 0x3e, 0x9c, 0x90, 0xf1, 0x28, 0x50, 0x60, 0x56, 0xb5, 0x48, 0xd2, 0xed, 0x5e,
                0x22, 0x3c, 0x81, 0xf5, 0xb0, 0x6a, 0x1e, 0xd8, 0x6b, 0x7c, 0xd9, 0x35, 0x40, 0x57,
                0xaa, 0x59, 0x51, 0x02,
            ],
            Elements::FeIsOdd => [
                0xdc, 0xf0, 0x37, 0x5d, 0x20, 0x81, 0x8a, 0x99, 0xf7, 0x23, 0xf8, 0x12, 0x3c, 0xbd,
                0x05, 0x1a, 0x38, 0x78, 0xa4, 0x28, 0x24, 0xb3, 0x74, 0x0f, 0x68, 0x21, 0xa5, 0xfa,
                0x12, 0x3f, 0x14, 0xc7,
            ],
            Elements::FeIsZero => [
                0x28, 0xff, 0x41, 0x69, 0x9a, 0x88, 0x1a, 0xaf, 0xb7, 0xa9, 0x76, 0xc0, 0xc5, 0x76,
                0x35, 0x3f, 0x7f, 0xe5, 0x44, 0x63, 0xb6, 0xaa, 0x75, 0x4c, 0xf2, 0xc6, 0x32, 0x9a,
                0xf2, 0x65, 0x0e, 0x3b,
            ],
            Elements::FeMultiply => [
                0x56, 0x69, 0x92, 0x9b, 0x5f, 0x31, 0xfa, 0x3d, 0x02, 0xc5, 0x83, 0x9d, 0xd0, 0x63,
                0x54, 0xcd, 0x17, 0x16, 0x35, 0xf3, 0xa0, 0x72, 0x7f, 0x32, 0x2a, 0xbf, 0xc9, 0x94,
                0xba, 0x62, 0x90, 0xde,
            ],
            Elements::FeMultiplyBeta => [
                0x7a, 0x78, 0x13, 0x45, 0x0d, 0x82, 0xe9, 0x35, 0x69, 0x0f, 0x43, 0x3e, 0x65, 0xdf,
                0x70, 0x7a, 0x4d, 0xd1, 0x75, 0x34, 0xa0, 0x0d, 0xdd, 0x40, 0xdd, 0x85, 0xe3, 0xe3,
                0xf7, 0x84, 0x02, 0xc3,
            ],
            Elements::FeNegate => [
                0x3b, 0x0d, 0x7b, 0x5c, 0x2e, 0x6c, 0x3a, 0xeb, 0x5e, 0x00, 0x08, 0x5b, 0x9d, 0x30,
                0x58, 0x5a, 0xff, 0x05, 0x4e, 0x32, 0x5a, 0x99, 0x83, 0x61, 0x11, 0x3b, 0xfd, 0x23,
                0x28, 0xc0, 0x08, 0xf6,
            ],
            Elements::FeNormalize => [
                0xc5, 0x1b, 0xef, 0xfa, 0x21, 0x5e, 0x9c, 0xde, 0x8e, 0x93, 0x3b, 0xb9, 0x46, 0x80,
                0xba, 0xe0, 0x12, 0xc4, 0xda, 0xab, 0x3d, 0x04, 0xb6, 0xcb, 0xf0, 0x73, 0x3f, 0xd7,
                0x35, 0x73, 0x35, 0x38,
            ],
            Elements::FeSquare => [
                0x5a, 0x6e, 0x7b, 0x2e, 0xac, 0x73, 0xf4, 0xe4, 0x4d, 0xfa, 0x28, 0xfb, 0x86, 0xbb,
                0x11, 0x7b, 0x65, 0x60, 0x6f, 0x28, 0x74, 0xd5, 0x65, 0xc9, 0x79, 0x9c, 0x63, 0xe0,
                0xfe, 0x69, 0x2b, 0x1a,
            ],
            Elements::FeSquareRoot => [
                0xe0, 0x01, 0x42, 0xea, 0x03, 0x09, 0x4a, 0x30, 0x4a, 0xc8, 0x2b, 0xc1, 0xe2, 0xd2,
                0xdc, 0x71, 0xfb, 0x06, 0x4e, 0xd0, 0x82, 0x85, 0x67, 0x35, 0xb1, 0x4f, 0xf2, 0xc7,
                0xfa, 0xf0, 0x36, 0xf0,
            ],
            Elements::FullAdd16 => [
                0xfc, 0x9e, 0x5d, 0xf8, 0x3b, 0xfd, 0xb9, 0x02, 0x8c, 0x87, 0xd1, 0x39, 0xf8, 0x58,
                0x39, 0x03, 0xcb, 0x2a, 0x07, 0x04, 0x2a, 0x73, 0xe5, 0x34, 0x81, 0xde, 0xb5, 0x2f,
                0xf1, 0xf1, 0xf8, 0x84,
            ],
            Elements::FullAdd32 => [
                0xa7, 0xd9, 0x8d, 0x50, 0xd0, 0x45, 0xcb, 0x90, 0x6b, 0x19, 0x5e, 0x65, 0x11, 0x87,
                0x94, 0x95, 0xc8, 0x51, 0x09, 0x59, 0x49, 0xa9, 0xc0, 0x1e, 0x60, 0x39, 0xa8, 0x4b,
                0x2a, 0x5e, 0xc9, 0x09,
            ],
            Elements::FullAdd64 => [
                0x7a, 0xec, 0xc8, 0xc9, 0x05, 0x3b, 0xb2, 0xfb, 0x17, 0x0c, 0x1c, 0x97, 0x2f, 0xd4,
                0x00, 0x25, 0x64, 0xe1, 0x52, 0xa0, 0x6d, 0x9f, 0x45, 0x80, 0x75, 0xe3, 0x8c, 0x7a,
                0x06, 0x98, 0xa7, 0xf4,
            ],
            Elements::FullAdd8 => [
                0xed, 0x3b, 0xa5, 0xb7, 0x9e, 0xa4, 0x5b, 0x18, 0x7a, 0x2d, 0x43, 0xe8, 0xed, 0x80,
                0x2d, 0xe1, 0xed, 0x44, 0x26, 0x59, 0x6c, 0xbe, 0x32, 0xe7, 0x57, 0xc8, 0x51, 0x19,
                0x15, 0xff, 0xa5, 0xcf,
            ],
            Elements::FullDecrement16 => [
                0xd4, 0xc2, 0xed, 0xda, 0x87, 0x2c, 0x05, 0x50, 0x6f, 0x79, 0x2c, 0xf5, 0x46, 0xa8,
                0x9d, 0x4d, 0x7c, 0xff, 0xcb, 0x1e, 0x17, 0xf5, 0xda, 0x61, 0x03, 0x10, 0x0e, 0x7e,
                0x73, 0xa7, 0x73, 0x7d,
            ],
            Elements::FullDecrement32 => [
                0x7c, 0xc2, 0x30, 0x4d, 0x17, 0x43, 0x12, 0x10, 0x2e, 0x9b, 0x73, 0x63, 0x45, 0xc7,
                0x7f, 0x77, 0x1d, 0x1f, 0x6a, 0x9c, 0x9e, 0x1d, 0x1c, 0xd8, 0xdb, 0x8c, 0xb4, 0x61,
                0x39, 0x80, 0xc8, 0xc2,
            ],
            Elements::FullDecrement64 => [
                0x15, 0xc1, 0x63, 0x45, 0x4b, 0xcd, 0x75, 0x44, 0x30, 0xda, 0x55, 0x79, 0xbb, 0xca,
                0xad, 0x26, 0xe5, 0x7e, 0x95, 0xc7, 0x72, 0x22, 0x4b, 0x7b, 0x83, 0xc7, 0x05, 0xf7,
                0xde, 0xb6, 0x4a, 0xa6,
            ],
            Elements::FullDecrement8 => [
                0x7c, 0x5e, 0x94, 0xa9, 0x98, 0x02, 0x81, 0x82, 0x17, 0x37, 0xb1, 0xce, 0x73, 0xbf,
                0xda, 0x4c, 0x79, 0xef, 0x64, 0x9b, 0x3d, 0x05, 0xcc, 0x1c, 0x00, 0xc4, 0xa8, 0xb6,
                0x4b, 0x94, 0x9b, 0xbe,
            ],
            Elements::FullIncrement16 => [
                0x81, 0x38, 0x0a, 0xda, 0xa3, 0xa5, 0x47, 0xf1, 0xbc, 0x4b, 0xbb, 0x64, 0x6b, 0xda,
                0x9d, 0x9f, 0xb7, 0xbd, 0x4d, 0xc1, 0xb3, 0xa9, 0xf3, 0xdd, 0x22, 0x0b, 0x56, 0xa4,
                0x7c, 0x27, 0x98, 0xfb,
            ],
            Elements::FullIncrement32 => [
                0xa7, 0x60, 0xa8, 0x44, 0x9a, 0x2a, 0xb5, 0xde, 0xdb, 0x4e, 0xe5, 0x1b, 0xf5, 0xc2,
                0x5a, 0x8f, 0x06, 0xaf, 0x06, 0x66, 0xdf, 0x7f, 0xc4, 0x19, 0xb4, 0x98, 0xb9, 0x09,
                0x76, 0xd6, 0x98, 0xcb,
            ],
            Elements::FullIncrement64 => [
                0xc6, 0xaf, 0x30, 0xdd, 0x28, 0x6d, 0x6e, 0x21, 0xc3, 0x88, 0x60, 0xed, 0x1e, 0x2f,
                0x21, 0x2a, 0x21, 0xb2, 0xfd, 0x1e, 0xde, 0xad, 0xb5, 0xe0, 0xfc, 0xe2, 0xe3, 0xfd,
                0x75, 0xb7, 0xf3, 0xc2,
            ],
            Elements::FullIncrement8 => [
                0xd3, 0x04, 0xea, 0x28, 0xa9, 0x5d, 0x49, 0x6d, 0x14, 0xb4, 0xf2, 0xfb, 0x5c, 0x86,
                0x03, 0x72, 0xec, 0xf2, 0x47, 0xbe, 0xfd, 0xe3, 0xea, 0x3b, 0x2a, 0xd6, 0x7b, 0xce,
                0x99, 0x03, 0x9d, 0xbc,
            ],
            Elements::FullLeftShift16_1 => [
                0x14, 0xdc, 0xc3, 0x46, 0x6f, 0xa8, 0x28, 0xa3, 0xf0, 0x74, 0x04, 0x51, 0xb8, 0x03,
                0x7d, 0x7a, 0xd6, 0x03, 0xea, 0xdc, 0x80, 0xaa, 0xea, 0xdc, 0x66, 0x44, 0x34, 0xac,
                0x2a, 0xd7, 0xfd, 0x9c,
            ],
            Elements::FullLeftShift16_2 => [
                0xaf, 0xb7, 0xe9, 0x28, 0xb0, 0x52, 0xc2, 0x28, 0x79, 0x21, 0x66, 0x2c, 0xd8, 0xab,
                0x12, 0x2f, 0xe0, 0x74, 0xef, 0xd2, 0x51, 0xa5, 0xc9, 0xcf, 0xbc, 0xaa, 0x36, 0x9d,
                0x06, 0x33, 0x73, 0x92,
            ],
            Elements::FullLeftShift16_4 => [
                0x16, 0x6f, 0x34, 0x8c, 0x59, 0xe2, 0x6f, 0x89, 0xa8, 0x3a, 0x99, 0x1f, 0x67, 0xe5,
                0xdb, 0xf7, 0x10, 0xcf, 0xae, 0x3d, 0x6d, 0x96, 0x93, 0x82, 0x82, 0xbb, 0x44, 0xc1,
                0xaf, 0xa7, 0x10, 0x9b,
            ],
            Elements::FullLeftShift16_8 => [
                0xc0, 0xcd, 0x01, 0x5d, 0xe8, 0xac, 0x4f, 0xcc, 0xd8, 0xdb, 0x89, 0xf4, 0xe5, 0x14,
                0x2f, 0xde, 0x27, 0x97, 0x55, 0xb5, 0x42, 0xa2, 0x4f, 0x57, 0xa2, 0xa3, 0xc7, 0xc1,
                0xf5, 0x0d, 0x1d, 0xb5,
            ],
            Elements::FullLeftShift32_1 => [
                0xce, 0x33, 0xb5, 0xd0, 0xc5, 0x8d, 0x2d, 0x0b, 0x9b, 0x5a, 0x99, 0x44, 0xd3, 0xda,
                0xbd, 0xa0, 0x23, 0xcd, 0x44, 0x64, 0x7b, 0xe6, 0x7c, 0xf4, 0x08, 0x28, 0x30, 0xbb,
                0x20, 0x5f, 0x8f, 0xbb,
            ],
            Elements::FullLeftShift32_16 => [
                0x1c, 0xb3, 0x6e, 0x6f, 0x99, 0x30, 0x85, 0x15, 0xd4, 0xb7, 0x11, 0x90, 0x9c, 0x57,
                0x4b, 0x21, 0x24, 0xc1, 0xff, 0x42, 0x2d, 0x8d, 0x7d, 0x94, 0x82, 0xe2, 0x5d, 0x87,
                0x88, 0xb3, 0xb9, 0x57,
            ],
            Elements::FullLeftShift32_2 => [
                0x3f, 0xae, 0xa9, 0xb5, 0x73, 0xfc, 0x06, 0x9d, 0x8f, 0x43, 0x0f, 0xac, 0xa8, 0x97,
                0xb6, 0x87, 0x1e, 0xa0, 0x95, 0x73, 0xc7, 0x15, 0x09, 0x4b, 0x1f, 0x1b, 0xe0, 0x81,
                0x84, 0x88, 0xa7, 0x16,
            ],
            Elements::FullLeftShift32_4 => [
                0xcd, 0xbb, 0x0d, 0x23, 0x31, 0x05, 0x90, 0x11, 0x3c, 0x93, 0x4f, 0xe6, 0x60, 0x04,
                0xd2, 0xa1, 0x1d, 0xa9, 0xcb, 0xf8, 0x87, 0x3d, 0x00, 0xde, 0xe7, 0xf0, 0x22, 0x96,
                0xff, 0x0a, 0x2f, 0x12,
            ],
            Elements::FullLeftShift32_8 => [
                0xcc, 0xd9, 0x24, 0xe1, 0xa6, 0x18, 0x49, 0x42, 0x0f, 0xf6, 0x2e, 0xd8, 0xb2, 0x45,
                0xa3, 0xaa, 0x18, 0xc9, 0x8c, 0x41, 0xf9, 0xc5, 0xa3, 0xc0, 0xb8, 0x85, 0x86, 0x3c,
                0x44, 0x9b, 0x7d, 0x14,
            ],
            Elements::FullLeftShift64_1 => [
                0xd4, 0x63, 0xcc, 0xdc, 0x7f, 0xd1, 0x4e, 0x5e, 0x89, 0x41, 0x62, 0xb2, 0xae, 0x71,
                0x41, 0x28, 0xa1, 0x0d, 0xc9, 0x20, 0x00, 0xb5, 0x4c, 0x84, 0x3b, 0x64, 0x9c, 0xcb,
                0x77, 0x56, 0x26, 0xe5,
            ],
            Elements::FullLeftShift64_16 => [
                0x88, 0x2d, 0xce, 0x21, 0x2a, 0x0e, 0x61, 0xf8, 0xf9, 0x4c, 0xb5, 0xe3, 0x2e, 0x00,
                0xa5, 0x28, 0x7c, 0xf6, 0x4f, 0x20, 0xc2, 0x1f, 0xca, 0x84, 0xf1, 0xe3, 0xdf, 0x7f,
                0x4a, 0x62, 0x91, 0xcd,
            ],
            Elements::FullLeftShift64_2 => [
                0x48, 0xc8, 0x9b, 0x19, 0x1a, 0x51, 0xb6, 0xab, 0x03, 0x4c, 0x80, 0xea, 0xff, 0x34,
                0x82, 0x38, 0xd9, 0x3f, 0xb3, 0x1c, 0x1e, 0x92, 0xe7, 0xf2, 0xae, 0x49, 0x31, 0x7e,
                0x0e, 0x33, 0xf8, 0x2d,
            ],
            Elements::FullLeftShift64_32 => [
                0x39, 0x75, 0x90, 0x73, 0x33, 0xe1, 0x27, 0x30, 0x62, 0x55, 0xb7, 0xf8, 0x89, 0x39,
                0xe2, 0x85, 0x7f, 0x42, 0xae, 0x1b, 0xf0, 0xc6, 0x62, 0x40, 0xa8, 0x22, 0x4c, 0x8d,
                0xa3, 0x8b, 0xb1, 0xbe,
            ],
            Elements::FullLeftShift64_4 => [
                0x29, 0x31, 0x32, 0xeb, 0x15, 0xdd, 0xf4, 0x17, 0x74, 0xb0, 0x00, 0x5a, 0x3b, 0x5c,
                0x50, 0x95, 0x9f, 0xa8, 0x98, 0x2b, 0x75, 0x9e, 0x83, 0x28, 0x27, 0xc7, 0x4f, 0xa8,
                0x28, 0x50, 0x66, 0x6c,
            ],
            Elements::FullLeftShift64_8 => [
                0xe6, 0xab, 0xde, 0xd8, 0xbe, 0x58, 0x5e, 0xb0, 0xb6, 0xd4, 0x6e, 0x0c, 0x5e, 0xb2,
                0x8a, 0x74, 0x5f, 0x4e, 0x5c, 0x56, 0xfd, 0x65, 0x21, 0xf8, 0xf3, 0x96, 0xcb, 0x21,
                0xa7, 0x58, 0xf7, 0x4c,
            ],
            Elements::FullLeftShift8_1 => [
                0x73, 0x3f, 0xed, 0x08, 0x47, 0xa2, 0xff, 0xac, 0x9a, 0xab, 0xf5, 0x0a, 0x2f, 0xeb,
                0x50, 0x59, 0x89, 0x84, 0xf1, 0x6d, 0x8b, 0x73, 0x24, 0x68, 0xb3, 0xd3, 0x15, 0xc0,
                0x1e, 0xa4, 0x29, 0x9b,
            ],
            Elements::FullLeftShift8_2 => [
                0xb4, 0x47, 0x4d, 0x0b, 0xa1, 0xcf, 0x4f, 0xa2, 0xd6, 0x4c, 0xd4, 0xfe, 0x67, 0xbd,
                0xc9, 0x2c, 0xb8, 0x9e, 0xfa, 0x70, 0xcb, 0x99, 0xaf, 0x77, 0x91, 0xbf, 0x7e, 0xf6,
                0xe9, 0x09, 0xd2, 0xc7,
            ],
            Elements::FullLeftShift8_4 => [
                0x8e, 0xb5, 0x22, 0xb9, 0x97, 0x04, 0x74, 0xad, 0xbb, 0x7a, 0xb0, 0xde, 0x37, 0xc4,
                0xe7, 0xa0, 0x56, 0xa1, 0xcb, 0x21, 0x2e, 0x41, 0x03, 0xe4, 0xa8, 0xcb, 0xbb, 0xb6,
                0x3d, 0x97, 0x56, 0x06,
            ],
            Elements::FullMultiply16 => [
                0x88, 0x47, 0x0c, 0xbf, 0x9b, 0x4d, 0xec, 0x37, 0xea, 0x05, 0xd7, 0xb6, 0x30, 0xf2,
                0xf1, 0x12, 0x54, 0x75, 0x67, 0xd3, 0x4f, 0x33, 0xd9, 0x6e, 0x5f, 0x61, 0x1b, 0xd9,
                0xda, 0x97, 0xab, 0xb5,
            ],
            Elements::FullMultiply32 => [
                0x28, 0x04, 0x06, 0x00, 0xa6, 0x6e, 0x1a, 0x0c, 0x52, 0x25, 0x85, 0x20, 0x48, 0x8b,
                0x94, 0xc8, 0x20, 0xc6, 0xcf, 0x86, 0xca, 0x27, 0xae, 0x39, 0x03, 0x4d, 0xdd, 0xca,
                0xb9, 0x04, 0xd1, 0xd5,
            ],
            Elements::FullMultiply64 => [
                0x53, 0x01, 0x4f, 0x35, 0xa8, 0xdf, 0x20, 0x91, 0xaf, 0x3e, 0xf9, 0xb8, 0xd1, 0x6b,
                0x38, 0xb9, 0xbc, 0x96, 0x61, 0xbf, 0xdb, 0xc9, 0x57, 0x33, 0x3f, 0xba, 0x2a, 0x94,
                0x8c, 0x1e, 0x8c, 0x25,
            ],
            Elements::FullMultiply8 => [
                0xd3, 0xd2, 0x45, 0x54, 0xc4, 0x66, 0xdd, 0x60, 0x37, 0x54, 0x52, 0x47, 0x36, 0xa7,
                0x1e, 0xb2, 0x35, 0xde, 0xf9, 0xb5, 0x06, 0x96, 0x5e, 0x32, 0xd5, 0x68, 0x26, 0xe1,
                0x9f, 0xba, 0xd6, 0xc1,
            ],
            Elements::FullRightShift16_1 => [
                0xb3, 0x79, 0xe2, 0x96, 0xe9, 0xa9, 0x8f, 0xb3, 0xb5, 0x66, 0x2b, 0x8b, 0xa0, 0x4e,
                0x3c, 0xc1, 0xa4, 0x3c, 0x74, 0x42, 0x9e, 0x93, 0x12, 0x33, 0xfd, 0xd7, 0xfc, 0x8f,
                0xe6, 0xb7, 0xa2, 0xe0,
            ],
            Elements::FullRightShift16_2 => [
                0xae, 0xb8, 0xc6, 0x08, 0x06, 0xa4, 0x79, 0x20, 0x77, 0x58, 0xe3, 0x90, 0x83, 0xb4,
                0xa9, 0xa7, 0xa1, 0x4d, 0xa4, 0xee, 0x9b, 0xc1, 0x09, 0x7f, 0xc5, 0xcb, 0x4b, 0x75,
                0x54, 0x0d, 0x75, 0x78,
            ],
            Elements::FullRightShift16_4 => [
                0x60, 0xb7, 0xf0, 0x84, 0x75, 0xcc, 0x0c, 0xce, 0x64, 0xdc, 0xa1, 0x2d, 0x9f, 0x6a,
                0x91, 0x9c, 0x30, 0x61, 0x81, 0x10, 0xed, 0xa1, 0x40, 0x65, 0x92, 0x9c, 0x00, 0x4e,
                0x7f, 0xc1, 0xb0, 0xfb,
            ],
            Elements::FullRightShift16_8 => [
                0xf7, 0x9d, 0xba, 0x3e, 0x0a, 0xf3, 0xd6, 0xa5, 0x59, 0xa9, 0xe9, 0xdf, 0xfe, 0xa7,
                0x10, 0xaf, 0x62, 0x3f, 0xe6, 0xe6, 0x64, 0x4b, 0x89, 0x79, 0x95, 0xd7, 0x1b, 0x8a,
                0x41, 0x67, 0xdd, 0xb0,
            ],
            Elements::FullRightShift32_1 => [
                0xad, 0x0d, 0x5c, 0x75, 0xea, 0x68, 0x43, 0x71, 0x91, 0x77, 0x0d, 0x7f, 0xdf, 0x80,
                0x4b, 0xbc, 0x9d, 0x57, 0x3d, 0x5f, 0x10, 0x19, 0x98, 0x23, 0xd8, 0x09, 0xc9, 0xc4,
                0x6c, 0xd2, 0x75, 0xad,
            ],
            Elements::FullRightShift32_16 => [
                0x45, 0x52, 0x99, 0xfd, 0x6f, 0x42, 0xab, 0x49, 0xdb, 0xb7, 0x09, 0xe6, 0x5a, 0x3b,
                0x53, 0x66, 0x25, 0x0b, 0xdc, 0x54, 0x5d, 0x62, 0x29, 0xe8, 0xe2, 0x36, 0x05, 0x6d,
                0xdd, 0x19, 0x77, 0xfd,
            ],
            Elements::FullRightShift32_2 => [
                0x44, 0x38, 0x4b, 0x15, 0x06, 0xd4, 0x43, 0xd2, 0xf8, 0xa2, 0x88, 0x2b, 0x45, 0x63,
                0xd7, 0x93, 0x1a, 0x7e, 0xbc, 0xe6, 0x4a, 0xcf, 0x0d, 0x02, 0xee, 0x59, 0xec, 0x69,
                0xd3, 0x06, 0x52, 0x39,
            ],
            Elements::FullRightShift32_4 => [
                0x2e, 0x9a, 0x8a, 0xb5, 0xa1, 0x81, 0x7b, 0xd0, 0xb8, 0xa4, 0x66, 0x26, 0x99, 0x49,
                0x17, 0xa0, 0xde, 0x1a, 0x74, 0x5e, 0x99, 0x52, 0x0c, 0xe6, 0xeb, 0xcc, 0x67, 0xd4,
                0x63, 0x65, 0x51, 0xb7,
            ],
            Elements::FullRightShift32_8 => [
                0xaf, 0x47, 0xd4, 0xf9, 0x6e, 0x7d, 0x80, 0x26, 0xd4, 0x4e, 0x6e, 0xca, 0x1b, 0x80,
                0x7f, 0x73, 0x34, 0x4c, 0xe2, 0xea, 0xf7, 0x00, 0xb2, 0xc8, 0x2b, 0x4b, 0xb0, 0x02,
                0x61, 0xa8, 0x6f, 0x94,
            ],
            Elements::FullRightShift64_1 => [
                0x03, 0xaf, 0xb5, 0x47, 0xc3, 0x09, 0x13, 0xf1, 0x6f, 0x3e, 0x37, 0x0d, 0x7f, 0x9c,
                0xa0, 0x29, 0x0b, 0x61, 0x5b, 0x42, 0x85, 0x05, 0x1b, 0xb9, 0x3c, 0x3c, 0x1a, 0x9b,
                0x72, 0xee, 0x8d, 0xe4,
            ],
            Elements::FullRightShift64_16 => [
                0x1f, 0xb0, 0x56, 0xfc, 0xb6, 0x90, 0xce, 0xe3, 0xcf, 0xf7, 0x2c, 0x7d, 0xec, 0xda,
                0x80, 0x6d, 0x21, 0x46, 0xc4, 0x92, 0xae, 0x73, 0x1a, 0x6b, 0x94, 0xb8, 0xbb, 0x4f,
                0x15, 0x99, 0xb0, 0xcc,
            ],
            Elements::FullRightShift64_2 => [
                0x06, 0x73, 0xbf, 0xf2, 0x1e, 0x37, 0x5e, 0x5d, 0xbc, 0xaf, 0x38, 0x04, 0x66, 0x48,
                0x25, 0xdd, 0x67, 0x48, 0x44, 0xd2, 0xfd, 0xb7, 0x84, 0xa4, 0xfe, 0xfb, 0xc9, 0x25,
                0xcf, 0x6b, 0x27, 0xad,
            ],
            Elements::FullRightShift64_32 => [
                0x35, 0x6f, 0x7d, 0xd4, 0x6b, 0xa3, 0x3f, 0x84, 0xb0, 0x66, 0x72, 0xfd, 0xe9, 0xa2,
                0x97, 0x2e, 0x80, 0xf3, 0xea, 0x96, 0x5a, 0xe8, 0xbc, 0x0b, 0xff, 0x67, 0xaa, 0x2f,
                0x69, 0xf1, 0x0b, 0x56,
            ],
            Elements::FullRightShift64_4 => [
                0x4c, 0x25, 0xf6, 0x01, 0x1f, 0xd3, 0xd1, 0xac, 0x18, 0xe1, 0x1e, 0xb4, 0x30, 0x61,
                0xfa, 0xd6, 0x9f, 0x3c, 0xe3, 0x9f, 0x7a, 0x99, 0xce, 0xde, 0x50, 0xcc, 0x85, 0xbf,
                0x88, 0xbf, 0xba, 0x82,
            ],
            Elements::FullRightShift64_8 => [
                0xa5, 0x1d, 0xf9, 0x44, 0x86, 0x02, 0xfa, 0x81, 0x00, 0x1a, 0xa1, 0xb5, 0xb1, 0x3b,
                0xe8, 0x8d, 0x4b, 0x2f, 0x4d, 0x0f, 0x60, 0x74, 0x08, 0x01, 0xce, 0xf9, 0x91, 0x00,
                0x2f, 0xe3, 0x7d, 0x6d,
            ],
            Elements::FullRightShift8_1 => [
                0xd9, 0xd4, 0xb1, 0x6d, 0x37, 0xe4, 0xeb, 0x5c, 0xc5, 0x15, 0x04, 0x26, 0xe3, 0xe8,
                0x6c, 0xf6, 0x0a, 0xbb, 0xdf, 0xa1, 0xd0, 0xec, 0xb4, 0x15, 0x82, 0x96, 0x5e, 0x80,
                0x00, 0xcb, 0xd2, 0x91,
            ],
            Elements::FullRightShift8_2 => [
                0x07, 0x9a, 0xa1, 0x66, 0x17, 0x19, 0x8a, 0xd5, 0xdf, 0x2c, 0x98, 0xa6, 0x3a, 0xf7,
                0x6c, 0x1b, 0x3e, 0x12, 0x0f, 0xd2, 0x10, 0x6b, 0x22, 0x5f, 0x63, 0xfd, 0x06, 0xac,
                0x57, 0x1d, 0x04, 0xa4,
            ],
            Elements::FullRightShift8_4 => [
                0x9d, 0x9d, 0x3f, 0x63, 0x8a, 0x84, 0x63, 0x86, 0xa2, 0x1e, 0x71, 0x5f, 0x39, 0x46,
                0x16, 0x86, 0x4a, 0x2e, 0xf7, 0x98, 0x4a, 0x88, 0xcd, 0x95, 0x50, 0x55, 0x66, 0x29,
                0x7b, 0xe7, 0xe0, 0x6c,
            ],
            Elements::FullSubtract16 => [
                0x1f, 0xc8, 0x8e, 0x23, 0x29, 0xf4, 0xaa, 0xf1, 0x2b, 0x30, 0x51, 0x3f, 0x7a, 0x21,
                0xcf, 0x5d, 0x8d, 0xe2, 0x4b, 0x60, 0x0a, 0x19, 0xa2, 0x17, 0x41, 0x28, 0x1b, 0x4d,
                0x61, 0xaa, 0xc6, 0x33,
            ],
            Elements::FullSubtract32 => [
                0x78, 0x27, 0x05, 0xfb, 0x42, 0xe3, 0x6a, 0x7e, 0xf8, 0x31, 0x20, 0x0c, 0x61, 0x77,
                0x38, 0xd3, 0x1e, 0x13, 0xb1, 0xd0, 0xe7, 0xce, 0xed, 0x69, 0x3f, 0x13, 0x33, 0x88,
                0x35, 0xb3, 0x0a, 0xcb,
            ],
            Elements::FullSubtract64 => [
                0xb2, 0x85, 0x6a, 0x91, 0x80, 0x23, 0x1b, 0xee, 0x3c, 0xb8, 0x92, 0x30, 0xf7, 0x5c,
                0x29, 0x2a, 0xf3, 0xe7, 0x52, 0x39, 0xdb, 0xeb, 0x39, 0x65, 0x48, 0x44, 0x1e, 0x6b,
                0x5a, 0x27, 0xe8, 0x13,
            ],
            Elements::FullSubtract8 => [
                0x68, 0x85, 0xe1, 0x41, 0xae, 0x23, 0x4c, 0x1e, 0x2a, 0x7e, 0x4f, 0x23, 0x52, 0x98,
                0x93, 0x90, 0x36, 0x96, 0x9c, 0x95, 0x0f, 0x2c, 0xef, 0xd4, 0x59, 0xb4, 0x98, 0xac,
                0x3d, 0xd8, 0x92, 0x20,
            ],
            Elements::GeIsOnCurve => [
                0x69, 0xf0, 0xe7, 0xa0, 0xc5, 0xff, 0xf8, 0x70, 0x84, 0xed, 0x69, 0x25, 0xf8, 0xdb,
                0x76, 0x2e, 0x41, 0x9e, 0x05, 0x7b, 0x96, 0x83, 0x4d, 0xce, 0x96, 0x99, 0xb0, 0xb0,
                0x09, 0x42, 0x30, 0x59,
            ],
            Elements::GeNegate => [
                0x1e, 0xd0, 0xce, 0xd8, 0xdd, 0x25, 0x58, 0xe3, 0x48, 0x5f, 0x6f, 0xc3, 0x2d, 0x69,
                0xa2, 0x40, 0x5e, 0xca, 0xee, 0x31, 0x2d, 0xc4, 0xdc, 0x65, 0xe0, 0xfd, 0x34, 0x77,
                0x73, 0xf5, 0x98, 0x3d,
            ],
            Elements::GejAdd => [
                0x5a, 0x1c, 0x31, 0x03, 0x49, 0xe8, 0xff, 0x5c, 0x5a, 0x61, 0xac, 0x3e, 0x10, 0x12,
                0x3f, 0x74, 0xe8, 0x7f, 0xab, 0xa1, 0x4c, 0x78, 0xbc, 0x83, 0xf9, 0xe3, 0x41, 0x36,
                0x87, 0xec, 0xf2, 0x8b,
            ],
            Elements::GejDouble => [
                0x1e, 0xdd, 0x05, 0x82, 0xe2, 0xfc, 0xad, 0x99, 0xb1, 0x2d, 0x50, 0x6d, 0x29, 0xb5,
                0x0a, 0x63, 0x01, 0x7f, 0x67, 0x69, 0x28, 0xbe, 0x51, 0x13, 0x69, 0x00, 0x6e, 0x07,
                0xcb, 0x80, 0xd9, 0x82,
            ],
            Elements::GejEquiv => [
                0x02, 0x74, 0x71, 0x05, 0x94, 0x87, 0xa1, 0x2c, 0xa2, 0x07, 0xf0, 0x94, 0x05, 0x94,
                0xd6, 0xcd, 0x87, 0xfc, 0x93, 0x0a, 0x8b, 0x5b, 0x31, 0x43, 0x4a, 0x16, 0xa2, 0xd6,
                0x7f, 0x1d, 0x8d, 0xd4,
            ],
            Elements::GejGeAdd => [
                0x1e, 0xa7, 0x10, 0xd5, 0x6e, 0xaf, 0xee, 0x32, 0x5d, 0x26, 0x07, 0xdd, 0xb4, 0x5f,
                0xf0, 0x17, 0x0a, 0xde, 0xc2, 0xe0, 0xee, 0x9b, 0xcc, 0x68, 0xe4, 0xb9, 0x3e, 0x1d,
                0xe6, 0xad, 0x35, 0x68,
            ],
            Elements::GejGeAddEx => [
                0x78, 0xf0, 0x87, 0x1b, 0x81, 0x73, 0xab, 0xde, 0x71, 0x87, 0x11, 0x26, 0x3b, 0x3a,
                0xc1, 0xd9, 0x22, 0x33, 0x7e, 0xd5, 0xed, 0x13, 0x8d, 0x29, 0x49, 0x62, 0xd6, 0x5c,
                0xe5, 0x59, 0xbd, 0x92,
            ],
            Elements::GejGeEquiv => [
                0xba, 0x89, 0x9a, 0x00, 0x62, 0x16, 0xd1, 0xc9, 0x3b, 0xd5, 0xec, 0xbe, 0x00, 0x80,
                0xd9, 0x07, 0x8a, 0x50, 0x0a, 0x72, 0x9b, 0xbd, 0x39, 0x6a, 0x00, 0x4a, 0xf5, 0x1d,
                0x4f, 0xf7, 0xd9, 0x3a,
            ],
            Elements::GejInfinity => [
                0x88, 0xa9, 0x52, 0xdb, 0x38, 0x16, 0xe9, 0x42, 0x59, 0xa6, 0x75, 0x37, 0xfa, 0x8f,
                0xca, 0x1a, 0x35, 0xa9, 0x07, 0xa8, 0x6f, 0x51, 0xed, 0xe4, 0x51, 0xfd, 0x32, 0xec,
                0x25, 0x3d, 0x9c, 0x62,
            ],
            Elements::GejIsInfinity => [
                0x29, 0x80, 0xa7, 0x35, 0x41, 0x4e, 0x43, 0x21, 0xaf, 0xef, 0xfe, 0xfa, 0x88, 0x37,
                0xed, 0xb0, 0xa3, 0x30, 0x9a, 0x33, 0x7d, 0x59, 0xb7, 0xbd, 0xea, 0x92, 0x1c, 0x13,
                0x05, 0x6b, 0x04, 0x28,
            ],
            Elements::GejIsOnCurve => [
                0x01, 0x87, 0xe1, 0xe5, 0xef, 0x76, 0x34, 0xa5, 0xf0, 0x16, 0x12, 0x4d, 0x4f, 0xeb,
                0x5a, 0x93, 0xdd, 0xe6, 0xaa, 0x78, 0x17, 0x6c, 0xda, 0x48, 0xb1, 0x65, 0xa9, 0xaa,
                0x8e, 0x04, 0x49, 0xf2,
            ],
            Elements::GejNegate => [
                0xb3, 0x2c, 0x74, 0xca, 0xb2, 0xc7, 0x50, 0x0b, 0x73, 0xf8, 0xec, 0x05, 0x60, 0xfe,
                0x23, 0xfc, 0x4c, 0x21, 0xaa, 0x66, 0x59, 0x6d, 0x7f, 0x2a, 0xcf, 0x49, 0x67, 0x88,
                0x6b, 0x76, 0xd8, 0x56,
            ],
            Elements::GejNormalize => [
                0x5d, 0xe0, 0x97, 0x6a, 0xe7, 0xf3, 0x8b, 0x36, 0xf0, 0x02, 0x28, 0x14, 0x96, 0x6d,
                0xb2, 0xba, 0xed, 0x5c, 0x47, 0x67, 0x14, 0x94, 0x4d, 0x74, 0x1a, 0x89, 0x79, 0xc4,
                0xbc, 0xf8, 0xbe, 0x25,
            ],
            Elements::GejRescale => [
                0xdc, 0xfc, 0x72, 0xa7, 0x68, 0xd5, 0xbe, 0x77, 0x0f, 0x8d, 0xb2, 0x78, 0xae, 0xaf,
                0xd1, 0x8e, 0x27, 0x70, 0x4c, 0x64, 0xf8, 0xb4, 0x0f, 0xa6, 0xfe, 0x54, 0xca, 0x94,
                0x72, 0x7a, 0x07, 0x6e,
            ],
            Elements::GejXEquiv => [
                0x52, 0xcc, 0x21, 0x47, 0x09, 0xc0, 0xd9, 0xfc, 0xa9, 0xdb, 0x1d, 0x09, 0xcc, 0x80,
                0x7c, 0x75, 0xcf, 0x5a, 0x63, 0x13, 0xca, 0x54, 0x0a, 0x77, 0x2d, 0x4e, 0xa9, 0x92,
                0x1f, 0x37, 0xe6, 0x24,
            ],
            Elements::GejYIsOdd => [
                0xfe, 0x01, 0x06, 0xaf, 0xb9, 0xd9, 0xe2, 0x4f, 0xd4, 0xdb, 0xe5, 0x45, 0x11, 0xfe,
                0x27, 0x2f, 0x4d, 0xcb, 0x30, 0x7a, 0x0e, 0xa5, 0x6d, 0x59, 0x1c, 0xeb, 0x93, 0xab,
                0x4b, 0xf8, 0x87, 0x45,
            ],
            Elements::Generate => [
                0xdf, 0x44, 0xe1, 0x7d, 0x2a, 0x55, 0x9d, 0xd0, 0xa7, 0x03, 0x49, 0x54, 0xab, 0x33,
                0x37, 0x77, 0x78, 0xb1, 0x51, 0xf1, 0xcd, 0x1e, 0x4f, 0x9f, 0xd3, 0x1b, 0x36, 0x1d,
                0x34, 0xa8, 0xd9, 0x73,
            ],
            Elements::GenesisBlockHash => [
                0x00, 0x60, 0xb6, 0x0d, 0x02, 0xb1, 0x33, 0x60, 0x72, 0x84, 0x6a, 0xcf, 0xf4, 0x4d,
                0xd0, 0xf4, 0x34, 0x6c, 0x74, 0xa3, 0x5e, 0x7b, 0x56, 0x42, 0x3f, 0x95, 0x19, 0xc4,
                0x42, 0xd1, 0x5d, 0xaf,
            ],
            Elements::HashToCurve => [
                0x76, 0xf7, 0xca, 0x1d, 0xb9, 0x44, 0xee, 0x31, 0x5e, 0xd3, 0x62, 0xfe, 0xe0, 0x67,
                0x3c, 0x58, 0x94, 0xf8, 0x85, 0x3b, 0x44, 0x60, 0x70, 0x90, 0x1b, 0x85, 0x79, 0x01,
                0xf9, 0x49, 0x9d, 0x9b,
            ],
            Elements::High1 => [
                0xc3, 0x2d, 0x87, 0x7e, 0x67, 0x0d, 0x6c, 0x03, 0x7c, 0xb3, 0x35, 0x33, 0x28, 0x9e,
                0x19, 0xa7, 0x24, 0xc3, 0x68, 0xaa, 0x75, 0x51, 0xda, 0xa6, 0xd2, 0xda, 0xcc, 0xcd,
                0x8c, 0x95, 0xf4, 0xd0,
            ],
            Elements::High16 => [
                0x41, 0x64, 0xab, 0x6e, 0x2f, 0xf8, 0xee, 0xf6, 0x3c, 0x06, 0xc0, 0x80, 0xf1, 0xde,
                0xc6, 0x97, 0x0b, 0x4c, 0x5c, 0x31, 0xc0, 0x23, 0x05, 0xab, 0xcc, 0xd8, 0xed, 0x2c,
                0x5e, 0x1c, 0x45, 0xce,
            ],
            Elements::High32 => [
                0xd3, 0xa7, 0xce, 0x9c, 0xd5, 0xd5, 0xfb, 0x67, 0x9a, 0x98, 0xef, 0x57, 0xb8, 0x63,
                0x22, 0x77, 0x0c, 0xb6, 0x6f, 0xb6, 0xf0, 0x61, 0x6e, 0x16, 0x34, 0xcf, 0xa8, 0x4c,
                0x8f, 0x68, 0x09, 0xc6,
            ],
            Elements::High64 => [
                0x4a, 0xf9, 0x1f, 0xaf, 0x8e, 0x39, 0xf4, 0xda, 0x7c, 0x28, 0xa8, 0x79, 0x65, 0x94,
                0xa9, 0x22, 0x82, 0x13, 0xd7, 0x32, 0x3e, 0xea, 0x2c, 0xa6, 0x30, 0x75, 0x2c, 0xe4,
                0xc5, 0x7f, 0x16, 0xe1,
            ],
            Elements::High8 => [
                0xcb, 0xd7, 0x8d, 0x50, 0xaf, 0x77, 0x99, 0x85, 0x5a, 0xdc, 0x49, 0x03, 0xdb, 0xbe,
                0xfc, 0x13, 0x45, 0xd5, 0x14, 0x84, 0xf0, 0x3d, 0x3c, 0x75, 0x5c, 0xaa, 0xa5, 0xca,
                0xa9, 0x7d, 0x4a, 0x14,
            ],
            Elements::Increment16 => [
                0xdf, 0x27, 0x48, 0x88, 0xce, 0x4c, 0xeb, 0xdd, 0x57, 0x08, 0xb3, 0x8d, 0xc3, 0xdb,
                0xb1, 0x9c, 0xc2, 0xf0, 0x36, 0x4b, 0x24, 0x63, 0xe9, 0x9c, 0xf5, 0xaa, 0xb4, 0xf8,
                0xa2, 0x3e, 0xa5, 0x8a,
            ],
            Elements::Increment32 => [
                0x54, 0xf7, 0x57, 0xae, 0xa7, 0x6b, 0xc7, 0xa3, 0x9f, 0xc4, 0x3d, 0x19, 0xb8, 0xdd,
                0x56, 0x3a, 0x68, 0x07, 0xdf, 0x02, 0x77, 0xa5, 0x6f, 0xcb, 0x50, 0x10, 0x89, 0xce,
                0x7d, 0x06, 0x77, 0x4c,
            ],
            Elements::Increment64 => [
                0x79, 0xed, 0x5f, 0x77, 0x99, 0xfb, 0x09, 0xda, 0x51, 0x04, 0x29, 0xa2, 0x01, 0x28,
                0xbe, 0xd0, 0x91, 0xd8, 0x58, 0x76, 0x47, 0x07, 0x12, 0x85, 0xcd, 0xec, 0x3a, 0x0c,
                0x95, 0x70, 0x9e, 0x5b,
            ],
            Elements::Increment8 => [
                0x0c, 0x71, 0x7e, 0x84, 0xdf, 0x67, 0x82, 0x3f, 0x57, 0x41, 0xb3, 0xd5, 0x5d, 0xbe,
                0xb4, 0x72, 0x9c, 0x2b, 0xd6, 0x2f, 0x5d, 0x1d, 0xef, 0x3c, 0xab, 0xcc, 0xdd, 0x6c,
                0xb8, 0xdc, 0xb5, 0x6c,
            ],
            Elements::InputAmount => [
                0xdd, 0x65, 0xac, 0x32, 0xc8, 0x7d, 0x99, 0x6b, 0x40, 0x72, 0x00, 0xfe, 0xef, 0xc4,
                0x79, 0xa9, 0x11, 0x9c, 0x1f, 0x28, 0xca, 0xa4, 0x88, 0xb0, 0xaa, 0xc3, 0xac, 0xf5,
                0x93, 0x78, 0xd6, 0xba,
            ],
            Elements::InputAmountsHash => [
                0x2b, 0x4d, 0xf4, 0x7e, 0x75, 0x8f, 0xfe, 0x88, 0xe9, 0xcd, 0x5d, 0x39, 0x6d, 0xf3,
                0xd7, 0xf9, 0xec, 0x2f, 0xbd, 0x5a, 0x53, 0x95, 0xa6, 0xd8, 0xc1, 0xab, 0x70, 0x44,
                0x00, 0xc6, 0x4f, 0x70,
            ],
            Elements::InputAnnexHash => [
                0x0f, 0xd9, 0xe4, 0xc6, 0x31, 0x6b, 0x37, 0x28, 0x6e, 0x88, 0x54, 0x43, 0x0e, 0x7d,
                0x74, 0x18, 0x21, 0xdc, 0x89, 0x54, 0x4a, 0x3a, 0x59, 0xf0, 0x3d, 0xfa, 0x40, 0xe5,
                0x32, 0xb8, 0x10, 0x7f,
            ],
            Elements::InputAnnexesHash => [
                0xde, 0xd1, 0x52, 0x2e, 0xfa, 0x5a, 0x2b, 0x37, 0x61, 0x73, 0xe4, 0x40, 0xde, 0x22,
                0x7a, 0xd0, 0xc9, 0x2b, 0xe2, 0xed, 0xce, 0x7c, 0x23, 0xb5, 0x14, 0x80, 0xe5, 0x0a,
                0x77, 0xfc, 0x56, 0x4b,
            ],
            Elements::InputAsset => [
                0x87, 0x2b, 0x36, 0x53, 0xd6, 0x5a, 0xe7, 0xf9, 0x16, 0x6c, 0xbf, 0x70, 0xd4, 0xfd,
                0x31, 0x40, 0x47, 0x98, 0x88, 0x4d, 0x61, 0x1b, 0x64, 0x7f, 0xb9, 0xf4, 0xa3, 0x07,
                0xc4, 0x80, 0x11, 0xb8,
            ],
            Elements::InputHash => [
                0x33, 0x09, 0xbb, 0x46, 0xb3, 0x15, 0x8d, 0x23, 0x12, 0x4f, 0x8c, 0xed, 0xaa, 0xa1,
                0xed, 0x3b, 0x09, 0xa8, 0xae, 0xfe, 0x81, 0x21, 0x2e, 0x11, 0x33, 0x55, 0x23, 0xb6,
                0xb2, 0x07, 0xc5, 0x44,
            ],
            Elements::InputOutpointsHash => [
                0x14, 0x0a, 0x2b, 0xae, 0xad, 0x08, 0x3b, 0xaa, 0xc8, 0x06, 0x12, 0x8a, 0xb6, 0x49,
                0xe5, 0x2b, 0xbd, 0x3c, 0x64, 0x56, 0x70, 0x0c, 0xbc, 0xfc, 0x6d, 0xbb, 0x29, 0x59,
                0x17, 0x3a, 0x4d, 0xf2,
            ],
            Elements::InputPegin => [
                0xdb, 0x37, 0x6f, 0x4a, 0xe8, 0x4e, 0xc0, 0x51, 0xa4, 0x94, 0xd7, 0x66, 0x80, 0x39,
                0x72, 0x7b, 0x53, 0x35, 0x3b, 0xe5, 0xae, 0x04, 0x5b, 0xe7, 0x6c, 0x4b, 0x04, 0xda,
                0xbd, 0x5b, 0xbf, 0xbb,
            ],
            Elements::InputPrevOutpoint => [
                0x91, 0xf4, 0xd6, 0xec, 0x48, 0x4f, 0x60, 0xca, 0xe9, 0x29, 0x78, 0x58, 0xca, 0xc4,
                0x7d, 0xad, 0x50, 0xdb, 0x07, 0xdc, 0x1f, 0x93, 0x7a, 0xc6, 0x14, 0x4c, 0x9c, 0xb4,
                0xa9, 0xef, 0x79, 0xd2,
            ],
            Elements::InputScriptHash => [
                0xc3, 0x16, 0xdf, 0x21, 0x77, 0x8e, 0x62, 0x41, 0x05, 0xca, 0x59, 0x90, 0x4b, 0x92,
                0x08, 0xe2, 0xd4, 0x23, 0x22, 0x8b, 0x3e, 0xb1, 0xcf, 0x68, 0xb8, 0xec, 0xa4, 0x7b,
                0xbc, 0x7b, 0x2f, 0xf3,
            ],
            Elements::InputScriptSigHash => [
                0x94, 0x1f, 0x2b, 0x4f, 0x3b, 0x7f, 0x20, 0x97, 0xd6, 0xea, 0x00, 0xe9, 0x7c, 0x1e,
                0x2b, 0xa0, 0x1c, 0xb2, 0x64, 0x87, 0xc5, 0xd2, 0xf4, 0x72, 0x37, 0x38, 0xb3, 0x7c,
                0x73, 0x2c, 0x05, 0xc4,
            ],
            Elements::InputScriptSigsHash => [
                0x6c, 0x74, 0x81, 0x42, 0x01, 0x24, 0xb6, 0x64, 0x5d, 0x32, 0xff, 0x9b, 0x45, 0x4a,
                0x1c, 0x92, 0xc6, 0x71, 0x86, 0x41, 0x0e, 0x23, 0xcf, 0x60, 0x99, 0x15, 0xdf, 0x90,
                0x72, 0x12, 0x0f, 0xc6,
            ],
            Elements::InputScriptsHash => [
                0xdc, 0x13, 0x51, 0x9e, 0x75, 0x3e, 0x58, 0x75, 0x3c, 0x70, 0x98, 0x85, 0x41, 0xad,
                0x8a, 0x0b, 0xb5, 0x88, 0x8d, 0x08, 0x46, 0xe3, 0x5e, 0x4d, 0x42, 0x73, 0xf6, 0x56,
                0x85, 0x32, 0x2a, 0xe9,
            ],
            Elements::InputSequence => [
                0x3c, 0x6e, 0x4a, 0x3c, 0x21, 0xce, 0xd5, 0x24, 0x73, 0x6c, 0x54, 0x5c, 0x20, 0xe4,
                0xe2, 0xde, 0xf7, 0x67, 0x8f, 0x08, 0x4c, 0xd1, 0x2b, 0xcd, 0x65, 0x06, 0xa5, 0x20,
                0x0b, 0x5c, 0x17, 0x38,
            ],
            Elements::InputSequencesHash => [
                0xc8, 0x88, 0xc8, 0x73, 0x30, 0x2b, 0x92, 0xb0, 0x0f, 0x20, 0x68, 0x52, 0xc9, 0xea,
                0xef, 0x00, 0x0b, 0x40, 0x10, 0x9a, 0x53, 0xdd, 0x68, 0x6a, 0x53, 0xf1, 0x1d, 0xb5,
                0x07, 0x4d, 0x99, 0x37,
            ],
            Elements::InputUtxoHash => [
                0x85, 0x03, 0xd0, 0x52, 0x6b, 0xa9, 0x5a, 0x8f, 0xba, 0x8c, 0xa1, 0x35, 0x1c, 0x79,
                0x91, 0xc2, 0xa3, 0x86, 0xb7, 0x2f, 0xa8, 0x14, 0x31, 0xea, 0xdb, 0xab, 0x62, 0xcc,
                0x1f, 0x5d, 0xc5, 0x18,
            ],
            Elements::InputUtxosHash => [
                0x9c, 0x48, 0xab, 0x31, 0xc0, 0x50, 0xed, 0x06, 0x5a, 0xed, 0x51, 0x68, 0x66, 0xc1,
                0x6c, 0x1b, 0x96, 0xc1, 0x95, 0xea, 0x8e, 0x18, 0xb0, 0x24, 0xbf, 0xc0, 0xb8, 0x17,
                0x58, 0x49, 0xf0, 0x4e,
            ],
            Elements::InputsHash => [
                0x52, 0x3c, 0xd7, 0x1c, 0x57, 0x89, 0x88, 0x2f, 0x83, 0x75, 0xa5, 0xa5, 0x45, 0x60,
                0xde, 0xf8, 0x46, 0x3d, 0x1c, 0x51, 0xe5, 0x6b, 0x7e, 0x11, 0x26, 0x93, 0xe7, 0x7b,
                0x84, 0x4f, 0x47, 0x40,
            ],
            Elements::InternalKey => [
                0x9b, 0x74, 0x2e, 0x47, 0x1b, 0x0e, 0x59, 0x1a, 0x97, 0x2b, 0x0d, 0x44, 0x21, 0xf9,
                0xc9, 0x65, 0x93, 0x97, 0xf5, 0xda, 0xba, 0x2c, 0x14, 0xda, 0xc6, 0x2c, 0x4a, 0x33,
                0x9c, 0xc3, 0xe7, 0xec,
            ],
            Elements::IsOne16 => [
                0x84, 0x35, 0x87, 0x9c, 0xcb, 0x86, 0x44, 0x19, 0x8d, 0xcb, 0x9a, 0x0c, 0xd7, 0x35,
                0x46, 0xd7, 0x01, 0xfd, 0xd5, 0xa4, 0xc4, 0x43, 0x23, 0xf5, 0x63, 0x97, 0x15, 0x99,
                0xc3, 0x7d, 0x16, 0xfb,
            ],
            Elements::IsOne32 => [
                0xdd, 0xfb, 0xd9, 0xf0, 0xa2, 0xe6, 0x7c, 0x07, 0xde, 0xdb, 0x89, 0xe8, 0x96, 0xb6,
                0xc4, 0xf7, 0xd4, 0x5c, 0x51, 0x47, 0xee, 0xd0, 0x61, 0x4e, 0x4c, 0xe7, 0xd0, 0x87,
                0x69, 0xaf, 0xf8, 0x2d,
            ],
            Elements::IsOne64 => [
                0x35, 0xc5, 0x25, 0x54, 0x8e, 0x48, 0xee, 0xa0, 0xf7, 0x7b, 0x3b, 0xf9, 0x7a, 0xb6,
                0x7a, 0x1f, 0xfe, 0x8f, 0xb0, 0x94, 0xed, 0xe3, 0x32, 0x5e, 0x40, 0x64, 0xb1, 0x65,
                0x9c, 0x6d, 0x07, 0x65,
            ],
            Elements::IsOne8 => [
                0x0a, 0xba, 0x9e, 0x57, 0x6e, 0x64, 0xd2, 0x80, 0x4c, 0x8a, 0xc4, 0x68, 0x2b, 0xbb,
                0xa5, 0x39, 0x0e, 0xbc, 0x31, 0xa6, 0xe3, 0xe2, 0x65, 0x0f, 0x92, 0x19, 0x23, 0x5d,
                0xf4, 0xa6, 0xec, 0xbb,
            ],
            Elements::IsZero16 => [
                0xa2, 0x5a, 0xbd, 0x9c, 0xd2, 0xa4, 0x07, 0x0c, 0x74, 0x2e, 0xf8, 0xde, 0xb0, 0x68,
                0x29, 0x22, 0x46, 0x03, 0x2b, 0x96, 0xa5, 0x17, 0x22, 0x3b, 0x12, 0x8c, 0xfc, 0x12,
                0xd2, 0x15, 0xc5, 0xba,
            ],
            Elements::IsZero32 => [
                0x61, 0x2a, 0x48, 0x0c, 0xed, 0x6a, 0x79, 0xda, 0x61, 0x19, 0x54, 0x6e, 0x05, 0x6b,
                0x8d, 0xf9, 0xfa, 0x95, 0xd1, 0x12, 0x4b, 0x96, 0xd6, 0x01, 0xe1, 0xd3, 0xea, 0x91,
                0x8c, 0xc5, 0x60, 0x69,
            ],
            Elements::IsZero64 => [
                0x18, 0xe8, 0xe1, 0x77, 0x6b, 0xa0, 0x80, 0xcc, 0xd3, 0xe1, 0xd6, 0x0c, 0xb7, 0x53,
                0x41, 0x45, 0x36, 0xbf, 0x70, 0xdf, 0x18, 0x5f, 0x72, 0xc9, 0xe0, 0x70, 0x79, 0x6f,
                0x4c, 0x63, 0xcc, 0x71,
            ],
            Elements::IsZero8 => [
                0xb4, 0xba, 0xa5, 0x09, 0x38, 0x10, 0x84, 0x26, 0x74, 0x0d, 0x82, 0xcf, 0x12, 0x11,
                0xe0, 0xed, 0x12, 0x6d, 0xe3, 0xb7, 0x6b, 0x8d, 0x25, 0x9c, 0x50, 0xad, 0x4b, 0x8f,
                0xca, 0xb1, 0x0a, 0xb6,
            ],
            Elements::Issuance => [
                0x5c, 0x64, 0x63, 0x12, 0xc1, 0x69, 0xc6, 0x80, 0x27, 0x97, 0x9e, 0x1b, 0xc3, 0x26,
                0xc5, 0xdc, 0x95, 0xe5, 0xc5, 0x16, 0x8d, 0x00, 0xd9, 0x8e, 0x9d, 0x50, 0x4a, 0x7c,
                0xde, 0x21, 0xd7, 0x68,
            ],
            Elements::IssuanceAsset => [
                0x69, 0x0b, 0xf9, 0x18, 0xe1, 0x52, 0x77, 0x56, 0xcf, 0xbb, 0xf5, 0x1c, 0x83, 0x13,
                0x62, 0x14, 0x37, 0x56, 0xd5, 0x2d, 0x04, 0xe1, 0x29, 0x4f, 0x12, 0x64, 0x95, 0x0c,
                0x62, 0x67, 0xe5, 0xc9,
            ],
            Elements::IssuanceAssetAmount => [
                0x83, 0x60, 0x51, 0x46, 0xff, 0x26, 0xa4, 0x9c, 0xa1, 0x6f, 0x1c, 0x4a, 0x6e, 0x0c,
                0xe0, 0xb7, 0xc2, 0x14, 0xdf, 0x12, 0xc3, 0x77, 0xf5, 0x22, 0x52, 0x3d, 0xeb, 0x6d,
                0x5d, 0x0e, 0xe4, 0x4d,
            ],
            Elements::IssuanceAssetAmountsHash => [
                0x70, 0xf1, 0x1c, 0x3b, 0x3b, 0x5d, 0x16, 0x57, 0xd3, 0xb7, 0xc7, 0xee, 0xec, 0x85,
                0x0c, 0x30, 0xa3, 0x2e, 0xa9, 0x6a, 0xa3, 0x64, 0x26, 0xfa, 0x30, 0xeb, 0xf5, 0x49,
                0x98, 0x51, 0xa8, 0x80,
            ],
            Elements::IssuanceAssetProof => [
                0x0b, 0x06, 0x1f, 0x9f, 0x67, 0x0c, 0x05, 0xdc, 0x39, 0xf4, 0x38, 0x31, 0x3c, 0x13,
                0x79, 0xd9, 0xbf, 0x63, 0xf4, 0x4f, 0x98, 0x76, 0xb9, 0x56, 0x69, 0x3b, 0x41, 0xb7,
                0xf9, 0x4c, 0x6c, 0x9d,
            ],
            Elements::IssuanceBlindingEntropyHash => [
                0xbf, 0xb6, 0xa9, 0x34, 0x44, 0xe2, 0xa0, 0xb3, 0xfb, 0x7c, 0x13, 0xd1, 0x58, 0x9a,
                0x78, 0x7b, 0x66, 0xe8, 0x4b, 0x69, 0xb6, 0x27, 0x4c, 0xea, 0x20, 0xad, 0x99, 0x99,
                0xeb, 0x8c, 0xa6, 0x7c,
            ],
            Elements::IssuanceEntropy => [
                0x6d, 0xdf, 0x99, 0xf2, 0x8c, 0x70, 0x19, 0x8d, 0x65, 0x41, 0x7a, 0x83, 0x60, 0x74,
                0xf1, 0xb9, 0xb6, 0x10, 0xf2, 0x1f, 0x51, 0x19, 0xfd, 0x55, 0x0f, 0x60, 0xb6, 0xf8,
                0xe9, 0x95, 0xbb, 0xf3,
            ],
            Elements::IssuanceHash => [
                0x6e, 0x9f, 0x8a, 0xe8, 0xc4, 0x5b, 0xfb, 0x7e, 0x4e, 0x0c, 0xf2, 0xd0, 0x0f, 0x66,
                0x83, 0x2b, 0x47, 0x9d, 0xe7, 0x58, 0x2c, 0xdb, 0xc8, 0x51, 0xd0, 0xb3, 0x23, 0x0f,
                0xdf, 0xbb, 0xb7, 0x8d,
            ],
            Elements::IssuanceRangeProofsHash => [
                0x7e, 0x97, 0xf5, 0x19, 0x53, 0x48, 0x31, 0xc7, 0x7a, 0xcf, 0x73, 0xa2, 0x0f, 0xd5,
                0x3c, 0xb3, 0x1d, 0x1d, 0x15, 0x6c, 0x9a, 0xcb, 0xbf, 0x87, 0x3d, 0x14, 0x92, 0xd6,
                0x0d, 0x13, 0x67, 0xd8,
            ],
            Elements::IssuanceToken => [
                0xbc, 0x2c, 0xb7, 0xe9, 0x64, 0xb2, 0xa2, 0xae, 0xd7, 0x9b, 0x75, 0xc5, 0xf4, 0x40,
                0x8f, 0xe8, 0x4a, 0x6d, 0xc9, 0xb9, 0xe8, 0xcd, 0x2c, 0x69, 0x4d, 0x57, 0xc5, 0xf9,
                0xb6, 0x12, 0x05, 0xb9,
            ],
            Elements::IssuanceTokenAmount => [
                0x7e, 0x10, 0x64, 0x64, 0xa1, 0x3b, 0x67, 0x64, 0xce, 0x74, 0xd3, 0xfe, 0x6b, 0xa1,
                0xcb, 0xd5, 0xf4, 0xae, 0xb8, 0x6b, 0xbb, 0xdf, 0x58, 0x88, 0xdd, 0xf9, 0x90, 0x1d,
                0xd8, 0xb6, 0x19, 0xaa,
            ],
            Elements::IssuanceTokenAmountsHash => [
                0x94, 0xb5, 0xa4, 0x59, 0x5b, 0x67, 0x4d, 0xbf, 0xc4, 0x9c, 0x4e, 0x8e, 0xa1, 0xa9,
                0xab, 0xc9, 0x3e, 0xd2, 0xf7, 0x70, 0x7c, 0x42, 0x12, 0xa3, 0x2b, 0x5b, 0x86, 0xac,
                0x13, 0x18, 0xc9, 0xde,
            ],
            Elements::IssuanceTokenProof => [
                0x46, 0xed, 0x52, 0x75, 0xf5, 0x5a, 0xb3, 0x41, 0xf5, 0x7c, 0x35, 0x19, 0xff, 0xed,
                0xf4, 0xe3, 0x7f, 0x3f, 0x59, 0xc9, 0x7c, 0xe9, 0xb8, 0xe1, 0x7f, 0xff, 0xae, 0x16,
                0xc6, 0xcc, 0x1c, 0x0e,
            ],
            Elements::IssuancesHash => [
                0x58, 0x65, 0x9f, 0x3f, 0x7e, 0x01, 0x30, 0xc4, 0xfc, 0x91, 0x6f, 0x49, 0xbc, 0x32,
                0xea, 0xa0, 0x88, 0x3a, 0xcb, 0x90, 0x22, 0xfc, 0xad, 0x5e, 0x78, 0x05, 0x83, 0x9a,
                0x99, 0xe5, 0x0c, 0xf9,
            ],
            Elements::LbtcAsset => [
                0x2a, 0xf7, 0xf7, 0x3a, 0xe3, 0xfb, 0xbf, 0xa2, 0xd2, 0x3a, 0x87, 0x74, 0xe6, 0xe3,
                0xca, 0x09, 0x32, 0x54, 0xda, 0x2f, 0xd2, 0xa9, 0xf3, 0x97, 0xa6, 0xae, 0x0a, 0x30,
                0xbb, 0x48, 0xf0, 0xd7,
            ],
            Elements::Le16 => [
                0x63, 0xda, 0x72, 0x7c, 0xcb, 0x4c, 0x6a, 0x9d, 0x4e, 0x00, 0x09, 0x64, 0xe7, 0x63,
                0xbf, 0xf9, 0x34, 0xea, 0xaf, 0xd0, 0x44, 0x28, 0x7e, 0x12, 0x68, 0xd0, 0x7e, 0xcd,
                0xfd, 0xe2, 0x07, 0xe1,
            ],
            Elements::Le32 => [
                0xde, 0xe2, 0x9a, 0x91, 0x65, 0x6d, 0x7a, 0xe7, 0x3d, 0xf4, 0x95, 0x6f, 0xd8, 0xa2,
                0xc6, 0xb6, 0x27, 0xaa, 0xb5, 0x1c, 0x11, 0x29, 0xf9, 0xfe, 0x7f, 0x6e, 0xd3, 0xe3,
                0x47, 0x92, 0xc7, 0x62,
            ],
            Elements::Le64 => [
                0x01, 0xc5, 0x5d, 0xf7, 0xd4, 0x46, 0x59, 0x66, 0x65, 0x9d, 0xdf, 0xc9, 0x4b, 0x36,
                0xd0, 0x33, 0x24, 0x2c, 0x2e, 0xc5, 0x93, 0xce, 0xe1, 0x21, 0x22, 0x44, 0x07, 0x75,
                0x66, 0xed, 0x01, 0x5f,
            ],
            Elements::Le8 => [
                0x0f, 0xb7, 0x2d, 0x9f, 0x8e, 0xe2, 0x37, 0x0a, 0xba, 0x55, 0x66, 0x3a, 0x48, 0x99,
                0x16, 0x2e, 0x40, 0xca, 0x55, 0x14, 0x71, 0x3e, 0xfb, 0x25, 0xe4, 0xa8, 0x9e, 0x2a,
                0x10, 0x4b, 0x34, 0xdb,
            ],
            Elements::LeftExtend16_32 => [
                0xdc, 0xf4, 0x2b, 0x65, 0x42, 0xf6, 0xd4, 0x1c, 0xb7, 0xb5, 0x0e, 0x7c, 0x77, 0x2f,
                0x3c, 0x7f, 0x6e, 0x43, 0x22, 0x32, 0xf2, 0xba, 0x20, 0x79, 0xb3, 0x86, 0xa0, 0x5d,
                0x7b, 0x46, 0x6a, 0xdd,
            ],
            Elements::LeftExtend16_64 => [
                0x2e, 0xee, 0x48, 0xa9, 0x22, 0x37, 0x94, 0x7c, 0x1a, 0x51, 0x7d, 0xf9, 0x95, 0xf4,
                0x4f, 0x1d, 0xfe, 0xf2, 0x0d, 0xdb, 0x4e, 0x9b, 0x53, 0x0b, 0x22, 0xd1, 0x8a, 0x0a,
                0x7f, 0xd6, 0x28, 0xaa,
            ],
            Elements::LeftExtend1_16 => [
                0x9a, 0x48, 0xa4, 0x77, 0x8e, 0x7c, 0x3c, 0x28, 0x5a, 0xb6, 0x53, 0x29, 0xd1, 0xcc,
                0xc4, 0x99, 0x9d, 0x2d, 0x19, 0x4e, 0x00, 0x5b, 0xd7, 0x94, 0x69, 0x49, 0x53, 0x3d,
                0x8c, 0xba, 0x80, 0x6c,
            ],
            Elements::LeftExtend1_32 => [
                0xda, 0xb6, 0xa5, 0x33, 0xcb, 0xcb, 0xe8, 0x36, 0x2c, 0xf1, 0xd5, 0xa1, 0x6e, 0xa3,
                0x7c, 0xbc, 0x7e, 0xdc, 0x7f, 0xc8, 0xa9, 0x42, 0x85, 0x71, 0xe1, 0x71, 0xec, 0x6e,
                0xe4, 0x4d, 0x08, 0x00,
            ],
            Elements::LeftExtend1_64 => [
                0x11, 0x0e, 0x5c, 0x1e, 0xf0, 0xb4, 0x69, 0xa7, 0x63, 0x85, 0x70, 0xda, 0x94, 0x4d,
                0x23, 0x2e, 0x0f, 0x28, 0xc4, 0x61, 0x51, 0xa2, 0x25, 0x35, 0x7d, 0xe3, 0xe9, 0x04,
                0x57, 0xa8, 0x8e, 0xa2,
            ],
            Elements::LeftExtend1_8 => [
                0x5a, 0x83, 0x1c, 0xa9, 0x96, 0x21, 0x51, 0x7a, 0x2b, 0x35, 0x4e, 0x5c, 0xac, 0x38,
                0xbc, 0x3a, 0x30, 0xc4, 0x00, 0x1f, 0x20, 0xd2, 0x5d, 0x77, 0x97, 0xad, 0xdc, 0xac,
                0x5d, 0xa8, 0x61, 0x06,
            ],
            Elements::LeftExtend32_64 => [
                0x84, 0xfc, 0xc6, 0x9b, 0xa1, 0xdb, 0x50, 0xdb, 0xd5, 0x36, 0x3c, 0xf2, 0x77, 0x79,
                0x57, 0x60, 0x1d, 0xe2, 0x56, 0x8a, 0xdf, 0x07, 0xaf, 0x41, 0x61, 0xde, 0xbb, 0x1e,
                0x5e, 0x37, 0x31, 0x0a,
            ],
            Elements::LeftExtend8_16 => [
                0xfe, 0xa1, 0xf2, 0x5a, 0x82, 0xfd, 0xf6, 0xf8, 0x66, 0x9c, 0xc4, 0x0f, 0xbb, 0x8e,
                0x54, 0xa9, 0x26, 0x58, 0xbf, 0xab, 0x94, 0xeb, 0x08, 0x2f, 0x71, 0x7b, 0xa2, 0x65,
                0xb5, 0xd8, 0x44, 0xb4,
            ],
            Elements::LeftExtend8_32 => [
                0x09, 0xd7, 0x03, 0xca, 0x46, 0xf7, 0x5d, 0x05, 0x1a, 0x93, 0xd0, 0xe8, 0xa2, 0xaf,
                0x05, 0x01, 0xa3, 0x8e, 0x84, 0x86, 0x83, 0xef, 0x10, 0x9c, 0x1f, 0xb4, 0xb5, 0xbe,
                0x20, 0xe6, 0x31, 0x5d,
            ],
            Elements::LeftExtend8_64 => [
                0xd3, 0xda, 0xfc, 0xbd, 0xab, 0x69, 0xa2, 0xbb, 0x32, 0x0f, 0x8d, 0x23, 0x0c, 0xef,
                0xd0, 0x9c, 0x27, 0xa1, 0x54, 0xc5, 0x1e, 0x7e, 0x5c, 0xd5, 0x33, 0x4e, 0xaf, 0xed,
                0x19, 0xe2, 0x0d, 0xf4,
            ],
            Elements::LeftPadHigh16_32 => [
                0x88, 0x8c, 0x7e, 0x0a, 0xb0, 0x03, 0x14, 0x75, 0xc5, 0x14, 0xf9, 0xb3, 0x7c, 0x81,
                0xf4, 0x5a, 0x47, 0x31, 0x49, 0x84, 0xe5, 0x02, 0x75, 0x08, 0xdd, 0xc5, 0xeb, 0x8d,
                0x8d, 0x10, 0xbe, 0xb9,
            ],
            Elements::LeftPadHigh16_64 => [
                0x52, 0x6b, 0x35, 0x05, 0x45, 0x01, 0x36, 0xd6, 0x81, 0xa5, 0x0b, 0x4b, 0xde, 0x4f,
                0xa6, 0x12, 0xda, 0x9d, 0x69, 0xbd, 0x08, 0x17, 0x0e, 0xa3, 0x2d, 0x0a, 0x26, 0x51,
                0x11, 0x50, 0x72, 0xeb,
            ],
            Elements::LeftPadHigh1_16 => [
                0x93, 0xae, 0xd6, 0xf6, 0x87, 0x50, 0x77, 0x4b, 0x2d, 0xbf, 0x83, 0x14, 0xca, 0xde,
                0xbe, 0x5a, 0x41, 0x52, 0x43, 0xfb, 0xdf, 0x7c, 0x2e, 0xea, 0x8b, 0x22, 0x3d, 0xf3,
                0x26, 0x1e, 0x3b, 0xdb,
            ],
            Elements::LeftPadHigh1_32 => [
                0x00, 0x82, 0x98, 0xf8, 0x2f, 0xb6, 0xcf, 0x37, 0xe9, 0xdc, 0x70, 0x3e, 0xa4, 0xf9,
                0x49, 0x56, 0x5c, 0x29, 0x65, 0xa7, 0xc7, 0xf4, 0xfa, 0x22, 0xf5, 0x54, 0x56, 0x42,
                0x34, 0x08, 0xa3, 0xab,
            ],
            Elements::LeftPadHigh1_64 => [
                0x2b, 0x45, 0x4e, 0xbd, 0x79, 0x1e, 0xc7, 0xda, 0xce, 0xdc, 0xb8, 0x6c, 0x69, 0xd0,
                0x26, 0x79, 0x4a, 0x5d, 0xc3, 0x72, 0x52, 0x61, 0xe7, 0xdc, 0x16, 0x50, 0xcc, 0x88,
                0x81, 0x17, 0xfc, 0x4f,
            ],
            Elements::LeftPadHigh1_8 => [
                0x6c, 0x27, 0x7c, 0x4c, 0xd0, 0x53, 0xdd, 0x35, 0x02, 0xdb, 0xe0, 0xbb, 0xc1, 0x4e,
                0xb0, 0xb3, 0x6a, 0x20, 0x1a, 0xbe, 0xf3, 0xb1, 0x74, 0xb0, 0xeb, 0xfe, 0x05, 0x20,
                0x18, 0xb6, 0x7e, 0x67,
            ],
            Elements::LeftPadHigh32_64 => [
                0x5d, 0x41, 0x22, 0x1c, 0xf6, 0x15, 0x82, 0x97, 0xb0, 0x6c, 0x19, 0x57, 0x11, 0x2c,
                0x0d, 0x12, 0xf3, 0xeb, 0x91, 0x7a, 0x2f, 0x50, 0x9a, 0x53, 0x9d, 0x5c, 0x9b, 0x79,
                0x10, 0x21, 0x9b, 0x65,
            ],
            Elements::LeftPadHigh8_16 => [
                0x21, 0x78, 0xdc, 0x76, 0xc0, 0x4c, 0x79, 0xd9, 0x18, 0x15, 0xd3, 0x8c, 0x96, 0x7f,
                0x34, 0x21, 0x3f, 0xfc, 0xc6, 0xc5, 0xf2, 0x43, 0xc9, 0x56, 0x29, 0x73, 0xf0, 0x90,
                0xca, 0x5c, 0xae, 0xfa,
            ],
            Elements::LeftPadHigh8_32 => [
                0xa4, 0xe8, 0x6b, 0x53, 0xe5, 0xd0, 0x0f, 0xaf, 0x0b, 0x3e, 0x9d, 0x53, 0x20, 0x2a,
                0xf7, 0x73, 0x8d, 0xcb, 0x88, 0x87, 0xa1, 0x8d, 0xfe, 0xe5, 0xbe, 0x34, 0xc4, 0x97,
                0x69, 0x8c, 0xa6, 0xb7,
            ],
            Elements::LeftPadHigh8_64 => [
                0xc8, 0x43, 0xa7, 0x2c, 0x41, 0x17, 0x0f, 0x40, 0x34, 0x33, 0xc4, 0x36, 0xa3, 0x9b,
                0x05, 0xcf, 0x19, 0x3c, 0x27, 0xd8, 0xbe, 0x35, 0x30, 0xf9, 0xb9, 0x4e, 0x42, 0xd7,
                0x63, 0x00, 0x3d, 0x54,
            ],
            Elements::LeftPadLow16_32 => [
                0x21, 0x53, 0x7f, 0x7d, 0x8f, 0x97, 0xf2, 0x20, 0x3c, 0xcc, 0xb0, 0x35, 0xef, 0x1d,
                0x46, 0x28, 0x9e, 0xe8, 0xaa, 0x50, 0xf0, 0x23, 0x60, 0x77, 0xd0, 0xd0, 0xb2, 0x10,
                0x70, 0x04, 0x40, 0xa1,
            ],
            Elements::LeftPadLow16_64 => [
                0x6b, 0x2e, 0xa9, 0x63, 0x0c, 0x5d, 0xde, 0x03, 0x7a, 0xab, 0x2b, 0xf7, 0x33, 0x21,
                0x9b, 0x99, 0xc7, 0xed, 0xc2, 0xec, 0xed, 0xb9, 0xa0, 0x3a, 0xdf, 0xd1, 0x69, 0x43,
                0x0b, 0x08, 0xbb, 0x9c,
            ],
            Elements::LeftPadLow1_16 => [
                0x4a, 0xa4, 0x05, 0x20, 0xfa, 0xed, 0x72, 0xe6, 0xe9, 0xbe, 0x3b, 0xe6, 0x93, 0x0f,
                0x1e, 0x32, 0xb0, 0xb1, 0x82, 0xc4, 0x32, 0x7a, 0xda, 0x94, 0xa7, 0x1f, 0x00, 0x6d,
                0x14, 0x90, 0x15, 0xf9,
            ],
            Elements::LeftPadLow1_32 => [
                0xcf, 0xb4, 0x75, 0x3b, 0xb9, 0xba, 0x36, 0x21, 0xba, 0x09, 0x37, 0x82, 0x5f, 0xad,
                0xe6, 0x43, 0x09, 0x8e, 0x38, 0x5e, 0xd6, 0x8e, 0xfb, 0x16, 0xff, 0x58, 0xec, 0xf3,
                0x65, 0xd7, 0xe5, 0xe2,
            ],
            Elements::LeftPadLow1_64 => [
                0xe6, 0xf1, 0xc0, 0x9b, 0x5f, 0xe1, 0x26, 0xd0, 0xea, 0x86, 0xe7, 0xbf, 0xc0, 0xb2,
                0x8e, 0x84, 0x9f, 0x8f, 0x7e, 0xfd, 0x31, 0x06, 0x4e, 0xa4, 0xfd, 0x1c, 0xca, 0x07,
                0x1b, 0x45, 0xdb, 0x93,
            ],
            Elements::LeftPadLow1_8 => [
                0xdc, 0x5a, 0x47, 0xf8, 0xd7, 0x77, 0x65, 0xc9, 0x94, 0xcb, 0xe8, 0x6a, 0xae, 0x44,
                0xa9, 0xc5, 0xff, 0x2e, 0xbc, 0x38, 0x10, 0xd7, 0x9c, 0xd8, 0x3b, 0xd2, 0xc4, 0x09,
                0x8c, 0x76, 0x2b, 0xf5,
            ],
            Elements::LeftPadLow32_64 => [
                0x2d, 0x88, 0xe4, 0xd0, 0x1e, 0x01, 0x08, 0xc0, 0xd6, 0x88, 0x0f, 0x3c, 0xe8, 0x48,
                0x2b, 0xb0, 0x95, 0x1f, 0x2b, 0x3f, 0xc5, 0xdf, 0x4b, 0x1a, 0xdb, 0x18, 0x4a, 0x1b,
                0xfd, 0x1f, 0x64, 0x65,
            ],
            Elements::LeftPadLow8_16 => [
                0xac, 0x1a, 0x4c, 0x97, 0x83, 0xe4, 0xdb, 0xed, 0x27, 0x00, 0xeb, 0x29, 0x52, 0xe3,
                0x06, 0x2a, 0x5a, 0x72, 0x71, 0x2f, 0x82, 0x15, 0x98, 0x61, 0xb0, 0x8e, 0x67, 0xef,
                0x4a, 0x71, 0xf5, 0xf2,
            ],
            Elements::LeftPadLow8_32 => [
                0x3d, 0xa5, 0xf1, 0xa8, 0xc9, 0x78, 0x19, 0xae, 0x7e, 0x10, 0xb9, 0x36, 0x4f, 0xf8,
                0x49, 0x96, 0xd0, 0xd7, 0x3e, 0x69, 0x8a, 0x49, 0xda, 0x69, 0x1f, 0x69, 0xa2, 0x73,
                0x25, 0x42, 0x01, 0xcd,
            ],
            Elements::LeftPadLow8_64 => [
                0x25, 0xbc, 0x18, 0xd4, 0x9f, 0x93, 0x40, 0x72, 0x27, 0x7d, 0x3f, 0x61, 0x3b, 0xf1,
                0x6c, 0x11, 0x8d, 0xf1, 0x97, 0xbc, 0x92, 0x87, 0x2d, 0x2a, 0xff, 0xe4, 0x17, 0xad,
                0xea, 0xaf, 0x1a, 0x85,
            ],
            Elements::LeftRotate16 => [
                0x88, 0xc1, 0x23, 0x37, 0xcd, 0x75, 0x4f, 0x83, 0x80, 0x98, 0x6d, 0x86, 0xfe, 0x3a,
                0x89, 0xe2, 0x62, 0x74, 0x66, 0x53, 0xe1, 0xba, 0xdd, 0x9c, 0xc9, 0xb4, 0x76, 0x45,
                0xfe, 0x57, 0x19, 0x5a,
            ],
            Elements::LeftRotate32 => [
                0x39, 0x81, 0x6c, 0xcd, 0x9e, 0x9c, 0xf1, 0x19, 0x1f, 0x06, 0x5d, 0x2e, 0xb7, 0xa7,
                0xfb, 0x83, 0x82, 0x8d, 0x91, 0xec, 0x7d, 0x99, 0x77, 0xa1, 0xfc, 0x70, 0xbe, 0x9b,
                0x31, 0xa4, 0x68, 0xb9,
            ],
            Elements::LeftRotate64 => [
                0x8b, 0x23, 0x55, 0xc3, 0x1e, 0x3b, 0x61, 0x4b, 0xd4, 0xb4, 0x1c, 0x3e, 0xcf, 0x27,
                0x74, 0x24, 0xd0, 0x26, 0x76, 0x6b, 0x37, 0xbc, 0x6c, 0x10, 0x56, 0x21, 0xf4, 0xf6,
                0xa1, 0x6f, 0x9b, 0xdf,
            ],
            Elements::LeftRotate8 => [
                0x9e, 0x96, 0x6e, 0x88, 0x0c, 0x6b, 0x0c, 0x48, 0x3c, 0x90, 0xbe, 0xee, 0xd7, 0xc5,
                0x73, 0x7c, 0xa5, 0xf3, 0xfa, 0xcf, 0x85, 0xaa, 0xb3, 0xd5, 0x31, 0xad, 0x34, 0xbd,
                0x7b, 0x1a, 0x9b, 0x68,
            ],
            Elements::LeftShift16 => [
                0xb0, 0x53, 0x60, 0x18, 0x4d, 0x06, 0x02, 0xb5, 0x81, 0x40, 0x5e, 0x32, 0x96, 0x0b,
                0x31, 0xc0, 0x52, 0x19, 0x35, 0x8d, 0xe8, 0x9e, 0xfd, 0xf4, 0x94, 0x64, 0x72, 0x3d,
                0xd6, 0x25, 0x61, 0x7a,
            ],
            Elements::LeftShift32 => [
                0x34, 0xbf, 0x54, 0xf5, 0x94, 0xc2, 0x62, 0x10, 0x07, 0xf8, 0xc7, 0x8b, 0x30, 0xfa,
                0xd3, 0x96, 0x72, 0x00, 0x9b, 0xb3, 0x66, 0xaa, 0xce, 0x1e, 0x5e, 0x41, 0xee, 0x4d,
                0x9c, 0xc5, 0x41, 0xa8,
            ],
            Elements::LeftShift64 => [
                0x5d, 0xe9, 0x53, 0xf0, 0x4d, 0xea, 0xed, 0x90, 0x47, 0x56, 0x76, 0x47, 0xa1, 0xeb,
                0x7a, 0xbe, 0x66, 0x5f, 0xec, 0xcb, 0xe7, 0xed, 0x10, 0xcb, 0x7d, 0xbe, 0x69, 0x12,
                0x73, 0xc0, 0x94, 0xb0,
            ],
            Elements::LeftShift8 => [
                0xab, 0x9d, 0x3e, 0x9a, 0xc3, 0x90, 0x38, 0xad, 0x88, 0xb1, 0x03, 0xf0, 0x72, 0x25,
                0x4c, 0x0e, 0xc6, 0xe2, 0x74, 0x75, 0xe2, 0x75, 0xc2, 0x45, 0xe8, 0x8c, 0xce, 0x0d,
                0x07, 0x2e, 0x64, 0x46,
            ],
            Elements::LeftShiftWith16 => [
                0xe2, 0x91, 0x07, 0x88, 0x55, 0x50, 0x45, 0x0e, 0xb7, 0x27, 0xd0, 0xcf, 0x14, 0xe1,
                0x04, 0xae, 0x12, 0xf8, 0x3a, 0x24, 0xe2, 0xe2, 0xac, 0xa3, 0xcc, 0xe4, 0x33, 0xde,
                0x2f, 0x35, 0xd7, 0xb3,
            ],
            Elements::LeftShiftWith32 => [
                0xf3, 0x92, 0x50, 0xc4, 0x5a, 0x13, 0x10, 0xcc, 0x63, 0x8c, 0x78, 0x8d, 0xee, 0xc5,
                0xc3, 0x65, 0xb4, 0xd1, 0x76, 0xd1, 0x0e, 0xfb, 0xf4, 0xc6, 0x01, 0xcf, 0x5e, 0xeb,
                0xe0, 0xa5, 0x73, 0xe9,
            ],
            Elements::LeftShiftWith64 => [
                0xad, 0x87, 0x94, 0xcf, 0xae, 0xf2, 0xb7, 0xf7, 0x74, 0xfa, 0x68, 0xd3, 0x09, 0xbb,
                0xc9, 0x8d, 0xfe, 0xe5, 0x8c, 0x40, 0x40, 0x0b, 0x2e, 0xb5, 0x78, 0xa2, 0x12, 0xf4,
                0x38, 0xbd, 0x07, 0xab,
            ],
            Elements::LeftShiftWith8 => [
                0xdd, 0x9c, 0xc1, 0xce, 0xa7, 0x49, 0x09, 0x48, 0x1f, 0xf5, 0x8f, 0x87, 0x6f, 0xf6,
                0x6e, 0x0f, 0x5d, 0x52, 0xbf, 0x89, 0xb0, 0x25, 0x8f, 0xa9, 0x5b, 0x32, 0x00, 0x02,
                0xc3, 0x2a, 0x79, 0x15,
            ],
            Elements::Leftmost16_1 => [
                0xde, 0x6a, 0x4c, 0x98, 0x33, 0x7e, 0x68, 0x0d, 0x6e, 0x6e, 0xe2, 0xbf, 0x36, 0xd3,
                0xa0, 0x81, 0x7d, 0x2a, 0x9a, 0x98, 0x32, 0x5f, 0x87, 0xe5, 0xec, 0xeb, 0x8a, 0x6f,
                0x11, 0x68, 0xf5, 0xca,
            ],
            Elements::Leftmost16_2 => [
                0x00, 0x58, 0x09, 0xb8, 0x05, 0x1a, 0x2a, 0x50, 0x28, 0x33, 0xb2, 0x2c, 0x2c, 0x17,
                0x98, 0x1e, 0xaf, 0x9d, 0xd1, 0xd3, 0xdb, 0xc8, 0xf8, 0xc8, 0x94, 0x51, 0x6c, 0x1d,
                0x5f, 0x31, 0x14, 0x6c,
            ],
            Elements::Leftmost16_4 => [
                0x9c, 0x50, 0xee, 0x22, 0x84, 0xd8, 0x57, 0xc4, 0x7c, 0x05, 0x44, 0x47, 0x13, 0x54,
                0x10, 0x5e, 0x98, 0xdf, 0xe0, 0x27, 0x54, 0xd2, 0xe4, 0x2d, 0xe1, 0x1d, 0x32, 0x34,
                0xed, 0x10, 0xb6, 0x42,
            ],
            Elements::Leftmost16_8 => [
                0x5a, 0x1a, 0x72, 0x91, 0x4e, 0x14, 0x9c, 0x22, 0xb4, 0x64, 0xc8, 0xf6, 0xa3, 0xd9,
                0xcf, 0x41, 0xb0, 0x71, 0x92, 0xbe, 0xf0, 0xd8, 0xa1, 0xcc, 0x7c, 0xbe, 0x57, 0x04,
                0xa9, 0xe8, 0xea, 0x70,
            ],
            Elements::Leftmost32_1 => [
                0x5f, 0xb8, 0xe6, 0x34, 0x2a, 0xb7, 0x4e, 0xe2, 0xc9, 0x22, 0x5b, 0x87, 0x2f, 0xa0,
                0xc9, 0x12, 0x04, 0x6a, 0x69, 0xdb, 0xb7, 0x19, 0xbc, 0xd6, 0xc8, 0xd7, 0x9b, 0x76,
                0x60, 0xc4, 0xeb, 0xca,
            ],
            Elements::Leftmost32_16 => [
                0x54, 0xae, 0x50, 0xb4, 0x6b, 0x5b, 0x2e, 0x68, 0xf5, 0x36, 0xc0, 0x1c, 0x39, 0x61,
                0x7b, 0x0c, 0xee, 0x42, 0xe1, 0xc4, 0x9a, 0x2c, 0xd1, 0xd2, 0x6a, 0xf8, 0xea, 0x87,
                0x15, 0xac, 0x4d, 0x11,
            ],
            Elements::Leftmost32_2 => [
                0x12, 0x30, 0x68, 0x55, 0x45, 0x95, 0x42, 0x7e, 0x3c, 0x1d, 0xe2, 0x43, 0xba, 0xb6,
                0x6f, 0x33, 0x48, 0x36, 0x8a, 0xaa, 0x44, 0x61, 0x7d, 0x6a, 0x02, 0x47, 0x9f, 0xb7,
                0x04, 0xbc, 0xfd, 0x1e,
            ],
            Elements::Leftmost32_4 => [
                0x55, 0xa8, 0x7b, 0x66, 0xc3, 0x39, 0xe3, 0x63, 0xe0, 0x3d, 0x4d, 0xaa, 0xc6, 0x22,
                0x90, 0xeb, 0xa9, 0x3c, 0x1a, 0x3a, 0x73, 0x82, 0xcb, 0xf6, 0x1f, 0x20, 0xb3, 0x4a,
                0x50, 0x51, 0x24, 0xad,
            ],
            Elements::Leftmost32_8 => [
                0x9f, 0x34, 0x5b, 0xee, 0x0b, 0x16, 0x2d, 0x42, 0xa0, 0x35, 0x71, 0x8f, 0x8c, 0xa1,
                0xad, 0xc8, 0xac, 0x2f, 0x71, 0x0d, 0xc4, 0x00, 0x52, 0xa8, 0x25, 0x66, 0xe6, 0xd8,
                0x07, 0xbe, 0xf8, 0xb8,
            ],
            Elements::Leftmost64_1 => [
                0xb9, 0x24, 0xd3, 0x3b, 0x5e, 0xfe, 0xfc, 0x8e, 0x20, 0x42, 0x08, 0x19, 0x25, 0x91,
                0x7c, 0xff, 0x23, 0x9b, 0x31, 0xc8, 0xbd, 0xbd, 0xf4, 0xac, 0xae, 0x6b, 0xb8, 0xd9,
                0xcd, 0x21, 0x7b, 0x4f,
            ],
            Elements::Leftmost64_16 => [
                0x12, 0xaa, 0x85, 0xe0, 0x5c, 0x1e, 0x96, 0x22, 0x27, 0x9c, 0x4c, 0x2d, 0xdc, 0xf8,
                0x97, 0xc9, 0x5d, 0xdc, 0xc0, 0x11, 0x39, 0x97, 0x28, 0x3b, 0x6b, 0x3e, 0x09, 0x49,
                0xbc, 0x81, 0x13, 0xcb,
            ],
            Elements::Leftmost64_2 => [
                0xbc, 0xec, 0x97, 0xf4, 0x3b, 0xa5, 0x5c, 0xd4, 0x0d, 0x85, 0xa1, 0xe7, 0x6c, 0xba,
                0xde, 0x7b, 0x0b, 0x1e, 0x9f, 0x13, 0x97, 0x47, 0x79, 0x3d, 0xcb, 0x34, 0x80, 0xbe,
                0xe1, 0xf7, 0x51, 0xca,
            ],
            Elements::Leftmost64_32 => [
                0x9c, 0x89, 0x69, 0x39, 0x86, 0xe5, 0x57, 0x33, 0xab, 0x96, 0x2a, 0x30, 0x0b, 0x05,
                0x79, 0x50, 0x3d, 0x83, 0xde, 0x8a, 0xc1, 0x9b, 0x17, 0x9b, 0x41, 0x7e, 0x1c, 0xa2,
                0x53, 0x85, 0xb3, 0x8f,
            ],
            Elements::Leftmost64_4 => [
                0xd2, 0xd6, 0x45, 0x20, 0x92, 0xd6, 0x56, 0x6f, 0x89, 0xa1, 0xf6, 0x4e, 0x73, 0x65,
                0x96, 0xf9, 0x00, 0x0e, 0x5e, 0x6f, 0x63, 0xe4, 0x00, 0x17, 0xd0, 0xcb, 0x80, 0xf3,
                0xf7, 0xad, 0xfd, 0x18,
            ],
            Elements::Leftmost64_8 => [
                0xf7, 0x7b, 0x62, 0xbb, 0x01, 0xb9, 0x05, 0x11, 0xb6, 0xd0, 0x6e, 0xbf, 0x2e, 0x36,
                0xc0, 0x65, 0x65, 0xac, 0xb5, 0xaa, 0xd1, 0xef, 0xc7, 0x7c, 0x36, 0xa1, 0x0a, 0x26,
                0x1d, 0xe9, 0x21, 0xda,
            ],
            Elements::Leftmost8_1 => [
                0x5a, 0x73, 0x0b, 0x58, 0xe3, 0xab, 0xcb, 0x2f, 0x4d, 0xe2, 0x21, 0x59, 0x80, 0x30,
                0x23, 0x10, 0x2c, 0xd6, 0x64, 0x21, 0x91, 0x19, 0x20, 0xca, 0x21, 0xa2, 0xa0, 0x5c,
                0x9b, 0x21, 0x1c, 0xe8,
            ],
            Elements::Leftmost8_2 => [
                0x25, 0x79, 0x08, 0x56, 0x10, 0x3d, 0xce, 0x6c, 0x7b, 0xbb, 0x3d, 0xd7, 0x18, 0xb1,
                0x69, 0x10, 0x9c, 0xae, 0x85, 0x37, 0x99, 0xd1, 0x24, 0x56, 0xc8, 0x5d, 0x83, 0x49,
                0xec, 0x18, 0xdc, 0x53,
            ],
            Elements::Leftmost8_4 => [
                0x73, 0xd9, 0xf0, 0x18, 0x15, 0x7a, 0x14, 0x78, 0x4e, 0xe7, 0x0b, 0x21, 0x9c, 0xeb,
                0x40, 0x42, 0xfa, 0x62, 0x1d, 0x0e, 0xe6, 0xd5, 0x45, 0xa0, 0xfd, 0xba, 0xb9, 0x44,
                0x43, 0x46, 0xe3, 0x31,
            ],
            Elements::LinearCombination1 => [
                0x6d, 0x9f, 0x4a, 0x87, 0x0f, 0xbf, 0x74, 0x0c, 0x22, 0x0e, 0xff, 0xf3, 0x07, 0xb5,
                0xed, 0x91, 0xa5, 0x8c, 0x5e, 0x51, 0xa8, 0xad, 0xfc, 0x3b, 0x15, 0x90, 0x30, 0xf5,
                0x12, 0xd3, 0x99, 0x41,
            ],
            Elements::LinearVerify1 => [
                0x27, 0x83, 0x13, 0xd7, 0xce, 0x4a, 0xd5, 0x89, 0x11, 0xde, 0x24, 0xee, 0x54, 0x0d,
                0x19, 0xec, 0xeb, 0xb6, 0x2f, 0x4a, 0xb4, 0xa7, 0x1e, 0x2a, 0xad, 0xd4, 0x51, 0x2b,
                0x2e, 0x4b, 0xc2, 0xe2,
            ],
            Elements::LockTime => [
                0xa1, 0x68, 0x97, 0xc5, 0xdb, 0x49, 0x27, 0xc6, 0xcb, 0x07, 0xb1, 0x65, 0xbd, 0x2c,
                0xc3, 0x8b, 0x36, 0xa6, 0x02, 0x68, 0x6c, 0x74, 0xa1, 0x90, 0x34, 0xaa, 0x37, 0x87,
                0xac, 0x58, 0x2a, 0x0b,
            ],
            Elements::Low1 => [
                0xf2, 0x7b, 0x69, 0xbb, 0x09, 0x16, 0x09, 0xf5, 0x9e, 0x00, 0x33, 0x05, 0x0d, 0x01,
                0xa5, 0xbc, 0x77, 0xff, 0x07, 0xd9, 0x42, 0x70, 0x7a, 0x79, 0xcf, 0x5e, 0xe4, 0x10,
                0xa9, 0x98, 0xa0, 0x43,
            ],
            Elements::Low16 => [
                0x97, 0x7c, 0xbd, 0x1e, 0x7f, 0xfc, 0x05, 0xe7, 0x16, 0xd9, 0xc1, 0xb4, 0x9f, 0x7d,
                0x51, 0x7f, 0x85, 0x3d, 0xbf, 0x3e, 0x98, 0xa4, 0xc7, 0x48, 0x04, 0x6e, 0xac, 0xf4,
                0x17, 0xf8, 0x9c, 0x2f,
            ],
            Elements::Low32 => [
                0xbe, 0x41, 0x69, 0x82, 0x8f, 0x07, 0x67, 0x78, 0xb6, 0x0d, 0x54, 0x56, 0xf7, 0x88,
                0x6e, 0xd7, 0xf3, 0x0b, 0x10, 0x1d, 0x6c, 0xcb, 0xd9, 0xea, 0x0c, 0x4d, 0xb1, 0x42,
                0xea, 0xc6, 0x6b, 0x12,
            ],
            Elements::Low64 => [
                0x01, 0x9a, 0x66, 0xbf, 0xba, 0x17, 0x51, 0x68, 0x8b, 0xe7, 0x13, 0x89, 0xed, 0x7b,
                0xf3, 0x71, 0xb3, 0x01, 0x4d, 0xfb, 0x32, 0x95, 0x62, 0xac, 0x3b, 0x3e, 0x9d, 0xfe,
                0x92, 0x06, 0xa5, 0xbc,
            ],
            Elements::Low8 => [
                0x21, 0x7b, 0x56, 0x43, 0x95, 0x6b, 0x48, 0x33, 0xaa, 0x56, 0x22, 0xf0, 0x0f, 0x06,
                0x88, 0xba, 0x86, 0x0d, 0x4a, 0xdb, 0xf9, 0x40, 0xcb, 0xdc, 0xd2, 0xb5, 0x9f, 0x26,
                0xd6, 0x15, 0x93, 0xb1,
            ],
            Elements::Lt16 => [
                0x56, 0xa2, 0x0d, 0x55, 0xed, 0xb4, 0x43, 0x88, 0x18, 0x05, 0x44, 0xc3, 0xed, 0x40,
                0x41, 0x45, 0xa3, 0xb6, 0x6f, 0xd2, 0xc4, 0x11, 0x38, 0x42, 0xf6, 0x4e, 0xae, 0xaf,
                0xba, 0xd4, 0xbb, 0x06,
            ],
            Elements::Lt32 => [
                0xca, 0xb0, 0xdc, 0x5b, 0x0e, 0xcb, 0xf6, 0xd2, 0x48, 0x16, 0xfc, 0x20, 0x10, 0xfc,
                0x31, 0x19, 0x36, 0x63, 0xc3, 0x06, 0x96, 0x8d, 0x9c, 0xee, 0x3b, 0x00, 0x4c, 0x0b,
                0xc1, 0x84, 0xb4, 0x78,
            ],
            Elements::Lt64 => [
                0x47, 0xd6, 0x7e, 0x52, 0xb2, 0x7b, 0xa7, 0x8e, 0xdd, 0x07, 0x5a, 0xa2, 0x70, 0xde,
                0xd0, 0x07, 0xa7, 0xa9, 0xa6, 0x84, 0x99, 0x34, 0x4f, 0x28, 0x62, 0xf5, 0x06, 0x90,
                0x49, 0xa0, 0xce, 0xfe,
            ],
            Elements::Lt8 => [
                0x73, 0xd0, 0x04, 0x46, 0x55, 0xc0, 0xdf, 0x45, 0xc2, 0x71, 0xa1, 0x71, 0x3f, 0xf9,
                0xb9, 0xa4, 0x3d, 0xde, 0x56, 0xe6, 0x74, 0xd1, 0x75, 0x4e, 0x76, 0xed, 0xb1, 0x6f,
                0x94, 0x9c, 0x4f, 0xab,
            ],
            Elements::Maj1 => [
                0xcb, 0x2d, 0x98, 0x6d, 0x7f, 0x00, 0x10, 0x7a, 0x3c, 0x25, 0xf6, 0xb2, 0xf1, 0x48,
                0x91, 0xd0, 0x2e, 0x20, 0xae, 0x16, 0xf0, 0xa1, 0x25, 0x2c, 0x92, 0xd9, 0xb5, 0x8a,
                0xe7, 0x33, 0x88, 0xaa,
            ],
            Elements::Maj16 => [
                0x0a, 0xf6, 0xd0, 0xc1, 0x71, 0xfe, 0x33, 0xa2, 0x15, 0x9b, 0xf9, 0x88, 0x00, 0xf0,
                0x41, 0x2c, 0x25, 0x97, 0xe9, 0x97, 0x84, 0xd0, 0x74, 0xfd, 0xfa, 0x33, 0xd7, 0xfd,
                0xe5, 0x97, 0xdd, 0xfd,
            ],
            Elements::Maj32 => [
                0x30, 0x60, 0x83, 0x8d, 0x48, 0x45, 0x6f, 0x33, 0x92, 0xd5, 0xd6, 0x9b, 0x5e, 0xec,
                0x08, 0x92, 0x76, 0xcd, 0x58, 0xbb, 0x67, 0xa1, 0x2c, 0x64, 0x2e, 0xc7, 0x3a, 0xeb,
                0x9a, 0xda, 0xcb, 0xdd,
            ],
            Elements::Maj64 => [
                0x8e, 0xbc, 0xc1, 0x74, 0x57, 0xea, 0x2b, 0x14, 0x23, 0x1b, 0x0e, 0x90, 0x1e, 0xa7,
                0xb1, 0xd4, 0x7b, 0x9b, 0x78, 0x98, 0x63, 0x72, 0xa4, 0x41, 0x6f, 0xe7, 0x3f, 0x67,
                0x63, 0xfe, 0xb2, 0x4b,
            ],
            Elements::Maj8 => [
                0x89, 0x30, 0xd1, 0xd0, 0x99, 0x1b, 0x0a, 0x58, 0x1d, 0x0b, 0x1d, 0x85, 0xad, 0x72,
                0x14, 0x7d, 0x66, 0x49, 0xa3, 0x59, 0x93, 0x28, 0x3f, 0xc9, 0x72, 0x14, 0x43, 0x1f,
                0x0b, 0x6a, 0x7a, 0xa8,
            ],
            Elements::Max16 => [
                0xe0, 0x11, 0x47, 0x17, 0x69, 0x1a, 0xc1, 0xa7, 0x39, 0x28, 0x8f, 0xc6, 0xff, 0xa1,
                0xc6, 0x50, 0x7c, 0x43, 0xe6, 0xf1, 0xd4, 0xc1, 0x87, 0x70, 0xff, 0xa1, 0x66, 0xae,
                0x83, 0x9d, 0xd5, 0x33,
            ],
            Elements::Max32 => [
                0x1d, 0x72, 0x3c, 0xb3, 0x89, 0x94, 0x22, 0x19, 0xec, 0x10, 0x34, 0x85, 0x31, 0x7f,
                0xa5, 0xd8, 0x7e, 0xe1, 0x5c, 0x24, 0xb2, 0x08, 0x0f, 0x50, 0x46, 0x65, 0x0d, 0x80,
                0x30, 0x8b, 0x18, 0x9d,
            ],
            Elements::Max64 => [
                0x00, 0x73, 0xac, 0x3c, 0x6e, 0xa9, 0x39, 0xdc, 0xc7, 0xee, 0xe4, 0xea, 0x63, 0xdc,
                0xfd, 0x75, 0x20, 0x37, 0x35, 0x5b, 0x48, 0x4f, 0x6e, 0x70, 0x16, 0xb3, 0x00, 0xe2,
                0xd2, 0x8c, 0x07, 0xc3,
            ],
            Elements::Max8 => [
                0x6b, 0xc1, 0x03, 0x70, 0xf3, 0xe7, 0xa7, 0xb9, 0x2a, 0xcb, 0x14, 0x23, 0xbb, 0xdf,
                0x0b, 0x3d, 0x7e, 0x3c, 0xd0, 0xd2, 0xdb, 0xc7, 0x05, 0xa3, 0x4d, 0x8d, 0xc9, 0x9c,
                0x91, 0x04, 0x22, 0xfb,
            ],
            Elements::Median16 => [
                0x24, 0x14, 0xe3, 0xc4, 0x39, 0x65, 0x9d, 0x8a, 0xa9, 0xd0, 0x87, 0xe1, 0xad, 0xe7,
                0x72, 0x66, 0x67, 0x3d, 0x1c, 0x8b, 0xd4, 0xe7, 0x50, 0x1b, 0x22, 0xac, 0x46, 0xa3,
                0xff, 0x39, 0x97, 0x5d,
            ],
            Elements::Median32 => [
                0x07, 0x92, 0x35, 0x6b, 0x61, 0x0b, 0x57, 0xd0, 0xec, 0x19, 0x9e, 0x98, 0x53, 0x5e,
                0xa9, 0xbc, 0xcc, 0xe8, 0x43, 0xa5, 0xdf, 0x5d, 0xd2, 0x40, 0x8c, 0x41, 0x48, 0x86,
                0xdf, 0xd6, 0xbd, 0x1e,
            ],
            Elements::Median64 => [
                0x07, 0x66, 0xd8, 0x9b, 0x43, 0x0f, 0xfd, 0xf0, 0x38, 0x69, 0x1b, 0x18, 0x43, 0x9c,
                0xd6, 0xfc, 0x49, 0x29, 0x17, 0x2e, 0xa8, 0x84, 0xfd, 0xaf, 0x16, 0x69, 0x36, 0xb3,
                0x8b, 0x15, 0xfd, 0x0c,
            ],
            Elements::Median8 => [
                0xa4, 0xa0, 0xb6, 0x31, 0x0f, 0xf0, 0xed, 0x4a, 0x4c, 0x3e, 0x03, 0xeb, 0xc7, 0xa9,
                0x13, 0x06, 0xef, 0x66, 0x04, 0x24, 0xbc, 0x95, 0xa0, 0xd3, 0xf2, 0xfd, 0xb7, 0x1f,
                0xb6, 0xaf, 0xd8, 0xb7,
            ],
            Elements::Min16 => [
                0xf1, 0x58, 0xf4, 0x0a, 0x86, 0x09, 0x93, 0xb4, 0x10, 0x7f, 0xb2, 0x71, 0xfb, 0x4c,
                0x8f, 0x95, 0x5b, 0xa4, 0x54, 0x2a, 0xd1, 0x82, 0x1c, 0xd2, 0xf1, 0x3c, 0x88, 0x0c,
                0xa4, 0xbe, 0xe2, 0xe2,
            ],
            Elements::Min32 => [
                0xe5, 0xe4, 0x13, 0xdc, 0x5d, 0xe5, 0xe2, 0x2d, 0x66, 0xf3, 0x2d, 0x8d, 0xbf, 0x50,
                0x05, 0x3e, 0xd2, 0x78, 0xe1, 0x75, 0xc0, 0xd4, 0xb3, 0x44, 0xeb, 0xd4, 0x61, 0xbe,
                0xb1, 0x08, 0xe5, 0x5e,
            ],
            Elements::Min64 => [
                0x43, 0xd8, 0x2f, 0x6c, 0x61, 0x28, 0xaa, 0x01, 0xa9, 0x97, 0xbb, 0x17, 0xe5, 0xe7,
                0xf5, 0x01, 0xe7, 0xbe, 0x7d, 0xb9, 0x58, 0x9e, 0x56, 0x6d, 0xe9, 0x7a, 0x32, 0xea,
                0xe7, 0xe7, 0xb3, 0x39,
            ],
            Elements::Min8 => [
                0x6b, 0x01, 0x2c, 0xa3, 0x18, 0x5d, 0xc0, 0x05, 0xe8, 0x94, 0x2c, 0xfb, 0xc9, 0xf2,
                0x38, 0xdc, 0xed, 0xaf, 0x0c, 0x00, 0x43, 0x52, 0x64, 0x47, 0xe3, 0xec, 0x31, 0xce,
                0xfa, 0x6e, 0x40, 0x64,
            ],
            Elements::Modulo16 => [
                0x62, 0xc1, 0x79, 0xac, 0x84, 0xc5, 0x75, 0x0b, 0x42, 0x5f, 0x9a, 0x1b, 0x8f, 0x81,
                0xed, 0xaa, 0x7f, 0x5c, 0xf2, 0x2c, 0x19, 0xd8, 0x6b, 0x0d, 0xcf, 0x96, 0xde, 0xa6,
                0xba, 0xd9, 0x9b, 0x3b,
            ],
            Elements::Modulo32 => [
                0xa1, 0xf0, 0x1c, 0x10, 0x6f, 0xc3, 0x6a, 0x76, 0x4e, 0x99, 0xb2, 0x33, 0x98, 0xe2,
                0x1e, 0x7c, 0x26, 0x7f, 0x88, 0x9f, 0xcc, 0xeb, 0xd1, 0x48, 0x7d, 0x3d, 0xe1, 0xcc,
                0x67, 0xc3, 0x2b, 0xd9,
            ],
            Elements::Modulo64 => [
                0x50, 0xc8, 0x2f, 0xd0, 0x31, 0x09, 0xc9, 0x8b, 0x72, 0x37, 0xe9, 0x16, 0x74, 0x04,
                0x19, 0x64, 0x38, 0x1e, 0x6c, 0x2e, 0xbb, 0xe2, 0x5b, 0xf3, 0xe0, 0xd3, 0x7a, 0x9f,
                0x06, 0x0f, 0x15, 0x02,
            ],
            Elements::Modulo8 => [
                0x5c, 0x63, 0xc7, 0x7a, 0x16, 0x08, 0xe2, 0xf6, 0xa3, 0x74, 0x8c, 0x11, 0x0f, 0xbb,
                0x9a, 0x1c, 0x56, 0x9f, 0xb4, 0xd5, 0x40, 0xf3, 0xdd, 0x2e, 0x4f, 0x80, 0xe9, 0x0d,
                0xd5, 0xea, 0x99, 0x82,
            ],
            Elements::Multiply16 => [
                0x46, 0xe6, 0x2a, 0xbf, 0x8e, 0x30, 0xa7, 0x74, 0x6d, 0xe0, 0xe9, 0x29, 0xf7, 0xbe,
                0xed, 0xdb, 0xde, 0x8b, 0x26, 0x9b, 0xab, 0x08, 0xf7, 0x6e, 0x95, 0x47, 0x10, 0x8b,
                0x1c, 0x36, 0x01, 0x74,
            ],
            Elements::Multiply32 => [
                0x2d, 0xec, 0xdc, 0x5b, 0x0c, 0x6f, 0xf6, 0x3d, 0x11, 0xf5, 0x38, 0x52, 0xe0, 0xde,
                0xed, 0x11, 0x44, 0x81, 0x35, 0x5b, 0xb6, 0xc6, 0xce, 0x15, 0x46, 0xae, 0x9f, 0x81,
                0x5b, 0xee, 0x77, 0x50,
            ],
            Elements::Multiply64 => [
                0xbf, 0xa8, 0x62, 0x6d, 0xbf, 0x10, 0x00, 0x1d, 0xe3, 0x90, 0xd9, 0x97, 0xf2, 0xee,
                0x7b, 0x19, 0x0c, 0x24, 0xa7, 0x8c, 0xfe, 0xcb, 0x91, 0xf5, 0xd7, 0xc1, 0x0c, 0x3f,
                0x9d, 0xdb, 0xb1, 0xe6,
            ],
            Elements::Multiply8 => [
                0x29, 0xda, 0x13, 0x37, 0x4f, 0x7c, 0xb3, 0x08, 0x40, 0x5f, 0xe2, 0x30, 0xf8, 0x99,
                0x48, 0x5c, 0x50, 0x0e, 0x6e, 0x95, 0x20, 0xc1, 0x5e, 0x8a, 0x76, 0xe5, 0x3a, 0x92,
                0xe7, 0xac, 0x64, 0xd6,
            ],
            Elements::Negate16 => [
                0xf6, 0x42, 0x17, 0x3b, 0x85, 0xef, 0x21, 0x96, 0x9d, 0x8d, 0x90, 0x48, 0x80, 0x7e,
                0x3d, 0x4f, 0xac, 0xf3, 0xf5, 0xf9, 0xe5, 0x9a, 0xa5, 0xcf, 0x0c, 0x60, 0xf8, 0x74,
                0x22, 0xed, 0x7c, 0x8f,
            ],
            Elements::Negate32 => [
                0x54, 0x9b, 0x65, 0xce, 0x97, 0xc6, 0xb3, 0x34, 0xb8, 0xae, 0x94, 0x56, 0x96, 0x0e,
                0x36, 0x5b, 0xb2, 0x84, 0xd7, 0x6d, 0x40, 0x05, 0xe9, 0x21, 0xf4, 0x89, 0xbc, 0x36,
                0x26, 0x17, 0x1b, 0x06,
            ],
            Elements::Negate64 => [
                0x35, 0xac, 0xca, 0x27, 0xce, 0x65, 0x85, 0x79, 0xef, 0x1c, 0x55, 0xad, 0x1a, 0xbe,
                0xa0, 0x05, 0x0d, 0x93, 0x66, 0xd1, 0x22, 0x09, 0xad, 0x13, 0x05, 0x25, 0x49, 0xc3,
                0x43, 0x64, 0x91, 0xd0,
            ],
            Elements::Negate8 => [
                0xd8, 0x71, 0xc5, 0x42, 0x47, 0x3f, 0x4d, 0xd9, 0x02, 0xd3, 0x1f, 0xe3, 0xfc, 0x9a,
                0xc0, 0xf3, 0x31, 0x9e, 0x42, 0xe8, 0x0c, 0xae, 0x21, 0x81, 0xff, 0xc8, 0x5e, 0x6c,
                0x60, 0xfb, 0x09, 0x88,
            ],
            Elements::NewIssuanceContract => [
                0xdb, 0x64, 0x80, 0x8c, 0x3f, 0xf4, 0x48, 0x80, 0xd7, 0x2b, 0xc2, 0x95, 0xd9, 0xac,
                0x06, 0x43, 0xe5, 0x14, 0x04, 0xde, 0x53, 0x98, 0xad, 0x9e, 0x93, 0x1b, 0xd3, 0xd2,
                0xad, 0xbc, 0xa6, 0xc4,
            ],
            Elements::NonceHash => [
                0xfb, 0x1e, 0x03, 0x44, 0xdc, 0xda, 0xf9, 0xdb, 0x91, 0xb9, 0x87, 0xe8, 0xf9, 0x66,
                0x10, 0x67, 0x9f, 0x05, 0xde, 0xb2, 0x06, 0xb4, 0x01, 0x05, 0x75, 0x02, 0xa4, 0x84,
                0xaa, 0x87, 0xf8, 0xbf,
            ],
            Elements::NumInputs => [
                0xb2, 0x28, 0x8e, 0xba, 0xad, 0xcb, 0xcf, 0xce, 0x1c, 0x63, 0x19, 0x64, 0xc8, 0x6b,
                0x12, 0x7d, 0x6f, 0x91, 0xdc, 0x65, 0x7c, 0x59, 0xa7, 0xfb, 0x45, 0x3e, 0x91, 0x11,
                0xd8, 0x74, 0x81, 0xf5,
            ],
            Elements::NumOutputs => [
                0x26, 0x7d, 0x41, 0x65, 0x44, 0x93, 0x90, 0x3e, 0x02, 0x57, 0x75, 0x1a, 0xe6, 0xd4,
                0x2d, 0x9c, 0xe1, 0xbb, 0x92, 0x24, 0x6d, 0x0e, 0xf7, 0x67, 0xc3, 0x37, 0xb0, 0xc3,
                0x2b, 0x19, 0x72, 0xfd,
            ],
            Elements::One16 => [
                0x3f, 0x9f, 0x8d, 0xd1, 0x4c, 0x46, 0xee, 0x02, 0x47, 0x15, 0x57, 0x92, 0x9a, 0xc2,
                0xbb, 0x6c, 0x1a, 0xca, 0x00, 0x52, 0x1d, 0x8a, 0xfa, 0xf0, 0xdc, 0xd9, 0xf2, 0xca,
                0x7f, 0x31, 0xe6, 0x04,
            ],
            Elements::One32 => [
                0x47, 0x8d, 0xc3, 0x9d, 0xc3, 0x99, 0x5e, 0x2e, 0xdb, 0x7e, 0xc6, 0x74, 0x65, 0x6c,
                0xae, 0x79, 0x8f, 0x52, 0xe5, 0x72, 0x92, 0x61, 0x74, 0xa6, 0x68, 0xcc, 0x97, 0xbc,
                0xa4, 0x48, 0xd1, 0xcc,
            ],
            Elements::One64 => [
                0xa3, 0x92, 0xce, 0xfc, 0x0d, 0xa5, 0x3c, 0x65, 0xae, 0xe6, 0x12, 0xf5, 0xc6, 0x81,
                0x6c, 0xa8, 0x92, 0xfc, 0x15, 0x6d, 0x43, 0x71, 0x48, 0x76, 0xb3, 0xa0, 0x05, 0x68,
                0xe1, 0xba, 0x3e, 0xba,
            ],
            Elements::One8 => [
                0xff, 0x59, 0x4e, 0x22, 0xbf, 0xd7, 0x58, 0x13, 0xc0, 0x56, 0xe0, 0xa2, 0x34, 0xed,
                0x12, 0xfa, 0x82, 0x87, 0xd1, 0xd5, 0x31, 0x6f, 0x23, 0x90, 0x2b, 0xf0, 0x79, 0xdb,
                0xcc, 0x4f, 0x4e, 0xa8,
            ],
            Elements::Or1 => [
                0x9b, 0xf5, 0x91, 0x74, 0x41, 0x0a, 0x80, 0x9d, 0x3d, 0xa2, 0xb5, 0x8c, 0x7e, 0x0d,
                0x05, 0xc5, 0x5c, 0xec, 0x38, 0xbd, 0xaa, 0x5f, 0xca, 0xc3, 0x82, 0xa3, 0x11, 0x77,
                0x0e, 0xe0, 0xeb, 0x38,
            ],
            Elements::Or16 => [
                0xdd, 0x9a, 0x31, 0x93, 0xd6, 0x19, 0xd9, 0x59, 0xfa, 0x0b, 0x6d, 0x8b, 0x47, 0xaf,
                0x78, 0x54, 0xf7, 0xe0, 0x46, 0x7b, 0xa3, 0x59, 0x01, 0xce, 0x43, 0xd8, 0x00, 0xfc,
                0xaf, 0x73, 0x0f, 0xf9,
            ],
            Elements::Or32 => [
                0x9a, 0x01, 0x9f, 0x07, 0xdf, 0x49, 0x96, 0xb3, 0x3e, 0x64, 0x7f, 0x4d, 0xe7, 0xe5,
                0x6c, 0x1d, 0x8f, 0x03, 0x26, 0x9c, 0xbf, 0xa3, 0xc7, 0x58, 0x2c, 0xfe, 0x80, 0x8e,
                0x90, 0x98, 0x70, 0xb7,
            ],
            Elements::Or64 => [
                0xc2, 0x4f, 0x35, 0x80, 0x05, 0xf8, 0x03, 0x77, 0x2b, 0x1c, 0x3e, 0x43, 0x9c, 0xf1,
                0xb7, 0x09, 0xbd, 0x9f, 0x4d, 0x42, 0x52, 0x75, 0x91, 0x30, 0x3a, 0x36, 0xf6, 0xb1,
                0xc3, 0xcf, 0x29, 0xcc,
            ],
            Elements::Or8 => [
                0x84, 0xb5, 0x36, 0x89, 0xf2, 0x1d, 0x4e, 0x69, 0x7d, 0x0f, 0xe8, 0x98, 0x8c, 0xe7,
                0x36, 0xab, 0x72, 0xc9, 0xc8, 0x6f, 0x84, 0x75, 0x89, 0xda, 0xa9, 0xae, 0x6a, 0x78,
                0x46, 0x30, 0xe6, 0x20,
            ],
            Elements::OutpointHash => [
                0x8d, 0x2a, 0x5a, 0xa0, 0x6e, 0x98, 0xc4, 0x1b, 0x30, 0xfd, 0x07, 0x03, 0xe5, 0xe5,
                0x4c, 0x86, 0xb4, 0x2b, 0xee, 0x33, 0xbc, 0x89, 0xcb, 0x60, 0x93, 0xcd, 0xb0, 0x94,
                0x05, 0xa7, 0x2d, 0x9d,
            ],
            Elements::OutputAmount => [
                0x55, 0xe4, 0x13, 0x99, 0x21, 0x69, 0x40, 0x8e, 0x33, 0x8e, 0x11, 0xa8, 0xd7, 0xbe,
                0x60, 0x2b, 0x3e, 0xc9, 0x51, 0x1e, 0x7e, 0xb6, 0x91, 0xdf, 0x32, 0xd9, 0x79, 0x7a,
                0x63, 0x95, 0xa1, 0xa1,
            ],
            Elements::OutputAmountsHash => [
                0x40, 0x23, 0x69, 0x63, 0xe1, 0x0e, 0xf8, 0xad, 0x9c, 0xb6, 0xe0, 0xa7, 0xc7, 0x9a,
                0xc8, 0x66, 0x0d, 0x2f, 0xc1, 0xa5, 0x7f, 0xef, 0xf8, 0x85, 0x15, 0xa7, 0x85, 0x27,
                0x91, 0xb1, 0x86, 0xcb,
            ],
            Elements::OutputAsset => [
                0x57, 0x5a, 0x49, 0xba, 0xb2, 0x1f, 0xfe, 0xe8, 0xe5, 0xbd, 0x00, 0xed, 0xce, 0x32,
                0x40, 0x61, 0x04, 0xa0, 0x5d, 0xde, 0xf5, 0xfc, 0x31, 0xa1, 0xa6, 0x45, 0xb1, 0x29,
                0x54, 0x8c, 0x8f, 0x3e,
            ],
            Elements::OutputHash => [
                0x7c, 0xb1, 0x7f, 0x8f, 0xc7, 0xa1, 0xae, 0x4e, 0xfc, 0xe3, 0x0a, 0x14, 0x54, 0xe5,
                0x2f, 0x85, 0x85, 0x21, 0x3c, 0xd0, 0xf3, 0x67, 0xa1, 0x27, 0xac, 0x27, 0xbb, 0x97,
                0x66, 0xea, 0x9e, 0xee,
            ],
            Elements::OutputIsFee => [
                0x9f, 0xbb, 0xe8, 0xae, 0xbd, 0xed, 0xd5, 0x77, 0xae, 0x46, 0x62, 0x80, 0xa9, 0x9b,
                0x67, 0xb7, 0x34, 0x32, 0x87, 0xef, 0x34, 0x06, 0x92, 0x53, 0x4a, 0x00, 0x98, 0x03,
                0x15, 0xe5, 0x17, 0x72,
            ],
            Elements::OutputNonce => [
                0x71, 0x75, 0xfd, 0x99, 0xa2, 0xba, 0xa1, 0xc3, 0xaf, 0xaa, 0xf7, 0xfa, 0xbd, 0xaa,
                0x67, 0xc2, 0xc8, 0x27, 0x48, 0x3b, 0xc8, 0xa0, 0xb9, 0x76, 0x50, 0xcb, 0x79, 0x42,
                0xea, 0x29, 0x5a, 0x18,
            ],
            Elements::OutputNoncesHash => [
                0x50, 0xf1, 0x87, 0x66, 0xde, 0x98, 0x28, 0xdb, 0x0a, 0x8c, 0x41, 0xd7, 0xbe, 0xb8,
                0x40, 0xf7, 0x7b, 0xc6, 0xe1, 0x21, 0xff, 0x12, 0x3d, 0x54, 0x96, 0xf1, 0xea, 0x0d,
                0x6a, 0x78, 0xb8, 0x3a,
            ],
            Elements::OutputNullDatum => [
                0x38, 0xa5, 0xa7, 0xe8, 0x61, 0xb3, 0xb3, 0x6c, 0xce, 0x68, 0x3a, 0x77, 0x2d, 0xb0,
                0xc8, 0x62, 0x8b, 0x7f, 0xd3, 0xb4, 0xcf, 0x8e, 0x7b, 0xce, 0x70, 0x75, 0x8f, 0xa6,
                0xf6, 0xf1, 0x47, 0xbf,
            ],
            Elements::OutputRangeProof => [
                0x6f, 0xf9, 0xef, 0x3d, 0xac, 0x50, 0x43, 0x10, 0xdf, 0xbc, 0x60, 0x56, 0x1d, 0x2a,
                0xdd, 0xeb, 0x44, 0x93, 0x2f, 0x77, 0x7e, 0x7f, 0x13, 0xad, 0xf0, 0x77, 0x4e, 0xb6,
                0xb1, 0x4d, 0xf9, 0x66,
            ],
            Elements::OutputRangeProofsHash => [
                0x0d, 0x7d, 0x8a, 0xc9, 0x7a, 0xf9, 0x53, 0x9a, 0xbb, 0xa6, 0x4d, 0x81, 0x04, 0xd1,
                0x7e, 0x97, 0x08, 0x54, 0x67, 0x7c, 0x27, 0x46, 0x1d, 0x25, 0xdf, 0x21, 0xa7, 0xf6,
                0xb6, 0x62, 0x08, 0x1e,
            ],
            Elements::OutputScriptHash => [
                0x2f, 0x51, 0x39, 0x26, 0x6f, 0x14, 0x3a, 0x33, 0xe1, 0x66, 0x8e, 0x59, 0x4f, 0x43,
                0xe8, 0xd1, 0x39, 0x44, 0x07, 0xe3, 0xa9, 0xb8, 0xb8, 0x02, 0xb3, 0x9a, 0x5e, 0x1e,
                0xdc, 0x54, 0xe8, 0x51,
            ],
            Elements::OutputScriptsHash => [
                0x21, 0x33, 0x3a, 0xc9, 0x58, 0xa4, 0xeb, 0xfd, 0x5e, 0xab, 0x79, 0x56, 0xdb, 0x73,
                0xed, 0x03, 0x26, 0x94, 0x26, 0xaf, 0x8d, 0x7d, 0x24, 0x98, 0xde, 0x57, 0xee, 0xf6,
                0x87, 0x04, 0xfb, 0x0a,
            ],
            Elements::OutputSurjectionProof => [
                0x09, 0x16, 0x60, 0x08, 0x1d, 0x52, 0xc3, 0x3f, 0x17, 0xa9, 0x56, 0xbd, 0xda, 0xc6,
                0x2a, 0xa2, 0xd2, 0xed, 0x92, 0x8f, 0x8c, 0xff, 0xef, 0x4c, 0xf8, 0xdb, 0xb6, 0x97,
                0xb1, 0x53, 0xaa, 0x65,
            ],
            Elements::OutputSurjectionProofsHash => [
                0x36, 0x30, 0xb5, 0x20, 0x83, 0x02, 0x64, 0x1b, 0xf1, 0x9c, 0x4b, 0xf5, 0xca, 0x04,
                0x7a, 0xc3, 0xeb, 0x7a, 0xf5, 0x4d, 0x5d, 0x3f, 0x5b, 0xf8, 0xf1, 0xd4, 0x13, 0x12,
                0x49, 0x9d, 0x63, 0x84,
            ],
            Elements::OutputsHash => [
                0x95, 0xb3, 0x51, 0x1a, 0xea, 0x00, 0xd0, 0x43, 0xa4, 0xe0, 0x28, 0xb2, 0x5a, 0x17,
                0x1c, 0x74, 0x2e, 0xc2, 0xff, 0x7a, 0xd3, 0xa7, 0xdf, 0x2f, 0x1f, 0x6e, 0xda, 0xd0,
                0x8b, 0xad, 0x2b, 0x69,
            ],
            Elements::ParseLock => [
                0x3d, 0x38, 0x36, 0xfd, 0x30, 0x85, 0xc1, 0xfb, 0xac, 0x6c, 0xd5, 0xfa, 0x0d, 0xbf,
                0x4a, 0x3f, 0xb2, 0x55, 0x45, 0x93, 0x17, 0xa2, 0x66, 0xd6, 0xd6, 0xf7, 0x38, 0x2b,
                0xb0, 0x5f, 0x07, 0xad,
            ],
            Elements::ParseSequence => [
                0x74, 0xf3, 0x5c, 0x01, 0x9e, 0xf5, 0x14, 0xb7, 0x0a, 0xb0, 0x08, 0xbf, 0x2a, 0x12,
                0x6d, 0xe7, 0xe0, 0x0f, 0x6e, 0x3c, 0xcd, 0x28, 0x5d, 0x51, 0xdb, 0xd3, 0xac, 0x71,
                0xbe, 0xa9, 0xc8, 0x8d,
            ],
            Elements::PointVerify1 => [
                0x90, 0xa3, 0xd6, 0x69, 0xb0, 0x0d, 0xa7, 0x95, 0xef, 0xb2, 0xbe, 0xd8, 0xc3, 0x70,
                0xc9, 0xe3, 0xea, 0x0f, 0x19, 0xc4, 0x1c, 0x7c, 0xf2, 0x3e, 0x49, 0x2e, 0x33, 0x17,
                0x1a, 0x47, 0xf5, 0xff,
            ],
            Elements::ReissuanceBlinding => [
                0xeb, 0x6e, 0x58, 0x1e, 0x20, 0xba, 0x0b, 0xab, 0xe9, 0x04, 0xc8, 0xda, 0x71, 0xa8,
                0x1f, 0x7c, 0x88, 0xb9, 0xcf, 0x4f, 0x42, 0xd7, 0x8b, 0x54, 0x0c, 0x1b, 0x39, 0x8c,
                0x3f, 0x10, 0x5c, 0xcc,
            ],
            Elements::ReissuanceEntropy => [
                0xd1, 0xd2, 0x2c, 0x49, 0x7f, 0x12, 0x9c, 0x6d, 0x6f, 0xdb, 0xe4, 0x91, 0x36, 0x4a,
                0x35, 0x13, 0xbc, 0xd6, 0x6e, 0x15, 0x61, 0x8c, 0xbb, 0xe2, 0xe5, 0x07, 0x39, 0xbd,
                0x83, 0x4d, 0xb7, 0xf8,
            ],
            Elements::RightExtend16_32 => [
                0x78, 0x07, 0x16, 0xd3, 0xe8, 0x29, 0x1a, 0x51, 0xe4, 0x5a, 0xda, 0x50, 0x55, 0x8e,
                0xfe, 0x41, 0x1c, 0x47, 0x5c, 0x08, 0x5e, 0xec, 0x5a, 0x28, 0xad, 0x97, 0x91, 0xc3,
                0x12, 0xfe, 0xe2, 0xbc,
            ],
            Elements::RightExtend16_64 => [
                0xc7, 0x70, 0x49, 0x7e, 0x45, 0x23, 0x08, 0xeb, 0xf5, 0x2e, 0x51, 0xb0, 0x58, 0x5e,
                0x91, 0x51, 0xe0, 0xff, 0xc3, 0x50, 0x86, 0xab, 0x77, 0x2d, 0x72, 0x41, 0x53, 0x2a,
                0x1b, 0xe1, 0x5e, 0x07,
            ],
            Elements::RightExtend32_64 => [
                0x42, 0xb4, 0x3a, 0xdc, 0x74, 0xb5, 0x26, 0x6c, 0x91, 0xd7, 0x3d, 0xf4, 0x91, 0xdc,
                0xae, 0x59, 0x73, 0x88, 0x04, 0xeb, 0x44, 0x0b, 0x23, 0xda, 0x32, 0x75, 0x30, 0x48,
                0x74, 0x86, 0xb7, 0xe8,
            ],
            Elements::RightExtend8_16 => [
                0xbc, 0xb2, 0x68, 0x3a, 0x8c, 0xb8, 0xb8, 0xc2, 0x35, 0xfa, 0xa8, 0x96, 0xa9, 0xc0,
                0x69, 0xe1, 0xb5, 0x5b, 0xb0, 0x55, 0x8e, 0x73, 0x9e, 0x70, 0xe2, 0x89, 0x14, 0x21,
                0x1e, 0x32, 0x75, 0xc8,
            ],
            Elements::RightExtend8_32 => [
                0x6d, 0xdb, 0x55, 0x48, 0xfd, 0x58, 0x3c, 0xd2, 0xd3, 0x58, 0x6e, 0x6b, 0x8b, 0xf9,
                0x95, 0x24, 0x6b, 0x61, 0x93, 0x4f, 0x49, 0x76, 0x44, 0x67, 0x77, 0xdd, 0x57, 0x40,
                0xb3, 0x19, 0xe4, 0x62,
            ],
            Elements::RightExtend8_64 => [
                0xda, 0x4f, 0x9c, 0x21, 0x45, 0x51, 0x26, 0x82, 0x07, 0x58, 0xa2, 0xe4, 0xb5, 0x3f,
                0xce, 0xb4, 0x52, 0x3e, 0x6e, 0x7a, 0x29, 0x23, 0xa1, 0xa1, 0x61, 0xfc, 0x37, 0x89,
                0x2a, 0xc8, 0xda, 0x2a,
            ],
            Elements::RightPadHigh16_32 => [
                0x3e, 0x4e, 0x5e, 0x9e, 0x71, 0xe1, 0x37, 0xa2, 0x68, 0x63, 0x43, 0xe0, 0x5a, 0xc5,
                0x63, 0x16, 0xac, 0xfc, 0x58, 0x99, 0x1c, 0xb3, 0x8d, 0xb1, 0xb3, 0x23, 0x44, 0x13,
                0xf7, 0x30, 0xa1, 0x42,
            ],
            Elements::RightPadHigh16_64 => [
                0xde, 0x09, 0xdf, 0x9d, 0x43, 0xdd, 0xad, 0x2d, 0x69, 0x12, 0x04, 0x98, 0x6c, 0xf0,
                0x81, 0x9d, 0x6b, 0x80, 0x45, 0xbc, 0xa4, 0x14, 0xd8, 0x0a, 0xf2, 0x16, 0x28, 0x92,
                0xa9, 0x25, 0x7e, 0xad,
            ],
            Elements::RightPadHigh1_16 => [
                0xff, 0x12, 0x97, 0xd8, 0x78, 0xe2, 0x6e, 0x19, 0x59, 0xbc, 0xc7, 0xe8, 0xae, 0xf9,
                0x7a, 0xc0, 0xb6, 0x5a, 0xdc, 0x39, 0x92, 0x3e, 0xc6, 0x50, 0x5e, 0x50, 0xf9, 0x83,
                0x05, 0x73, 0x3b, 0x6c,
            ],
            Elements::RightPadHigh1_32 => [
                0x28, 0x3f, 0x8a, 0xfb, 0x41, 0x38, 0x2d, 0x2b, 0xe1, 0x8f, 0x8a, 0x77, 0xc3, 0x14,
                0xba, 0x17, 0x76, 0xcb, 0x80, 0xc8, 0xec, 0x36, 0xca, 0x12, 0xaa, 0x67, 0xb3, 0x2b,
                0xb6, 0x4e, 0xd8, 0x43,
            ],
            Elements::RightPadHigh1_64 => [
                0xa3, 0x42, 0x35, 0x28, 0x60, 0xa3, 0x35, 0x0d, 0x79, 0xc3, 0xe9, 0xfc, 0x7a, 0x4a,
                0xb3, 0x78, 0x9b, 0x8b, 0x02, 0x97, 0x85, 0x6f, 0xd1, 0x69, 0xca, 0x4d, 0x7d, 0xe2,
                0x5f, 0x7d, 0x7c, 0xc4,
            ],
            Elements::RightPadHigh1_8 => [
                0x71, 0x03, 0xc0, 0xfe, 0x00, 0xf5, 0x22, 0xa2, 0x21, 0x6c, 0x4a, 0x6b, 0xe5, 0xf7,
                0xe0, 0xeb, 0x4d, 0x70, 0x3c, 0xa7, 0x8f, 0x9c, 0x59, 0x8f, 0x6b, 0x3d, 0xfd, 0xe4,
                0x37, 0xd8, 0x0c, 0x84,
            ],
            Elements::RightPadHigh32_64 => [
                0x5d, 0xc9, 0x10, 0x7d, 0x45, 0x34, 0x95, 0x8c, 0xe4, 0x42, 0x27, 0x67, 0x56, 0x3a,
                0x03, 0x1a, 0x38, 0x0f, 0x60, 0xd3, 0x83, 0x71, 0x48, 0xab, 0x3c, 0x8c, 0xc9, 0xc4,
                0xc7, 0xd9, 0x96, 0xa2,
            ],
            Elements::RightPadHigh8_16 => [
                0xc0, 0xe2, 0xfd, 0x46, 0xf7, 0x88, 0x3b, 0x12, 0x85, 0xa6, 0xf1, 0xa1, 0xdb, 0x96,
                0xd9, 0x3c, 0x25, 0x48, 0x04, 0x0f, 0xcd, 0x3f, 0x5c, 0x23, 0xfb, 0xb2, 0x0b, 0x5e,
                0x83, 0x03, 0x7c, 0x96,
            ],
            Elements::RightPadHigh8_32 => [
                0x29, 0x1e, 0x62, 0x77, 0x08, 0x52, 0x0c, 0x2c, 0xa6, 0xae, 0xce, 0x32, 0xa8, 0x77,
                0xb7, 0x78, 0x49, 0xc4, 0xa7, 0xa2, 0x13, 0xcb, 0x89, 0xe1, 0xbd, 0xa7, 0xc5, 0xc5,
                0xfe, 0x75, 0x5f, 0x73,
            ],
            Elements::RightPadHigh8_64 => [
                0x6b, 0x6f, 0xa2, 0x37, 0x2e, 0xd2, 0x5e, 0x4a, 0x34, 0xd4, 0xae, 0x17, 0x23, 0x42,
                0xad, 0xbb, 0x25, 0x9b, 0xe8, 0x98, 0x76, 0x00, 0xdb, 0x19, 0x2e, 0xcb, 0x8d, 0xa4,
                0x34, 0xb9, 0xd8, 0x8f,
            ],
            Elements::RightPadLow16_32 => [
                0x77, 0x31, 0xd5, 0x60, 0xd3, 0x75, 0x92, 0xd1, 0xa3, 0x1f, 0x73, 0x62, 0x96, 0x7a,
                0xb2, 0xe4, 0x75, 0x92, 0xac, 0xa6, 0xe9, 0x2a, 0xb8, 0x58, 0x82, 0x37, 0x92, 0xda,
                0xe5, 0xd2, 0xdb, 0x52,
            ],
            Elements::RightPadLow16_64 => [
                0x0f, 0xe1, 0xc0, 0xdb, 0x9d, 0x4a, 0x2d, 0x63, 0xe2, 0xba, 0x4a, 0x33, 0x11, 0x7a,
                0xad, 0xba, 0x64, 0x51, 0x4a, 0x2b, 0x87, 0xa7, 0xa4, 0xe7, 0x93, 0xfa, 0xac, 0xfe,
                0x6b, 0x36, 0x34, 0x47,
            ],
            Elements::RightPadLow1_16 => [
                0x79, 0x14, 0xc8, 0xf2, 0x22, 0x47, 0xc2, 0xc3, 0x4b, 0x9c, 0x84, 0xe9, 0x2d, 0x14,
                0x44, 0xae, 0xc2, 0xe1, 0x7a, 0x0e, 0xf5, 0x86, 0xba, 0xb2, 0x78, 0x8e, 0xe6, 0xef,
                0x68, 0x84, 0x0d, 0x98,
            ],
            Elements::RightPadLow1_32 => [
                0x31, 0xb6, 0xce, 0x26, 0xe5, 0x59, 0xf7, 0x6c, 0xf3, 0x66, 0xf4, 0x80, 0x69, 0x85,
                0xec, 0xc2, 0x99, 0x55, 0x0f, 0x15, 0xd4, 0xc3, 0xa6, 0x72, 0x9e, 0x29, 0xd7, 0x0e,
                0x39, 0x89, 0x56, 0x52,
            ],
            Elements::RightPadLow1_64 => [
                0xc5, 0x52, 0x4a, 0xe6, 0x54, 0x8a, 0xcd, 0x63, 0x08, 0x2d, 0x94, 0x89, 0x3e, 0x18,
                0xf9, 0xed, 0xbb, 0x92, 0x31, 0xe7, 0x6b, 0xb4, 0xe1, 0x1b, 0xbf, 0xf6, 0xa7, 0xbd,
                0x16, 0xf4, 0xb0, 0x29,
            ],
            Elements::RightPadLow1_8 => [
                0x59, 0xd7, 0x22, 0x70, 0xef, 0x0e, 0x8f, 0x77, 0x0c, 0x8d, 0x11, 0xf3, 0x17, 0x73,
                0xf9, 0xb6, 0xe9, 0x0a, 0x4a, 0xec, 0xeb, 0x5b, 0xfb, 0x3d, 0xfe, 0x96, 0x8c, 0x4e,
                0x9d, 0xac, 0x5f, 0xe8,
            ],
            Elements::RightPadLow32_64 => [
                0xd4, 0x22, 0x7d, 0x06, 0x6f, 0x18, 0xb9, 0x11, 0xd6, 0xf5, 0xd9, 0xbf, 0xb9, 0xd9,
                0xf4, 0x6e, 0x9a, 0xea, 0xdb, 0xbe, 0xfa, 0x34, 0xd4, 0x74, 0x43, 0x2a, 0x1e, 0x78,
                0x9e, 0x48, 0x86, 0xff,
            ],
            Elements::RightPadLow8_16 => [
                0xab, 0xa4, 0x7a, 0x53, 0x6e, 0x12, 0x27, 0xe1, 0x22, 0xba, 0xac, 0xf1, 0x9c, 0xfd,
                0x28, 0x23, 0xb9, 0xb7, 0x8d, 0x79, 0xcc, 0x06, 0xd3, 0x4c, 0x34, 0x8b, 0x14, 0xa1,
                0xa1, 0x5a, 0xbd, 0x64,
            ],
            Elements::RightPadLow8_32 => [
                0x8f, 0x80, 0xa6, 0xc2, 0x74, 0x71, 0x6b, 0x67, 0x22, 0x04, 0x11, 0x34, 0xea, 0x1c,
                0x68, 0xaa, 0xbf, 0x02, 0x13, 0x29, 0x8f, 0x4e, 0x18, 0xf8, 0xf4, 0x92, 0xdc, 0x53,
                0x80, 0x8a, 0x31, 0x74,
            ],
            Elements::RightPadLow8_64 => [
                0xd6, 0x9c, 0x85, 0xe7, 0xb2, 0xd7, 0xe9, 0x49, 0x43, 0x6c, 0xb1, 0x29, 0x5e, 0x4a,
                0xa7, 0x05, 0x57, 0xd7, 0x5e, 0x7c, 0xbd, 0xec, 0x02, 0xcc, 0xa8, 0x5f, 0xbf, 0xb1,
                0x33, 0x08, 0xb2, 0x10,
            ],
            Elements::RightRotate16 => [
                0xe5, 0x10, 0x70, 0x82, 0x47, 0xf9, 0x1b, 0x4f, 0x0a, 0x8a, 0x22, 0xa4, 0x46, 0xb8,
                0x13, 0x7d, 0x0d, 0x42, 0xbe, 0xe7, 0x4c, 0x8c, 0x1e, 0xdd, 0x6d, 0x44, 0x6e, 0xdb,
                0x20, 0x13, 0xb5, 0x98,
            ],
            Elements::RightRotate32 => [
                0x98, 0x91, 0x57, 0x31, 0x41, 0x29, 0x22, 0xdb, 0xc5, 0x16, 0xa7, 0x37, 0x3a, 0xfc,
                0x4d, 0xe6, 0x48, 0x09, 0xf8, 0x3b, 0x26, 0x4b, 0xcf, 0xca, 0x6a, 0xe7, 0x48, 0x83,
                0xdb, 0xe1, 0x04, 0xd6,
            ],
            Elements::RightRotate64 => [
                0x9e, 0x2f, 0xb9, 0x8a, 0xdf, 0x10, 0x29, 0x33, 0x9d, 0xbe, 0x45, 0xa2, 0x2a, 0x54,
                0xa3, 0x90, 0xca, 0x09, 0x86, 0xed, 0xce, 0xa3, 0x2e, 0xac, 0xb8, 0x2e, 0xbc, 0xc8,
                0x94, 0xa2, 0x71, 0x1a,
            ],
            Elements::RightRotate8 => [
                0x00, 0xc7, 0xc2, 0x6d, 0x95, 0xa5, 0x0b, 0x5a, 0xf9, 0x34, 0x9f, 0xfe, 0x47, 0xe1,
                0xd4, 0x3f, 0x3d, 0x76, 0x1f, 0x17, 0xa7, 0x45, 0x3c, 0x98, 0x47, 0x91, 0xe8, 0x7d,
                0xc6, 0xa3, 0x11, 0xc8,
            ],
            Elements::RightShift16 => [
                0x8b, 0x5e, 0x0f, 0xeb, 0x95, 0x81, 0x30, 0xf0, 0x50, 0x83, 0x32, 0x15, 0x9e, 0x54,
                0xc2, 0xdf, 0x98, 0xaf, 0x83, 0x52, 0x1a, 0xca, 0xb3, 0x08, 0x4f, 0xd4, 0xf7, 0xc3,
                0xa2, 0xcc, 0xea, 0x77,
            ],
            Elements::RightShift32 => [
                0x4b, 0x1f, 0x25, 0x80, 0xe0, 0x85, 0x0d, 0x38, 0xe2, 0xa1, 0x15, 0x73, 0x38, 0x05,
                0x2f, 0x1c, 0x37, 0x9f, 0x9d, 0x81, 0x57, 0xf6, 0x2d, 0x33, 0x89, 0x0a, 0xf2, 0x4f,
                0xd9, 0xa7, 0xf7, 0x3e,
            ],
            Elements::RightShift64 => [
                0x91, 0xa2, 0x97, 0xd7, 0xb5, 0x8a, 0x39, 0x3b, 0xf5, 0x90, 0x25, 0x94, 0x77, 0x47,
                0xc8, 0x6d, 0xd4, 0x87, 0x65, 0x9c, 0xc5, 0x6f, 0xb5, 0xa6, 0xf6, 0x43, 0x99, 0x55,
                0x12, 0x9a, 0x95, 0x63,
            ],
            Elements::RightShift8 => [
                0xa4, 0xc3, 0x54, 0x6f, 0xf2, 0x7e, 0x56, 0xd6, 0x4e, 0x91, 0x8a, 0xb2, 0xfa, 0x6d,
                0x00, 0xfc, 0x27, 0x04, 0x58, 0x5b, 0x25, 0xbd, 0xe0, 0x04, 0x9d, 0x6d, 0x8f, 0x48,
                0xd8, 0xcf, 0x1c, 0xd0,
            ],
            Elements::RightShiftWith16 => [
                0xfd, 0x97, 0x70, 0x30, 0xe3, 0xa2, 0x5a, 0x32, 0xe7, 0x75, 0xb8, 0xd5, 0xe8, 0x71,
                0x74, 0xa7, 0xa9, 0xe8, 0x73, 0x1e, 0xc3, 0x6c, 0xf1, 0x32, 0x64, 0x20, 0xad, 0x91,
                0x50, 0x2e, 0x6e, 0x98,
            ],
            Elements::RightShiftWith32 => [
                0x28, 0x29, 0xba, 0x02, 0x1f, 0x54, 0x07, 0x7a, 0xff, 0xb6, 0x6a, 0xc6, 0xb6, 0xdf,
                0xd3, 0xfe, 0xf3, 0x8b, 0xc4, 0x14, 0x91, 0x84, 0x5a, 0x41, 0xce, 0x9d, 0xd3, 0x70,
                0x58, 0x6c, 0x2d, 0x04,
            ],
            Elements::RightShiftWith64 => [
                0x00, 0x6f, 0xa3, 0xc5, 0x45, 0x79, 0x75, 0x47, 0x86, 0xfc, 0x64, 0xdc, 0x32, 0xe1,
                0x9a, 0x22, 0x5c, 0xc1, 0x52, 0xc9, 0x4d, 0xee, 0xb3, 0xc6, 0xab, 0x29, 0x67, 0xdd,
                0xbf, 0xc6, 0x46, 0x53,
            ],
            Elements::RightShiftWith8 => [
                0xfc, 0xb5, 0xbe, 0x65, 0x07, 0xf0, 0xca, 0x44, 0xbe, 0x2b, 0xe1, 0xcc, 0x3c, 0x3c,
                0xfe, 0x39, 0x94, 0x40, 0x4b, 0x80, 0x83, 0xbd, 0x76, 0x02, 0xb2, 0x10, 0x2c, 0xb1,
                0xfc, 0xfa, 0x2c, 0x61,
            ],
            Elements::Rightmost16_1 => [
                0x3f, 0x3c, 0x43, 0x46, 0x87, 0x17, 0x42, 0x26, 0x5e, 0x87, 0xf0, 0x01, 0xb4, 0x6d,
                0xe7, 0xd1, 0x98, 0x75, 0x1b, 0x34, 0xfa, 0xa1, 0x80, 0x18, 0xde, 0x60, 0xc8, 0x46,
                0x8d, 0x9b, 0x98, 0xa4,
            ],
            Elements::Rightmost16_2 => [
                0x78, 0xf1, 0x71, 0x47, 0x6a, 0x3b, 0x0e, 0xd1, 0xe3, 0xa5, 0x45, 0x5a, 0x5f, 0xbb,
                0xcc, 0x90, 0x19, 0x81, 0xb3, 0x23, 0x0f, 0xea, 0x12, 0x64, 0x20, 0x4d, 0xac, 0xd0,
                0x81, 0xf9, 0x40, 0x80,
            ],
            Elements::Rightmost16_4 => [
                0x75, 0xa1, 0xdf, 0xb6, 0xae, 0x2c, 0x06, 0x6b, 0x2d, 0x0e, 0x20, 0x93, 0x04, 0x8a,
                0xdb, 0xc5, 0x0d, 0x46, 0x50, 0x65, 0x6f, 0xb2, 0xd3, 0x57, 0x8b, 0x57, 0xd9, 0xde,
                0x4c, 0x61, 0xc8, 0xb5,
            ],
            Elements::Rightmost16_8 => [
                0xee, 0x76, 0x9c, 0x1c, 0xc8, 0xa3, 0xfd, 0xd1, 0x83, 0x8f, 0xc9, 0xf0, 0x49, 0x0c,
                0xe7, 0x03, 0x93, 0xfd, 0x91, 0xba, 0x3c, 0xbd, 0x4a, 0xbd, 0x08, 0x64, 0x9f, 0xb9,
                0xc4, 0x43, 0x11, 0xbd,
            ],
            Elements::Rightmost32_1 => [
                0xcb, 0x0d, 0xb5, 0x69, 0xa3, 0x61, 0x86, 0xa2, 0x56, 0x05, 0xa9, 0xd2, 0xe4, 0xe1,
                0x0a, 0x20, 0xc1, 0x11, 0xd5, 0x0c, 0x34, 0xf1, 0x72, 0x46, 0x52, 0x0b, 0xc4, 0x54,
                0xd8, 0x68, 0x28, 0x36,
            ],
            Elements::Rightmost32_16 => [
                0x06, 0xfa, 0xa3, 0xbe, 0x67, 0x8c, 0xd6, 0xfd, 0xd7, 0xf3, 0x11, 0x2e, 0xbf, 0x2c,
                0x48, 0x62, 0x7a, 0xfa, 0x78, 0x75, 0xf7, 0x06, 0x8d, 0x26, 0xa9, 0xcc, 0x04, 0x5b,
                0x2c, 0x8f, 0x11, 0xbc,
            ],
            Elements::Rightmost32_2 => [
                0x00, 0xb8, 0x81, 0x5a, 0xd7, 0x42, 0x3d, 0xd5, 0x8c, 0xb9, 0x8b, 0xe8, 0x2c, 0xad,
                0x26, 0x67, 0x5c, 0x3b, 0xf5, 0x4a, 0x0b, 0xed, 0xba, 0xde, 0x34, 0x64, 0xb4, 0xfe,
                0x5a, 0x4e, 0x8c, 0xe6,
            ],
            Elements::Rightmost32_4 => [
                0x3d, 0xfa, 0x7a, 0x20, 0x19, 0x8e, 0x42, 0xd6, 0xa7, 0x94, 0x8c, 0x8e, 0xd8, 0xe0,
                0xd4, 0x7e, 0xc7, 0xc0, 0x00, 0x7b, 0x3d, 0x68, 0x66, 0xca, 0x15, 0xe3, 0xda, 0x04,
                0x5b, 0x85, 0x63, 0xc7,
            ],
            Elements::Rightmost32_8 => [
                0x17, 0xb5, 0x8d, 0x6e, 0x30, 0x4b, 0x1c, 0x7e, 0x5d, 0xbf, 0x0c, 0x4d, 0xf6, 0xfc,
                0xc8, 0x03, 0xc0, 0x08, 0x94, 0x4c, 0x79, 0x95, 0x55, 0x5b, 0x94, 0xe1, 0x28, 0x9b,
                0x25, 0x49, 0xbe, 0x99,
            ],
            Elements::Rightmost64_1 => [
                0x5e, 0x8f, 0xb4, 0x9f, 0xac, 0xe0, 0x34, 0x48, 0x1d, 0xc6, 0x53, 0x61, 0x8e, 0x2a,
                0x8b, 0x65, 0xea, 0xf0, 0x99, 0x3f, 0x28, 0x84, 0x4c, 0xc9, 0xb1, 0x30, 0xca, 0xcc,
                0xe4, 0x5e, 0x82, 0xde,
            ],
            Elements::Rightmost64_16 => [
                0xc6, 0x4c, 0xa9, 0x96, 0x55, 0x36, 0xf2, 0x37, 0xbc, 0x4d, 0x16, 0x6e, 0x4a, 0xec,
                0xa5, 0x6e, 0xac, 0x26, 0x62, 0xe6, 0x3a, 0xcc, 0xb9, 0x8b, 0x6e, 0x54, 0x25, 0x60,
                0xf9, 0xe5, 0x38, 0xda,
            ],
            Elements::Rightmost64_2 => [
                0x83, 0xd2, 0xda, 0x6f, 0x34, 0x20, 0xd7, 0x79, 0xbc, 0xb8, 0xf6, 0x0d, 0x0b, 0x69,
                0x6e, 0xed, 0x74, 0xc3, 0x1d, 0xb0, 0x8a, 0xdd, 0xbe, 0xbd, 0x12, 0x35, 0xa5, 0xdf,
                0x8f, 0x59, 0xc4, 0x2f,
            ],
            Elements::Rightmost64_32 => [
                0x7d, 0x2d, 0xff, 0x6e, 0x3d, 0xd5, 0x04, 0xbb, 0x0e, 0x57, 0x03, 0xa0, 0x33, 0x58,
                0x6d, 0x27, 0xd9, 0x66, 0x44, 0xc0, 0x48, 0xab, 0x34, 0xa4, 0x5b, 0xf5, 0x35, 0x12,
                0x9d, 0x50, 0x11, 0x67,
            ],
            Elements::Rightmost64_4 => [
                0x84, 0x1b, 0xbd, 0x65, 0x27, 0x42, 0xdd, 0xd3, 0xad, 0xea, 0xe4, 0x3c, 0xfe, 0xd6,
                0x32, 0x9f, 0x2f, 0xd6, 0x2e, 0x6f, 0xec, 0xd0, 0xfd, 0x58, 0xe3, 0xc3, 0xfb, 0x8b,
                0x5a, 0x0e, 0x4d, 0xd5,
            ],
            Elements::Rightmost64_8 => [
                0xa0, 0xa6, 0x1c, 0x76, 0x58, 0xa1, 0x86, 0x23, 0xbf, 0x1d, 0x01, 0x1a, 0x97, 0x92,
                0xd5, 0x18, 0xfb, 0xd0, 0x24, 0x14, 0x2a, 0x90, 0x44, 0x00, 0xec, 0xde, 0xea, 0x92,
                0x45, 0x7a, 0x0a, 0x81,
            ],
            Elements::Rightmost8_1 => [
                0x99, 0x9b, 0x68, 0x6e, 0x60, 0xb3, 0xd1, 0xec, 0xd6, 0xc6, 0xd7, 0x7f, 0xbc, 0xa8,
                0x2c, 0xb2, 0xab, 0xbd, 0x41, 0x82, 0xc8, 0x21, 0x12, 0x67, 0x47, 0x5f, 0xa0, 0xc1,
                0x90, 0x1d, 0x89, 0xf9,
            ],
            Elements::Rightmost8_2 => [
                0x53, 0x07, 0xff, 0xbf, 0x51, 0x6c, 0xd0, 0xee, 0xf3, 0xff, 0x43, 0x87, 0xb9, 0x05,
                0x2c, 0x14, 0x4a, 0x4d, 0xfa, 0x23, 0x29, 0x23, 0x7c, 0x6b, 0x27, 0x49, 0x92, 0xb2,
                0xc8, 0x04, 0x7b, 0x60,
            ],
            Elements::Rightmost8_4 => [
                0x7f, 0x52, 0xe6, 0x45, 0xbb, 0xbb, 0xd7, 0x92, 0x69, 0xc4, 0x3e, 0xf0, 0x2d, 0xb9,
                0x82, 0xf8, 0xc6, 0x36, 0x33, 0xc1, 0x79, 0xe4, 0x06, 0x91, 0x73, 0x93, 0x36, 0x04,
                0xcc, 0x63, 0x5b, 0xca,
            ],
            Elements::ScalarAdd => [
                0x34, 0xba, 0xa4, 0x0b, 0x2e, 0x0a, 0xa8, 0xcb, 0x7e, 0x97, 0xc7, 0x3e, 0x3e, 0xd3,
                0xb3, 0x65, 0xa1, 0x5b, 0x7c, 0x3f, 0x76, 0x61, 0xfb, 0x19, 0x71, 0x5e, 0xc6, 0x05,
                0xc1, 0x14, 0x9d, 0x11,
            ],
            Elements::ScalarInvert => [
                0x62, 0x31, 0xbd, 0xab, 0x73, 0xca, 0x34, 0xea, 0x7e, 0x83, 0x7d, 0xaa, 0xd6, 0x92,
                0xed, 0xe5, 0xba, 0xbf, 0xae, 0x09, 0xb5, 0x75, 0x6d, 0x2a, 0xb3, 0x6c, 0x5a, 0x36,
                0x47, 0x5a, 0x65, 0x89,
            ],
            Elements::ScalarIsZero => [
                0xf7, 0x5e, 0xda, 0x06, 0xce, 0x6a, 0xf0, 0x9f, 0xae, 0x37, 0xdb, 0x4e, 0x62, 0x25,
                0xe6, 0xa8, 0xac, 0x86, 0xa2, 0x36, 0x37, 0x62, 0x7d, 0x62, 0x64, 0x09, 0x19, 0x0f,
                0xf3, 0xb3, 0x9d, 0x90,
            ],
            Elements::ScalarMultiply => [
                0xb2, 0xbc, 0xc3, 0x90, 0xd6, 0x37, 0xb9, 0xe0, 0x3f, 0xbf, 0xc4, 0x2f, 0xff, 0x71,
                0xd2, 0x2e, 0x72, 0x00, 0xf6, 0x93, 0x29, 0xce, 0xf7, 0x16, 0x9e, 0x68, 0xa8, 0xc7,
                0x1a, 0x7f, 0x0a, 0x4b,
            ],
            Elements::ScalarMultiplyLambda => [
                0x89, 0xd5, 0x85, 0x5c, 0x5f, 0x85, 0xc0, 0x03, 0x5d, 0x27, 0xb0, 0xc0, 0x9e, 0x20,
                0x33, 0x0b, 0x00, 0x1c, 0x68, 0x4b, 0x59, 0x86, 0xab, 0xce, 0xd8, 0x36, 0x0c, 0xd3,
                0x9b, 0x08, 0xc4, 0xe1,
            ],
            Elements::ScalarNegate => [
                0x07, 0x05, 0xac, 0xdf, 0xb8, 0x66, 0x40, 0x00, 0x0e, 0x3d, 0x3b, 0xad, 0x50, 0x9a,
                0x14, 0xa7, 0x8c, 0x17, 0x1f, 0x61, 0xed, 0xc0, 0x84, 0x23, 0xb0, 0x42, 0xb9, 0x47,
                0x48, 0x43, 0x9c, 0xf8,
            ],
            Elements::ScalarNormalize => [
                0xa0, 0x61, 0xe1, 0x9d, 0x75, 0xc3, 0x25, 0xa2, 0x6d, 0x56, 0x5a, 0xad, 0x7e, 0x3f,
                0x9a, 0xe2, 0x6b, 0x22, 0x2f, 0x25, 0xe8, 0x02, 0x17, 0x4f, 0x6b, 0xac, 0xd5, 0x11,
                0x27, 0x7a, 0xea, 0xa5,
            ],
            Elements::ScalarSquare => [
                0x49, 0xf7, 0x34, 0xa2, 0x65, 0x9c, 0xa0, 0xab, 0x7c, 0x9e, 0x67, 0xfc, 0xfc, 0x3c,
                0x0d, 0x72, 0xaf, 0x0f, 0x91, 0x7c, 0x9e, 0xdc, 0xb9, 0x92, 0x9d, 0x17, 0x7a, 0x0f,
                0x0d, 0xe8, 0x9d, 0x59,
            ],
            Elements::Scale => [
                0xc0, 0x45, 0x43, 0xdc, 0x85, 0xef, 0x11, 0x37, 0x4a, 0x93, 0x0f, 0x4a, 0x94, 0x8e,
                0xb7, 0x35, 0xa6, 0x50, 0x0a, 0x1a, 0x71, 0x58, 0xd5, 0x73, 0x12, 0x3f, 0x07, 0x21,
                0x71, 0x75, 0xf3, 0x18,
            ],
            Elements::ScriptCMR => [
                0xec, 0x15, 0xf5, 0x9c, 0x52, 0x4d, 0x94, 0x14, 0x69, 0x81, 0x19, 0x54, 0x65, 0x4f,
                0x63, 0x58, 0x30, 0xec, 0xd7, 0xde, 0x51, 0x76, 0x62, 0xb2, 0x5c, 0xa4, 0xe8, 0xd0,
                0x5c, 0x2c, 0xce, 0x89,
            ],
            Elements::Sha256Block => [
                0x0c, 0x97, 0xa0, 0x08, 0xad, 0xe8, 0x7b, 0xb1, 0xe0, 0xac, 0x06, 0xb7, 0xd0, 0x31,
                0x30, 0x23, 0x36, 0x28, 0x58, 0xef, 0x90, 0xec, 0x14, 0xec, 0x9c, 0xb9, 0x5f, 0x0d,
                0xa9, 0x64, 0xe0, 0x08,
            ],
            Elements::Sha256Ctx8Add1 => [
                0x37, 0x06, 0x6c, 0x67, 0xad, 0x95, 0x24, 0x9d, 0x4b, 0xa6, 0xe1, 0x81, 0x44, 0xca,
                0x0a, 0x41, 0x5d, 0x9c, 0x83, 0x2a, 0xa6, 0xb6, 0x06, 0x28, 0xe9, 0x7c, 0x96, 0x7e,
                0xb1, 0x79, 0x33, 0x83,
            ],
            Elements::Sha256Ctx8Add128 => [
                0x2d, 0xcf, 0x48, 0x4c, 0x25, 0x7f, 0x67, 0x94, 0x0c, 0xa3, 0x75, 0xba, 0x98, 0xe8,
                0x3c, 0xe0, 0xe2, 0xa7, 0x1e, 0x16, 0xda, 0x50, 0x51, 0xd1, 0xbb, 0x19, 0xfb, 0x5f,
                0x34, 0x6f, 0x15, 0x4f,
            ],
            Elements::Sha256Ctx8Add16 => [
                0x82, 0x99, 0x25, 0x20, 0x40, 0xcb, 0x39, 0xe3, 0x26, 0xa2, 0x48, 0xd5, 0xc7, 0x88,
                0xf9, 0x51, 0x6d, 0x15, 0xa2, 0xff, 0x41, 0x45, 0xbb, 0x64, 0xad, 0x65, 0x77, 0xae,
                0x1a, 0x3e, 0xf7, 0x27,
            ],
            Elements::Sha256Ctx8Add2 => [
                0x8b, 0xae, 0x3e, 0x7e, 0x1e, 0xd4, 0xdc, 0xba, 0x6e, 0x64, 0x5a, 0xa1, 0x43, 0x41,
                0xbb, 0xae, 0x0d, 0xbb, 0x3a, 0xe2, 0x1b, 0xb6, 0x3d, 0xc0, 0x30, 0xca, 0x0e, 0x44,
                0x7a, 0x85, 0x7e, 0xc2,
            ],
            Elements::Sha256Ctx8Add256 => [
                0x44, 0xb7, 0x17, 0xe1, 0x97, 0x09, 0x99, 0xb6, 0x6b, 0x69, 0x3d, 0x8c, 0x9d, 0x1d,
                0x3b, 0x06, 0x05, 0xc2, 0xb7, 0xa6, 0x21, 0x3e, 0x6b, 0xa5, 0x6c, 0x69, 0xaf, 0x8d,
                0x7f, 0xae, 0x16, 0x86,
            ],
            Elements::Sha256Ctx8Add32 => [
                0x39, 0x23, 0x9a, 0x43, 0xa8, 0x4b, 0xac, 0x6f, 0x29, 0x69, 0xbf, 0xa9, 0x5b, 0xfe,
                0x6a, 0x04, 0xfc, 0xba, 0x80, 0x92, 0x89, 0x59, 0x39, 0xf1, 0x2a, 0x1c, 0xe0, 0xe2,
                0x63, 0x21, 0xec, 0x10,
            ],
            Elements::Sha256Ctx8Add4 => [
                0xd7, 0xd7, 0x45, 0x61, 0x4b, 0x37, 0xa7, 0xe0, 0x7d, 0xce, 0x22, 0xf6, 0x4e, 0x7b,
                0x1e, 0xdf, 0xe2, 0x3b, 0xed, 0xa8, 0x51, 0xf1, 0xe7, 0x6f, 0x1a, 0x6b, 0x02, 0x8f,
                0xcc, 0x5e, 0x9f, 0xc0,
            ],
            Elements::Sha256Ctx8Add512 => [
                0xbe, 0x36, 0x80, 0x32, 0xd8, 0x6e, 0xbc, 0xf2, 0x13, 0xca, 0x45, 0xba, 0x6e, 0xca,
                0xb5, 0x4c, 0xb1, 0xf2, 0x66, 0x1d, 0x40, 0x3d, 0xa0, 0x59, 0x06, 0x30, 0x0b, 0xc5,
                0x11, 0x37, 0xaa, 0xb5,
            ],
            Elements::Sha256Ctx8Add64 => [
                0xfd, 0xc4, 0x34, 0xce, 0x83, 0xdb, 0xdc, 0xe0, 0x78, 0x2a, 0xa3, 0x6d, 0x41, 0x8d,
                0xef, 0x7f, 0x99, 0xaf, 0x82, 0x93, 0xaf, 0xb2, 0x9e, 0x83, 0x9f, 0xe4, 0x94, 0x8f,
                0x62, 0x34, 0xf7, 0x7f,
            ],
            Elements::Sha256Ctx8Add8 => [
                0x9c, 0x98, 0x83, 0x30, 0x79, 0x9a, 0x68, 0x0b, 0xfe, 0x73, 0xd7, 0xca, 0xa3, 0x68,
                0x9f, 0xe4, 0xe4, 0x83, 0xda, 0x4e, 0xe6, 0xd8, 0x18, 0x58, 0x79, 0x27, 0xc7, 0xf4,
                0x33, 0x92, 0xde, 0xf7,
            ],
            Elements::Sha256Ctx8AddBuffer511 => [
                0xc0, 0x27, 0xe1, 0x06, 0x29, 0x96, 0xae, 0x94, 0xac, 0x39, 0x71, 0xa2, 0xc4, 0xfa,
                0xe5, 0x49, 0x97, 0xeb, 0xf0, 0x9b, 0x9f, 0x7d, 0xa5, 0x75, 0x63, 0x9b, 0xe6, 0x17,
                0x16, 0x7f, 0x02, 0xe3,
            ],
            Elements::Sha256Ctx8Finalize => [
                0xcb, 0xba, 0x1f, 0x1d, 0x8a, 0x97, 0xab, 0x4d, 0x1f, 0xa9, 0x68, 0x6e, 0x7a, 0xee,
                0xf0, 0x66, 0xfb, 0x5b, 0xf2, 0x90, 0x71, 0x6e, 0xae, 0x10, 0xe7, 0x0b, 0x61, 0x99,
                0x96, 0xc5, 0x95, 0x94,
            ],
            Elements::Sha256Ctx8Init => [
                0xa5, 0x3c, 0x76, 0x79, 0xe3, 0xae, 0x03, 0x47, 0xd4, 0xd7, 0x91, 0x26, 0xa7, 0xc7,
                0xe4, 0x9a, 0xc0, 0xde, 0xc9, 0x0c, 0xdf, 0x93, 0x57, 0x99, 0xcd, 0xdb, 0x58, 0xda,
                0x8f, 0x44, 0x96, 0xe4,
            ],
            Elements::Sha256Iv => [
                0x73, 0x89, 0xf0, 0x02, 0x53, 0x05, 0xdc, 0xe8, 0x28, 0xd4, 0xa1, 0xfe, 0x83, 0x74,
                0x30, 0x46, 0xa3, 0x67, 0xc9, 0x23, 0xf1, 0x8a, 0xbf, 0x36, 0x5e, 0x39, 0x1e, 0x5b,
                0x04, 0xaf, 0x1a, 0x47,
            ],
            Elements::SigAllHash => [
                0x6a, 0xc5, 0x3d, 0x3f, 0x93, 0xb8, 0xca, 0xf3, 0xea, 0x85, 0x34, 0xae, 0x61, 0x2a,
                0xbd, 0x32, 0x32, 0x5d, 0x2b, 0x0f, 0xec, 0x17, 0xde, 0x36, 0xaf, 0x0f, 0x71, 0xaa,
                0x97, 0x8f, 0x7c, 0xde,
            ],
            Elements::Some1 => [
                0xfb, 0xda, 0xd6, 0xb0, 0x22, 0xa0, 0xc7, 0x8f, 0xf3, 0x56, 0x04, 0xaa, 0xfa, 0xcd,
                0x27, 0xcc, 0x10, 0xf5, 0x1e, 0xe0, 0x69, 0x8c, 0x41, 0xf1, 0xad, 0xa9, 0x03, 0x97,
                0x61, 0x8d, 0x52, 0x6f,
            ],
            Elements::Some16 => [
                0x7e, 0x2c, 0xcd, 0xbf, 0xc2, 0x4d, 0xd8, 0xd8, 0xa9, 0x04, 0xb0, 0x17, 0xdd, 0x4f,
                0x57, 0xe7, 0xc8, 0x74, 0x96, 0x34, 0x8a, 0xca, 0x7d, 0x04, 0x58, 0xc9, 0xd1, 0x6b,
                0x68, 0xbc, 0xda, 0x1c,
            ],
            Elements::Some32 => [
                0x45, 0x36, 0xae, 0xb1, 0x21, 0xc4, 0x27, 0x3f, 0xfc, 0x2a, 0x48, 0xfe, 0xd9, 0xee,
                0xd0, 0x31, 0x2e, 0xbd, 0x97, 0x2d, 0xec, 0x56, 0x81, 0xf4, 0x7e, 0xad, 0x0f, 0x62,
                0xd9, 0x54, 0x45, 0x2a,
            ],
            Elements::Some64 => [
                0x7f, 0x0b, 0xbd, 0x9d, 0x66, 0x31, 0xc1, 0x30, 0x9f, 0x90, 0x1c, 0x2f, 0x0d, 0x7a,
                0x0d, 0x28, 0x4a, 0x34, 0x41, 0x6c, 0xf7, 0x50, 0xdb, 0x1f, 0xe2, 0xb9, 0xf3, 0xd6,
                0xed, 0x70, 0x94, 0x09,
            ],
            Elements::Some8 => [
                0x2d, 0x8c, 0x8f, 0x71, 0xee, 0x5e, 0x75, 0x82, 0xf0, 0xed, 0x65, 0xf5, 0x26, 0xc0,
                0x26, 0x05, 0xdc, 0xb9, 0x3c, 0x0b, 0xdd, 0xb9, 0x43, 0x3a, 0xff, 0x3f, 0x25, 0xc2,
                0x28, 0xac, 0xda, 0x8a,
            ],
            Elements::Subtract16 => [
                0x56, 0x9e, 0x6c, 0x6b, 0x39, 0xe7, 0xd8, 0x12, 0x65, 0x9b, 0x67, 0xaa, 0xc0, 0x8a,
                0xd1, 0x50, 0x99, 0xee, 0xad, 0x79, 0x8f, 0xd1, 0xd4, 0x2d, 0xa1, 0x7e, 0xe3, 0xf0,
                0xd4, 0xd4, 0x49, 0x2a,
            ],
            Elements::Subtract32 => [
                0x19, 0xd3, 0x5e, 0x0a, 0xf1, 0xe1, 0x65, 0x14, 0xa6, 0xdf, 0xc2, 0x9a, 0x91, 0x41,
                0x87, 0x13, 0x39, 0x64, 0xc4, 0x80, 0xf6, 0x60, 0xe7, 0xeb, 0x92, 0x4e, 0xe1, 0x6d,
                0xba, 0xa2, 0x49, 0xcb,
            ],
            Elements::Subtract64 => [
                0x52, 0x3e, 0x11, 0x86, 0x28, 0xbf, 0x3a, 0xc1, 0xa6, 0xbe, 0x5a, 0x72, 0xbd, 0xb1,
                0x14, 0x1b, 0x89, 0xe0, 0xe0, 0x01, 0xe4, 0x02, 0xad, 0xda, 0x82, 0x58, 0x79, 0x00,
                0x03, 0xf8, 0x8a, 0xd8,
            ],
            Elements::Subtract8 => [
                0x40, 0x95, 0x0b, 0x86, 0xf6, 0xf1, 0xf9, 0x93, 0x55, 0xde, 0xe1, 0x1f, 0x77, 0xda,
                0xf2, 0x79, 0xa0, 0xcb, 0x6c, 0x6d, 0x15, 0x6a, 0xe4, 0x4b, 0x7d, 0x5d, 0x25, 0x71,
                0x64, 0xb2, 0x67, 0xc5,
            ],
            Elements::Swu => [
                0xab, 0xf7, 0x0b, 0xe0, 0x0b, 0x30, 0xf5, 0x77, 0xf9, 0x87, 0xcb, 0x50, 0x48, 0x89,
                0x96, 0xba, 0x35, 0x96, 0xdb, 0xf9, 0xc1, 0xe8, 0x44, 0xa8, 0xb1, 0xb8, 0xb7, 0x10,
                0x85, 0x3b, 0x65, 0xeb,
            ],
            Elements::TapEnvHash => [
                0x72, 0x27, 0x07, 0x71, 0x93, 0xec, 0x17, 0x41, 0x8e, 0xbf, 0x78, 0x77, 0xaf, 0xdc,
                0x70, 0x97, 0xbd, 0x5e, 0xed, 0x57, 0xe8, 0x26, 0xdf, 0x66, 0x05, 0xc6, 0x48, 0x69,
                0x1d, 0x00, 0x34, 0x03,
            ],
            Elements::TapdataInit => [
                0x6c, 0x67, 0xe5, 0xc1, 0x07, 0x35, 0x30, 0x5e, 0xe7, 0xde, 0xb5, 0x9a, 0x6c, 0x6a,
                0xc2, 0xef, 0xfc, 0xab, 0x4f, 0xf7, 0xbb, 0x47, 0x9e, 0xa7, 0x00, 0x81, 0x60, 0x6e,
                0x60, 0x48, 0x4c, 0xa7,
            ],
            Elements::TapleafHash => [
                0xd4, 0x5a, 0x66, 0x1f, 0xc9, 0x10, 0x0e, 0x2e, 0xa9, 0xdf, 0x78, 0x9c, 0xa2, 0x34,
                0x6d, 0xa8, 0x42, 0x71, 0x7e, 0xf6, 0xa8, 0xc8, 0x24, 0x18, 0x4f, 0x2d, 0xa8, 0xc7,
                0xfb, 0x01, 0x62, 0x03,
            ],
            Elements::TapleafVersion => [
                0xdd, 0xb0, 0xed, 0xd6, 0xce, 0x1e, 0x07, 0x98, 0x3d, 0x1a, 0x62, 0x26, 0xf0, 0x0b,
                0x12, 0x51, 0x89, 0xaf, 0x13, 0xbd, 0x5c, 0xd0, 0x68, 0x12, 0x3a, 0xd4, 0x3f, 0x48,
                0x20, 0x1b, 0x00, 0x6f,
            ],
            Elements::Tappath => [
                0x42, 0xc0, 0xc1, 0xf2, 0xef, 0x0e, 0x28, 0xae, 0x7d, 0xef, 0xa1, 0x5e, 0x84, 0xa0,
                0x6f, 0x35, 0xfb, 0xf6, 0xfc, 0x89, 0x7b, 0x29, 0xf6, 0x83, 0xfe, 0x1a, 0x24, 0xe3,
                0x0d, 0xc5, 0x8b, 0xaf,
            ],
            Elements::TappathHash => [
                0x9d, 0xa9, 0x06, 0xe3, 0xec, 0x08, 0x60, 0x65, 0xbe, 0x14, 0x41, 0x36, 0xb5, 0x16,
                0x20, 0xb6, 0x6d, 0xdf, 0x4d, 0x71, 0x38, 0x57, 0x35, 0x4e, 0xba, 0x3b, 0x97, 0xc7,
                0x75, 0x85, 0xf4, 0x43,
            ],
            Elements::TotalFee => [
                0x4e, 0xba, 0x3f, 0x22, 0xba, 0xc0, 0xa4, 0x65, 0xe4, 0xd0, 0x8a, 0x9a, 0xc4, 0x05,
                0x18, 0xef, 0x97, 0x2c, 0x2e, 0x86, 0x12, 0x09, 0x0a, 0x51, 0xe3, 0x20, 0x3f, 0xcb,
                0xd2, 0xf4, 0xed, 0xbe,
            ],
            Elements::TransactionId => [
                0x08, 0xb8, 0xeb, 0x3c, 0x2d, 0x2d, 0x95, 0x84, 0x58, 0xe1, 0x63, 0xe9, 0x43, 0x34,
                0x95, 0x0c, 0x0b, 0xb9, 0x14, 0xa0, 0xb9, 0xd2, 0x14, 0xfc, 0x5e, 0x9d, 0xe8, 0xf7,
                0x0a, 0x17, 0xa0, 0xd4,
            ],
            Elements::TxHash => [
                0xc3, 0x61, 0x02, 0xae, 0xda, 0x8b, 0xcf, 0x62, 0xa8, 0x5e, 0x71, 0x4b, 0x18, 0xe4,
                0xc5, 0x0c, 0xd4, 0x17, 0xff, 0xbf, 0x86, 0xd7, 0x37, 0x34, 0xe0, 0x2c, 0xbf, 0xcb,
                0x53, 0x13, 0xfc, 0xcf,
            ],
            Elements::TxIsFinal => [
                0x8b, 0x31, 0x45, 0x72, 0x24, 0x70, 0xa0, 0x7d, 0xe9, 0x0a, 0x28, 0xba, 0x89, 0xf3,
                0xf8, 0x86, 0x42, 0x61, 0x00, 0x96, 0x54, 0xce, 0x86, 0x6c, 0xd8, 0xea, 0xf7, 0x6c,
                0x5d, 0x86, 0x26, 0xeb,
            ],
            Elements::TxLockDistance => [
                0x4c, 0x77, 0x73, 0xb8, 0x18, 0xcb, 0x7e, 0xe5, 0xf5, 0x4f, 0x92, 0x5a, 0xad, 0x01,
                0x56, 0x77, 0xa0, 0x43, 0xa7, 0x2f, 0x31, 0x6a, 0x18, 0x7c, 0xc2, 0x8c, 0x69, 0x6c,
                0xfc, 0xb9, 0x08, 0x07,
            ],
            Elements::TxLockDuration => [
                0xcc, 0x9c, 0x64, 0xc8, 0xb6, 0xeb, 0x4b, 0xf0, 0x96, 0x94, 0xaf, 0x5a, 0x35, 0xd9,
                0x57, 0xa4, 0x05, 0xe6, 0x6c, 0x1b, 0x35, 0x22, 0x4e, 0xd6, 0x75, 0x87, 0x89, 0x18,
                0x45, 0x24, 0x40, 0xb2,
            ],
            Elements::TxLockHeight => [
                0xc2, 0x02, 0x57, 0xf8, 0xe7, 0x6e, 0xcd, 0x0a, 0xe7, 0xad, 0x63, 0x4f, 0x5d, 0xfa,
                0x68, 0xae, 0x9a, 0x5e, 0xde, 0xd0, 0xe2, 0xee, 0xbe, 0x4e, 0xe5, 0x2c, 0xb4, 0x7a,
                0xcf, 0xb0, 0x26, 0x4c,
            ],
            Elements::TxLockTime => [
                0x3e, 0xe1, 0x90, 0x05, 0x42, 0xd0, 0x1e, 0xfd, 0x4e, 0x9a, 0x01, 0xd4, 0xef, 0xb1,
                0xf9, 0xdd, 0x99, 0x2c, 0xed, 0x35, 0xb7, 0xa7, 0x52, 0xf8, 0x3d, 0xa5, 0x93, 0x38,
                0x15, 0x38, 0xde, 0xa4,
            ],
            Elements::Verify => [
                0x34, 0x3e, 0x6d, 0xc1, 0x6b, 0x3f, 0x52, 0xe8, 0x3e, 0x3b, 0x4c, 0xcc, 0x99, 0xb8,
                0xc6, 0xf9, 0x6a, 0x07, 0x4f, 0xe3, 0x99, 0x32, 0x7a, 0xf3, 0x64, 0xbc, 0x28, 0x5e,
                0x29, 0x97, 0x45, 0xa2,
            ],
            Elements::Version => [
                0x08, 0x7f, 0xc9, 0x5c, 0x41, 0x00, 0x33, 0x48, 0x75, 0x9f, 0xd8, 0x40, 0x37, 0x2c,
                0x6f, 0x91, 0x2d, 0x9b, 0xe4, 0xe6, 0x1e, 0xee, 0x3c, 0x6a, 0x7a, 0x40, 0xdc, 0x13,
                0xc9, 0xc1, 0xbc, 0x70,
            ],
            Elements::Xor1 => [
                0x9d, 0xc9, 0xfe, 0x42, 0xf7, 0xeb, 0x34, 0x64, 0x9f, 0x1c, 0x72, 0xd2, 0xe5, 0xdd,
                0x16, 0x7d, 0xb2, 0x1b, 0xe5, 0x32, 0x13, 0x72, 0xd5, 0xca, 0x7f, 0x6a, 0x18, 0x4f,
                0x93, 0xe0, 0x5e, 0xe3,
            ],
            Elements::Xor16 => [
                0x1f, 0xca, 0xf4, 0x0b, 0xde, 0xdd, 0x72, 0xe7, 0x97, 0xb0, 0x9f, 0xe7, 0x87, 0x53,
                0xb0, 0xab, 0x27, 0x87, 0x2c, 0x0b, 0xd1, 0x2b, 0x03, 0x49, 0x55, 0xfb, 0xfa, 0xc2,
                0x38, 0x12, 0xef, 0x26,
            ],
            Elements::Xor32 => [
                0x1d, 0x49, 0xfc, 0x94, 0xf2, 0x2b, 0x5d, 0x31, 0xb7, 0xf9, 0xef, 0xb5, 0x37, 0x8e,
                0x5f, 0x8a, 0x42, 0x62, 0x6a, 0xed, 0x4e, 0x92, 0x79, 0x93, 0x48, 0xd6, 0xb7, 0x88,
                0xdf, 0xe8, 0x6b, 0x1c,
            ],
            Elements::Xor64 => [
                0x7a, 0x3f, 0x3f, 0x55, 0x20, 0x47, 0x83, 0x65, 0x33, 0x44, 0x31, 0x1d, 0x1d, 0xc5,
                0x09, 0xd3, 0x5b, 0x66, 0x39, 0xc0, 0xd8, 0xb9, 0x67, 0xa2, 0x07, 0x80, 0x6c, 0xd8,
                0x7d, 0x31, 0xd6, 0xe6,
            ],
            Elements::Xor8 => [
                0xd8, 0x33, 0x5f, 0x48, 0x90, 0xc1, 0xd8, 0xed, 0x76, 0x6c, 0x71, 0x35, 0x90, 0x2e,
                0x01, 0xa0, 0x09, 0x4e, 0x3a, 0x98, 0x16, 0xf7, 0x0c, 0x84, 0x7c, 0xc3, 0xd7, 0xc0,
                0x00, 0x40, 0x6e, 0xfe,
            ],
            Elements::XorXor1 => [
                0x1e, 0x10, 0x7b, 0x05, 0xff, 0x94, 0x1d, 0x31, 0xd7, 0x57, 0x8b, 0x43, 0x73, 0x28,
                0xba, 0x52, 0xf3, 0xff, 0x20, 0xa0, 0x68, 0xc0, 0xd2, 0xbd, 0xef, 0x08, 0x76, 0x80,
                0x93, 0xcc, 0x7c, 0x63,
            ],
            Elements::XorXor16 => [
                0xb7, 0x76, 0x98, 0x9d, 0xa5, 0x09, 0x5c, 0x4b, 0xe9, 0x4b, 0x1a, 0xef, 0x75, 0x94,
                0x66, 0xe1, 0x1f, 0x63, 0x9c, 0x19, 0x39, 0x47, 0x1f, 0xa1, 0x8e, 0x36, 0xe7, 0xe4,
                0x90, 0xc3, 0x89, 0x61,
            ],
            Elements::XorXor32 => [
                0xd1, 0x68, 0xfa, 0xc1, 0xac, 0x7f, 0xc4, 0x83, 0x57, 0xbe, 0x1b, 0x65, 0x33, 0x75,
                0xec, 0x5e, 0x3f, 0x05, 0x82, 0x3a, 0xae, 0x6a, 0xc9, 0x85, 0xe9, 0x40, 0x3e, 0xea,
                0xb1, 0x2b, 0xb9, 0xf8,
            ],
            Elements::XorXor64 => [
                0x36, 0x1c, 0x57, 0x93, 0x0e, 0xf9, 0x7d, 0x49, 0xcb, 0xc6, 0x79, 0xfa, 0xef, 0x1e,
                0x3b, 0xcf, 0xfb, 0x78, 0x79, 0x95, 0xb9, 0x61, 0xe5, 0x53, 0x7d, 0x2b, 0x1e, 0xeb,
                0xc9, 0xc9, 0xa6, 0xe8,
            ],
            Elements::XorXor8 => [
                0xc2, 0xda, 0x6e, 0x9c, 0xa6, 0x4d, 0x8a, 0x73, 0xc1, 0x77, 0x26, 0x67, 0xb3, 0xd7,
                0xa0, 0x93, 0x8b, 0xcb, 0x8a, 0x6c, 0x43, 0xfd, 0x04, 0x73, 0xee, 0xc7, 0x1b, 0x77,
                0x49, 0x4a, 0xad, 0x94,
            ],
        };

        Cmr(Midstate(bytes))
    }

    fn source_ty(&self) -> TypeName {
        let name: &'static [u8] = match self {
            Elements::Add16 => b"i",
            Elements::Add32 => b"l",
            Elements::Add64 => b"*ll",
            Elements::Add8 => b"****22*22**22*22***22*22**22*22",
            Elements::All16 => b"****22*22**22*22***22*22**22*22",
            Elements::All32 => b"i",
            Elements::All64 => b"l",
            Elements::All8 => b"***22*22**22*22",
            Elements::And1 => b"*22",
            Elements::And16 => b"i",
            Elements::And32 => b"l",
            Elements::And64 => b"*ll",
            Elements::And8 => b"****22*22**22*22***22*22**22*22",
            Elements::AnnexHash => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh+1h",
            Elements::AssetAmountHash => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh*+*2hh+*2hl",
            Elements::Bip0340Verify => b"**hh*hh",
            Elements::BuildTapbranch => b"*hh",
            Elements::BuildTapleafSimplicity => b"h",
            Elements::BuildTaptweak => b"*hh",
            Elements::CalculateAsset => b"h",
            Elements::CalculateConfidentialToken => b"h",
            Elements::CalculateExplicitToken => b"h",
            Elements::CalculateIssuanceEntropy => b"**hih",
            Elements::Ch1 => b"*2*22",
            Elements::Ch16 => b"*****22*22**22*22***22*22**22*22i",
            Elements::Ch32 => b"*il",
            Elements::Ch64 => b"*l*ll",
            Elements::Ch8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Elements::CheckLockDistance => b"****22*22**22*22***22*22**22*22",
            Elements::CheckLockDuration => b"****22*22**22*22***22*22**22*22",
            Elements::CheckLockHeight => b"i",
            Elements::CheckLockTime => b"i",
            Elements::CheckSigVerify => b"**h*hh*hh",
            Elements::Complement1 => b"2",
            Elements::Complement16 => b"****22*22**22*22***22*22**22*22",
            Elements::Complement32 => b"i",
            Elements::Complement64 => b"l",
            Elements::Complement8 => b"***22*22**22*22",
            Elements::CurrentAmount => b"1",
            Elements::CurrentAnnexHash => b"1",
            Elements::CurrentAsset => b"1",
            Elements::CurrentIndex => b"1",
            Elements::CurrentIssuanceAssetAmount => b"1",
            Elements::CurrentIssuanceAssetProof => b"1",
            Elements::CurrentIssuanceTokenAmount => b"1",
            Elements::CurrentIssuanceTokenProof => b"1",
            Elements::CurrentNewIssuanceContract => b"1",
            Elements::CurrentPegin => b"1",
            Elements::CurrentPrevOutpoint => b"1",
            Elements::CurrentReissuanceBlinding => b"1",
            Elements::CurrentReissuanceEntropy => b"1",
            Elements::CurrentScriptHash => b"1",
            Elements::CurrentScriptSigHash => b"1",
            Elements::CurrentSequence => b"1",
            Elements::Decompress => b"*2h",
            Elements::Decrement16 => b"****22*22**22*22***22*22**22*22",
            Elements::Decrement32 => b"i",
            Elements::Decrement64 => b"l",
            Elements::Decrement8 => b"***22*22**22*22",
            Elements::DivMod128_64 => b"**lll",
            Elements::DivMod16 => b"i",
            Elements::DivMod32 => b"l",
            Elements::DivMod64 => b"*ll",
            Elements::DivMod8 => b"****22*22**22*22***22*22**22*22",
            Elements::Divide16 => b"i",
            Elements::Divide32 => b"l",
            Elements::Divide64 => b"*ll",
            Elements::Divide8 => b"****22*22**22*22***22*22**22*22",
            Elements::Divides16 => b"i",
            Elements::Divides32 => b"l",
            Elements::Divides64 => b"*ll",
            Elements::Divides8 => b"****22*22**22*22***22*22**22*22",
            Elements::Eq1 => b"*22",
            Elements::Eq16 => b"i",
            Elements::Eq256 => b"*hh",
            Elements::Eq32 => b"l",
            Elements::Eq64 => b"*ll",
            Elements::Eq8 => b"****22*22**22*22***22*22**22*22",
            Elements::FeAdd => b"*hh",
            Elements::FeInvert => b"h",
            Elements::FeIsOdd => b"h",
            Elements::FeIsZero => b"h",
            Elements::FeMultiply => b"*hh",
            Elements::FeMultiplyBeta => b"h",
            Elements::FeNegate => b"h",
            Elements::FeNormalize => b"h",
            Elements::FeSquare => b"h",
            Elements::FeSquareRoot => b"h",
            Elements::FullAdd16 => b"*2i",
            Elements::FullAdd32 => b"*2l",
            Elements::FullAdd64 => b"*2*ll",
            Elements::FullAdd8 => b"*2****22*22**22*22***22*22**22*22",
            Elements::FullDecrement16 => b"*2****22*22**22*22***22*22**22*22",
            Elements::FullDecrement32 => b"*2i",
            Elements::FullDecrement64 => b"*2l",
            Elements::FullDecrement8 => b"*2***22*22**22*22",
            Elements::FullIncrement16 => b"*2****22*22**22*22***22*22**22*22",
            Elements::FullIncrement32 => b"*2i",
            Elements::FullIncrement64 => b"*2l",
            Elements::FullIncrement8 => b"*2***22*22**22*22",
            Elements::FullLeftShift16_1 => b"*****22*22**22*22***22*22**22*222",
            Elements::FullLeftShift16_2 => b"*****22*22**22*22***22*22**22*22*22",
            Elements::FullLeftShift16_4 => b"*****22*22**22*22***22*22**22*22**22*22",
            Elements::FullLeftShift16_8 => b"*****22*22**22*22***22*22**22*22***22*22**22*22",
            Elements::FullLeftShift32_1 => b"*i2",
            Elements::FullLeftShift32_16 => b"*i****22*22**22*22***22*22**22*22",
            Elements::FullLeftShift32_2 => b"*i*22",
            Elements::FullLeftShift32_4 => b"*i**22*22",
            Elements::FullLeftShift32_8 => b"*i***22*22**22*22",
            Elements::FullLeftShift64_1 => b"*l2",
            Elements::FullLeftShift64_16 => b"*l****22*22**22*22***22*22**22*22",
            Elements::FullLeftShift64_2 => b"*l*22",
            Elements::FullLeftShift64_32 => b"*li",
            Elements::FullLeftShift64_4 => b"*l**22*22",
            Elements::FullLeftShift64_8 => b"*l***22*22**22*22",
            Elements::FullLeftShift8_1 => b"****22*22**22*222",
            Elements::FullLeftShift8_2 => b"****22*22**22*22*22",
            Elements::FullLeftShift8_4 => b"****22*22**22*22**22*22",
            Elements::FullMultiply16 => b"l",
            Elements::FullMultiply32 => b"*ll",
            Elements::FullMultiply64 => b"h",
            Elements::FullMultiply8 => b"i",
            Elements::FullRightShift16_1 => b"*2****22*22**22*22***22*22**22*22",
            Elements::FullRightShift16_2 => b"**22****22*22**22*22***22*22**22*22",
            Elements::FullRightShift16_4 => b"***22*22****22*22**22*22***22*22**22*22",
            Elements::FullRightShift16_8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Elements::FullRightShift32_1 => b"*2i",
            Elements::FullRightShift32_16 => b"*****22*22**22*22***22*22**22*22i",
            Elements::FullRightShift32_2 => b"**22i",
            Elements::FullRightShift32_4 => b"***22*22i",
            Elements::FullRightShift32_8 => b"****22*22**22*22i",
            Elements::FullRightShift64_1 => b"*2l",
            Elements::FullRightShift64_16 => b"*****22*22**22*22***22*22**22*22l",
            Elements::FullRightShift64_2 => b"**22l",
            Elements::FullRightShift64_32 => b"*il",
            Elements::FullRightShift64_4 => b"***22*22l",
            Elements::FullRightShift64_8 => b"****22*22**22*22l",
            Elements::FullRightShift8_1 => b"*2***22*22**22*22",
            Elements::FullRightShift8_2 => b"**22***22*22**22*22",
            Elements::FullRightShift8_4 => b"***22*22***22*22**22*22",
            Elements::FullSubtract16 => b"*2i",
            Elements::FullSubtract32 => b"*2l",
            Elements::FullSubtract64 => b"*2*ll",
            Elements::FullSubtract8 => b"*2****22*22**22*22***22*22**22*22",
            Elements::GeIsOnCurve => b"*hh",
            Elements::GeNegate => b"*hh",
            Elements::GejAdd => b"***hhh**hhh",
            Elements::GejDouble => b"**hhh",
            Elements::GejEquiv => b"***hhh**hhh",
            Elements::GejGeAdd => b"***hhh*hh",
            Elements::GejGeAddEx => b"***hhh*hh",
            Elements::GejGeEquiv => b"***hhh*hh",
            Elements::GejInfinity => b"1",
            Elements::GejIsInfinity => b"**hhh",
            Elements::GejIsOnCurve => b"**hhh",
            Elements::GejNegate => b"**hhh",
            Elements::GejNormalize => b"**hhh",
            Elements::GejRescale => b"***hhhh",
            Elements::GejXEquiv => b"*h**hhh",
            Elements::GejYIsOdd => b"**hhh",
            Elements::Generate => b"h",
            Elements::GenesisBlockHash => b"1",
            Elements::HashToCurve => b"h",
            Elements::High1 => b"1",
            Elements::High16 => b"1",
            Elements::High32 => b"1",
            Elements::High64 => b"1",
            Elements::High8 => b"1",
            Elements::Increment16 => b"****22*22**22*22***22*22**22*22",
            Elements::Increment32 => b"i",
            Elements::Increment64 => b"l",
            Elements::Increment8 => b"***22*22**22*22",
            Elements::InputAmount => b"i",
            Elements::InputAmountsHash => b"1",
            Elements::InputAnnexHash => b"i",
            Elements::InputAnnexesHash => b"1",
            Elements::InputAsset => b"i",
            Elements::InputHash => b"i",
            Elements::InputOutpointsHash => b"1",
            Elements::InputPegin => b"i",
            Elements::InputPrevOutpoint => b"i",
            Elements::InputScriptHash => b"i",
            Elements::InputScriptSigHash => b"i",
            Elements::InputScriptSigsHash => b"1",
            Elements::InputScriptsHash => b"1",
            Elements::InputSequence => b"i",
            Elements::InputSequencesHash => b"1",
            Elements::InputUtxoHash => b"i",
            Elements::InputUtxosHash => b"1",
            Elements::InputsHash => b"1",
            Elements::InternalKey => b"1",
            Elements::IsOne16 => b"****22*22**22*22***22*22**22*22",
            Elements::IsOne32 => b"i",
            Elements::IsOne64 => b"l",
            Elements::IsOne8 => b"***22*22**22*22",
            Elements::IsZero16 => b"****22*22**22*22***22*22**22*22",
            Elements::IsZero32 => b"i",
            Elements::IsZero64 => b"l",
            Elements::IsZero8 => b"***22*22**22*22",
            Elements::Issuance => b"i",
            Elements::IssuanceAsset => b"i",
            Elements::IssuanceAssetAmount => b"i",
            Elements::IssuanceAssetAmountsHash => b"1",
            Elements::IssuanceAssetProof => b"i",
            Elements::IssuanceBlindingEntropyHash => b"1",
            Elements::IssuanceEntropy => b"i",
            Elements::IssuanceHash => b"i",
            Elements::IssuanceRangeProofsHash => b"1",
            Elements::IssuanceToken => b"i",
            Elements::IssuanceTokenAmount => b"i",
            Elements::IssuanceTokenAmountsHash => b"1",
            Elements::IssuanceTokenProof => b"i",
            Elements::IssuancesHash => b"1",
            Elements::LbtcAsset => b"1",
            Elements::Le16 => b"i",
            Elements::Le32 => b"l",
            Elements::Le64 => b"*ll",
            Elements::Le8 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftExtend16_32 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftExtend16_64 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftExtend1_16 => b"2",
            Elements::LeftExtend1_32 => b"2",
            Elements::LeftExtend1_64 => b"2",
            Elements::LeftExtend1_8 => b"2",
            Elements::LeftExtend32_64 => b"i",
            Elements::LeftExtend8_16 => b"***22*22**22*22",
            Elements::LeftExtend8_32 => b"***22*22**22*22",
            Elements::LeftExtend8_64 => b"***22*22**22*22",
            Elements::LeftPadHigh16_32 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftPadHigh16_64 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftPadHigh1_16 => b"2",
            Elements::LeftPadHigh1_32 => b"2",
            Elements::LeftPadHigh1_64 => b"2",
            Elements::LeftPadHigh1_8 => b"2",
            Elements::LeftPadHigh32_64 => b"i",
            Elements::LeftPadHigh8_16 => b"***22*22**22*22",
            Elements::LeftPadHigh8_32 => b"***22*22**22*22",
            Elements::LeftPadHigh8_64 => b"***22*22**22*22",
            Elements::LeftPadLow16_32 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftPadLow16_64 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftPadLow1_16 => b"2",
            Elements::LeftPadLow1_32 => b"2",
            Elements::LeftPadLow1_64 => b"2",
            Elements::LeftPadLow1_8 => b"2",
            Elements::LeftPadLow32_64 => b"i",
            Elements::LeftPadLow8_16 => b"***22*22**22*22",
            Elements::LeftPadLow8_32 => b"***22*22**22*22",
            Elements::LeftPadLow8_64 => b"***22*22**22*22",
            Elements::LeftRotate16 => b"***22*22****22*22**22*22***22*22**22*22",
            Elements::LeftRotate32 => b"****22*22**22*22i",
            Elements::LeftRotate64 => b"****22*22**22*22l",
            Elements::LeftRotate8 => b"***22*22***22*22**22*22",
            Elements::LeftShift16 => b"***22*22****22*22**22*22***22*22**22*22",
            Elements::LeftShift32 => b"****22*22**22*22i",
            Elements::LeftShift64 => b"****22*22**22*22l",
            Elements::LeftShift8 => b"***22*22***22*22**22*22",
            Elements::LeftShiftWith16 => b"*2***22*22****22*22**22*22***22*22**22*22",
            Elements::LeftShiftWith32 => b"*2****22*22**22*22i",
            Elements::LeftShiftWith64 => b"*2****22*22**22*22l",
            Elements::LeftShiftWith8 => b"*2***22*22***22*22**22*22",
            Elements::Leftmost16_1 => b"****22*22**22*22***22*22**22*22",
            Elements::Leftmost16_2 => b"****22*22**22*22***22*22**22*22",
            Elements::Leftmost16_4 => b"****22*22**22*22***22*22**22*22",
            Elements::Leftmost16_8 => b"****22*22**22*22***22*22**22*22",
            Elements::Leftmost32_1 => b"i",
            Elements::Leftmost32_16 => b"i",
            Elements::Leftmost32_2 => b"i",
            Elements::Leftmost32_4 => b"i",
            Elements::Leftmost32_8 => b"i",
            Elements::Leftmost64_1 => b"l",
            Elements::Leftmost64_16 => b"l",
            Elements::Leftmost64_2 => b"l",
            Elements::Leftmost64_32 => b"l",
            Elements::Leftmost64_4 => b"l",
            Elements::Leftmost64_8 => b"l",
            Elements::Leftmost8_1 => b"***22*22**22*22",
            Elements::Leftmost8_2 => b"***22*22**22*22",
            Elements::Leftmost8_4 => b"***22*22**22*22",
            Elements::LinearCombination1 => b"**h**hhhh",
            Elements::LinearVerify1 => b"***h*hhh*hh",
            Elements::LockTime => b"1",
            Elements::Low1 => b"1",
            Elements::Low16 => b"1",
            Elements::Low32 => b"1",
            Elements::Low64 => b"1",
            Elements::Low8 => b"1",
            Elements::Lt16 => b"i",
            Elements::Lt32 => b"l",
            Elements::Lt64 => b"*ll",
            Elements::Lt8 => b"****22*22**22*22***22*22**22*22",
            Elements::Maj1 => b"*2*22",
            Elements::Maj16 => b"*****22*22**22*22***22*22**22*22i",
            Elements::Maj32 => b"*il",
            Elements::Maj64 => b"*l*ll",
            Elements::Maj8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Elements::Max16 => b"i",
            Elements::Max32 => b"l",
            Elements::Max64 => b"*ll",
            Elements::Max8 => b"****22*22**22*22***22*22**22*22",
            Elements::Median16 => b"*****22*22**22*22***22*22**22*22i",
            Elements::Median32 => b"*il",
            Elements::Median64 => b"*l*ll",
            Elements::Median8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Elements::Min16 => b"i",
            Elements::Min32 => b"l",
            Elements::Min64 => b"*ll",
            Elements::Min8 => b"****22*22**22*22***22*22**22*22",
            Elements::Modulo16 => b"i",
            Elements::Modulo32 => b"l",
            Elements::Modulo64 => b"*ll",
            Elements::Modulo8 => b"****22*22**22*22***22*22**22*22",
            Elements::Multiply16 => b"i",
            Elements::Multiply32 => b"l",
            Elements::Multiply64 => b"*ll",
            Elements::Multiply8 => b"****22*22**22*22***22*22**22*22",
            Elements::Negate16 => b"****22*22**22*22***22*22**22*22",
            Elements::Negate32 => b"i",
            Elements::Negate64 => b"l",
            Elements::Negate8 => b"***22*22**22*22",
            Elements::NewIssuanceContract => b"i",
            Elements::NonceHash => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh+1+*2hh",
            Elements::NumInputs => b"1",
            Elements::NumOutputs => b"1",
            Elements::One16 => b"1",
            Elements::One32 => b"1",
            Elements::One64 => b"1",
            Elements::One8 => b"1",
            Elements::Or1 => b"*22",
            Elements::Or16 => b"i",
            Elements::Or32 => b"l",
            Elements::Or64 => b"*ll",
            Elements::Or8 => b"****22*22**22*22***22*22**22*22",
            Elements::OutpointHash => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh*+1h*hi",
            Elements::OutputAmount => b"i",
            Elements::OutputAmountsHash => b"1",
            Elements::OutputAsset => b"i",
            Elements::OutputHash => b"i",
            Elements::OutputIsFee => b"i",
            Elements::OutputNonce => b"i",
            Elements::OutputNoncesHash => b"1",
            Elements::OutputNullDatum => b"l",
            Elements::OutputRangeProof => b"i",
            Elements::OutputRangeProofsHash => b"1",
            Elements::OutputScriptHash => b"i",
            Elements::OutputScriptsHash => b"1",
            Elements::OutputSurjectionProof => b"i",
            Elements::OutputSurjectionProofsHash => b"1",
            Elements::OutputsHash => b"1",
            Elements::ParseLock => b"i",
            Elements::ParseSequence => b"i",
            Elements::PointVerify1 => b"***h*2hh*2h",
            Elements::ReissuanceBlinding => b"i",
            Elements::ReissuanceEntropy => b"i",
            Elements::RightExtend16_32 => b"****22*22**22*22***22*22**22*22",
            Elements::RightExtend16_64 => b"****22*22**22*22***22*22**22*22",
            Elements::RightExtend32_64 => b"i",
            Elements::RightExtend8_16 => b"***22*22**22*22",
            Elements::RightExtend8_32 => b"***22*22**22*22",
            Elements::RightExtend8_64 => b"***22*22**22*22",
            Elements::RightPadHigh16_32 => b"****22*22**22*22***22*22**22*22",
            Elements::RightPadHigh16_64 => b"****22*22**22*22***22*22**22*22",
            Elements::RightPadHigh1_16 => b"2",
            Elements::RightPadHigh1_32 => b"2",
            Elements::RightPadHigh1_64 => b"2",
            Elements::RightPadHigh1_8 => b"2",
            Elements::RightPadHigh32_64 => b"i",
            Elements::RightPadHigh8_16 => b"***22*22**22*22",
            Elements::RightPadHigh8_32 => b"***22*22**22*22",
            Elements::RightPadHigh8_64 => b"***22*22**22*22",
            Elements::RightPadLow16_32 => b"****22*22**22*22***22*22**22*22",
            Elements::RightPadLow16_64 => b"****22*22**22*22***22*22**22*22",
            Elements::RightPadLow1_16 => b"2",
            Elements::RightPadLow1_32 => b"2",
            Elements::RightPadLow1_64 => b"2",
            Elements::RightPadLow1_8 => b"2",
            Elements::RightPadLow32_64 => b"i",
            Elements::RightPadLow8_16 => b"***22*22**22*22",
            Elements::RightPadLow8_32 => b"***22*22**22*22",
            Elements::RightPadLow8_64 => b"***22*22**22*22",
            Elements::RightRotate16 => b"***22*22****22*22**22*22***22*22**22*22",
            Elements::RightRotate32 => b"****22*22**22*22i",
            Elements::RightRotate64 => b"****22*22**22*22l",
            Elements::RightRotate8 => b"***22*22***22*22**22*22",
            Elements::RightShift16 => b"***22*22****22*22**22*22***22*22**22*22",
            Elements::RightShift32 => b"****22*22**22*22i",
            Elements::RightShift64 => b"****22*22**22*22l",
            Elements::RightShift8 => b"***22*22***22*22**22*22",
            Elements::RightShiftWith16 => b"*2***22*22****22*22**22*22***22*22**22*22",
            Elements::RightShiftWith32 => b"*2****22*22**22*22i",
            Elements::RightShiftWith64 => b"*2****22*22**22*22l",
            Elements::RightShiftWith8 => b"*2***22*22***22*22**22*22",
            Elements::Rightmost16_1 => b"****22*22**22*22***22*22**22*22",
            Elements::Rightmost16_2 => b"****22*22**22*22***22*22**22*22",
            Elements::Rightmost16_4 => b"****22*22**22*22***22*22**22*22",
            Elements::Rightmost16_8 => b"****22*22**22*22***22*22**22*22",
            Elements::Rightmost32_1 => b"i",
            Elements::Rightmost32_16 => b"i",
            Elements::Rightmost32_2 => b"i",
            Elements::Rightmost32_4 => b"i",
            Elements::Rightmost32_8 => b"i",
            Elements::Rightmost64_1 => b"l",
            Elements::Rightmost64_16 => b"l",
            Elements::Rightmost64_2 => b"l",
            Elements::Rightmost64_32 => b"l",
            Elements::Rightmost64_4 => b"l",
            Elements::Rightmost64_8 => b"l",
            Elements::Rightmost8_1 => b"***22*22**22*22",
            Elements::Rightmost8_2 => b"***22*22**22*22",
            Elements::Rightmost8_4 => b"***22*22**22*22",
            Elements::ScalarAdd => b"*hh",
            Elements::ScalarInvert => b"h",
            Elements::ScalarIsZero => b"h",
            Elements::ScalarMultiply => b"*hh",
            Elements::ScalarMultiplyLambda => b"h",
            Elements::ScalarNegate => b"h",
            Elements::ScalarNormalize => b"h",
            Elements::ScalarSquare => b"h",
            Elements::Scale => b"*h**hhh",
            Elements::ScriptCMR => b"1",
            Elements::Sha256Block => b"*h*hh",
            Elements::Sha256Ctx8Add1 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh***22*22**22*22",
            Elements::Sha256Ctx8Add128 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh**hh*hh",
            Elements::Sha256Ctx8Add16 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh*ll",
            Elements::Sha256Ctx8Add2 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh****22*22**22*22***22*22**22*22",
            Elements::Sha256Ctx8Add256 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh***hh*hh**hh*hh",
            Elements::Sha256Ctx8Add32 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lhh",
            Elements::Sha256Ctx8Add4 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lhi",
            Elements::Sha256Ctx8Add512 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh****hh*hh**hh*hh***hh*hh**hh*hh",
            Elements::Sha256Ctx8Add64 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh*hh",
            Elements::Sha256Ctx8Add8 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lhl",
            Elements::Sha256Ctx8AddBuffer511 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh*+1***hh*hh**hh*hh*+1**hh*hh*+1*hh*+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22",
            Elements::Sha256Ctx8Finalize => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Sha256Ctx8Init => b"1",
            Elements::Sha256Iv => b"1",
            Elements::SigAllHash => b"1",
            Elements::Some1 => b"2",
            Elements::Some16 => b"****22*22**22*22***22*22**22*22",
            Elements::Some32 => b"i",
            Elements::Some64 => b"l",
            Elements::Some8 => b"***22*22**22*22",
            Elements::Subtract16 => b"i",
            Elements::Subtract32 => b"l",
            Elements::Subtract64 => b"*ll",
            Elements::Subtract8 => b"****22*22**22*22***22*22**22*22",
            Elements::Swu => b"h",
            Elements::TapEnvHash => b"1",
            Elements::TapdataInit => b"1",
            Elements::TapleafHash => b"1",
            Elements::TapleafVersion => b"1",
            Elements::Tappath => b"***22*22**22*22",
            Elements::TappathHash => b"1",
            Elements::TotalFee => b"h",
            Elements::TransactionId => b"1",
            Elements::TxHash => b"1",
            Elements::TxIsFinal => b"1",
            Elements::TxLockDistance => b"1",
            Elements::TxLockDuration => b"1",
            Elements::TxLockHeight => b"1",
            Elements::TxLockTime => b"1",
            Elements::Verify => b"2",
            Elements::Version => b"1",
            Elements::Xor1 => b"*22",
            Elements::Xor16 => b"i",
            Elements::Xor32 => b"l",
            Elements::Xor64 => b"*ll",
            Elements::Xor8 => b"****22*22**22*22***22*22**22*22",
            Elements::XorXor1 => b"*2*22",
            Elements::XorXor16 => b"*****22*22**22*22***22*22**22*22i",
            Elements::XorXor32 => b"*il",
            Elements::XorXor64 => b"*l*ll",
            Elements::XorXor8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
        };

        TypeName(name)
    }

    fn target_ty(&self) -> TypeName {
        let name: &'static [u8] = match self {
            Elements::Add16 => b"*2****22*22**22*22***22*22**22*22",
            Elements::Add32 => b"*2i",
            Elements::Add64 => b"*2l",
            Elements::Add8 => b"*2***22*22**22*22",
            Elements::All16 => b"2",
            Elements::All32 => b"2",
            Elements::All64 => b"2",
            Elements::All8 => b"2",
            Elements::And1 => b"2",
            Elements::And16 => b"****22*22**22*22***22*22**22*22",
            Elements::And32 => b"i",
            Elements::And64 => b"l",
            Elements::And8 => b"***22*22**22*22",
            Elements::AnnexHash => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::AssetAmountHash => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Bip0340Verify => b"1",
            Elements::BuildTapbranch => b"h",
            Elements::BuildTapleafSimplicity => b"h",
            Elements::BuildTaptweak => b"h",
            Elements::CalculateAsset => b"h",
            Elements::CalculateConfidentialToken => b"h",
            Elements::CalculateExplicitToken => b"h",
            Elements::CalculateIssuanceEntropy => b"h",
            Elements::Ch1 => b"2",
            Elements::Ch16 => b"****22*22**22*22***22*22**22*22",
            Elements::Ch32 => b"i",
            Elements::Ch64 => b"l",
            Elements::Ch8 => b"***22*22**22*22",
            Elements::CheckLockDistance => b"1",
            Elements::CheckLockDuration => b"1",
            Elements::CheckLockHeight => b"1",
            Elements::CheckLockTime => b"1",
            Elements::CheckSigVerify => b"1",
            Elements::Complement1 => b"2",
            Elements::Complement16 => b"****22*22**22*22***22*22**22*22",
            Elements::Complement32 => b"i",
            Elements::Complement64 => b"l",
            Elements::Complement8 => b"***22*22**22*22",
            Elements::CurrentAmount => b"*+*2hh+*2hl",
            Elements::CurrentAnnexHash => b"+1h",
            Elements::CurrentAsset => b"+*2hh",
            Elements::CurrentIndex => b"i",
            Elements::CurrentIssuanceAssetAmount => b"+1+*2hl",
            Elements::CurrentIssuanceAssetProof => b"h",
            Elements::CurrentIssuanceTokenAmount => b"+1+*2hl",
            Elements::CurrentIssuanceTokenProof => b"h",
            Elements::CurrentNewIssuanceContract => b"+1h",
            Elements::CurrentPegin => b"+1h",
            Elements::CurrentPrevOutpoint => b"*hi",
            Elements::CurrentReissuanceBlinding => b"+1h",
            Elements::CurrentReissuanceEntropy => b"+1h",
            Elements::CurrentScriptHash => b"h",
            Elements::CurrentScriptSigHash => b"h",
            Elements::CurrentSequence => b"i",
            Elements::Decompress => b"+1*hh",
            Elements::Decrement16 => b"*2****22*22**22*22***22*22**22*22",
            Elements::Decrement32 => b"*2i",
            Elements::Decrement64 => b"*2l",
            Elements::Decrement8 => b"*2***22*22**22*22",
            Elements::DivMod128_64 => b"*ll",
            Elements::DivMod16 => b"i",
            Elements::DivMod32 => b"l",
            Elements::DivMod64 => b"*ll",
            Elements::DivMod8 => b"****22*22**22*22***22*22**22*22",
            Elements::Divide16 => b"****22*22**22*22***22*22**22*22",
            Elements::Divide32 => b"i",
            Elements::Divide64 => b"l",
            Elements::Divide8 => b"***22*22**22*22",
            Elements::Divides16 => b"2",
            Elements::Divides32 => b"2",
            Elements::Divides64 => b"2",
            Elements::Divides8 => b"2",
            Elements::Eq1 => b"2",
            Elements::Eq16 => b"2",
            Elements::Eq256 => b"2",
            Elements::Eq32 => b"2",
            Elements::Eq64 => b"2",
            Elements::Eq8 => b"2",
            Elements::FeAdd => b"h",
            Elements::FeInvert => b"h",
            Elements::FeIsOdd => b"2",
            Elements::FeIsZero => b"2",
            Elements::FeMultiply => b"h",
            Elements::FeMultiplyBeta => b"h",
            Elements::FeNegate => b"h",
            Elements::FeNormalize => b"h",
            Elements::FeSquare => b"h",
            Elements::FeSquareRoot => b"+1h",
            Elements::FullAdd16 => b"*2****22*22**22*22***22*22**22*22",
            Elements::FullAdd32 => b"*2i",
            Elements::FullAdd64 => b"*2l",
            Elements::FullAdd8 => b"*2***22*22**22*22",
            Elements::FullDecrement16 => b"*2****22*22**22*22***22*22**22*22",
            Elements::FullDecrement32 => b"*2i",
            Elements::FullDecrement64 => b"*2l",
            Elements::FullDecrement8 => b"*2***22*22**22*22",
            Elements::FullIncrement16 => b"*2****22*22**22*22***22*22**22*22",
            Elements::FullIncrement32 => b"*2i",
            Elements::FullIncrement64 => b"*2l",
            Elements::FullIncrement8 => b"*2***22*22**22*22",
            Elements::FullLeftShift16_1 => b"*2****22*22**22*22***22*22**22*22",
            Elements::FullLeftShift16_2 => b"**22****22*22**22*22***22*22**22*22",
            Elements::FullLeftShift16_4 => b"***22*22****22*22**22*22***22*22**22*22",
            Elements::FullLeftShift16_8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Elements::FullLeftShift32_1 => b"*2i",
            Elements::FullLeftShift32_16 => b"*****22*22**22*22***22*22**22*22i",
            Elements::FullLeftShift32_2 => b"**22i",
            Elements::FullLeftShift32_4 => b"***22*22i",
            Elements::FullLeftShift32_8 => b"****22*22**22*22i",
            Elements::FullLeftShift64_1 => b"*2l",
            Elements::FullLeftShift64_16 => b"*****22*22**22*22***22*22**22*22l",
            Elements::FullLeftShift64_2 => b"**22l",
            Elements::FullLeftShift64_32 => b"*il",
            Elements::FullLeftShift64_4 => b"***22*22l",
            Elements::FullLeftShift64_8 => b"****22*22**22*22l",
            Elements::FullLeftShift8_1 => b"*2***22*22**22*22",
            Elements::FullLeftShift8_2 => b"**22***22*22**22*22",
            Elements::FullLeftShift8_4 => b"***22*22***22*22**22*22",
            Elements::FullMultiply16 => b"i",
            Elements::FullMultiply32 => b"l",
            Elements::FullMultiply64 => b"*ll",
            Elements::FullMultiply8 => b"****22*22**22*22***22*22**22*22",
            Elements::FullRightShift16_1 => b"*****22*22**22*22***22*22**22*222",
            Elements::FullRightShift16_2 => b"*****22*22**22*22***22*22**22*22*22",
            Elements::FullRightShift16_4 => b"*****22*22**22*22***22*22**22*22**22*22",
            Elements::FullRightShift16_8 => b"*****22*22**22*22***22*22**22*22***22*22**22*22",
            Elements::FullRightShift32_1 => b"*i2",
            Elements::FullRightShift32_16 => b"*i****22*22**22*22***22*22**22*22",
            Elements::FullRightShift32_2 => b"*i*22",
            Elements::FullRightShift32_4 => b"*i**22*22",
            Elements::FullRightShift32_8 => b"*i***22*22**22*22",
            Elements::FullRightShift64_1 => b"*l2",
            Elements::FullRightShift64_16 => b"*l****22*22**22*22***22*22**22*22",
            Elements::FullRightShift64_2 => b"*l*22",
            Elements::FullRightShift64_32 => b"*li",
            Elements::FullRightShift64_4 => b"*l**22*22",
            Elements::FullRightShift64_8 => b"*l***22*22**22*22",
            Elements::FullRightShift8_1 => b"****22*22**22*222",
            Elements::FullRightShift8_2 => b"****22*22**22*22*22",
            Elements::FullRightShift8_4 => b"****22*22**22*22**22*22",
            Elements::FullSubtract16 => b"*2****22*22**22*22***22*22**22*22",
            Elements::FullSubtract32 => b"*2i",
            Elements::FullSubtract64 => b"*2l",
            Elements::FullSubtract8 => b"*2***22*22**22*22",
            Elements::GeIsOnCurve => b"2",
            Elements::GeNegate => b"*hh",
            Elements::GejAdd => b"**hhh",
            Elements::GejDouble => b"**hhh",
            Elements::GejEquiv => b"2",
            Elements::GejGeAdd => b"**hhh",
            Elements::GejGeAddEx => b"*h**hhh",
            Elements::GejGeEquiv => b"2",
            Elements::GejInfinity => b"**hhh",
            Elements::GejIsInfinity => b"2",
            Elements::GejIsOnCurve => b"2",
            Elements::GejNegate => b"**hhh",
            Elements::GejNormalize => b"+1*hh",
            Elements::GejRescale => b"**hhh",
            Elements::GejXEquiv => b"2",
            Elements::GejYIsOdd => b"2",
            Elements::Generate => b"**hhh",
            Elements::GenesisBlockHash => b"h",
            Elements::HashToCurve => b"*hh",
            Elements::High1 => b"2",
            Elements::High16 => b"****22*22**22*22***22*22**22*22",
            Elements::High32 => b"i",
            Elements::High64 => b"l",
            Elements::High8 => b"***22*22**22*22",
            Elements::Increment16 => b"*2****22*22**22*22***22*22**22*22",
            Elements::Increment32 => b"*2i",
            Elements::Increment64 => b"*2l",
            Elements::Increment8 => b"*2***22*22**22*22",
            Elements::InputAmount => b"+1*+*2hh+*2hl",
            Elements::InputAmountsHash => b"h",
            Elements::InputAnnexHash => b"+1+1h",
            Elements::InputAnnexesHash => b"h",
            Elements::InputAsset => b"+1+*2hh",
            Elements::InputHash => b"+1h",
            Elements::InputOutpointsHash => b"h",
            Elements::InputPegin => b"+1+1h",
            Elements::InputPrevOutpoint => b"+1*hi",
            Elements::InputScriptHash => b"+1h",
            Elements::InputScriptSigHash => b"+1h",
            Elements::InputScriptSigsHash => b"h",
            Elements::InputScriptsHash => b"h",
            Elements::InputSequence => b"+1i",
            Elements::InputSequencesHash => b"h",
            Elements::InputUtxoHash => b"+1h",
            Elements::InputUtxosHash => b"h",
            Elements::InputsHash => b"h",
            Elements::InternalKey => b"h",
            Elements::IsOne16 => b"2",
            Elements::IsOne32 => b"2",
            Elements::IsOne64 => b"2",
            Elements::IsOne8 => b"2",
            Elements::IsZero16 => b"2",
            Elements::IsZero32 => b"2",
            Elements::IsZero64 => b"2",
            Elements::IsZero8 => b"2",
            Elements::Issuance => b"+1+12",
            Elements::IssuanceAsset => b"+1+1h",
            Elements::IssuanceAssetAmount => b"+1+1+*2hl",
            Elements::IssuanceAssetAmountsHash => b"h",
            Elements::IssuanceAssetProof => b"+1h",
            Elements::IssuanceBlindingEntropyHash => b"h",
            Elements::IssuanceEntropy => b"+1+1h",
            Elements::IssuanceHash => b"+1h",
            Elements::IssuanceRangeProofsHash => b"h",
            Elements::IssuanceToken => b"+1+1h",
            Elements::IssuanceTokenAmount => b"+1+1+*2hl",
            Elements::IssuanceTokenAmountsHash => b"h",
            Elements::IssuanceTokenProof => b"+1h",
            Elements::IssuancesHash => b"h",
            Elements::LbtcAsset => b"h",
            Elements::Le16 => b"2",
            Elements::Le32 => b"2",
            Elements::Le64 => b"2",
            Elements::Le8 => b"2",
            Elements::LeftExtend16_32 => b"i",
            Elements::LeftExtend16_64 => b"l",
            Elements::LeftExtend1_16 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftExtend1_32 => b"i",
            Elements::LeftExtend1_64 => b"l",
            Elements::LeftExtend1_8 => b"***22*22**22*22",
            Elements::LeftExtend32_64 => b"l",
            Elements::LeftExtend8_16 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftExtend8_32 => b"i",
            Elements::LeftExtend8_64 => b"l",
            Elements::LeftPadHigh16_32 => b"i",
            Elements::LeftPadHigh16_64 => b"l",
            Elements::LeftPadHigh1_16 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftPadHigh1_32 => b"i",
            Elements::LeftPadHigh1_64 => b"l",
            Elements::LeftPadHigh1_8 => b"***22*22**22*22",
            Elements::LeftPadHigh32_64 => b"l",
            Elements::LeftPadHigh8_16 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftPadHigh8_32 => b"i",
            Elements::LeftPadHigh8_64 => b"l",
            Elements::LeftPadLow16_32 => b"i",
            Elements::LeftPadLow16_64 => b"l",
            Elements::LeftPadLow1_16 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftPadLow1_32 => b"i",
            Elements::LeftPadLow1_64 => b"l",
            Elements::LeftPadLow1_8 => b"***22*22**22*22",
            Elements::LeftPadLow32_64 => b"l",
            Elements::LeftPadLow8_16 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftPadLow8_32 => b"i",
            Elements::LeftPadLow8_64 => b"l",
            Elements::LeftRotate16 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftRotate32 => b"i",
            Elements::LeftRotate64 => b"l",
            Elements::LeftRotate8 => b"***22*22**22*22",
            Elements::LeftShift16 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftShift32 => b"i",
            Elements::LeftShift64 => b"l",
            Elements::LeftShift8 => b"***22*22**22*22",
            Elements::LeftShiftWith16 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftShiftWith32 => b"i",
            Elements::LeftShiftWith64 => b"l",
            Elements::LeftShiftWith8 => b"***22*22**22*22",
            Elements::Leftmost16_1 => b"2",
            Elements::Leftmost16_2 => b"*22",
            Elements::Leftmost16_4 => b"**22*22",
            Elements::Leftmost16_8 => b"***22*22**22*22",
            Elements::Leftmost32_1 => b"2",
            Elements::Leftmost32_16 => b"****22*22**22*22***22*22**22*22",
            Elements::Leftmost32_2 => b"*22",
            Elements::Leftmost32_4 => b"**22*22",
            Elements::Leftmost32_8 => b"***22*22**22*22",
            Elements::Leftmost64_1 => b"2",
            Elements::Leftmost64_16 => b"****22*22**22*22***22*22**22*22",
            Elements::Leftmost64_2 => b"*22",
            Elements::Leftmost64_32 => b"i",
            Elements::Leftmost64_4 => b"**22*22",
            Elements::Leftmost64_8 => b"***22*22**22*22",
            Elements::Leftmost8_1 => b"2",
            Elements::Leftmost8_2 => b"*22",
            Elements::Leftmost8_4 => b"**22*22",
            Elements::LinearCombination1 => b"**hhh",
            Elements::LinearVerify1 => b"1",
            Elements::LockTime => b"i",
            Elements::Low1 => b"2",
            Elements::Low16 => b"****22*22**22*22***22*22**22*22",
            Elements::Low32 => b"i",
            Elements::Low64 => b"l",
            Elements::Low8 => b"***22*22**22*22",
            Elements::Lt16 => b"2",
            Elements::Lt32 => b"2",
            Elements::Lt64 => b"2",
            Elements::Lt8 => b"2",
            Elements::Maj1 => b"2",
            Elements::Maj16 => b"****22*22**22*22***22*22**22*22",
            Elements::Maj32 => b"i",
            Elements::Maj64 => b"l",
            Elements::Maj8 => b"***22*22**22*22",
            Elements::Max16 => b"****22*22**22*22***22*22**22*22",
            Elements::Max32 => b"i",
            Elements::Max64 => b"l",
            Elements::Max8 => b"***22*22**22*22",
            Elements::Median16 => b"****22*22**22*22***22*22**22*22",
            Elements::Median32 => b"i",
            Elements::Median64 => b"l",
            Elements::Median8 => b"***22*22**22*22",
            Elements::Min16 => b"****22*22**22*22***22*22**22*22",
            Elements::Min32 => b"i",
            Elements::Min64 => b"l",
            Elements::Min8 => b"***22*22**22*22",
            Elements::Modulo16 => b"****22*22**22*22***22*22**22*22",
            Elements::Modulo32 => b"i",
            Elements::Modulo64 => b"l",
            Elements::Modulo8 => b"***22*22**22*22",
            Elements::Multiply16 => b"i",
            Elements::Multiply32 => b"l",
            Elements::Multiply64 => b"*ll",
            Elements::Multiply8 => b"****22*22**22*22***22*22**22*22",
            Elements::Negate16 => b"*2****22*22**22*22***22*22**22*22",
            Elements::Negate32 => b"*2i",
            Elements::Negate64 => b"*2l",
            Elements::Negate8 => b"*2***22*22**22*22",
            Elements::NewIssuanceContract => b"+1+1h",
            Elements::NonceHash => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::NumInputs => b"i",
            Elements::NumOutputs => b"i",
            Elements::One16 => b"****22*22**22*22***22*22**22*22",
            Elements::One32 => b"i",
            Elements::One64 => b"l",
            Elements::One8 => b"***22*22**22*22",
            Elements::Or1 => b"2",
            Elements::Or16 => b"****22*22**22*22***22*22**22*22",
            Elements::Or32 => b"i",
            Elements::Or64 => b"l",
            Elements::Or8 => b"***22*22**22*22",
            Elements::OutpointHash => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::OutputAmount => b"+1*+*2hh+*2hl",
            Elements::OutputAmountsHash => b"h",
            Elements::OutputAsset => b"+1+*2hh",
            Elements::OutputHash => b"+1h",
            Elements::OutputIsFee => b"+12",
            Elements::OutputNonce => b"+1+1+*2hh",
            Elements::OutputNoncesHash => b"h",
            Elements::OutputNullDatum => b"+1+1+**22h+2**22*22",
            Elements::OutputRangeProof => b"+1h",
            Elements::OutputRangeProofsHash => b"h",
            Elements::OutputScriptHash => b"+1h",
            Elements::OutputScriptsHash => b"h",
            Elements::OutputSurjectionProof => b"+1h",
            Elements::OutputSurjectionProofsHash => b"h",
            Elements::OutputsHash => b"h",
            Elements::ParseLock => b"+ii",
            Elements::ParseSequence => b"+1+****22*22**22*22***22*22**22*22****22*22**22*22***22*22**22*22",
            Elements::PointVerify1 => b"1",
            Elements::ReissuanceBlinding => b"+1+1h",
            Elements::ReissuanceEntropy => b"+1+1h",
            Elements::RightExtend16_32 => b"i",
            Elements::RightExtend16_64 => b"l",
            Elements::RightExtend32_64 => b"l",
            Elements::RightExtend8_16 => b"****22*22**22*22***22*22**22*22",
            Elements::RightExtend8_32 => b"i",
            Elements::RightExtend8_64 => b"l",
            Elements::RightPadHigh16_32 => b"i",
            Elements::RightPadHigh16_64 => b"l",
            Elements::RightPadHigh1_16 => b"****22*22**22*22***22*22**22*22",
            Elements::RightPadHigh1_32 => b"i",
            Elements::RightPadHigh1_64 => b"l",
            Elements::RightPadHigh1_8 => b"***22*22**22*22",
            Elements::RightPadHigh32_64 => b"l",
            Elements::RightPadHigh8_16 => b"****22*22**22*22***22*22**22*22",
            Elements::RightPadHigh8_32 => b"i",
            Elements::RightPadHigh8_64 => b"l",
            Elements::RightPadLow16_32 => b"i",
            Elements::RightPadLow16_64 => b"l",
            Elements::RightPadLow1_16 => b"****22*22**22*22***22*22**22*22",
            Elements::RightPadLow1_32 => b"i",
            Elements::RightPadLow1_64 => b"l",
            Elements::RightPadLow1_8 => b"***22*22**22*22",
            Elements::RightPadLow32_64 => b"l",
            Elements::RightPadLow8_16 => b"****22*22**22*22***22*22**22*22",
            Elements::RightPadLow8_32 => b"i",
            Elements::RightPadLow8_64 => b"l",
            Elements::RightRotate16 => b"****22*22**22*22***22*22**22*22",
            Elements::RightRotate32 => b"i",
            Elements::RightRotate64 => b"l",
            Elements::RightRotate8 => b"***22*22**22*22",
            Elements::RightShift16 => b"****22*22**22*22***22*22**22*22",
            Elements::RightShift32 => b"i",
            Elements::RightShift64 => b"l",
            Elements::RightShift8 => b"***22*22**22*22",
            Elements::RightShiftWith16 => b"****22*22**22*22***22*22**22*22",
            Elements::RightShiftWith32 => b"i",
            Elements::RightShiftWith64 => b"l",
            Elements::RightShiftWith8 => b"***22*22**22*22",
            Elements::Rightmost16_1 => b"2",
            Elements::Rightmost16_2 => b"*22",
            Elements::Rightmost16_4 => b"**22*22",
            Elements::Rightmost16_8 => b"***22*22**22*22",
            Elements::Rightmost32_1 => b"2",
            Elements::Rightmost32_16 => b"****22*22**22*22***22*22**22*22",
            Elements::Rightmost32_2 => b"*22",
            Elements::Rightmost32_4 => b"**22*22",
            Elements::Rightmost32_8 => b"***22*22**22*22",
            Elements::Rightmost64_1 => b"2",
            Elements::Rightmost64_16 => b"****22*22**22*22***22*22**22*22",
            Elements::Rightmost64_2 => b"*22",
            Elements::Rightmost64_32 => b"i",
            Elements::Rightmost64_4 => b"**22*22",
            Elements::Rightmost64_8 => b"***22*22**22*22",
            Elements::Rightmost8_1 => b"2",
            Elements::Rightmost8_2 => b"*22",
            Elements::Rightmost8_4 => b"**22*22",
            Elements::ScalarAdd => b"h",
            Elements::ScalarInvert => b"h",
            Elements::ScalarIsZero => b"2",
            Elements::ScalarMultiply => b"h",
            Elements::ScalarMultiplyLambda => b"h",
            Elements::ScalarNegate => b"h",
            Elements::ScalarNormalize => b"h",
            Elements::ScalarSquare => b"h",
            Elements::Scale => b"**hhh",
            Elements::ScriptCMR => b"h",
            Elements::Sha256Block => b"h",
            Elements::Sha256Ctx8Add1 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Sha256Ctx8Add128 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Sha256Ctx8Add16 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Sha256Ctx8Add2 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Sha256Ctx8Add256 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Sha256Ctx8Add32 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Sha256Ctx8Add4 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Sha256Ctx8Add512 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Sha256Ctx8Add64 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Sha256Ctx8Add8 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Sha256Ctx8AddBuffer511 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Sha256Ctx8Finalize => b"h",
            Elements::Sha256Ctx8Init => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Sha256Iv => b"h",
            Elements::SigAllHash => b"h",
            Elements::Some1 => b"2",
            Elements::Some16 => b"2",
            Elements::Some32 => b"2",
            Elements::Some64 => b"2",
            Elements::Some8 => b"2",
            Elements::Subtract16 => b"*2****22*22**22*22***22*22**22*22",
            Elements::Subtract32 => b"*2i",
            Elements::Subtract64 => b"*2l",
            Elements::Subtract8 => b"*2***22*22**22*22",
            Elements::Swu => b"*hh",
            Elements::TapEnvHash => b"h",
            Elements::TapdataInit => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::TapleafHash => b"h",
            Elements::TapleafVersion => b"***22*22**22*22",
            Elements::Tappath => b"+1h",
            Elements::TappathHash => b"h",
            Elements::TotalFee => b"l",
            Elements::TransactionId => b"h",
            Elements::TxHash => b"h",
            Elements::TxIsFinal => b"2",
            Elements::TxLockDistance => b"****22*22**22*22***22*22**22*22",
            Elements::TxLockDuration => b"****22*22**22*22***22*22**22*22",
            Elements::TxLockHeight => b"i",
            Elements::TxLockTime => b"i",
            Elements::Verify => b"1",
            Elements::Version => b"i",
            Elements::Xor1 => b"2",
            Elements::Xor16 => b"****22*22**22*22***22*22**22*22",
            Elements::Xor32 => b"i",
            Elements::Xor64 => b"l",
            Elements::Xor8 => b"***22*22**22*22",
            Elements::XorXor1 => b"2",
            Elements::XorXor16 => b"****22*22**22*22***22*22**22*22",
            Elements::XorXor32 => b"i",
            Elements::XorXor64 => b"l",
            Elements::XorXor8 => b"***22*22**22*22",
        };

        TypeName(name)
    }

    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> std::io::Result<usize> {
        let (n, len) = match self {
            Elements::Verify => (0, 3),
            Elements::Low1 => (8, 6),
            Elements::Low8 => (37, 8),
            Elements::Low16 => (304, 11),
            Elements::Low32 => (305, 11),
            Elements::Low64 => (306, 11),
            Elements::High1 => (10, 6),
            Elements::High8 => (45, 8),
            Elements::High16 => (368, 11),
            Elements::High32 => (369, 11),
            Elements::High64 => (370, 11),
            Elements::Complement1 => (96, 9),
            Elements::Complement8 => (389, 11),
            Elements::Complement16 => (3120, 14),
            Elements::Complement32 => (3121, 14),
            Elements::Complement64 => (3122, 14),
            Elements::And1 => (98, 9),
            Elements::And8 => (397, 11),
            Elements::And16 => (3184, 14),
            Elements::And32 => (3185, 14),
            Elements::And64 => (3186, 14),
            Elements::Or1 => (100, 9),
            Elements::Or8 => (405, 11),
            Elements::Or16 => (3248, 14),
            Elements::Or32 => (3249, 14),
            Elements::Or64 => (3250, 14),
            Elements::Xor1 => (102, 9),
            Elements::Xor8 => (413, 11),
            Elements::Xor16 => (3312, 14),
            Elements::Xor32 => (3313, 14),
            Elements::Xor64 => (3314, 14),
            Elements::Maj1 => (208, 10),
            Elements::Maj8 => (837, 12),
            Elements::Maj16 => (6704, 15),
            Elements::Maj32 => (6705, 15),
            Elements::Maj64 => (6706, 15),
            Elements::XorXor1 => (210, 10),
            Elements::XorXor8 => (845, 12),
            Elements::XorXor16 => (6768, 15),
            Elements::XorXor32 => (6769, 15),
            Elements::XorXor64 => (6770, 15),
            Elements::Ch1 => (212, 10),
            Elements::Ch8 => (853, 12),
            Elements::Ch16 => (6832, 15),
            Elements::Ch32 => (6833, 15),
            Elements::Ch64 => (6834, 15),
            Elements::Some1 => (214, 10),
            Elements::Some8 => (861, 12),
            Elements::Some16 => (6896, 15),
            Elements::Some32 => (6897, 15),
            Elements::Some64 => (6898, 15),
            Elements::All8 => (869, 12),
            Elements::All16 => (6960, 15),
            Elements::All32 => (6961, 15),
            Elements::All64 => (6962, 15),
            Elements::Eq1 => (218, 10),
            Elements::Eq8 => (877, 12),
            Elements::Eq16 => (7024, 15),
            Elements::Eq32 => (7025, 15),
            Elements::Eq64 => (7026, 15),
            Elements::Eq256 => (14056, 16),
            Elements::FullLeftShift8_1 => (1765, 13),
            Elements::FullLeftShift16_1 => (14128, 16),
            Elements::FullLeftShift32_1 => (14129, 16),
            Elements::FullLeftShift64_1 => (14130, 16),
            Elements::FullLeftShift8_2 => (7076, 15),
            Elements::FullLeftShift16_2 => (7077, 15),
            Elements::FullLeftShift32_2 => (56624, 18),
            Elements::FullLeftShift64_2 => (56625, 18),
            Elements::FullLeftShift8_4 => (1770, 13),
            Elements::FullLeftShift16_4 => (7084, 15),
            Elements::FullLeftShift32_4 => (7085, 15),
            Elements::FullLeftShift64_4 => (56688, 18),
            Elements::FullLeftShift16_8 => (14176, 16),
            Elements::FullLeftShift32_8 => (56708, 18),
            Elements::FullLeftShift64_8 => (56709, 18),
            Elements::FullLeftShift32_16 => (14178, 16),
            Elements::FullLeftShift64_16 => (56716, 18),
            Elements::FullLeftShift64_32 => (14180, 16),
            Elements::FullRightShift8_1 => (1781, 13),
            Elements::FullRightShift16_1 => (14256, 16),
            Elements::FullRightShift32_1 => (14257, 16),
            Elements::FullRightShift64_1 => (14258, 16),
            Elements::FullRightShift8_2 => (7140, 15),
            Elements::FullRightShift16_2 => (7141, 15),
            Elements::FullRightShift32_2 => (57136, 18),
            Elements::FullRightShift64_2 => (57137, 18),
            Elements::FullRightShift8_4 => (1786, 13),
            Elements::FullRightShift16_4 => (7148, 15),
            Elements::FullRightShift32_4 => (7149, 15),
            Elements::FullRightShift64_4 => (57200, 18),
            Elements::FullRightShift16_8 => (14304, 16),
            Elements::FullRightShift32_8 => (57220, 18),
            Elements::FullRightShift64_8 => (57221, 18),
            Elements::FullRightShift32_16 => (14306, 16),
            Elements::FullRightShift64_16 => (57228, 18),
            Elements::FullRightShift64_32 => (14308, 16),
            Elements::Leftmost8_1 => (28677, 17),
            Elements::Leftmost16_1 => (229424, 20),
            Elements::Leftmost32_1 => (229425, 20),
            Elements::Leftmost64_1 => (229426, 20),
            Elements::Leftmost8_2 => (114724, 19),
            Elements::Leftmost16_2 => (114725, 19),
            Elements::Leftmost32_2 => (917808, 22),
            Elements::Leftmost64_2 => (917809, 22),
            Elements::Leftmost8_4 => (28682, 17),
            Elements::Leftmost16_4 => (114732, 19),
            Elements::Leftmost32_4 => (114733, 19),
            Elements::Leftmost64_4 => (917872, 22),
            Elements::Leftmost16_8 => (229472, 20),
            Elements::Leftmost32_8 => (917892, 22),
            Elements::Leftmost64_8 => (917893, 22),
            Elements::Leftmost32_16 => (229474, 20),
            Elements::Leftmost64_16 => (917900, 22),
            Elements::Leftmost64_32 => (229476, 20),
            Elements::Rightmost8_1 => (28693, 17),
            Elements::Rightmost16_1 => (229552, 20),
            Elements::Rightmost32_1 => (229553, 20),
            Elements::Rightmost64_1 => (229554, 20),
            Elements::Rightmost8_2 => (114788, 19),
            Elements::Rightmost16_2 => (114789, 19),
            Elements::Rightmost32_2 => (918320, 22),
            Elements::Rightmost64_2 => (918321, 22),
            Elements::Rightmost8_4 => (28698, 17),
            Elements::Rightmost16_4 => (114796, 19),
            Elements::Rightmost32_4 => (114797, 19),
            Elements::Rightmost64_4 => (918384, 22),
            Elements::Rightmost16_8 => (229600, 20),
            Elements::Rightmost32_8 => (918404, 22),
            Elements::Rightmost64_8 => (918405, 22),
            Elements::Rightmost32_16 => (229602, 20),
            Elements::Rightmost64_16 => (918412, 22),
            Elements::Rightmost64_32 => (229604, 20),
            Elements::LeftPadLow1_8 => (28709, 17),
            Elements::LeftPadLow1_16 => (229680, 20),
            Elements::LeftPadLow1_32 => (229681, 20),
            Elements::LeftPadLow1_64 => (229682, 20),
            Elements::LeftPadLow8_16 => (229728, 20),
            Elements::LeftPadLow8_32 => (918916, 22),
            Elements::LeftPadLow8_64 => (918917, 22),
            Elements::LeftPadLow16_32 => (229730, 20),
            Elements::LeftPadLow16_64 => (918924, 22),
            Elements::LeftPadLow32_64 => (229732, 20),
            Elements::LeftPadHigh1_8 => (28725, 17),
            Elements::LeftPadHigh1_16 => (229808, 20),
            Elements::LeftPadHigh1_32 => (229809, 20),
            Elements::LeftPadHigh1_64 => (229810, 20),
            Elements::LeftPadHigh8_16 => (229856, 20),
            Elements::LeftPadHigh8_32 => (919428, 22),
            Elements::LeftPadHigh8_64 => (919429, 22),
            Elements::LeftPadHigh16_32 => (229858, 20),
            Elements::LeftPadHigh16_64 => (919436, 22),
            Elements::LeftPadHigh32_64 => (229860, 20),
            Elements::LeftExtend1_8 => (28741, 17),
            Elements::LeftExtend1_16 => (229936, 20),
            Elements::LeftExtend1_32 => (229937, 20),
            Elements::LeftExtend1_64 => (229938, 20),
            Elements::LeftExtend8_16 => (229984, 20),
            Elements::LeftExtend8_32 => (919940, 22),
            Elements::LeftExtend8_64 => (919941, 22),
            Elements::LeftExtend16_32 => (229986, 20),
            Elements::LeftExtend16_64 => (919948, 22),
            Elements::LeftExtend32_64 => (229988, 20),
            Elements::RightPadLow1_8 => (28757, 17),
            Elements::RightPadLow1_16 => (230064, 20),
            Elements::RightPadLow1_32 => (230065, 20),
            Elements::RightPadLow1_64 => (230066, 20),
            Elements::RightPadLow8_16 => (230112, 20),
            Elements::RightPadLow8_32 => (920452, 22),
            Elements::RightPadLow8_64 => (920453, 22),
            Elements::RightPadLow16_32 => (230114, 20),
            Elements::RightPadLow16_64 => (920460, 22),
            Elements::RightPadLow32_64 => (230116, 20),
            Elements::RightPadHigh1_8 => (28773, 17),
            Elements::RightPadHigh1_16 => (230192, 20),
            Elements::RightPadHigh1_32 => (230193, 20),
            Elements::RightPadHigh1_64 => (230194, 20),
            Elements::RightPadHigh8_16 => (230240, 20),
            Elements::RightPadHigh8_32 => (920964, 22),
            Elements::RightPadHigh8_64 => (920965, 22),
            Elements::RightPadHigh16_32 => (230242, 20),
            Elements::RightPadHigh16_64 => (920972, 22),
            Elements::RightPadHigh32_64 => (230244, 20),
            Elements::RightExtend8_16 => (230368, 20),
            Elements::RightExtend8_32 => (921476, 22),
            Elements::RightExtend8_64 => (921477, 22),
            Elements::RightExtend16_32 => (230370, 20),
            Elements::RightExtend16_64 => (921484, 22),
            Elements::RightExtend32_64 => (230372, 20),
            Elements::LeftShiftWith8 => (14405, 16),
            Elements::LeftShiftWith16 => (115248, 19),
            Elements::LeftShiftWith32 => (115249, 19),
            Elements::LeftShiftWith64 => (115250, 19),
            Elements::RightShiftWith8 => (14413, 16),
            Elements::RightShiftWith16 => (115312, 19),
            Elements::RightShiftWith32 => (115313, 19),
            Elements::RightShiftWith64 => (115314, 19),
            Elements::LeftShift8 => (14421, 16),
            Elements::LeftShift16 => (115376, 19),
            Elements::LeftShift32 => (115377, 19),
            Elements::LeftShift64 => (115378, 19),
            Elements::RightShift8 => (14429, 16),
            Elements::RightShift16 => (115440, 19),
            Elements::RightShift32 => (115441, 19),
            Elements::RightShift64 => (115442, 19),
            Elements::LeftRotate8 => (14437, 16),
            Elements::LeftRotate16 => (115504, 19),
            Elements::LeftRotate32 => (115505, 19),
            Elements::LeftRotate64 => (115506, 19),
            Elements::RightRotate8 => (14445, 16),
            Elements::RightRotate16 => (115568, 19),
            Elements::RightRotate32 => (115569, 19),
            Elements::RightRotate64 => (115570, 19),
            Elements::One8 => (69, 8),
            Elements::One16 => (560, 11),
            Elements::One32 => (561, 11),
            Elements::One64 => (562, 11),
            Elements::FullAdd8 => (293, 10),
            Elements::FullAdd16 => (2352, 13),
            Elements::FullAdd32 => (2353, 13),
            Elements::FullAdd64 => (2354, 13),
            Elements::Add8 => (301, 10),
            Elements::Add16 => (2416, 13),
            Elements::Add32 => (2417, 13),
            Elements::Add64 => (2418, 13),
            Elements::FullIncrement8 => (2437, 13),
            Elements::FullIncrement16 => (19504, 16),
            Elements::FullIncrement32 => (19505, 16),
            Elements::FullIncrement64 => (19506, 16),
            Elements::Increment8 => (2445, 13),
            Elements::Increment16 => (19568, 16),
            Elements::Increment32 => (19569, 16),
            Elements::Increment64 => (19570, 16),
            Elements::FullSubtract8 => (2461, 13),
            Elements::FullSubtract16 => (19696, 16),
            Elements::FullSubtract32 => (19697, 16),
            Elements::FullSubtract64 => (19698, 16),
            Elements::Subtract8 => (4933, 14),
            Elements::Subtract16 => (39472, 17),
            Elements::Subtract32 => (39473, 17),
            Elements::Subtract64 => (39474, 17),
            Elements::Negate8 => (4941, 14),
            Elements::Negate16 => (39536, 17),
            Elements::Negate32 => (39537, 17),
            Elements::Negate64 => (39538, 17),
            Elements::FullDecrement8 => (4949, 14),
            Elements::FullDecrement16 => (39600, 17),
            Elements::FullDecrement32 => (39601, 17),
            Elements::FullDecrement64 => (39602, 17),
            Elements::Decrement8 => (4957, 14),
            Elements::Decrement16 => (39664, 17),
            Elements::Decrement32 => (39665, 17),
            Elements::Decrement64 => (39666, 17),
            Elements::FullMultiply8 => (4965, 14),
            Elements::FullMultiply16 => (39728, 17),
            Elements::FullMultiply32 => (39729, 17),
            Elements::FullMultiply64 => (39730, 17),
            Elements::Multiply8 => (4973, 14),
            Elements::Multiply16 => (39792, 17),
            Elements::Multiply32 => (39793, 17),
            Elements::Multiply64 => (39794, 17),
            Elements::IsZero8 => (4981, 14),
            Elements::IsZero16 => (39856, 17),
            Elements::IsZero32 => (39857, 17),
            Elements::IsZero64 => (39858, 17),
            Elements::IsOne8 => (4989, 14),
            Elements::IsOne16 => (39920, 17),
            Elements::IsOne32 => (39921, 17),
            Elements::IsOne64 => (39922, 17),
            Elements::Le8 => (79877, 18),
            Elements::Le16 => (639024, 21),
            Elements::Le32 => (639025, 21),
            Elements::Le64 => (639026, 21),
            Elements::Lt8 => (79885, 18),
            Elements::Lt16 => (639088, 21),
            Elements::Lt32 => (639089, 21),
            Elements::Lt64 => (639090, 21),
            Elements::Min8 => (79893, 18),
            Elements::Min16 => (639152, 21),
            Elements::Min32 => (639153, 21),
            Elements::Min64 => (639154, 21),
            Elements::Max8 => (79901, 18),
            Elements::Max16 => (639216, 21),
            Elements::Max32 => (639217, 21),
            Elements::Max64 => (639218, 21),
            Elements::Median8 => (79909, 18),
            Elements::Median16 => (639280, 21),
            Elements::Median32 => (639281, 21),
            Elements::Median64 => (639282, 21),
            Elements::DivMod128_64 => (639346, 21),
            Elements::DivMod8 => (79925, 18),
            Elements::DivMod16 => (639408, 21),
            Elements::DivMod32 => (639409, 21),
            Elements::DivMod64 => (639410, 21),
            Elements::Divide8 => (79933, 18),
            Elements::Divide16 => (639472, 21),
            Elements::Divide32 => (639473, 21),
            Elements::Divide64 => (639474, 21),
            Elements::Modulo8 => (79941, 18),
            Elements::Modulo16 => (639536, 21),
            Elements::Modulo32 => (639537, 21),
            Elements::Modulo64 => (639538, 21),
            Elements::Divides8 => (79949, 18),
            Elements::Divides16 => (639600, 21),
            Elements::Divides32 => (639601, 21),
            Elements::Divides64 => (639602, 21),
            Elements::Sha256Block => (20, 6),
            Elements::Sha256Iv => (84, 8),
            Elements::Sha256Ctx8Add1 => (170, 9),
            Elements::Sha256Ctx8Add2 => (684, 11),
            Elements::Sha256Ctx8Add4 => (685, 11),
            Elements::Sha256Ctx8Add8 => (5488, 14),
            Elements::Sha256Ctx8Add16 => (5489, 14),
            Elements::Sha256Ctx8Add32 => (5490, 14),
            Elements::Sha256Ctx8Add64 => (5491, 14),
            Elements::Sha256Ctx8Add128 => (10984, 15),
            Elements::Sha256Ctx8Add256 => (10985, 15),
            Elements::Sha256Ctx8Add512 => (10986, 15),
            Elements::Sha256Ctx8AddBuffer511 => (688, 11),
            Elements::Sha256Ctx8Finalize => (689, 11),
            Elements::Sha256Ctx8Init => (690, 11),
            Elements::PointVerify1 => (192, 9),
            Elements::Decompress => (388, 10),
            Elements::LinearVerify1 => (778, 11),
            Elements::LinearCombination1 => (6240, 14),
            Elements::Scale => (3121, 13),
            Elements::Generate => (3122, 13),
            Elements::GejInfinity => (3123, 13),
            Elements::GejNormalize => (6248, 14),
            Elements::GejNegate => (6249, 14),
            Elements::GeNegate => (6250, 14),
            Elements::GejDouble => (6251, 14),
            Elements::GejAdd => (6252, 14),
            Elements::GejGeAddEx => (6253, 14),
            Elements::GejGeAdd => (6254, 14),
            Elements::GejRescale => (6255, 14),
            Elements::GejIsInfinity => (100096, 18),
            Elements::GejEquiv => (100097, 18),
            Elements::GejGeEquiv => (100098, 18),
            Elements::GejXEquiv => (100099, 18),
            Elements::GejYIsOdd => (100100, 18),
            Elements::GejIsOnCurve => (100101, 18),
            Elements::GeIsOnCurve => (100102, 18),
            Elements::ScalarNormalize => (100103, 18),
            Elements::ScalarNegate => (100104, 18),
            Elements::ScalarAdd => (100105, 18),
            Elements::ScalarSquare => (100106, 18),
            Elements::ScalarMultiply => (100107, 18),
            Elements::ScalarMultiplyLambda => (100108, 18),
            Elements::ScalarInvert => (100109, 18),
            Elements::ScalarIsZero => (100110, 18),
            Elements::FeNormalize => (200227, 19),
            Elements::FeNegate => (200228, 19),
            Elements::FeAdd => (200229, 19),
            Elements::FeSquare => (200230, 19),
            Elements::FeMultiply => (200231, 19),
            Elements::FeMultiplyBeta => (200232, 19),
            Elements::FeInvert => (200233, 19),
            Elements::FeSquareRoot => (200234, 19),
            Elements::FeIsZero => (200235, 19),
            Elements::FeIsOdd => (200236, 19),
            Elements::HashToCurve => (200238, 19),
            Elements::Swu => (200239, 19),
            Elements::CheckSigVerify => (98, 8),
            Elements::Bip0340Verify => (396, 10),
            Elements::ParseLock => (102, 8),
            Elements::ParseSequence => (412, 10),
            Elements::TapdataInit => (413, 10),
            Elements::SigAllHash => (4, 3),
            Elements::TxHash => (20, 5),
            Elements::TapEnvHash => (21, 5),
            Elements::OutputsHash => (176, 8),
            Elements::InputsHash => (177, 8),
            Elements::IssuancesHash => (178, 8),
            Elements::InputUtxosHash => (179, 8),
            Elements::OutputHash => (360, 9),
            Elements::OutputAmountsHash => (361, 9),
            Elements::OutputScriptsHash => (362, 9),
            Elements::OutputNoncesHash => (363, 9),
            Elements::OutputRangeProofsHash => (364, 9),
            Elements::OutputSurjectionProofsHash => (365, 9),
            Elements::InputHash => (366, 9),
            Elements::InputOutpointsHash => (367, 9),
            Elements::InputSequencesHash => (5888, 13),
            Elements::InputAnnexesHash => (5889, 13),
            Elements::InputScriptSigsHash => (5890, 13),
            Elements::IssuanceHash => (5891, 13),
            Elements::IssuanceAssetAmountsHash => (5892, 13),
            Elements::IssuanceTokenAmountsHash => (5893, 13),
            Elements::IssuanceRangeProofsHash => (5894, 13),
            Elements::IssuanceBlindingEntropyHash => (5895, 13),
            Elements::InputUtxoHash => (5896, 13),
            Elements::InputAmountsHash => (5897, 13),
            Elements::InputScriptsHash => (5898, 13),
            Elements::TapleafHash => (5899, 13),
            Elements::TappathHash => (5900, 13),
            Elements::OutpointHash => (5901, 13),
            Elements::AssetAmountHash => (5902, 13),
            Elements::NonceHash => (5903, 13),
            Elements::AnnexHash => (11808, 14),
            Elements::BuildTapleafSimplicity => (11809, 14),
            Elements::BuildTapbranch => (11810, 14),
            Elements::BuildTaptweak => (11811, 14),
            Elements::CheckLockHeight => (24, 5),
            Elements::CheckLockTime => (100, 7),
            Elements::CheckLockDistance => (101, 7),
            Elements::CheckLockDuration => (816, 10),
            Elements::TxLockHeight => (817, 10),
            Elements::TxLockTime => (818, 10),
            Elements::TxLockDistance => (819, 10),
            Elements::TxLockDuration => (1640, 11),
            Elements::TxIsFinal => (1641, 11),
            Elements::Issuance => (26, 5),
            Elements::IssuanceAsset => (108, 7),
            Elements::IssuanceToken => (109, 7),
            Elements::IssuanceEntropy => (880, 10),
            Elements::CalculateIssuanceEntropy => (881, 10),
            Elements::CalculateAsset => (882, 10),
            Elements::CalculateExplicitToken => (883, 10),
            Elements::CalculateConfidentialToken => (1768, 11),
            Elements::LbtcAsset => (1769, 11),
            Elements::ScriptCMR => (224, 8),
            Elements::InternalKey => (900, 10),
            Elements::CurrentIndex => (901, 10),
            Elements::NumInputs => (7216, 13),
            Elements::NumOutputs => (7217, 13),
            Elements::LockTime => (7218, 13),
            Elements::OutputAsset => (7219, 13),
            Elements::OutputAmount => (14440, 14),
            Elements::OutputNonce => (14441, 14),
            Elements::OutputScriptHash => (14442, 14),
            Elements::OutputNullDatum => (14443, 14),
            Elements::OutputIsFee => (14444, 14),
            Elements::OutputSurjectionProof => (14445, 14),
            Elements::OutputRangeProof => (14446, 14),
            Elements::TotalFee => (14447, 14),
            Elements::CurrentPegin => (231168, 18),
            Elements::CurrentPrevOutpoint => (231169, 18),
            Elements::CurrentAsset => (231170, 18),
            Elements::CurrentAmount => (231171, 18),
            Elements::CurrentScriptHash => (231172, 18),
            Elements::CurrentSequence => (231173, 18),
            Elements::CurrentAnnexHash => (231174, 18),
            Elements::CurrentScriptSigHash => (231175, 18),
            Elements::CurrentReissuanceBlinding => (231176, 18),
            Elements::CurrentNewIssuanceContract => (231177, 18),
            Elements::CurrentReissuanceEntropy => (231178, 18),
            Elements::CurrentIssuanceAssetAmount => (231179, 18),
            Elements::CurrentIssuanceTokenAmount => (231180, 18),
            Elements::CurrentIssuanceAssetProof => (231181, 18),
            Elements::CurrentIssuanceTokenProof => (231182, 18),
            Elements::InputPegin => (231183, 18),
            Elements::InputPrevOutpoint => (462368, 19),
            Elements::InputAsset => (462369, 19),
            Elements::InputAmount => (462370, 19),
            Elements::InputScriptHash => (462371, 19),
            Elements::InputSequence => (462372, 19),
            Elements::InputAnnexHash => (462373, 19),
            Elements::InputScriptSigHash => (462374, 19),
            Elements::ReissuanceBlinding => (462375, 19),
            Elements::NewIssuanceContract => (462376, 19),
            Elements::ReissuanceEntropy => (462377, 19),
            Elements::IssuanceAssetAmount => (462378, 19),
            Elements::IssuanceTokenAmount => (462379, 19),
            Elements::IssuanceAssetProof => (462380, 19),
            Elements::IssuanceTokenProof => (462381, 19),
            Elements::TapleafVersion => (462382, 19),
            Elements::Tappath => (462383, 19),
            Elements::Version => (462384, 19),
            Elements::GenesisBlockHash => (462385, 19),
            Elements::TransactionId => (462386, 19),
        };

        w.write_bits_be(n, len)
    }

    fn decode<I: Iterator<Item = u8>>(bits: &mut BitIter<I>) -> Result<Self, decode::Error> {
        decode_bits!(bits, {
            0 => {
                0 => {
                    0 => {Elements::Verify},
                    1 => {
                        0 => {
                            0 => {
                                0 => {Elements::Low1},
                                1 => {
                                    0 => {
                                        0 => {},
                                        1 => {Elements::Low8}
                                    },
                                    1 => {
                                        0 => {
                                            0 => {
                                                0 => {
                                                    0 => {Elements::Low16},
                                                    1 => {Elements::Low32}
                                                },
                                                1 => {
                                                    0 => {Elements::Low64},
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
                                0 => {Elements::High1},
                                1 => {
                                    0 => {
                                        0 => {},
                                        1 => {Elements::High8}
                                    },
                                    1 => {
                                        0 => {
                                            0 => {
                                                0 => {
                                                    0 => {Elements::High16},
                                                    1 => {Elements::High32}
                                                },
                                                1 => {
                                                    0 => {Elements::High64},
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
                                            0 => {Elements::Complement1},
                                            1 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {Elements::Complement8}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Elements::Complement16},
                                                                1 => {Elements::Complement32}
                                                            },
                                                            1 => {
                                                                0 => {Elements::Complement64},
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
                                            0 => {Elements::And1},
                                            1 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {Elements::And8}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Elements::And16},
                                                                1 => {Elements::And32}
                                                            },
                                                            1 => {
                                                                0 => {Elements::And64},
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
                                            0 => {Elements::Or1},
                                            1 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {Elements::Or8}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Elements::Or16},
                                                                1 => {Elements::Or32}
                                                            },
                                                            1 => {
                                                                0 => {Elements::Or64},
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
                                            0 => {Elements::Xor1},
                                            1 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {Elements::Xor8}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Elements::Xor16},
                                                                1 => {Elements::Xor32}
                                                            },
                                                            1 => {
                                                                0 => {Elements::Xor64},
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
                                                0 => {Elements::Maj1},
                                                1 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {Elements::Maj8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Elements::Maj16},
                                                                    1 => {Elements::Maj32}
                                                                },
                                                                1 => {
                                                                    0 => {Elements::Maj64},
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
                                                0 => {Elements::XorXor1},
                                                1 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {Elements::XorXor8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Elements::XorXor16},
                                                                    1 => {Elements::XorXor32}
                                                                },
                                                                1 => {
                                                                    0 => {Elements::XorXor64},
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
                                                0 => {Elements::Ch1},
                                                1 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {Elements::Ch8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Elements::Ch16},
                                                                    1 => {Elements::Ch32}
                                                                },
                                                                1 => {
                                                                    0 => {Elements::Ch64},
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
                                                0 => {Elements::Some1},
                                                1 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {Elements::Some8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Elements::Some16},
                                                                    1 => {Elements::Some32}
                                                                },
                                                                1 => {
                                                                    0 => {Elements::Some64},
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
                                                        1 => {Elements::All8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Elements::All16},
                                                                    1 => {Elements::All32}
                                                                },
                                                                1 => {
                                                                    0 => {Elements::All64},
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
                                                0 => {Elements::Eq1},
                                                1 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {Elements::Eq8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Elements::Eq16},
                                                                    1 => {Elements::Eq32}
                                                                },
                                                                1 => {
                                                                    0 => {Elements::Eq64},
                                                                    1 => {}
                                                                }
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Elements::Eq256},
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
                                                            1 => {Elements::FullLeftShift8_1}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Elements::FullLeftShift16_1},
                                                                        1 => {Elements::FullLeftShift32_1}
                                                                    },
                                                                    1 => {
                                                                        0 => {Elements::FullLeftShift64_1},
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
                                                                    0 => {Elements::FullLeftShift8_2},
                                                                    1 => {Elements::FullLeftShift16_2}
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Elements::FullLeftShift32_2},
                                                                                1 => {Elements::FullLeftShift64_2}
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
                                                            0 => {Elements::FullLeftShift8_4},
                                                            1 => {
                                                                0 => {
                                                                    0 => {Elements::FullLeftShift16_4},
                                                                    1 => {Elements::FullLeftShift32_4}
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Elements::FullLeftShift64_4},
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
                                                                        0 => {Elements::FullLeftShift16_8},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Elements::FullLeftShift32_8},
                                                                                1 => {Elements::FullLeftShift64_8}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {Elements::FullLeftShift32_16},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Elements::FullLeftShift64_16},
                                                                                1 => {}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {Elements::FullLeftShift64_32},
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
                                                            1 => {Elements::FullRightShift8_1}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Elements::FullRightShift16_1},
                                                                        1 => {Elements::FullRightShift32_1}
                                                                    },
                                                                    1 => {
                                                                        0 => {Elements::FullRightShift64_1},
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
                                                                    0 => {Elements::FullRightShift8_2},
                                                                    1 => {Elements::FullRightShift16_2}
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Elements::FullRightShift32_2},
                                                                                1 => {Elements::FullRightShift64_2}
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
                                                            0 => {Elements::FullRightShift8_4},
                                                            1 => {
                                                                0 => {
                                                                    0 => {Elements::FullRightShift16_4},
                                                                    1 => {Elements::FullRightShift32_4}
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Elements::FullRightShift64_4},
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
                                                                        0 => {Elements::FullRightShift16_8},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Elements::FullRightShift32_8},
                                                                                1 => {Elements::FullRightShift64_8}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {Elements::FullRightShift32_16},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Elements::FullRightShift64_16},
                                                                                1 => {}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {Elements::FullRightShift64_32},
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
                                                                            1 => {Elements::Leftmost8_1}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Elements::Leftmost16_1},
                                                                                        1 => {Elements::Leftmost32_1}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::Leftmost64_1},
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
                                                                                    0 => {Elements::Leftmost8_2},
                                                                                    1 => {Elements::Leftmost16_2}
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {
                                                                                                0 => {Elements::Leftmost32_2},
                                                                                                1 => {Elements::Leftmost64_2}
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
                                                                            0 => {Elements::Leftmost8_4},
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Elements::Leftmost16_4},
                                                                                    1 => {Elements::Leftmost32_4}
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {
                                                                                                0 => {Elements::Leftmost64_4},
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
                                                                                        0 => {Elements::Leftmost16_8},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::Leftmost32_8},
                                                                                                1 => {Elements::Leftmost64_8}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::Leftmost32_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::Leftmost64_16},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Elements::Leftmost64_32},
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
                                                                            1 => {Elements::Rightmost8_1}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Elements::Rightmost16_1},
                                                                                        1 => {Elements::Rightmost32_1}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::Rightmost64_1},
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
                                                                                    0 => {Elements::Rightmost8_2},
                                                                                    1 => {Elements::Rightmost16_2}
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {
                                                                                                0 => {Elements::Rightmost32_2},
                                                                                                1 => {Elements::Rightmost64_2}
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
                                                                            0 => {Elements::Rightmost8_4},
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Elements::Rightmost16_4},
                                                                                    1 => {Elements::Rightmost32_4}
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {
                                                                                                0 => {Elements::Rightmost64_4},
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
                                                                                        0 => {Elements::Rightmost16_8},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::Rightmost32_8},
                                                                                                1 => {Elements::Rightmost64_8}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::Rightmost32_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::Rightmost64_16},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Elements::Rightmost64_32},
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
                                                                            1 => {Elements::LeftPadLow1_8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Elements::LeftPadLow1_16},
                                                                                        1 => {Elements::LeftPadLow1_32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::LeftPadLow1_64},
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
                                                                                        0 => {Elements::LeftPadLow8_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::LeftPadLow8_32},
                                                                                                1 => {Elements::LeftPadLow8_64}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::LeftPadLow16_32},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::LeftPadLow16_64},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Elements::LeftPadLow32_64},
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
                                                                            1 => {Elements::LeftPadHigh1_8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Elements::LeftPadHigh1_16},
                                                                                        1 => {Elements::LeftPadHigh1_32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::LeftPadHigh1_64},
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
                                                                                        0 => {Elements::LeftPadHigh8_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::LeftPadHigh8_32},
                                                                                                1 => {Elements::LeftPadHigh8_64}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::LeftPadHigh16_32},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::LeftPadHigh16_64},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Elements::LeftPadHigh32_64},
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
                                                                            1 => {Elements::LeftExtend1_8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Elements::LeftExtend1_16},
                                                                                        1 => {Elements::LeftExtend1_32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::LeftExtend1_64},
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
                                                                                        0 => {Elements::LeftExtend8_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::LeftExtend8_32},
                                                                                                1 => {Elements::LeftExtend8_64}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::LeftExtend16_32},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::LeftExtend16_64},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Elements::LeftExtend32_64},
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
                                                                            1 => {Elements::RightPadLow1_8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Elements::RightPadLow1_16},
                                                                                        1 => {Elements::RightPadLow1_32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::RightPadLow1_64},
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
                                                                                        0 => {Elements::RightPadLow8_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::RightPadLow8_32},
                                                                                                1 => {Elements::RightPadLow8_64}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::RightPadLow16_32},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::RightPadLow16_64},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Elements::RightPadLow32_64},
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
                                                                            1 => {Elements::RightPadHigh1_8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Elements::RightPadHigh1_16},
                                                                                        1 => {Elements::RightPadHigh1_32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::RightPadHigh1_64},
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
                                                                                        0 => {Elements::RightPadHigh8_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::RightPadHigh8_32},
                                                                                                1 => {Elements::RightPadHigh8_64}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::RightPadHigh16_32},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::RightPadHigh16_64},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Elements::RightPadHigh32_64},
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
                                                                                        0 => {Elements::RightExtend8_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::RightExtend8_32},
                                                                                                1 => {Elements::RightExtend8_64}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::RightExtend16_32},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::RightExtend16_64},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Elements::RightExtend32_64},
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
                                                                        1 => {Elements::LeftShiftWith8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Elements::LeftShiftWith16},
                                                                                    1 => {Elements::LeftShiftWith32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::LeftShiftWith64},
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
                                                                        1 => {Elements::RightShiftWith8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Elements::RightShiftWith16},
                                                                                    1 => {Elements::RightShiftWith32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::RightShiftWith64},
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
                                                                        1 => {Elements::LeftShift8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Elements::LeftShift16},
                                                                                    1 => {Elements::LeftShift32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::LeftShift64},
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
                                                                        1 => {Elements::RightShift8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Elements::RightShift16},
                                                                                    1 => {Elements::RightShift32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::RightShift64},
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
                                                                        1 => {Elements::LeftRotate8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Elements::LeftRotate16},
                                                                                    1 => {Elements::LeftRotate32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::LeftRotate64},
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
                                                                        1 => {Elements::RightRotate8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Elements::RightRotate16},
                                                                                    1 => {Elements::RightRotate32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::RightRotate64},
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
                                        1 => {Elements::One8}
                                    },
                                    1 => {
                                        0 => {
                                            0 => {
                                                0 => {
                                                    0 => {Elements::One16},
                                                    1 => {Elements::One32}
                                                },
                                                1 => {
                                                    0 => {Elements::One64},
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
                                                1 => {Elements::FullAdd8}
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Elements::FullAdd16},
                                                            1 => {Elements::FullAdd32}
                                                        },
                                                        1 => {
                                                            0 => {Elements::FullAdd64},
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
                                                1 => {Elements::Add8}
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Elements::Add16},
                                                            1 => {Elements::Add32}
                                                        },
                                                        1 => {
                                                            0 => {Elements::Add64},
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
                                                            1 => {Elements::FullIncrement8}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Elements::FullIncrement16},
                                                                        1 => {Elements::FullIncrement32}
                                                                    },
                                                                    1 => {
                                                                        0 => {Elements::FullIncrement64},
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
                                                            1 => {Elements::Increment8}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Elements::Increment16},
                                                                        1 => {Elements::Increment32}
                                                                    },
                                                                    1 => {
                                                                        0 => {Elements::Increment64},
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
                                                            1 => {Elements::FullSubtract8}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Elements::FullSubtract16},
                                                                        1 => {Elements::FullSubtract32}
                                                                    },
                                                                    1 => {
                                                                        0 => {Elements::FullSubtract64},
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
                                                                1 => {Elements::Subtract8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Elements::Subtract16},
                                                                            1 => {Elements::Subtract32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Elements::Subtract64},
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
                                                                1 => {Elements::Negate8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Elements::Negate16},
                                                                            1 => {Elements::Negate32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Elements::Negate64},
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
                                                                1 => {Elements::FullDecrement8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Elements::FullDecrement16},
                                                                            1 => {Elements::FullDecrement32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Elements::FullDecrement64},
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
                                                                1 => {Elements::Decrement8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Elements::Decrement16},
                                                                            1 => {Elements::Decrement32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Elements::Decrement64},
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
                                                                1 => {Elements::FullMultiply8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Elements::FullMultiply16},
                                                                            1 => {Elements::FullMultiply32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Elements::FullMultiply64},
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
                                                                1 => {Elements::Multiply8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Elements::Multiply16},
                                                                            1 => {Elements::Multiply32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Elements::Multiply64},
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
                                                                1 => {Elements::IsZero8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Elements::IsZero16},
                                                                            1 => {Elements::IsZero32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Elements::IsZero64},
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
                                                                1 => {Elements::IsOne8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Elements::IsOne16},
                                                                            1 => {Elements::IsOne32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Elements::IsOne64},
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
                                                                                1 => {Elements::Le8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Elements::Le16},
                                                                                            1 => {Elements::Le32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Elements::Le64},
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
                                                                                1 => {Elements::Lt8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Elements::Lt16},
                                                                                            1 => {Elements::Lt32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Elements::Lt64},
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
                                                                                1 => {Elements::Min8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Elements::Min16},
                                                                                            1 => {Elements::Min32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Elements::Min64},
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
                                                                                1 => {Elements::Max8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Elements::Max16},
                                                                                            1 => {Elements::Max32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Elements::Max64},
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
                                                                                1 => {Elements::Median8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Elements::Median16},
                                                                                            1 => {Elements::Median32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Elements::Median64},
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
                                                                                            0 => {Elements::DivMod128_64},
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
                                                                                1 => {Elements::DivMod8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Elements::DivMod16},
                                                                                            1 => {Elements::DivMod32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Elements::DivMod64},
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
                                                                                1 => {Elements::Divide8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Elements::Divide16},
                                                                                            1 => {Elements::Divide32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Elements::Divide64},
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
                                                                                1 => {Elements::Modulo8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Elements::Modulo16},
                                                                                            1 => {Elements::Modulo32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Elements::Modulo64},
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
                                                                                1 => {Elements::Divides8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Elements::Divides16},
                                                                                            1 => {Elements::Divides32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Elements::Divides64},
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
                                0 => {Elements::Sha256Block},
                                1 => {
                                    0 => {
                                        0 => {Elements::Sha256Iv},
                                        1 => {
                                            0 => {Elements::Sha256Ctx8Add1},
                                            1 => {
                                                0 => {
                                                    0 => {Elements::Sha256Ctx8Add2},
                                                    1 => {Elements::Sha256Ctx8Add4}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Elements::Sha256Ctx8Add8},
                                                                1 => {Elements::Sha256Ctx8Add16}
                                                            },
                                                            1 => {
                                                                0 => {Elements::Sha256Ctx8Add32},
                                                                1 => {Elements::Sha256Ctx8Add64}
                                                            }
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Elements::Sha256Ctx8Add128},
                                                                    1 => {Elements::Sha256Ctx8Add256}
                                                                },
                                                                1 => {
                                                                    0 => {Elements::Sha256Ctx8Add512},
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
                                                    0 => {Elements::Sha256Ctx8AddBuffer511},
                                                    1 => {Elements::Sha256Ctx8Finalize}
                                                },
                                                1 => {
                                                    0 => {Elements::Sha256Ctx8Init},
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
                                            0 => {Elements::PointVerify1},
                                            1 => {}
                                        },
                                        1 => {
                                            0 => {
                                                0 => {Elements::Decompress},
                                                1 => {
                                                    0 => {Elements::LinearVerify1},
                                                    1 => {}
                                                }
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Elements::LinearCombination1},
                                                                1 => {}
                                                            },
                                                            1 => {Elements::Scale}
                                                        },
                                                        1 => {
                                                            0 => {Elements::Generate},
                                                            1 => {Elements::GejInfinity}
                                                        }
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Elements::GejNormalize},
                                                                1 => {Elements::GejNegate}
                                                            },
                                                            1 => {
                                                                0 => {Elements::GeNegate},
                                                                1 => {Elements::GejDouble}
                                                            }
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {Elements::GejAdd},
                                                                1 => {Elements::GejGeAddEx}
                                                            },
                                                            1 => {
                                                                0 => {Elements::GejGeAdd},
                                                                1 => {Elements::GejRescale}
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
                                                                                0 => {Elements::GejIsInfinity},
                                                                                1 => {Elements::GejEquiv}
                                                                            },
                                                                            1 => {
                                                                                0 => {Elements::GejGeEquiv},
                                                                                1 => {Elements::GejXEquiv}
                                                                            }
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Elements::GejYIsOdd},
                                                                                1 => {Elements::GejIsOnCurve}
                                                                            },
                                                                            1 => {
                                                                                0 => {Elements::GeIsOnCurve},
                                                                                1 => {Elements::ScalarNormalize}
                                                                            }
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Elements::ScalarNegate},
                                                                                1 => {Elements::ScalarAdd}
                                                                            },
                                                                            1 => {
                                                                                0 => {Elements::ScalarSquare},
                                                                                1 => {Elements::ScalarMultiply}
                                                                            }
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Elements::ScalarMultiplyLambda},
                                                                                1 => {Elements::ScalarInvert}
                                                                            },
                                                                            1 => {
                                                                                0 => {Elements::ScalarIsZero},
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
                                                                                    1 => {Elements::FeNormalize}
                                                                                }
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Elements::FeNegate},
                                                                                    1 => {Elements::FeAdd}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::FeSquare},
                                                                                    1 => {Elements::FeMultiply}
                                                                                }
                                                                            }
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Elements::FeMultiplyBeta},
                                                                                    1 => {Elements::FeInvert}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::FeSquareRoot},
                                                                                    1 => {Elements::FeIsZero}
                                                                                }
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Elements::FeIsOdd},
                                                                                    1 => {}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::HashToCurve},
                                                                                    1 => {Elements::Swu}
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
                                        0 => {Elements::CheckSigVerify},
                                        1 => {
                                            0 => {
                                                0 => {Elements::Bip0340Verify},
                                                1 => {}
                                            },
                                            1 => {}
                                        }
                                    }
                                },
                                1 => {
                                    0 => {},
                                    1 => {
                                        0 => {Elements::ParseLock},
                                        1 => {
                                            0 => {
                                                0 => {Elements::ParseSequence},
                                                1 => {Elements::TapdataInit}
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
                    0 => {Elements::SigAllHash},
                    1 => {
                        0 => {
                            0 => {Elements::TxHash},
                            1 => {Elements::TapEnvHash}
                        },
                        1 => {
                            0 => {
                                0 => {
                                    0 => {
                                        0 => {Elements::OutputsHash},
                                        1 => {Elements::InputsHash}
                                    },
                                    1 => {
                                        0 => {Elements::IssuancesHash},
                                        1 => {Elements::InputUtxosHash}
                                    }
                                },
                                1 => {
                                    0 => {
                                        0 => {
                                            0 => {Elements::OutputHash},
                                            1 => {Elements::OutputAmountsHash}
                                        },
                                        1 => {
                                            0 => {Elements::OutputScriptsHash},
                                            1 => {Elements::OutputNoncesHash}
                                        }
                                    },
                                    1 => {
                                        0 => {
                                            0 => {Elements::OutputRangeProofsHash},
                                            1 => {Elements::OutputSurjectionProofsHash}
                                        },
                                        1 => {
                                            0 => {Elements::InputHash},
                                            1 => {Elements::InputOutpointsHash}
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
                                                            0 => {Elements::InputSequencesHash},
                                                            1 => {Elements::InputAnnexesHash}
                                                        },
                                                        1 => {
                                                            0 => {Elements::InputScriptSigsHash},
                                                            1 => {Elements::IssuanceHash}
                                                        }
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {Elements::IssuanceAssetAmountsHash},
                                                            1 => {Elements::IssuanceTokenAmountsHash}
                                                        },
                                                        1 => {
                                                            0 => {Elements::IssuanceRangeProofsHash},
                                                            1 => {Elements::IssuanceBlindingEntropyHash}
                                                        }
                                                    }
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Elements::InputUtxoHash},
                                                            1 => {Elements::InputAmountsHash}
                                                        },
                                                        1 => {
                                                            0 => {Elements::InputScriptsHash},
                                                            1 => {Elements::TapleafHash}
                                                        }
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {Elements::TappathHash},
                                                            1 => {Elements::OutpointHash}
                                                        },
                                                        1 => {
                                                            0 => {Elements::AssetAmountHash},
                                                            1 => {Elements::NonceHash}
                                                        }
                                                    }
                                                }
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Elements::AnnexHash},
                                                                1 => {Elements::BuildTapleafSimplicity}
                                                            },
                                                            1 => {
                                                                0 => {Elements::BuildTapbranch},
                                                                1 => {Elements::BuildTaptweak}
                                                            }
                                                        },
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
                            }
                        }
                    }
                },
                1 => {
                    0 => {
                        0 => {
                            0 => {Elements::CheckLockHeight},
                            1 => {
                                0 => {
                                    0 => {Elements::CheckLockTime},
                                    1 => {Elements::CheckLockDistance}
                                },
                                1 => {
                                    0 => {
                                        0 => {
                                            0 => {
                                                0 => {Elements::CheckLockDuration},
                                                1 => {Elements::TxLockHeight}
                                            },
                                            1 => {
                                                0 => {Elements::TxLockTime},
                                                1 => {Elements::TxLockDistance}
                                            }
                                        },
                                        1 => {
                                            0 => {
                                                0 => {
                                                    0 => {Elements::TxLockDuration},
                                                    1 => {Elements::TxIsFinal}
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
                            0 => {Elements::Issuance},
                            1 => {
                                0 => {
                                    0 => {Elements::IssuanceAsset},
                                    1 => {Elements::IssuanceToken}
                                },
                                1 => {
                                    0 => {
                                        0 => {
                                            0 => {
                                                0 => {Elements::IssuanceEntropy},
                                                1 => {Elements::CalculateIssuanceEntropy}
                                            },
                                            1 => {
                                                0 => {Elements::CalculateAsset},
                                                1 => {Elements::CalculateExplicitToken}
                                            }
                                        },
                                        1 => {
                                            0 => {
                                                0 => {
                                                    0 => {Elements::CalculateConfidentialToken},
                                                    1 => {Elements::LbtcAsset}
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
                                0 => {
                                    0 => {
                                        0 => {Elements::ScriptCMR},
                                        1 => {
                                            0 => {
                                                0 => {Elements::InternalKey},
                                                1 => {Elements::CurrentIndex}
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Elements::NumInputs},
                                                            1 => {Elements::NumOutputs}
                                                        },
                                                        1 => {
                                                            0 => {Elements::LockTime},
                                                            1 => {Elements::OutputAsset}
                                                        }
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Elements::OutputAmount},
                                                                1 => {Elements::OutputNonce}
                                                            },
                                                            1 => {
                                                                0 => {Elements::OutputScriptHash},
                                                                1 => {Elements::OutputNullDatum}
                                                            }
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {Elements::OutputIsFee},
                                                                1 => {Elements::OutputSurjectionProof}
                                                            },
                                                            1 => {
                                                                0 => {Elements::OutputRangeProof},
                                                                1 => {Elements::TotalFee}
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
                                                                                0 => {Elements::CurrentPegin},
                                                                                1 => {Elements::CurrentPrevOutpoint}
                                                                            },
                                                                            1 => {
                                                                                0 => {Elements::CurrentAsset},
                                                                                1 => {Elements::CurrentAmount}
                                                                            }
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Elements::CurrentScriptHash},
                                                                                1 => {Elements::CurrentSequence}
                                                                            },
                                                                            1 => {
                                                                                0 => {Elements::CurrentAnnexHash},
                                                                                1 => {Elements::CurrentScriptSigHash}
                                                                            }
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Elements::CurrentReissuanceBlinding},
                                                                                1 => {Elements::CurrentNewIssuanceContract}
                                                                            },
                                                                            1 => {
                                                                                0 => {Elements::CurrentReissuanceEntropy},
                                                                                1 => {Elements::CurrentIssuanceAssetAmount}
                                                                            }
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Elements::CurrentIssuanceTokenAmount},
                                                                                1 => {Elements::CurrentIssuanceAssetProof}
                                                                            },
                                                                            1 => {
                                                                                0 => {Elements::CurrentIssuanceTokenProof},
                                                                                1 => {Elements::InputPegin}
                                                                            }
                                                                        }
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Elements::InputPrevOutpoint},
                                                                                    1 => {Elements::InputAsset}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::InputAmount},
                                                                                    1 => {Elements::InputScriptHash}
                                                                                }
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Elements::InputSequence},
                                                                                    1 => {Elements::InputAnnexHash}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::InputScriptSigHash},
                                                                                    1 => {Elements::ReissuanceBlinding}
                                                                                }
                                                                            }
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Elements::NewIssuanceContract},
                                                                                    1 => {Elements::ReissuanceEntropy}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::IssuanceAssetAmount},
                                                                                    1 => {Elements::IssuanceTokenAmount}
                                                                                }
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Elements::IssuanceAssetProof},
                                                                                    1 => {Elements::IssuanceTokenProof}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::TapleafVersion},
                                                                                    1 => {Elements::Tappath}
                                                                                }
                                                                            }
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Elements::Version},
                                                                                    1 => {Elements::GenesisBlockHash}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::TransactionId},
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
                            },
                            1 => {}
                        },
                        1 => {}
                    }
                }
            }
        })
    }

    fn c_jet_ptr(&self) -> &dyn Fn(&mut CFrameItem, CFrameItem, &Self::CJetEnvironment) -> bool {
        match self {
            Elements::Add16 => &simplicity_sys::c_jets::jets_wrapper::add_16,
            Elements::Add32 => &simplicity_sys::c_jets::jets_wrapper::add_32,
            Elements::Add64 => &simplicity_sys::c_jets::jets_wrapper::add_64,
            Elements::Add8 => &simplicity_sys::c_jets::jets_wrapper::add_8,
            Elements::All16 => &simplicity_sys::c_jets::jets_wrapper::all_16,
            Elements::All32 => &simplicity_sys::c_jets::jets_wrapper::all_32,
            Elements::All64 => &simplicity_sys::c_jets::jets_wrapper::all_64,
            Elements::All8 => &simplicity_sys::c_jets::jets_wrapper::all_8,
            Elements::And1 => &simplicity_sys::c_jets::jets_wrapper::and_1,
            Elements::And16 => &simplicity_sys::c_jets::jets_wrapper::and_16,
            Elements::And32 => &simplicity_sys::c_jets::jets_wrapper::and_32,
            Elements::And64 => &simplicity_sys::c_jets::jets_wrapper::and_64,
            Elements::And8 => &simplicity_sys::c_jets::jets_wrapper::and_8,
            Elements::AnnexHash => &simplicity_sys::c_jets::jets_wrapper::annex_hash,
            Elements::AssetAmountHash => &simplicity_sys::c_jets::jets_wrapper::asset_amount_hash,
            Elements::Bip0340Verify => &simplicity_sys::c_jets::jets_wrapper::bip_0340_verify,
            Elements::BuildTapbranch => &simplicity_sys::c_jets::jets_wrapper::build_tapbranch,
            Elements::BuildTapleafSimplicity => &simplicity_sys::c_jets::jets_wrapper::build_tapleaf_simplicity,
            Elements::BuildTaptweak => &simplicity_sys::c_jets::jets_wrapper::build_taptweak,
            Elements::CalculateAsset => &simplicity_sys::c_jets::jets_wrapper::calculate_asset,
            Elements::CalculateConfidentialToken => &simplicity_sys::c_jets::jets_wrapper::calculate_confidential_token,
            Elements::CalculateExplicitToken => &simplicity_sys::c_jets::jets_wrapper::calculate_explicit_token,
            Elements::CalculateIssuanceEntropy => &simplicity_sys::c_jets::jets_wrapper::calculate_issuance_entropy,
            Elements::Ch1 => &simplicity_sys::c_jets::jets_wrapper::ch_1,
            Elements::Ch16 => &simplicity_sys::c_jets::jets_wrapper::ch_16,
            Elements::Ch32 => &simplicity_sys::c_jets::jets_wrapper::ch_32,
            Elements::Ch64 => &simplicity_sys::c_jets::jets_wrapper::ch_64,
            Elements::Ch8 => &simplicity_sys::c_jets::jets_wrapper::ch_8,
            Elements::CheckLockDistance => &simplicity_sys::c_jets::jets_wrapper::check_lock_distance,
            Elements::CheckLockDuration => &simplicity_sys::c_jets::jets_wrapper::check_lock_duration,
            Elements::CheckLockHeight => &simplicity_sys::c_jets::jets_wrapper::check_lock_height,
            Elements::CheckLockTime => &simplicity_sys::c_jets::jets_wrapper::check_lock_time,
            Elements::CheckSigVerify => &simplicity_sys::c_jets::jets_wrapper::check_sig_verify,
            Elements::Complement1 => &simplicity_sys::c_jets::jets_wrapper::complement_1,
            Elements::Complement16 => &simplicity_sys::c_jets::jets_wrapper::complement_16,
            Elements::Complement32 => &simplicity_sys::c_jets::jets_wrapper::complement_32,
            Elements::Complement64 => &simplicity_sys::c_jets::jets_wrapper::complement_64,
            Elements::Complement8 => &simplicity_sys::c_jets::jets_wrapper::complement_8,
            Elements::CurrentAmount => &simplicity_sys::c_jets::jets_wrapper::current_amount,
            Elements::CurrentAnnexHash => &simplicity_sys::c_jets::jets_wrapper::current_annex_hash,
            Elements::CurrentAsset => &simplicity_sys::c_jets::jets_wrapper::current_asset,
            Elements::CurrentIndex => &simplicity_sys::c_jets::jets_wrapper::current_index,
            Elements::CurrentIssuanceAssetAmount => &simplicity_sys::c_jets::jets_wrapper::current_issuance_asset_amount,
            Elements::CurrentIssuanceAssetProof => &simplicity_sys::c_jets::jets_wrapper::current_issuance_asset_proof,
            Elements::CurrentIssuanceTokenAmount => &simplicity_sys::c_jets::jets_wrapper::current_issuance_token_amount,
            Elements::CurrentIssuanceTokenProof => &simplicity_sys::c_jets::jets_wrapper::current_issuance_token_proof,
            Elements::CurrentNewIssuanceContract => &simplicity_sys::c_jets::jets_wrapper::current_new_issuance_contract,
            Elements::CurrentPegin => &simplicity_sys::c_jets::jets_wrapper::current_pegin,
            Elements::CurrentPrevOutpoint => &simplicity_sys::c_jets::jets_wrapper::current_prev_outpoint,
            Elements::CurrentReissuanceBlinding => &simplicity_sys::c_jets::jets_wrapper::current_reissuance_blinding,
            Elements::CurrentReissuanceEntropy => &simplicity_sys::c_jets::jets_wrapper::current_reissuance_entropy,
            Elements::CurrentScriptHash => &simplicity_sys::c_jets::jets_wrapper::current_script_hash,
            Elements::CurrentScriptSigHash => &simplicity_sys::c_jets::jets_wrapper::current_script_sig_hash,
            Elements::CurrentSequence => &simplicity_sys::c_jets::jets_wrapper::current_sequence,
            Elements::Decompress => &simplicity_sys::c_jets::jets_wrapper::decompress,
            Elements::Decrement16 => &simplicity_sys::c_jets::jets_wrapper::decrement_16,
            Elements::Decrement32 => &simplicity_sys::c_jets::jets_wrapper::decrement_32,
            Elements::Decrement64 => &simplicity_sys::c_jets::jets_wrapper::decrement_64,
            Elements::Decrement8 => &simplicity_sys::c_jets::jets_wrapper::decrement_8,
            Elements::DivMod128_64 => &simplicity_sys::c_jets::jets_wrapper::div_mod_128_64,
            Elements::DivMod16 => &simplicity_sys::c_jets::jets_wrapper::div_mod_16,
            Elements::DivMod32 => &simplicity_sys::c_jets::jets_wrapper::div_mod_32,
            Elements::DivMod64 => &simplicity_sys::c_jets::jets_wrapper::div_mod_64,
            Elements::DivMod8 => &simplicity_sys::c_jets::jets_wrapper::div_mod_8,
            Elements::Divide16 => &simplicity_sys::c_jets::jets_wrapper::divide_16,
            Elements::Divide32 => &simplicity_sys::c_jets::jets_wrapper::divide_32,
            Elements::Divide64 => &simplicity_sys::c_jets::jets_wrapper::divide_64,
            Elements::Divide8 => &simplicity_sys::c_jets::jets_wrapper::divide_8,
            Elements::Divides16 => &simplicity_sys::c_jets::jets_wrapper::divides_16,
            Elements::Divides32 => &simplicity_sys::c_jets::jets_wrapper::divides_32,
            Elements::Divides64 => &simplicity_sys::c_jets::jets_wrapper::divides_64,
            Elements::Divides8 => &simplicity_sys::c_jets::jets_wrapper::divides_8,
            Elements::Eq1 => &simplicity_sys::c_jets::jets_wrapper::eq_1,
            Elements::Eq16 => &simplicity_sys::c_jets::jets_wrapper::eq_16,
            Elements::Eq256 => &simplicity_sys::c_jets::jets_wrapper::eq_256,
            Elements::Eq32 => &simplicity_sys::c_jets::jets_wrapper::eq_32,
            Elements::Eq64 => &simplicity_sys::c_jets::jets_wrapper::eq_64,
            Elements::Eq8 => &simplicity_sys::c_jets::jets_wrapper::eq_8,
            Elements::FeAdd => &simplicity_sys::c_jets::jets_wrapper::fe_add,
            Elements::FeInvert => &simplicity_sys::c_jets::jets_wrapper::fe_invert,
            Elements::FeIsOdd => &simplicity_sys::c_jets::jets_wrapper::fe_is_odd,
            Elements::FeIsZero => &simplicity_sys::c_jets::jets_wrapper::fe_is_zero,
            Elements::FeMultiply => &simplicity_sys::c_jets::jets_wrapper::fe_multiply,
            Elements::FeMultiplyBeta => &simplicity_sys::c_jets::jets_wrapper::fe_multiply_beta,
            Elements::FeNegate => &simplicity_sys::c_jets::jets_wrapper::fe_negate,
            Elements::FeNormalize => &simplicity_sys::c_jets::jets_wrapper::fe_normalize,
            Elements::FeSquare => &simplicity_sys::c_jets::jets_wrapper::fe_square,
            Elements::FeSquareRoot => &simplicity_sys::c_jets::jets_wrapper::fe_square_root,
            Elements::FullAdd16 => &simplicity_sys::c_jets::jets_wrapper::full_add_16,
            Elements::FullAdd32 => &simplicity_sys::c_jets::jets_wrapper::full_add_32,
            Elements::FullAdd64 => &simplicity_sys::c_jets::jets_wrapper::full_add_64,
            Elements::FullAdd8 => &simplicity_sys::c_jets::jets_wrapper::full_add_8,
            Elements::FullDecrement16 => &simplicity_sys::c_jets::jets_wrapper::full_decrement_16,
            Elements::FullDecrement32 => &simplicity_sys::c_jets::jets_wrapper::full_decrement_32,
            Elements::FullDecrement64 => &simplicity_sys::c_jets::jets_wrapper::full_decrement_64,
            Elements::FullDecrement8 => &simplicity_sys::c_jets::jets_wrapper::full_decrement_8,
            Elements::FullIncrement16 => &simplicity_sys::c_jets::jets_wrapper::full_increment_16,
            Elements::FullIncrement32 => &simplicity_sys::c_jets::jets_wrapper::full_increment_32,
            Elements::FullIncrement64 => &simplicity_sys::c_jets::jets_wrapper::full_increment_64,
            Elements::FullIncrement8 => &simplicity_sys::c_jets::jets_wrapper::full_increment_8,
            Elements::FullLeftShift16_1 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_16_1,
            Elements::FullLeftShift16_2 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_16_2,
            Elements::FullLeftShift16_4 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_16_4,
            Elements::FullLeftShift16_8 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_16_8,
            Elements::FullLeftShift32_1 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_1,
            Elements::FullLeftShift32_16 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_16,
            Elements::FullLeftShift32_2 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_2,
            Elements::FullLeftShift32_4 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_4,
            Elements::FullLeftShift32_8 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_8,
            Elements::FullLeftShift64_1 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_1,
            Elements::FullLeftShift64_16 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_16,
            Elements::FullLeftShift64_2 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_2,
            Elements::FullLeftShift64_32 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_32,
            Elements::FullLeftShift64_4 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_4,
            Elements::FullLeftShift64_8 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_8,
            Elements::FullLeftShift8_1 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_8_1,
            Elements::FullLeftShift8_2 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_8_2,
            Elements::FullLeftShift8_4 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_8_4,
            Elements::FullMultiply16 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_16,
            Elements::FullMultiply32 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_32,
            Elements::FullMultiply64 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_64,
            Elements::FullMultiply8 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_8,
            Elements::FullRightShift16_1 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_16_1,
            Elements::FullRightShift16_2 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_16_2,
            Elements::FullRightShift16_4 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_16_4,
            Elements::FullRightShift16_8 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_16_8,
            Elements::FullRightShift32_1 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_1,
            Elements::FullRightShift32_16 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_16,
            Elements::FullRightShift32_2 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_2,
            Elements::FullRightShift32_4 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_4,
            Elements::FullRightShift32_8 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_8,
            Elements::FullRightShift64_1 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_1,
            Elements::FullRightShift64_16 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_16,
            Elements::FullRightShift64_2 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_2,
            Elements::FullRightShift64_32 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_32,
            Elements::FullRightShift64_4 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_4,
            Elements::FullRightShift64_8 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_8,
            Elements::FullRightShift8_1 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_8_1,
            Elements::FullRightShift8_2 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_8_2,
            Elements::FullRightShift8_4 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_8_4,
            Elements::FullSubtract16 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_16,
            Elements::FullSubtract32 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_32,
            Elements::FullSubtract64 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_64,
            Elements::FullSubtract8 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_8,
            Elements::GeIsOnCurve => &simplicity_sys::c_jets::jets_wrapper::ge_is_on_curve,
            Elements::GeNegate => &simplicity_sys::c_jets::jets_wrapper::ge_negate,
            Elements::GejAdd => &simplicity_sys::c_jets::jets_wrapper::gej_add,
            Elements::GejDouble => &simplicity_sys::c_jets::jets_wrapper::gej_double,
            Elements::GejEquiv => &simplicity_sys::c_jets::jets_wrapper::gej_equiv,
            Elements::GejGeAdd => &simplicity_sys::c_jets::jets_wrapper::gej_ge_add,
            Elements::GejGeAddEx => &simplicity_sys::c_jets::jets_wrapper::gej_ge_add_ex,
            Elements::GejGeEquiv => &simplicity_sys::c_jets::jets_wrapper::gej_ge_equiv,
            Elements::GejInfinity => &simplicity_sys::c_jets::jets_wrapper::gej_infinity,
            Elements::GejIsInfinity => &simplicity_sys::c_jets::jets_wrapper::gej_is_infinity,
            Elements::GejIsOnCurve => &simplicity_sys::c_jets::jets_wrapper::gej_is_on_curve,
            Elements::GejNegate => &simplicity_sys::c_jets::jets_wrapper::gej_negate,
            Elements::GejNormalize => &simplicity_sys::c_jets::jets_wrapper::gej_normalize,
            Elements::GejRescale => &simplicity_sys::c_jets::jets_wrapper::gej_rescale,
            Elements::GejXEquiv => &simplicity_sys::c_jets::jets_wrapper::gej_x_equiv,
            Elements::GejYIsOdd => &simplicity_sys::c_jets::jets_wrapper::gej_y_is_odd,
            Elements::Generate => &simplicity_sys::c_jets::jets_wrapper::generate,
            Elements::GenesisBlockHash => &simplicity_sys::c_jets::jets_wrapper::genesis_block_hash,
            Elements::HashToCurve => &simplicity_sys::c_jets::jets_wrapper::hash_to_curve,
            Elements::High1 => &simplicity_sys::c_jets::jets_wrapper::high_1,
            Elements::High16 => &simplicity_sys::c_jets::jets_wrapper::high_16,
            Elements::High32 => &simplicity_sys::c_jets::jets_wrapper::high_32,
            Elements::High64 => &simplicity_sys::c_jets::jets_wrapper::high_64,
            Elements::High8 => &simplicity_sys::c_jets::jets_wrapper::high_8,
            Elements::Increment16 => &simplicity_sys::c_jets::jets_wrapper::increment_16,
            Elements::Increment32 => &simplicity_sys::c_jets::jets_wrapper::increment_32,
            Elements::Increment64 => &simplicity_sys::c_jets::jets_wrapper::increment_64,
            Elements::Increment8 => &simplicity_sys::c_jets::jets_wrapper::increment_8,
            Elements::InputAmount => &simplicity_sys::c_jets::jets_wrapper::input_amount,
            Elements::InputAmountsHash => &simplicity_sys::c_jets::jets_wrapper::input_amounts_hash,
            Elements::InputAnnexHash => &simplicity_sys::c_jets::jets_wrapper::input_annex_hash,
            Elements::InputAnnexesHash => &simplicity_sys::c_jets::jets_wrapper::input_annexes_hash,
            Elements::InputAsset => &simplicity_sys::c_jets::jets_wrapper::input_asset,
            Elements::InputHash => &simplicity_sys::c_jets::jets_wrapper::input_hash,
            Elements::InputOutpointsHash => &simplicity_sys::c_jets::jets_wrapper::input_outpoints_hash,
            Elements::InputPegin => &simplicity_sys::c_jets::jets_wrapper::input_pegin,
            Elements::InputPrevOutpoint => &simplicity_sys::c_jets::jets_wrapper::input_prev_outpoint,
            Elements::InputScriptHash => &simplicity_sys::c_jets::jets_wrapper::input_script_hash,
            Elements::InputScriptSigHash => &simplicity_sys::c_jets::jets_wrapper::input_script_sig_hash,
            Elements::InputScriptSigsHash => &simplicity_sys::c_jets::jets_wrapper::input_script_sigs_hash,
            Elements::InputScriptsHash => &simplicity_sys::c_jets::jets_wrapper::input_scripts_hash,
            Elements::InputSequence => &simplicity_sys::c_jets::jets_wrapper::input_sequence,
            Elements::InputSequencesHash => &simplicity_sys::c_jets::jets_wrapper::input_sequences_hash,
            Elements::InputUtxoHash => &simplicity_sys::c_jets::jets_wrapper::input_utxo_hash,
            Elements::InputUtxosHash => &simplicity_sys::c_jets::jets_wrapper::input_utxos_hash,
            Elements::InputsHash => &simplicity_sys::c_jets::jets_wrapper::inputs_hash,
            Elements::InternalKey => &simplicity_sys::c_jets::jets_wrapper::internal_key,
            Elements::IsOne16 => &simplicity_sys::c_jets::jets_wrapper::is_one_16,
            Elements::IsOne32 => &simplicity_sys::c_jets::jets_wrapper::is_one_32,
            Elements::IsOne64 => &simplicity_sys::c_jets::jets_wrapper::is_one_64,
            Elements::IsOne8 => &simplicity_sys::c_jets::jets_wrapper::is_one_8,
            Elements::IsZero16 => &simplicity_sys::c_jets::jets_wrapper::is_zero_16,
            Elements::IsZero32 => &simplicity_sys::c_jets::jets_wrapper::is_zero_32,
            Elements::IsZero64 => &simplicity_sys::c_jets::jets_wrapper::is_zero_64,
            Elements::IsZero8 => &simplicity_sys::c_jets::jets_wrapper::is_zero_8,
            Elements::Issuance => &simplicity_sys::c_jets::jets_wrapper::issuance,
            Elements::IssuanceAsset => &simplicity_sys::c_jets::jets_wrapper::issuance_asset,
            Elements::IssuanceAssetAmount => &simplicity_sys::c_jets::jets_wrapper::issuance_asset_amount,
            Elements::IssuanceAssetAmountsHash => &simplicity_sys::c_jets::jets_wrapper::issuance_asset_amounts_hash,
            Elements::IssuanceAssetProof => &simplicity_sys::c_jets::jets_wrapper::issuance_asset_proof,
            Elements::IssuanceBlindingEntropyHash => &simplicity_sys::c_jets::jets_wrapper::issuance_blinding_entropy_hash,
            Elements::IssuanceEntropy => &simplicity_sys::c_jets::jets_wrapper::issuance_entropy,
            Elements::IssuanceHash => &simplicity_sys::c_jets::jets_wrapper::issuance_hash,
            Elements::IssuanceRangeProofsHash => &simplicity_sys::c_jets::jets_wrapper::issuance_range_proofs_hash,
            Elements::IssuanceToken => &simplicity_sys::c_jets::jets_wrapper::issuance_token,
            Elements::IssuanceTokenAmount => &simplicity_sys::c_jets::jets_wrapper::issuance_token_amount,
            Elements::IssuanceTokenAmountsHash => &simplicity_sys::c_jets::jets_wrapper::issuance_token_amounts_hash,
            Elements::IssuanceTokenProof => &simplicity_sys::c_jets::jets_wrapper::issuance_token_proof,
            Elements::IssuancesHash => &simplicity_sys::c_jets::jets_wrapper::issuances_hash,
            Elements::LbtcAsset => &simplicity_sys::c_jets::jets_wrapper::lbtc_asset,
            Elements::Le16 => &simplicity_sys::c_jets::jets_wrapper::le_16,
            Elements::Le32 => &simplicity_sys::c_jets::jets_wrapper::le_32,
            Elements::Le64 => &simplicity_sys::c_jets::jets_wrapper::le_64,
            Elements::Le8 => &simplicity_sys::c_jets::jets_wrapper::le_8,
            Elements::LeftExtend16_32 => &simplicity_sys::c_jets::jets_wrapper::left_extend_16_32,
            Elements::LeftExtend16_64 => &simplicity_sys::c_jets::jets_wrapper::left_extend_16_64,
            Elements::LeftExtend1_16 => &simplicity_sys::c_jets::jets_wrapper::left_extend_1_16,
            Elements::LeftExtend1_32 => &simplicity_sys::c_jets::jets_wrapper::left_extend_1_32,
            Elements::LeftExtend1_64 => &simplicity_sys::c_jets::jets_wrapper::left_extend_1_64,
            Elements::LeftExtend1_8 => &simplicity_sys::c_jets::jets_wrapper::left_extend_1_8,
            Elements::LeftExtend32_64 => &simplicity_sys::c_jets::jets_wrapper::left_extend_32_64,
            Elements::LeftExtend8_16 => &simplicity_sys::c_jets::jets_wrapper::left_extend_8_16,
            Elements::LeftExtend8_32 => &simplicity_sys::c_jets::jets_wrapper::left_extend_8_32,
            Elements::LeftExtend8_64 => &simplicity_sys::c_jets::jets_wrapper::left_extend_8_64,
            Elements::LeftPadHigh16_32 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_16_32,
            Elements::LeftPadHigh16_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_16_64,
            Elements::LeftPadHigh1_16 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_1_16,
            Elements::LeftPadHigh1_32 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_1_32,
            Elements::LeftPadHigh1_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_1_64,
            Elements::LeftPadHigh1_8 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_1_8,
            Elements::LeftPadHigh32_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_32_64,
            Elements::LeftPadHigh8_16 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_8_16,
            Elements::LeftPadHigh8_32 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_8_32,
            Elements::LeftPadHigh8_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_8_64,
            Elements::LeftPadLow16_32 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_16_32,
            Elements::LeftPadLow16_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_16_64,
            Elements::LeftPadLow1_16 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_1_16,
            Elements::LeftPadLow1_32 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_1_32,
            Elements::LeftPadLow1_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_1_64,
            Elements::LeftPadLow1_8 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_1_8,
            Elements::LeftPadLow32_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_32_64,
            Elements::LeftPadLow8_16 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_8_16,
            Elements::LeftPadLow8_32 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_8_32,
            Elements::LeftPadLow8_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_8_64,
            Elements::LeftRotate16 => &simplicity_sys::c_jets::jets_wrapper::left_rotate_16,
            Elements::LeftRotate32 => &simplicity_sys::c_jets::jets_wrapper::left_rotate_32,
            Elements::LeftRotate64 => &simplicity_sys::c_jets::jets_wrapper::left_rotate_64,
            Elements::LeftRotate8 => &simplicity_sys::c_jets::jets_wrapper::left_rotate_8,
            Elements::LeftShift16 => &simplicity_sys::c_jets::jets_wrapper::left_shift_16,
            Elements::LeftShift32 => &simplicity_sys::c_jets::jets_wrapper::left_shift_32,
            Elements::LeftShift64 => &simplicity_sys::c_jets::jets_wrapper::left_shift_64,
            Elements::LeftShift8 => &simplicity_sys::c_jets::jets_wrapper::left_shift_8,
            Elements::LeftShiftWith16 => &simplicity_sys::c_jets::jets_wrapper::left_shift_with_16,
            Elements::LeftShiftWith32 => &simplicity_sys::c_jets::jets_wrapper::left_shift_with_32,
            Elements::LeftShiftWith64 => &simplicity_sys::c_jets::jets_wrapper::left_shift_with_64,
            Elements::LeftShiftWith8 => &simplicity_sys::c_jets::jets_wrapper::left_shift_with_8,
            Elements::Leftmost16_1 => &simplicity_sys::c_jets::jets_wrapper::leftmost_16_1,
            Elements::Leftmost16_2 => &simplicity_sys::c_jets::jets_wrapper::leftmost_16_2,
            Elements::Leftmost16_4 => &simplicity_sys::c_jets::jets_wrapper::leftmost_16_4,
            Elements::Leftmost16_8 => &simplicity_sys::c_jets::jets_wrapper::leftmost_16_8,
            Elements::Leftmost32_1 => &simplicity_sys::c_jets::jets_wrapper::leftmost_32_1,
            Elements::Leftmost32_16 => &simplicity_sys::c_jets::jets_wrapper::leftmost_32_16,
            Elements::Leftmost32_2 => &simplicity_sys::c_jets::jets_wrapper::leftmost_32_2,
            Elements::Leftmost32_4 => &simplicity_sys::c_jets::jets_wrapper::leftmost_32_4,
            Elements::Leftmost32_8 => &simplicity_sys::c_jets::jets_wrapper::leftmost_32_8,
            Elements::Leftmost64_1 => &simplicity_sys::c_jets::jets_wrapper::leftmost_64_1,
            Elements::Leftmost64_16 => &simplicity_sys::c_jets::jets_wrapper::leftmost_64_16,
            Elements::Leftmost64_2 => &simplicity_sys::c_jets::jets_wrapper::leftmost_64_2,
            Elements::Leftmost64_32 => &simplicity_sys::c_jets::jets_wrapper::leftmost_64_32,
            Elements::Leftmost64_4 => &simplicity_sys::c_jets::jets_wrapper::leftmost_64_4,
            Elements::Leftmost64_8 => &simplicity_sys::c_jets::jets_wrapper::leftmost_64_8,
            Elements::Leftmost8_1 => &simplicity_sys::c_jets::jets_wrapper::leftmost_8_1,
            Elements::Leftmost8_2 => &simplicity_sys::c_jets::jets_wrapper::leftmost_8_2,
            Elements::Leftmost8_4 => &simplicity_sys::c_jets::jets_wrapper::leftmost_8_4,
            Elements::LinearCombination1 => &simplicity_sys::c_jets::jets_wrapper::linear_combination_1,
            Elements::LinearVerify1 => &simplicity_sys::c_jets::jets_wrapper::linear_verify_1,
            Elements::LockTime => &simplicity_sys::c_jets::jets_wrapper::lock_time,
            Elements::Low1 => &simplicity_sys::c_jets::jets_wrapper::low_1,
            Elements::Low16 => &simplicity_sys::c_jets::jets_wrapper::low_16,
            Elements::Low32 => &simplicity_sys::c_jets::jets_wrapper::low_32,
            Elements::Low64 => &simplicity_sys::c_jets::jets_wrapper::low_64,
            Elements::Low8 => &simplicity_sys::c_jets::jets_wrapper::low_8,
            Elements::Lt16 => &simplicity_sys::c_jets::jets_wrapper::lt_16,
            Elements::Lt32 => &simplicity_sys::c_jets::jets_wrapper::lt_32,
            Elements::Lt64 => &simplicity_sys::c_jets::jets_wrapper::lt_64,
            Elements::Lt8 => &simplicity_sys::c_jets::jets_wrapper::lt_8,
            Elements::Maj1 => &simplicity_sys::c_jets::jets_wrapper::maj_1,
            Elements::Maj16 => &simplicity_sys::c_jets::jets_wrapper::maj_16,
            Elements::Maj32 => &simplicity_sys::c_jets::jets_wrapper::maj_32,
            Elements::Maj64 => &simplicity_sys::c_jets::jets_wrapper::maj_64,
            Elements::Maj8 => &simplicity_sys::c_jets::jets_wrapper::maj_8,
            Elements::Max16 => &simplicity_sys::c_jets::jets_wrapper::max_16,
            Elements::Max32 => &simplicity_sys::c_jets::jets_wrapper::max_32,
            Elements::Max64 => &simplicity_sys::c_jets::jets_wrapper::max_64,
            Elements::Max8 => &simplicity_sys::c_jets::jets_wrapper::max_8,
            Elements::Median16 => &simplicity_sys::c_jets::jets_wrapper::median_16,
            Elements::Median32 => &simplicity_sys::c_jets::jets_wrapper::median_32,
            Elements::Median64 => &simplicity_sys::c_jets::jets_wrapper::median_64,
            Elements::Median8 => &simplicity_sys::c_jets::jets_wrapper::median_8,
            Elements::Min16 => &simplicity_sys::c_jets::jets_wrapper::min_16,
            Elements::Min32 => &simplicity_sys::c_jets::jets_wrapper::min_32,
            Elements::Min64 => &simplicity_sys::c_jets::jets_wrapper::min_64,
            Elements::Min8 => &simplicity_sys::c_jets::jets_wrapper::min_8,
            Elements::Modulo16 => &simplicity_sys::c_jets::jets_wrapper::modulo_16,
            Elements::Modulo32 => &simplicity_sys::c_jets::jets_wrapper::modulo_32,
            Elements::Modulo64 => &simplicity_sys::c_jets::jets_wrapper::modulo_64,
            Elements::Modulo8 => &simplicity_sys::c_jets::jets_wrapper::modulo_8,
            Elements::Multiply16 => &simplicity_sys::c_jets::jets_wrapper::multiply_16,
            Elements::Multiply32 => &simplicity_sys::c_jets::jets_wrapper::multiply_32,
            Elements::Multiply64 => &simplicity_sys::c_jets::jets_wrapper::multiply_64,
            Elements::Multiply8 => &simplicity_sys::c_jets::jets_wrapper::multiply_8,
            Elements::Negate16 => &simplicity_sys::c_jets::jets_wrapper::negate_16,
            Elements::Negate32 => &simplicity_sys::c_jets::jets_wrapper::negate_32,
            Elements::Negate64 => &simplicity_sys::c_jets::jets_wrapper::negate_64,
            Elements::Negate8 => &simplicity_sys::c_jets::jets_wrapper::negate_8,
            Elements::NewIssuanceContract => &simplicity_sys::c_jets::jets_wrapper::new_issuance_contract,
            Elements::NonceHash => &simplicity_sys::c_jets::jets_wrapper::nonce_hash,
            Elements::NumInputs => &simplicity_sys::c_jets::jets_wrapper::num_inputs,
            Elements::NumOutputs => &simplicity_sys::c_jets::jets_wrapper::num_outputs,
            Elements::One16 => &simplicity_sys::c_jets::jets_wrapper::one_16,
            Elements::One32 => &simplicity_sys::c_jets::jets_wrapper::one_32,
            Elements::One64 => &simplicity_sys::c_jets::jets_wrapper::one_64,
            Elements::One8 => &simplicity_sys::c_jets::jets_wrapper::one_8,
            Elements::Or1 => &simplicity_sys::c_jets::jets_wrapper::or_1,
            Elements::Or16 => &simplicity_sys::c_jets::jets_wrapper::or_16,
            Elements::Or32 => &simplicity_sys::c_jets::jets_wrapper::or_32,
            Elements::Or64 => &simplicity_sys::c_jets::jets_wrapper::or_64,
            Elements::Or8 => &simplicity_sys::c_jets::jets_wrapper::or_8,
            Elements::OutpointHash => &simplicity_sys::c_jets::jets_wrapper::outpoint_hash,
            Elements::OutputAmount => &simplicity_sys::c_jets::jets_wrapper::output_amount,
            Elements::OutputAmountsHash => &simplicity_sys::c_jets::jets_wrapper::output_amounts_hash,
            Elements::OutputAsset => &simplicity_sys::c_jets::jets_wrapper::output_asset,
            Elements::OutputHash => &simplicity_sys::c_jets::jets_wrapper::output_hash,
            Elements::OutputIsFee => &simplicity_sys::c_jets::jets_wrapper::output_is_fee,
            Elements::OutputNonce => &simplicity_sys::c_jets::jets_wrapper::output_nonce,
            Elements::OutputNoncesHash => &simplicity_sys::c_jets::jets_wrapper::output_nonces_hash,
            Elements::OutputNullDatum => &simplicity_sys::c_jets::jets_wrapper::output_null_datum,
            Elements::OutputRangeProof => &simplicity_sys::c_jets::jets_wrapper::output_range_proof,
            Elements::OutputRangeProofsHash => &simplicity_sys::c_jets::jets_wrapper::output_range_proofs_hash,
            Elements::OutputScriptHash => &simplicity_sys::c_jets::jets_wrapper::output_script_hash,
            Elements::OutputScriptsHash => &simplicity_sys::c_jets::jets_wrapper::output_scripts_hash,
            Elements::OutputSurjectionProof => &simplicity_sys::c_jets::jets_wrapper::output_surjection_proof,
            Elements::OutputSurjectionProofsHash => &simplicity_sys::c_jets::jets_wrapper::output_surjection_proofs_hash,
            Elements::OutputsHash => &simplicity_sys::c_jets::jets_wrapper::outputs_hash,
            Elements::ParseLock => &simplicity_sys::c_jets::jets_wrapper::parse_lock,
            Elements::ParseSequence => &simplicity_sys::c_jets::jets_wrapper::parse_sequence,
            Elements::PointVerify1 => &simplicity_sys::c_jets::jets_wrapper::point_verify_1,
            Elements::ReissuanceBlinding => &simplicity_sys::c_jets::jets_wrapper::reissuance_blinding,
            Elements::ReissuanceEntropy => &simplicity_sys::c_jets::jets_wrapper::reissuance_entropy,
            Elements::RightExtend16_32 => &simplicity_sys::c_jets::jets_wrapper::right_extend_16_32,
            Elements::RightExtend16_64 => &simplicity_sys::c_jets::jets_wrapper::right_extend_16_64,
            Elements::RightExtend32_64 => &simplicity_sys::c_jets::jets_wrapper::right_extend_32_64,
            Elements::RightExtend8_16 => &simplicity_sys::c_jets::jets_wrapper::right_extend_8_16,
            Elements::RightExtend8_32 => &simplicity_sys::c_jets::jets_wrapper::right_extend_8_32,
            Elements::RightExtend8_64 => &simplicity_sys::c_jets::jets_wrapper::right_extend_8_64,
            Elements::RightPadHigh16_32 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_16_32,
            Elements::RightPadHigh16_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_16_64,
            Elements::RightPadHigh1_16 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_1_16,
            Elements::RightPadHigh1_32 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_1_32,
            Elements::RightPadHigh1_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_1_64,
            Elements::RightPadHigh1_8 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_1_8,
            Elements::RightPadHigh32_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_32_64,
            Elements::RightPadHigh8_16 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_8_16,
            Elements::RightPadHigh8_32 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_8_32,
            Elements::RightPadHigh8_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_8_64,
            Elements::RightPadLow16_32 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_16_32,
            Elements::RightPadLow16_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_16_64,
            Elements::RightPadLow1_16 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_1_16,
            Elements::RightPadLow1_32 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_1_32,
            Elements::RightPadLow1_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_1_64,
            Elements::RightPadLow1_8 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_1_8,
            Elements::RightPadLow32_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_32_64,
            Elements::RightPadLow8_16 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_8_16,
            Elements::RightPadLow8_32 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_8_32,
            Elements::RightPadLow8_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_8_64,
            Elements::RightRotate16 => &simplicity_sys::c_jets::jets_wrapper::right_rotate_16,
            Elements::RightRotate32 => &simplicity_sys::c_jets::jets_wrapper::right_rotate_32,
            Elements::RightRotate64 => &simplicity_sys::c_jets::jets_wrapper::right_rotate_64,
            Elements::RightRotate8 => &simplicity_sys::c_jets::jets_wrapper::right_rotate_8,
            Elements::RightShift16 => &simplicity_sys::c_jets::jets_wrapper::right_shift_16,
            Elements::RightShift32 => &simplicity_sys::c_jets::jets_wrapper::right_shift_32,
            Elements::RightShift64 => &simplicity_sys::c_jets::jets_wrapper::right_shift_64,
            Elements::RightShift8 => &simplicity_sys::c_jets::jets_wrapper::right_shift_8,
            Elements::RightShiftWith16 => &simplicity_sys::c_jets::jets_wrapper::right_shift_with_16,
            Elements::RightShiftWith32 => &simplicity_sys::c_jets::jets_wrapper::right_shift_with_32,
            Elements::RightShiftWith64 => &simplicity_sys::c_jets::jets_wrapper::right_shift_with_64,
            Elements::RightShiftWith8 => &simplicity_sys::c_jets::jets_wrapper::right_shift_with_8,
            Elements::Rightmost16_1 => &simplicity_sys::c_jets::jets_wrapper::rightmost_16_1,
            Elements::Rightmost16_2 => &simplicity_sys::c_jets::jets_wrapper::rightmost_16_2,
            Elements::Rightmost16_4 => &simplicity_sys::c_jets::jets_wrapper::rightmost_16_4,
            Elements::Rightmost16_8 => &simplicity_sys::c_jets::jets_wrapper::rightmost_16_8,
            Elements::Rightmost32_1 => &simplicity_sys::c_jets::jets_wrapper::rightmost_32_1,
            Elements::Rightmost32_16 => &simplicity_sys::c_jets::jets_wrapper::rightmost_32_16,
            Elements::Rightmost32_2 => &simplicity_sys::c_jets::jets_wrapper::rightmost_32_2,
            Elements::Rightmost32_4 => &simplicity_sys::c_jets::jets_wrapper::rightmost_32_4,
            Elements::Rightmost32_8 => &simplicity_sys::c_jets::jets_wrapper::rightmost_32_8,
            Elements::Rightmost64_1 => &simplicity_sys::c_jets::jets_wrapper::rightmost_64_1,
            Elements::Rightmost64_16 => &simplicity_sys::c_jets::jets_wrapper::rightmost_64_16,
            Elements::Rightmost64_2 => &simplicity_sys::c_jets::jets_wrapper::rightmost_64_2,
            Elements::Rightmost64_32 => &simplicity_sys::c_jets::jets_wrapper::rightmost_64_32,
            Elements::Rightmost64_4 => &simplicity_sys::c_jets::jets_wrapper::rightmost_64_4,
            Elements::Rightmost64_8 => &simplicity_sys::c_jets::jets_wrapper::rightmost_64_8,
            Elements::Rightmost8_1 => &simplicity_sys::c_jets::jets_wrapper::rightmost_8_1,
            Elements::Rightmost8_2 => &simplicity_sys::c_jets::jets_wrapper::rightmost_8_2,
            Elements::Rightmost8_4 => &simplicity_sys::c_jets::jets_wrapper::rightmost_8_4,
            Elements::ScalarAdd => &simplicity_sys::c_jets::jets_wrapper::scalar_add,
            Elements::ScalarInvert => &simplicity_sys::c_jets::jets_wrapper::scalar_invert,
            Elements::ScalarIsZero => &simplicity_sys::c_jets::jets_wrapper::scalar_is_zero,
            Elements::ScalarMultiply => &simplicity_sys::c_jets::jets_wrapper::scalar_multiply,
            Elements::ScalarMultiplyLambda => &simplicity_sys::c_jets::jets_wrapper::scalar_multiply_lambda,
            Elements::ScalarNegate => &simplicity_sys::c_jets::jets_wrapper::scalar_negate,
            Elements::ScalarNormalize => &simplicity_sys::c_jets::jets_wrapper::scalar_normalize,
            Elements::ScalarSquare => &simplicity_sys::c_jets::jets_wrapper::scalar_square,
            Elements::Scale => &simplicity_sys::c_jets::jets_wrapper::scale,
            Elements::ScriptCMR => &simplicity_sys::c_jets::jets_wrapper::script_cmr,
            Elements::Sha256Block => &simplicity_sys::c_jets::jets_wrapper::sha_256_block,
            Elements::Sha256Ctx8Add1 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_1,
            Elements::Sha256Ctx8Add128 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_128,
            Elements::Sha256Ctx8Add16 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_16,
            Elements::Sha256Ctx8Add2 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_2,
            Elements::Sha256Ctx8Add256 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_256,
            Elements::Sha256Ctx8Add32 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_32,
            Elements::Sha256Ctx8Add4 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_4,
            Elements::Sha256Ctx8Add512 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_512,
            Elements::Sha256Ctx8Add64 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_64,
            Elements::Sha256Ctx8Add8 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_8,
            Elements::Sha256Ctx8AddBuffer511 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_buffer_511,
            Elements::Sha256Ctx8Finalize => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_finalize,
            Elements::Sha256Ctx8Init => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_init,
            Elements::Sha256Iv => &simplicity_sys::c_jets::jets_wrapper::sha_256_iv,
            Elements::SigAllHash => &simplicity_sys::c_jets::jets_wrapper::sig_all_hash,
            Elements::Some1 => &simplicity_sys::c_jets::jets_wrapper::some_1,
            Elements::Some16 => &simplicity_sys::c_jets::jets_wrapper::some_16,
            Elements::Some32 => &simplicity_sys::c_jets::jets_wrapper::some_32,
            Elements::Some64 => &simplicity_sys::c_jets::jets_wrapper::some_64,
            Elements::Some8 => &simplicity_sys::c_jets::jets_wrapper::some_8,
            Elements::Subtract16 => &simplicity_sys::c_jets::jets_wrapper::subtract_16,
            Elements::Subtract32 => &simplicity_sys::c_jets::jets_wrapper::subtract_32,
            Elements::Subtract64 => &simplicity_sys::c_jets::jets_wrapper::subtract_64,
            Elements::Subtract8 => &simplicity_sys::c_jets::jets_wrapper::subtract_8,
            Elements::Swu => &simplicity_sys::c_jets::jets_wrapper::swu,
            Elements::TapEnvHash => &simplicity_sys::c_jets::jets_wrapper::tap_env_hash,
            Elements::TapdataInit => &simplicity_sys::c_jets::jets_wrapper::tapdata_init,
            Elements::TapleafHash => &simplicity_sys::c_jets::jets_wrapper::tapleaf_hash,
            Elements::TapleafVersion => &simplicity_sys::c_jets::jets_wrapper::tapleaf_version,
            Elements::Tappath => &simplicity_sys::c_jets::jets_wrapper::tappath,
            Elements::TappathHash => &simplicity_sys::c_jets::jets_wrapper::tappath_hash,
            Elements::TotalFee => &simplicity_sys::c_jets::jets_wrapper::total_fee,
            Elements::TransactionId => &simplicity_sys::c_jets::jets_wrapper::transaction_id,
            Elements::TxHash => &simplicity_sys::c_jets::jets_wrapper::tx_hash,
            Elements::TxIsFinal => &simplicity_sys::c_jets::jets_wrapper::tx_is_final,
            Elements::TxLockDistance => &simplicity_sys::c_jets::jets_wrapper::tx_lock_distance,
            Elements::TxLockDuration => &simplicity_sys::c_jets::jets_wrapper::tx_lock_duration,
            Elements::TxLockHeight => &simplicity_sys::c_jets::jets_wrapper::tx_lock_height,
            Elements::TxLockTime => &simplicity_sys::c_jets::jets_wrapper::tx_lock_time,
            Elements::Verify => &simplicity_sys::c_jets::jets_wrapper::verify,
            Elements::Version => &simplicity_sys::c_jets::jets_wrapper::version,
            Elements::Xor1 => &simplicity_sys::c_jets::jets_wrapper::xor_1,
            Elements::Xor16 => &simplicity_sys::c_jets::jets_wrapper::xor_16,
            Elements::Xor32 => &simplicity_sys::c_jets::jets_wrapper::xor_32,
            Elements::Xor64 => &simplicity_sys::c_jets::jets_wrapper::xor_64,
            Elements::Xor8 => &simplicity_sys::c_jets::jets_wrapper::xor_8,
            Elements::XorXor1 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_1,
            Elements::XorXor16 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_16,
            Elements::XorXor32 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_32,
            Elements::XorXor64 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_64,
            Elements::XorXor8 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_8,
        }
    }

    fn cost(&self) -> Cost {
        match self {
            Elements::Add16 => Cost::from_milliweight(108),
            Elements::Add32 => Cost::from_milliweight(117),
            Elements::Add64 => Cost::from_milliweight(109),
            Elements::Add8 => Cost::from_milliweight(112),
            Elements::All16 => Cost::from_milliweight(62),
            Elements::All32 => Cost::from_milliweight(65),
            Elements::All64 => Cost::from_milliweight(79),
            Elements::All8 => Cost::from_milliweight(76),
            Elements::And1 => Cost::from_milliweight(79),
            Elements::And16 => Cost::from_milliweight(88),
            Elements::And32 => Cost::from_milliweight(94),
            Elements::And64 => Cost::from_milliweight(93),
            Elements::And8 => Cost::from_milliweight(91),
            Elements::AnnexHash => Cost::from_milliweight(241),
            Elements::AssetAmountHash => Cost::from_milliweight(308),
            Elements::Bip0340Verify => Cost::from_milliweight(49087),
            Elements::BuildTapbranch => Cost::from_milliweight(2563),
            Elements::BuildTapleafSimplicity => Cost::from_milliweight(1843),
            Elements::BuildTaptweak => Cost::from_milliweight(92813),
            Elements::CalculateAsset => Cost::from_milliweight(807),
            Elements::CalculateConfidentialToken => Cost::from_milliweight(707),
            Elements::CalculateExplicitToken => Cost::from_milliweight(771),
            Elements::CalculateIssuanceEntropy => Cost::from_milliweight(2095),
            Elements::Ch1 => Cost::from_milliweight(78),
            Elements::Ch16 => Cost::from_milliweight(94),
            Elements::Ch32 => Cost::from_milliweight(91),
            Elements::Ch64 => Cost::from_milliweight(91),
            Elements::Ch8 => Cost::from_milliweight(77),
            Elements::CheckLockDistance => Cost::from_milliweight(105),
            Elements::CheckLockDuration => Cost::from_milliweight(102),
            Elements::CheckLockHeight => Cost::from_milliweight(77),
            Elements::CheckLockTime => Cost::from_milliweight(93),
            Elements::CheckSigVerify => Cost::from_milliweight(50000),
            Elements::Complement1 => Cost::from_milliweight(79),
            Elements::Complement16 => Cost::from_milliweight(75),
            Elements::Complement32 => Cost::from_milliweight(93),
            Elements::Complement64 => Cost::from_milliweight(88),
            Elements::Complement8 => Cost::from_milliweight(80),
            Elements::CurrentAmount => Cost::from_milliweight(225),
            Elements::CurrentAnnexHash => Cost::from_milliweight(79),
            Elements::CurrentAsset => Cost::from_milliweight(171),
            Elements::CurrentIndex => Cost::from_milliweight(88),
            Elements::CurrentIssuanceAssetAmount => Cost::from_milliweight(165),
            Elements::CurrentIssuanceAssetProof => Cost::from_milliweight(140),
            Elements::CurrentIssuanceTokenAmount => Cost::from_milliweight(188),
            Elements::CurrentIssuanceTokenProof => Cost::from_milliweight(144),
            Elements::CurrentNewIssuanceContract => Cost::from_milliweight(145),
            Elements::CurrentPegin => Cost::from_milliweight(147),
            Elements::CurrentPrevOutpoint => Cost::from_milliweight(156),
            Elements::CurrentReissuanceBlinding => Cost::from_milliweight(94),
            Elements::CurrentReissuanceEntropy => Cost::from_milliweight(85),
            Elements::CurrentScriptHash => Cost::from_milliweight(134),
            Elements::CurrentScriptSigHash => Cost::from_milliweight(139),
            Elements::CurrentSequence => Cost::from_milliweight(89),
            Elements::Decompress => Cost::from_milliweight(10861),
            Elements::Decrement16 => Cost::from_milliweight(85),
            Elements::Decrement32 => Cost::from_milliweight(91),
            Elements::Decrement64 => Cost::from_milliweight(89),
            Elements::Decrement8 => Cost::from_milliweight(79),
            Elements::DivMod128_64 => Cost::from_milliweight(208),
            Elements::DivMod16 => Cost::from_milliweight(118),
            Elements::DivMod32 => Cost::from_milliweight(115),
            Elements::DivMod64 => Cost::from_milliweight(86),
            Elements::DivMod8 => Cost::from_milliweight(128),
            Elements::Divide16 => Cost::from_milliweight(98),
            Elements::Divide32 => Cost::from_milliweight(100),
            Elements::Divide64 => Cost::from_milliweight(101),
            Elements::Divide8 => Cost::from_milliweight(108),
            Elements::Divides16 => Cost::from_milliweight(93),
            Elements::Divides32 => Cost::from_milliweight(87),
            Elements::Divides64 => Cost::from_milliweight(91),
            Elements::Divides8 => Cost::from_milliweight(98),
            Elements::Eq1 => Cost::from_milliweight(74),
            Elements::Eq16 => Cost::from_milliweight(84),
            Elements::Eq256 => Cost::from_milliweight(225),
            Elements::Eq32 => Cost::from_milliweight(88),
            Elements::Eq64 => Cost::from_milliweight(100),
            Elements::Eq8 => Cost::from_milliweight(95),
            Elements::FeAdd => Cost::from_milliweight(755),
            Elements::FeInvert => Cost::from_milliweight(3175),
            Elements::FeIsOdd => Cost::from_milliweight(290),
            Elements::FeIsZero => Cost::from_milliweight(268),
            Elements::FeMultiply => Cost::from_milliweight(808),
            Elements::FeMultiplyBeta => Cost::from_milliweight(579),
            Elements::FeNegate => Cost::from_milliweight(531),
            Elements::FeNormalize => Cost::from_milliweight(521),
            Elements::FeSquare => Cost::from_milliweight(556),
            Elements::FeSquareRoot => Cost::from_milliweight(10275),
            Elements::FullAdd16 => Cost::from_milliweight(121),
            Elements::FullAdd32 => Cost::from_milliweight(119),
            Elements::FullAdd64 => Cost::from_milliweight(121),
            Elements::FullAdd8 => Cost::from_milliweight(127),
            Elements::FullDecrement16 => Cost::from_milliweight(92),
            Elements::FullDecrement32 => Cost::from_milliweight(107),
            Elements::FullDecrement64 => Cost::from_milliweight(81),
            Elements::FullDecrement8 => Cost::from_milliweight(91),
            Elements::FullIncrement16 => Cost::from_milliweight(89),
            Elements::FullIncrement32 => Cost::from_milliweight(104),
            Elements::FullIncrement64 => Cost::from_milliweight(99),
            Elements::FullIncrement8 => Cost::from_milliweight(72),
            Elements::FullLeftShift16_1 => Cost::from_milliweight(83),
            Elements::FullLeftShift16_2 => Cost::from_milliweight(83),
            Elements::FullLeftShift16_4 => Cost::from_milliweight(89),
            Elements::FullLeftShift16_8 => Cost::from_milliweight(65),
            Elements::FullLeftShift32_1 => Cost::from_milliweight(84),
            Elements::FullLeftShift32_16 => Cost::from_milliweight(81),
            Elements::FullLeftShift32_2 => Cost::from_milliweight(67),
            Elements::FullLeftShift32_4 => Cost::from_milliweight(84),
            Elements::FullLeftShift32_8 => Cost::from_milliweight(91),
            Elements::FullLeftShift64_1 => Cost::from_milliweight(99),
            Elements::FullLeftShift64_16 => Cost::from_milliweight(90),
            Elements::FullLeftShift64_2 => Cost::from_milliweight(94),
            Elements::FullLeftShift64_32 => Cost::from_milliweight(86),
            Elements::FullLeftShift64_4 => Cost::from_milliweight(94),
            Elements::FullLeftShift64_8 => Cost::from_milliweight(86),
            Elements::FullLeftShift8_1 => Cost::from_milliweight(96),
            Elements::FullLeftShift8_2 => Cost::from_milliweight(96),
            Elements::FullLeftShift8_4 => Cost::from_milliweight(85),
            Elements::FullMultiply16 => Cost::from_milliweight(112),
            Elements::FullMultiply32 => Cost::from_milliweight(96),
            Elements::FullMultiply64 => Cost::from_milliweight(127),
            Elements::FullMultiply8 => Cost::from_milliweight(109),
            Elements::FullRightShift16_1 => Cost::from_milliweight(80),
            Elements::FullRightShift16_2 => Cost::from_milliweight(79),
            Elements::FullRightShift16_4 => Cost::from_milliweight(88),
            Elements::FullRightShift16_8 => Cost::from_milliweight(57),
            Elements::FullRightShift32_1 => Cost::from_milliweight(74),
            Elements::FullRightShift32_16 => Cost::from_milliweight(64),
            Elements::FullRightShift32_2 => Cost::from_milliweight(63),
            Elements::FullRightShift32_4 => Cost::from_milliweight(71),
            Elements::FullRightShift32_8 => Cost::from_milliweight(84),
            Elements::FullRightShift64_1 => Cost::from_milliweight(99),
            Elements::FullRightShift64_16 => Cost::from_milliweight(86),
            Elements::FullRightShift64_2 => Cost::from_milliweight(86),
            Elements::FullRightShift64_32 => Cost::from_milliweight(73),
            Elements::FullRightShift64_4 => Cost::from_milliweight(93),
            Elements::FullRightShift64_8 => Cost::from_milliweight(99),
            Elements::FullRightShift8_1 => Cost::from_milliweight(88),
            Elements::FullRightShift8_2 => Cost::from_milliweight(86),
            Elements::FullRightShift8_4 => Cost::from_milliweight(89),
            Elements::FullSubtract16 => Cost::from_milliweight(121),
            Elements::FullSubtract32 => Cost::from_milliweight(116),
            Elements::FullSubtract64 => Cost::from_milliweight(98),
            Elements::FullSubtract8 => Cost::from_milliweight(126),
            Elements::GeIsOnCurve => Cost::from_milliweight(642),
            Elements::GeNegate => Cost::from_milliweight(945),
            Elements::GejAdd => Cost::from_milliweight(2897),
            Elements::GejDouble => Cost::from_milliweight(1764),
            Elements::GejEquiv => Cost::from_milliweight(2220),
            Elements::GejGeAdd => Cost::from_milliweight(2477),
            Elements::GejGeAddEx => Cost::from_milliweight(2719),
            Elements::GejGeEquiv => Cost::from_milliweight(1765),
            Elements::GejInfinity => Cost::from_milliweight(716),
            Elements::GejIsInfinity => Cost::from_milliweight(666),
            Elements::GejIsOnCurve => Cost::from_milliweight(1016),
            Elements::GejNegate => Cost::from_milliweight(1381),
            Elements::GejNormalize => Cost::from_milliweight(4099),
            Elements::GejRescale => Cost::from_milliweight(1908),
            Elements::GejXEquiv => Cost::from_milliweight(1047),
            Elements::GejYIsOdd => Cost::from_milliweight(3651),
            Elements::Generate => Cost::from_milliweight(50071),
            Elements::GenesisBlockHash => Cost::from_milliweight(148),
            Elements::HashToCurve => Cost::from_milliweight(68094),
            Elements::High1 => Cost::from_milliweight(57),
            Elements::High16 => Cost::from_milliweight(66),
            Elements::High32 => Cost::from_milliweight(58),
            Elements::High64 => Cost::from_milliweight(68),
            Elements::High8 => Cost::from_milliweight(59),
            Elements::Increment16 => Cost::from_milliweight(69),
            Elements::Increment32 => Cost::from_milliweight(92),
            Elements::Increment64 => Cost::from_milliweight(87),
            Elements::Increment8 => Cost::from_milliweight(85),
            Elements::InputAmount => Cost::from_milliweight(285),
            Elements::InputAmountsHash => Cost::from_milliweight(140),
            Elements::InputAnnexHash => Cost::from_milliweight(90),
            Elements::InputAnnexesHash => Cost::from_milliweight(155),
            Elements::InputAsset => Cost::from_milliweight(162),
            Elements::InputHash => Cost::from_milliweight(965),
            Elements::InputOutpointsHash => Cost::from_milliweight(142),
            Elements::InputPegin => Cost::from_milliweight(151),
            Elements::InputPrevOutpoint => Cost::from_milliweight(160),
            Elements::InputScriptHash => Cost::from_milliweight(147),
            Elements::InputScriptSigHash => Cost::from_milliweight(153),
            Elements::InputScriptSigsHash => Cost::from_milliweight(138),
            Elements::InputScriptsHash => Cost::from_milliweight(137),
            Elements::InputSequence => Cost::from_milliweight(99),
            Elements::InputSequencesHash => Cost::from_milliweight(142),
            Elements::InputUtxoHash => Cost::from_milliweight(1996),
            Elements::InputUtxosHash => Cost::from_milliweight(140),
            Elements::InputsHash => Cost::from_milliweight(154),
            Elements::InternalKey => Cost::from_milliweight(152),
            Elements::IsOne16 => Cost::from_milliweight(82),
            Elements::IsOne32 => Cost::from_milliweight(65),
            Elements::IsOne64 => Cost::from_milliweight(83),
            Elements::IsOne8 => Cost::from_milliweight(91),
            Elements::IsZero16 => Cost::from_milliweight(75),
            Elements::IsZero32 => Cost::from_milliweight(85),
            Elements::IsZero64 => Cost::from_milliweight(80),
            Elements::IsZero8 => Cost::from_milliweight(77),
            Elements::Issuance => Cost::from_milliweight(91),
            Elements::IssuanceAsset => Cost::from_milliweight(151),
            Elements::IssuanceAssetAmount => Cost::from_milliweight(162),
            Elements::IssuanceAssetAmountsHash => Cost::from_milliweight(139),
            Elements::IssuanceAssetProof => Cost::from_milliweight(150),
            Elements::IssuanceBlindingEntropyHash => Cost::from_milliweight(129),
            Elements::IssuanceEntropy => Cost::from_milliweight(153),
            Elements::IssuanceHash => Cost::from_milliweight(3738),
            Elements::IssuanceRangeProofsHash => Cost::from_milliweight(129),
            Elements::IssuanceToken => Cost::from_milliweight(149),
            Elements::IssuanceTokenAmount => Cost::from_milliweight(196),
            Elements::IssuanceTokenAmountsHash => Cost::from_milliweight(138),
            Elements::IssuanceTokenProof => Cost::from_milliweight(150),
            Elements::IssuancesHash => Cost::from_milliweight(141),
            Elements::LbtcAsset => Cost::from_milliweight(145),
            Elements::Le16 => Cost::from_milliweight(112),
            Elements::Le32 => Cost::from_milliweight(93),
            Elements::Le64 => Cost::from_milliweight(93),
            Elements::Le8 => Cost::from_milliweight(109),
            Elements::LeftExtend16_32 => Cost::from_milliweight(86),
            Elements::LeftExtend16_64 => Cost::from_milliweight(89),
            Elements::LeftExtend1_16 => Cost::from_milliweight(67),
            Elements::LeftExtend1_32 => Cost::from_milliweight(60),
            Elements::LeftExtend1_64 => Cost::from_milliweight(76),
            Elements::LeftExtend1_8 => Cost::from_milliweight(65),
            Elements::LeftExtend32_64 => Cost::from_milliweight(63),
            Elements::LeftExtend8_16 => Cost::from_milliweight(88),
            Elements::LeftExtend8_32 => Cost::from_milliweight(90),
            Elements::LeftExtend8_64 => Cost::from_milliweight(107),
            Elements::LeftPadHigh16_32 => Cost::from_milliweight(91),
            Elements::LeftPadHigh16_64 => Cost::from_milliweight(110),
            Elements::LeftPadHigh1_16 => Cost::from_milliweight(141),
            Elements::LeftPadHigh1_32 => Cost::from_milliweight(263),
            Elements::LeftPadHigh1_64 => Cost::from_milliweight(422),
            Elements::LeftPadHigh1_8 => Cost::from_milliweight(99),
            Elements::LeftPadHigh32_64 => Cost::from_milliweight(93),
            Elements::LeftPadHigh8_16 => Cost::from_milliweight(88),
            Elements::LeftPadHigh8_32 => Cost::from_milliweight(103),
            Elements::LeftPadHigh8_64 => Cost::from_milliweight(136),
            Elements::LeftPadLow16_32 => Cost::from_milliweight(69),
            Elements::LeftPadLow16_64 => Cost::from_milliweight(106),
            Elements::LeftPadLow1_16 => Cost::from_milliweight(65),
            Elements::LeftPadLow1_32 => Cost::from_milliweight(63),
            Elements::LeftPadLow1_64 => Cost::from_milliweight(61),
            Elements::LeftPadLow1_8 => Cost::from_milliweight(56),
            Elements::LeftPadLow32_64 => Cost::from_milliweight(91),
            Elements::LeftPadLow8_16 => Cost::from_milliweight(66),
            Elements::LeftPadLow8_32 => Cost::from_milliweight(61),
            Elements::LeftPadLow8_64 => Cost::from_milliweight(112),
            Elements::LeftRotate16 => Cost::from_milliweight(77),
            Elements::LeftRotate32 => Cost::from_milliweight(106),
            Elements::LeftRotate64 => Cost::from_milliweight(98),
            Elements::LeftRotate8 => Cost::from_milliweight(88),
            Elements::LeftShift16 => Cost::from_milliweight(72),
            Elements::LeftShift32 => Cost::from_milliweight(78),
            Elements::LeftShift64 => Cost::from_milliweight(82),
            Elements::LeftShift8 => Cost::from_milliweight(91),
            Elements::LeftShiftWith16 => Cost::from_milliweight(83),
            Elements::LeftShiftWith32 => Cost::from_milliweight(95),
            Elements::LeftShiftWith64 => Cost::from_milliweight(103),
            Elements::LeftShiftWith8 => Cost::from_milliweight(107),
            Elements::Leftmost16_1 => Cost::from_milliweight(93),
            Elements::Leftmost16_2 => Cost::from_milliweight(90),
            Elements::Leftmost16_4 => Cost::from_milliweight(75),
            Elements::Leftmost16_8 => Cost::from_milliweight(71),
            Elements::Leftmost32_1 => Cost::from_milliweight(77),
            Elements::Leftmost32_16 => Cost::from_milliweight(102),
            Elements::Leftmost32_2 => Cost::from_milliweight(66),
            Elements::Leftmost32_4 => Cost::from_milliweight(52),
            Elements::Leftmost32_8 => Cost::from_milliweight(103),
            Elements::Leftmost64_1 => Cost::from_milliweight(78),
            Elements::Leftmost64_16 => Cost::from_milliweight(88),
            Elements::Leftmost64_2 => Cost::from_milliweight(71),
            Elements::Leftmost64_32 => Cost::from_milliweight(90),
            Elements::Leftmost64_4 => Cost::from_milliweight(79),
            Elements::Leftmost64_8 => Cost::from_milliweight(86),
            Elements::Leftmost8_1 => Cost::from_milliweight(90),
            Elements::Leftmost8_2 => Cost::from_milliweight(90),
            Elements::Leftmost8_4 => Cost::from_milliweight(87),
            Elements::LinearCombination1 => Cost::from_milliweight(84674),
            Elements::LinearVerify1 => Cost::from_milliweight(43364),
            Elements::LockTime => Cost::from_milliweight(85),
            Elements::Low1 => Cost::from_milliweight(38),
            Elements::Low16 => Cost::from_milliweight(69),
            Elements::Low32 => Cost::from_milliweight(62),
            Elements::Low64 => Cost::from_milliweight(47),
            Elements::Low8 => Cost::from_milliweight(47),
            Elements::Lt16 => Cost::from_milliweight(123),
            Elements::Lt32 => Cost::from_milliweight(107),
            Elements::Lt64 => Cost::from_milliweight(76),
            Elements::Lt8 => Cost::from_milliweight(107),
            Elements::Maj1 => Cost::from_milliweight(62),
            Elements::Maj16 => Cost::from_milliweight(80),
            Elements::Maj32 => Cost::from_milliweight(96),
            Elements::Maj64 => Cost::from_milliweight(93),
            Elements::Maj8 => Cost::from_milliweight(94),
            Elements::Max16 => Cost::from_milliweight(114),
            Elements::Max32 => Cost::from_milliweight(92),
            Elements::Max64 => Cost::from_milliweight(104),
            Elements::Max8 => Cost::from_milliweight(96),
            Elements::Median16 => Cost::from_milliweight(123),
            Elements::Median32 => Cost::from_milliweight(101),
            Elements::Median64 => Cost::from_milliweight(109),
            Elements::Median8 => Cost::from_milliweight(122),
            Elements::Min16 => Cost::from_milliweight(97),
            Elements::Min32 => Cost::from_milliweight(113),
            Elements::Min64 => Cost::from_milliweight(102),
            Elements::Min8 => Cost::from_milliweight(99),
            Elements::Modulo16 => Cost::from_milliweight(103),
            Elements::Modulo32 => Cost::from_milliweight(102),
            Elements::Modulo64 => Cost::from_milliweight(85),
            Elements::Modulo8 => Cost::from_milliweight(102),
            Elements::Multiply16 => Cost::from_milliweight(90),
            Elements::Multiply32 => Cost::from_milliweight(90),
            Elements::Multiply64 => Cost::from_milliweight(85),
            Elements::Multiply8 => Cost::from_milliweight(93),
            Elements::Negate16 => Cost::from_milliweight(70),
            Elements::Negate32 => Cost::from_milliweight(85),
            Elements::Negate64 => Cost::from_milliweight(94),
            Elements::Negate8 => Cost::from_milliweight(91),
            Elements::NewIssuanceContract => Cost::from_milliweight(157),
            Elements::NonceHash => Cost::from_milliweight(317),
            Elements::NumInputs => Cost::from_milliweight(86),
            Elements::NumOutputs => Cost::from_milliweight(79),
            Elements::One16 => Cost::from_milliweight(60),
            Elements::One32 => Cost::from_milliweight(59),
            Elements::One64 => Cost::from_milliweight(59),
            Elements::One8 => Cost::from_milliweight(62),
            Elements::Or1 => Cost::from_milliweight(77),
            Elements::Or16 => Cost::from_milliweight(94),
            Elements::Or32 => Cost::from_milliweight(105),
            Elements::Or64 => Cost::from_milliweight(99),
            Elements::Or8 => Cost::from_milliweight(93),
            Elements::OutpointHash => Cost::from_milliweight(319),
            Elements::OutputAmount => Cost::from_milliweight(298),
            Elements::OutputAmountsHash => Cost::from_milliweight(140),
            Elements::OutputAsset => Cost::from_milliweight(170),
            Elements::OutputHash => Cost::from_milliweight(2849),
            Elements::OutputIsFee => Cost::from_milliweight(92),
            Elements::OutputNonce => Cost::from_milliweight(196),
            Elements::OutputNoncesHash => Cost::from_milliweight(151),
            Elements::OutputNullDatum => Cost::from_milliweight(87),
            Elements::OutputRangeProof => Cost::from_milliweight(154),
            Elements::OutputRangeProofsHash => Cost::from_milliweight(136),
            Elements::OutputScriptHash => Cost::from_milliweight(151),
            Elements::OutputScriptsHash => Cost::from_milliweight(142),
            Elements::OutputSurjectionProof => Cost::from_milliweight(151),
            Elements::OutputSurjectionProofsHash => Cost::from_milliweight(138),
            Elements::OutputsHash => Cost::from_milliweight(135),
            Elements::ParseLock => Cost::from_milliweight(97),
            Elements::ParseSequence => Cost::from_milliweight(116),
            Elements::PointVerify1 => Cost::from_milliweight(41494),
            Elements::ReissuanceBlinding => Cost::from_milliweight(91),
            Elements::ReissuanceEntropy => Cost::from_milliweight(93),
            Elements::RightExtend16_32 => Cost::from_milliweight(74),
            Elements::RightExtend16_64 => Cost::from_milliweight(82),
            Elements::RightExtend32_64 => Cost::from_milliweight(94),
            Elements::RightExtend8_16 => Cost::from_milliweight(76),
            Elements::RightExtend8_32 => Cost::from_milliweight(106),
            Elements::RightExtend8_64 => Cost::from_milliweight(124),
            Elements::RightPadHigh16_32 => Cost::from_milliweight(70),
            Elements::RightPadHigh16_64 => Cost::from_milliweight(88),
            Elements::RightPadHigh1_16 => Cost::from_milliweight(143),
            Elements::RightPadHigh1_32 => Cost::from_milliweight(223),
            Elements::RightPadHigh1_64 => Cost::from_milliweight(476),
            Elements::RightPadHigh1_8 => Cost::from_milliweight(107),
            Elements::RightPadHigh32_64 => Cost::from_milliweight(94),
            Elements::RightPadHigh8_16 => Cost::from_milliweight(89),
            Elements::RightPadHigh8_32 => Cost::from_milliweight(110),
            Elements::RightPadHigh8_64 => Cost::from_milliweight(107),
            Elements::RightPadLow16_32 => Cost::from_milliweight(71),
            Elements::RightPadLow16_64 => Cost::from_milliweight(96),
            Elements::RightPadLow1_16 => Cost::from_milliweight(81),
            Elements::RightPadLow1_32 => Cost::from_milliweight(75),
            Elements::RightPadLow1_64 => Cost::from_milliweight(73),
            Elements::RightPadLow1_8 => Cost::from_milliweight(68),
            Elements::RightPadLow32_64 => Cost::from_milliweight(80),
            Elements::RightPadLow8_16 => Cost::from_milliweight(75),
            Elements::RightPadLow8_32 => Cost::from_milliweight(77),
            Elements::RightPadLow8_64 => Cost::from_milliweight(82),
            Elements::RightRotate16 => Cost::from_milliweight(99),
            Elements::RightRotate32 => Cost::from_milliweight(92),
            Elements::RightRotate64 => Cost::from_milliweight(93),
            Elements::RightRotate8 => Cost::from_milliweight(75),
            Elements::RightShift16 => Cost::from_milliweight(84),
            Elements::RightShift32 => Cost::from_milliweight(88),
            Elements::RightShift64 => Cost::from_milliweight(91),
            Elements::RightShift8 => Cost::from_milliweight(88),
            Elements::RightShiftWith16 => Cost::from_milliweight(105),
            Elements::RightShiftWith32 => Cost::from_milliweight(92),
            Elements::RightShiftWith64 => Cost::from_milliweight(97),
            Elements::RightShiftWith8 => Cost::from_milliweight(103),
            Elements::Rightmost16_1 => Cost::from_milliweight(70),
            Elements::Rightmost16_2 => Cost::from_milliweight(82),
            Elements::Rightmost16_4 => Cost::from_milliweight(76),
            Elements::Rightmost16_8 => Cost::from_milliweight(69),
            Elements::Rightmost32_1 => Cost::from_milliweight(90),
            Elements::Rightmost32_16 => Cost::from_milliweight(64),
            Elements::Rightmost32_2 => Cost::from_milliweight(74),
            Elements::Rightmost32_4 => Cost::from_milliweight(92),
            Elements::Rightmost32_8 => Cost::from_milliweight(78),
            Elements::Rightmost64_1 => Cost::from_milliweight(77),
            Elements::Rightmost64_16 => Cost::from_milliweight(86),
            Elements::Rightmost64_2 => Cost::from_milliweight(74),
            Elements::Rightmost64_32 => Cost::from_milliweight(76),
            Elements::Rightmost64_4 => Cost::from_milliweight(70),
            Elements::Rightmost64_8 => Cost::from_milliweight(69),
            Elements::Rightmost8_1 => Cost::from_milliweight(79),
            Elements::Rightmost8_2 => Cost::from_milliweight(98),
            Elements::Rightmost8_4 => Cost::from_milliweight(98),
            Elements::ScalarAdd => Cost::from_milliweight(739),
            Elements::ScalarInvert => Cost::from_milliweight(3193),
            Elements::ScalarIsZero => Cost::from_milliweight(271),
            Elements::ScalarMultiply => Cost::from_milliweight(774),
            Elements::ScalarMultiplyLambda => Cost::from_milliweight(557),
            Elements::ScalarNegate => Cost::from_milliweight(490),
            Elements::ScalarNormalize => Cost::from_milliweight(472),
            Elements::ScalarSquare => Cost::from_milliweight(575),
            Elements::Scale => Cost::from_milliweight(72675),
            Elements::ScriptCMR => Cost::from_milliweight(136),
            Elements::Sha256Block => Cost::from_milliweight(771),
            Elements::Sha256Ctx8Add1 => Cost::from_milliweight(642),
            Elements::Sha256Ctx8Add128 => Cost::from_milliweight(1779),
            Elements::Sha256Ctx8Add16 => Cost::from_milliweight(747),
            Elements::Sha256Ctx8Add2 => Cost::from_milliweight(661),
            Elements::Sha256Ctx8Add256 => Cost::from_milliweight(2912),
            Elements::Sha256Ctx8Add32 => Cost::from_milliweight(896),
            Elements::Sha256Ctx8Add4 => Cost::from_milliweight(645),
            Elements::Sha256Ctx8Add512 => Cost::from_milliweight(5299),
            Elements::Sha256Ctx8Add64 => Cost::from_milliweight(1187),
            Elements::Sha256Ctx8Add8 => Cost::from_milliweight(674),
            Elements::Sha256Ctx8AddBuffer511 => Cost::from_milliweight(5060),
            Elements::Sha256Ctx8Finalize => Cost::from_milliweight(835),
            Elements::Sha256Ctx8Init => Cost::from_milliweight(118),
            Elements::Sha256Iv => Cost::from_milliweight(93),
            Elements::SigAllHash => Cost::from_milliweight(133),
            Elements::Some1 => Cost::from_milliweight(70),
            Elements::Some16 => Cost::from_milliweight(63),
            Elements::Some32 => Cost::from_milliweight(64),
            Elements::Some64 => Cost::from_milliweight(93),
            Elements::Some8 => Cost::from_milliweight(75),
            Elements::Subtract16 => Cost::from_milliweight(113),
            Elements::Subtract32 => Cost::from_milliweight(118),
            Elements::Subtract64 => Cost::from_milliweight(115),
            Elements::Subtract8 => Cost::from_milliweight(109),
            Elements::Swu => Cost::from_milliweight(32120),
            Elements::TapEnvHash => Cost::from_milliweight(162),
            Elements::TapdataInit => Cost::from_milliweight(1178),
            Elements::TapleafHash => Cost::from_milliweight(136),
            Elements::TapleafVersion => Cost::from_milliweight(105),
            Elements::Tappath => Cost::from_milliweight(83),
            Elements::TappathHash => Cost::from_milliweight(143),
            Elements::TotalFee => Cost::from_milliweight(230),
            Elements::TransactionId => Cost::from_milliweight(139),
            Elements::TxHash => Cost::from_milliweight(143),
            Elements::TxIsFinal => Cost::from_milliweight(71),
            Elements::TxLockDistance => Cost::from_milliweight(91),
            Elements::TxLockDuration => Cost::from_milliweight(84),
            Elements::TxLockHeight => Cost::from_milliweight(80),
            Elements::TxLockTime => Cost::from_milliweight(80),
            Elements::Verify => Cost::from_milliweight(57),
            Elements::Version => Cost::from_milliweight(93),
            Elements::Xor1 => Cost::from_milliweight(67),
            Elements::Xor16 => Cost::from_milliweight(83),
            Elements::Xor32 => Cost::from_milliweight(92),
            Elements::Xor64 => Cost::from_milliweight(95),
            Elements::Xor8 => Cost::from_milliweight(85),
            Elements::XorXor1 => Cost::from_milliweight(72),
            Elements::XorXor16 => Cost::from_milliweight(79),
            Elements::XorXor32 => Cost::from_milliweight(96),
            Elements::XorXor64 => Cost::from_milliweight(93),
            Elements::XorXor8 => Cost::from_milliweight(98),
        }
    }
}

impl fmt::Display for Elements {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Elements::Add16 => f.write_str("add_16"),
            Elements::Add32 => f.write_str("add_32"),
            Elements::Add64 => f.write_str("add_64"),
            Elements::Add8 => f.write_str("add_8"),
            Elements::All16 => f.write_str("all_16"),
            Elements::All32 => f.write_str("all_32"),
            Elements::All64 => f.write_str("all_64"),
            Elements::All8 => f.write_str("all_8"),
            Elements::And1 => f.write_str("and_1"),
            Elements::And16 => f.write_str("and_16"),
            Elements::And32 => f.write_str("and_32"),
            Elements::And64 => f.write_str("and_64"),
            Elements::And8 => f.write_str("and_8"),
            Elements::AnnexHash => f.write_str("annex_hash"),
            Elements::AssetAmountHash => f.write_str("asset_amount_hash"),
            Elements::Bip0340Verify => f.write_str("bip_0340_verify"),
            Elements::BuildTapbranch => f.write_str("build_tapbranch"),
            Elements::BuildTapleafSimplicity => f.write_str("build_tapleaf_simplicity"),
            Elements::BuildTaptweak => f.write_str("build_taptweak"),
            Elements::CalculateAsset => f.write_str("calculate_asset"),
            Elements::CalculateConfidentialToken => f.write_str("calculate_confidential_token"),
            Elements::CalculateExplicitToken => f.write_str("calculate_explicit_token"),
            Elements::CalculateIssuanceEntropy => f.write_str("calculate_issuance_entropy"),
            Elements::Ch1 => f.write_str("ch_1"),
            Elements::Ch16 => f.write_str("ch_16"),
            Elements::Ch32 => f.write_str("ch_32"),
            Elements::Ch64 => f.write_str("ch_64"),
            Elements::Ch8 => f.write_str("ch_8"),
            Elements::CheckLockDistance => f.write_str("check_lock_distance"),
            Elements::CheckLockDuration => f.write_str("check_lock_duration"),
            Elements::CheckLockHeight => f.write_str("check_lock_height"),
            Elements::CheckLockTime => f.write_str("check_lock_time"),
            Elements::CheckSigVerify => f.write_str("check_sig_verify"),
            Elements::Complement1 => f.write_str("complement_1"),
            Elements::Complement16 => f.write_str("complement_16"),
            Elements::Complement32 => f.write_str("complement_32"),
            Elements::Complement64 => f.write_str("complement_64"),
            Elements::Complement8 => f.write_str("complement_8"),
            Elements::CurrentAmount => f.write_str("current_amount"),
            Elements::CurrentAnnexHash => f.write_str("current_annex_hash"),
            Elements::CurrentAsset => f.write_str("current_asset"),
            Elements::CurrentIndex => f.write_str("current_index"),
            Elements::CurrentIssuanceAssetAmount => f.write_str("current_issuance_asset_amount"),
            Elements::CurrentIssuanceAssetProof => f.write_str("current_issuance_asset_proof"),
            Elements::CurrentIssuanceTokenAmount => f.write_str("current_issuance_token_amount"),
            Elements::CurrentIssuanceTokenProof => f.write_str("current_issuance_token_proof"),
            Elements::CurrentNewIssuanceContract => f.write_str("current_new_issuance_contract"),
            Elements::CurrentPegin => f.write_str("current_pegin"),
            Elements::CurrentPrevOutpoint => f.write_str("current_prev_outpoint"),
            Elements::CurrentReissuanceBlinding => f.write_str("current_reissuance_blinding"),
            Elements::CurrentReissuanceEntropy => f.write_str("current_reissuance_entropy"),
            Elements::CurrentScriptHash => f.write_str("current_script_hash"),
            Elements::CurrentScriptSigHash => f.write_str("current_script_sig_hash"),
            Elements::CurrentSequence => f.write_str("current_sequence"),
            Elements::Decompress => f.write_str("decompress"),
            Elements::Decrement16 => f.write_str("decrement_16"),
            Elements::Decrement32 => f.write_str("decrement_32"),
            Elements::Decrement64 => f.write_str("decrement_64"),
            Elements::Decrement8 => f.write_str("decrement_8"),
            Elements::DivMod128_64 => f.write_str("div_mod_128_64"),
            Elements::DivMod16 => f.write_str("div_mod_16"),
            Elements::DivMod32 => f.write_str("div_mod_32"),
            Elements::DivMod64 => f.write_str("div_mod_64"),
            Elements::DivMod8 => f.write_str("div_mod_8"),
            Elements::Divide16 => f.write_str("divide_16"),
            Elements::Divide32 => f.write_str("divide_32"),
            Elements::Divide64 => f.write_str("divide_64"),
            Elements::Divide8 => f.write_str("divide_8"),
            Elements::Divides16 => f.write_str("divides_16"),
            Elements::Divides32 => f.write_str("divides_32"),
            Elements::Divides64 => f.write_str("divides_64"),
            Elements::Divides8 => f.write_str("divides_8"),
            Elements::Eq1 => f.write_str("eq_1"),
            Elements::Eq16 => f.write_str("eq_16"),
            Elements::Eq256 => f.write_str("eq_256"),
            Elements::Eq32 => f.write_str("eq_32"),
            Elements::Eq64 => f.write_str("eq_64"),
            Elements::Eq8 => f.write_str("eq_8"),
            Elements::FeAdd => f.write_str("fe_add"),
            Elements::FeInvert => f.write_str("fe_invert"),
            Elements::FeIsOdd => f.write_str("fe_is_odd"),
            Elements::FeIsZero => f.write_str("fe_is_zero"),
            Elements::FeMultiply => f.write_str("fe_multiply"),
            Elements::FeMultiplyBeta => f.write_str("fe_multiply_beta"),
            Elements::FeNegate => f.write_str("fe_negate"),
            Elements::FeNormalize => f.write_str("fe_normalize"),
            Elements::FeSquare => f.write_str("fe_square"),
            Elements::FeSquareRoot => f.write_str("fe_square_root"),
            Elements::FullAdd16 => f.write_str("full_add_16"),
            Elements::FullAdd32 => f.write_str("full_add_32"),
            Elements::FullAdd64 => f.write_str("full_add_64"),
            Elements::FullAdd8 => f.write_str("full_add_8"),
            Elements::FullDecrement16 => f.write_str("full_decrement_16"),
            Elements::FullDecrement32 => f.write_str("full_decrement_32"),
            Elements::FullDecrement64 => f.write_str("full_decrement_64"),
            Elements::FullDecrement8 => f.write_str("full_decrement_8"),
            Elements::FullIncrement16 => f.write_str("full_increment_16"),
            Elements::FullIncrement32 => f.write_str("full_increment_32"),
            Elements::FullIncrement64 => f.write_str("full_increment_64"),
            Elements::FullIncrement8 => f.write_str("full_increment_8"),
            Elements::FullLeftShift16_1 => f.write_str("full_left_shift_16_1"),
            Elements::FullLeftShift16_2 => f.write_str("full_left_shift_16_2"),
            Elements::FullLeftShift16_4 => f.write_str("full_left_shift_16_4"),
            Elements::FullLeftShift16_8 => f.write_str("full_left_shift_16_8"),
            Elements::FullLeftShift32_1 => f.write_str("full_left_shift_32_1"),
            Elements::FullLeftShift32_16 => f.write_str("full_left_shift_32_16"),
            Elements::FullLeftShift32_2 => f.write_str("full_left_shift_32_2"),
            Elements::FullLeftShift32_4 => f.write_str("full_left_shift_32_4"),
            Elements::FullLeftShift32_8 => f.write_str("full_left_shift_32_8"),
            Elements::FullLeftShift64_1 => f.write_str("full_left_shift_64_1"),
            Elements::FullLeftShift64_16 => f.write_str("full_left_shift_64_16"),
            Elements::FullLeftShift64_2 => f.write_str("full_left_shift_64_2"),
            Elements::FullLeftShift64_32 => f.write_str("full_left_shift_64_32"),
            Elements::FullLeftShift64_4 => f.write_str("full_left_shift_64_4"),
            Elements::FullLeftShift64_8 => f.write_str("full_left_shift_64_8"),
            Elements::FullLeftShift8_1 => f.write_str("full_left_shift_8_1"),
            Elements::FullLeftShift8_2 => f.write_str("full_left_shift_8_2"),
            Elements::FullLeftShift8_4 => f.write_str("full_left_shift_8_4"),
            Elements::FullMultiply16 => f.write_str("full_multiply_16"),
            Elements::FullMultiply32 => f.write_str("full_multiply_32"),
            Elements::FullMultiply64 => f.write_str("full_multiply_64"),
            Elements::FullMultiply8 => f.write_str("full_multiply_8"),
            Elements::FullRightShift16_1 => f.write_str("full_right_shift_16_1"),
            Elements::FullRightShift16_2 => f.write_str("full_right_shift_16_2"),
            Elements::FullRightShift16_4 => f.write_str("full_right_shift_16_4"),
            Elements::FullRightShift16_8 => f.write_str("full_right_shift_16_8"),
            Elements::FullRightShift32_1 => f.write_str("full_right_shift_32_1"),
            Elements::FullRightShift32_16 => f.write_str("full_right_shift_32_16"),
            Elements::FullRightShift32_2 => f.write_str("full_right_shift_32_2"),
            Elements::FullRightShift32_4 => f.write_str("full_right_shift_32_4"),
            Elements::FullRightShift32_8 => f.write_str("full_right_shift_32_8"),
            Elements::FullRightShift64_1 => f.write_str("full_right_shift_64_1"),
            Elements::FullRightShift64_16 => f.write_str("full_right_shift_64_16"),
            Elements::FullRightShift64_2 => f.write_str("full_right_shift_64_2"),
            Elements::FullRightShift64_32 => f.write_str("full_right_shift_64_32"),
            Elements::FullRightShift64_4 => f.write_str("full_right_shift_64_4"),
            Elements::FullRightShift64_8 => f.write_str("full_right_shift_64_8"),
            Elements::FullRightShift8_1 => f.write_str("full_right_shift_8_1"),
            Elements::FullRightShift8_2 => f.write_str("full_right_shift_8_2"),
            Elements::FullRightShift8_4 => f.write_str("full_right_shift_8_4"),
            Elements::FullSubtract16 => f.write_str("full_subtract_16"),
            Elements::FullSubtract32 => f.write_str("full_subtract_32"),
            Elements::FullSubtract64 => f.write_str("full_subtract_64"),
            Elements::FullSubtract8 => f.write_str("full_subtract_8"),
            Elements::GeIsOnCurve => f.write_str("ge_is_on_curve"),
            Elements::GeNegate => f.write_str("ge_negate"),
            Elements::GejAdd => f.write_str("gej_add"),
            Elements::GejDouble => f.write_str("gej_double"),
            Elements::GejEquiv => f.write_str("gej_equiv"),
            Elements::GejGeAdd => f.write_str("gej_ge_add"),
            Elements::GejGeAddEx => f.write_str("gej_ge_add_ex"),
            Elements::GejGeEquiv => f.write_str("gej_ge_equiv"),
            Elements::GejInfinity => f.write_str("gej_infinity"),
            Elements::GejIsInfinity => f.write_str("gej_is_infinity"),
            Elements::GejIsOnCurve => f.write_str("gej_is_on_curve"),
            Elements::GejNegate => f.write_str("gej_negate"),
            Elements::GejNormalize => f.write_str("gej_normalize"),
            Elements::GejRescale => f.write_str("gej_rescale"),
            Elements::GejXEquiv => f.write_str("gej_x_equiv"),
            Elements::GejYIsOdd => f.write_str("gej_y_is_odd"),
            Elements::Generate => f.write_str("generate"),
            Elements::GenesisBlockHash => f.write_str("genesis_block_hash"),
            Elements::HashToCurve => f.write_str("hash_to_curve"),
            Elements::High1 => f.write_str("high_1"),
            Elements::High16 => f.write_str("high_16"),
            Elements::High32 => f.write_str("high_32"),
            Elements::High64 => f.write_str("high_64"),
            Elements::High8 => f.write_str("high_8"),
            Elements::Increment16 => f.write_str("increment_16"),
            Elements::Increment32 => f.write_str("increment_32"),
            Elements::Increment64 => f.write_str("increment_64"),
            Elements::Increment8 => f.write_str("increment_8"),
            Elements::InputAmount => f.write_str("input_amount"),
            Elements::InputAmountsHash => f.write_str("input_amounts_hash"),
            Elements::InputAnnexHash => f.write_str("input_annex_hash"),
            Elements::InputAnnexesHash => f.write_str("input_annexes_hash"),
            Elements::InputAsset => f.write_str("input_asset"),
            Elements::InputHash => f.write_str("input_hash"),
            Elements::InputOutpointsHash => f.write_str("input_outpoints_hash"),
            Elements::InputPegin => f.write_str("input_pegin"),
            Elements::InputPrevOutpoint => f.write_str("input_prev_outpoint"),
            Elements::InputScriptHash => f.write_str("input_script_hash"),
            Elements::InputScriptSigHash => f.write_str("input_script_sig_hash"),
            Elements::InputScriptSigsHash => f.write_str("input_script_sigs_hash"),
            Elements::InputScriptsHash => f.write_str("input_scripts_hash"),
            Elements::InputSequence => f.write_str("input_sequence"),
            Elements::InputSequencesHash => f.write_str("input_sequences_hash"),
            Elements::InputUtxoHash => f.write_str("input_utxo_hash"),
            Elements::InputUtxosHash => f.write_str("input_utxos_hash"),
            Elements::InputsHash => f.write_str("inputs_hash"),
            Elements::InternalKey => f.write_str("internal_key"),
            Elements::IsOne16 => f.write_str("is_one_16"),
            Elements::IsOne32 => f.write_str("is_one_32"),
            Elements::IsOne64 => f.write_str("is_one_64"),
            Elements::IsOne8 => f.write_str("is_one_8"),
            Elements::IsZero16 => f.write_str("is_zero_16"),
            Elements::IsZero32 => f.write_str("is_zero_32"),
            Elements::IsZero64 => f.write_str("is_zero_64"),
            Elements::IsZero8 => f.write_str("is_zero_8"),
            Elements::Issuance => f.write_str("issuance"),
            Elements::IssuanceAsset => f.write_str("issuance_asset"),
            Elements::IssuanceAssetAmount => f.write_str("issuance_asset_amount"),
            Elements::IssuanceAssetAmountsHash => f.write_str("issuance_asset_amounts_hash"),
            Elements::IssuanceAssetProof => f.write_str("issuance_asset_proof"),
            Elements::IssuanceBlindingEntropyHash => f.write_str("issuance_blinding_entropy_hash"),
            Elements::IssuanceEntropy => f.write_str("issuance_entropy"),
            Elements::IssuanceHash => f.write_str("issuance_hash"),
            Elements::IssuanceRangeProofsHash => f.write_str("issuance_range_proofs_hash"),
            Elements::IssuanceToken => f.write_str("issuance_token"),
            Elements::IssuanceTokenAmount => f.write_str("issuance_token_amount"),
            Elements::IssuanceTokenAmountsHash => f.write_str("issuance_token_amounts_hash"),
            Elements::IssuanceTokenProof => f.write_str("issuance_token_proof"),
            Elements::IssuancesHash => f.write_str("issuances_hash"),
            Elements::LbtcAsset => f.write_str("lbtc_asset"),
            Elements::Le16 => f.write_str("le_16"),
            Elements::Le32 => f.write_str("le_32"),
            Elements::Le64 => f.write_str("le_64"),
            Elements::Le8 => f.write_str("le_8"),
            Elements::LeftExtend16_32 => f.write_str("left_extend_16_32"),
            Elements::LeftExtend16_64 => f.write_str("left_extend_16_64"),
            Elements::LeftExtend1_16 => f.write_str("left_extend_1_16"),
            Elements::LeftExtend1_32 => f.write_str("left_extend_1_32"),
            Elements::LeftExtend1_64 => f.write_str("left_extend_1_64"),
            Elements::LeftExtend1_8 => f.write_str("left_extend_1_8"),
            Elements::LeftExtend32_64 => f.write_str("left_extend_32_64"),
            Elements::LeftExtend8_16 => f.write_str("left_extend_8_16"),
            Elements::LeftExtend8_32 => f.write_str("left_extend_8_32"),
            Elements::LeftExtend8_64 => f.write_str("left_extend_8_64"),
            Elements::LeftPadHigh16_32 => f.write_str("left_pad_high_16_32"),
            Elements::LeftPadHigh16_64 => f.write_str("left_pad_high_16_64"),
            Elements::LeftPadHigh1_16 => f.write_str("left_pad_high_1_16"),
            Elements::LeftPadHigh1_32 => f.write_str("left_pad_high_1_32"),
            Elements::LeftPadHigh1_64 => f.write_str("left_pad_high_1_64"),
            Elements::LeftPadHigh1_8 => f.write_str("left_pad_high_1_8"),
            Elements::LeftPadHigh32_64 => f.write_str("left_pad_high_32_64"),
            Elements::LeftPadHigh8_16 => f.write_str("left_pad_high_8_16"),
            Elements::LeftPadHigh8_32 => f.write_str("left_pad_high_8_32"),
            Elements::LeftPadHigh8_64 => f.write_str("left_pad_high_8_64"),
            Elements::LeftPadLow16_32 => f.write_str("left_pad_low_16_32"),
            Elements::LeftPadLow16_64 => f.write_str("left_pad_low_16_64"),
            Elements::LeftPadLow1_16 => f.write_str("left_pad_low_1_16"),
            Elements::LeftPadLow1_32 => f.write_str("left_pad_low_1_32"),
            Elements::LeftPadLow1_64 => f.write_str("left_pad_low_1_64"),
            Elements::LeftPadLow1_8 => f.write_str("left_pad_low_1_8"),
            Elements::LeftPadLow32_64 => f.write_str("left_pad_low_32_64"),
            Elements::LeftPadLow8_16 => f.write_str("left_pad_low_8_16"),
            Elements::LeftPadLow8_32 => f.write_str("left_pad_low_8_32"),
            Elements::LeftPadLow8_64 => f.write_str("left_pad_low_8_64"),
            Elements::LeftRotate16 => f.write_str("left_rotate_16"),
            Elements::LeftRotate32 => f.write_str("left_rotate_32"),
            Elements::LeftRotate64 => f.write_str("left_rotate_64"),
            Elements::LeftRotate8 => f.write_str("left_rotate_8"),
            Elements::LeftShift16 => f.write_str("left_shift_16"),
            Elements::LeftShift32 => f.write_str("left_shift_32"),
            Elements::LeftShift64 => f.write_str("left_shift_64"),
            Elements::LeftShift8 => f.write_str("left_shift_8"),
            Elements::LeftShiftWith16 => f.write_str("left_shift_with_16"),
            Elements::LeftShiftWith32 => f.write_str("left_shift_with_32"),
            Elements::LeftShiftWith64 => f.write_str("left_shift_with_64"),
            Elements::LeftShiftWith8 => f.write_str("left_shift_with_8"),
            Elements::Leftmost16_1 => f.write_str("leftmost_16_1"),
            Elements::Leftmost16_2 => f.write_str("leftmost_16_2"),
            Elements::Leftmost16_4 => f.write_str("leftmost_16_4"),
            Elements::Leftmost16_8 => f.write_str("leftmost_16_8"),
            Elements::Leftmost32_1 => f.write_str("leftmost_32_1"),
            Elements::Leftmost32_16 => f.write_str("leftmost_32_16"),
            Elements::Leftmost32_2 => f.write_str("leftmost_32_2"),
            Elements::Leftmost32_4 => f.write_str("leftmost_32_4"),
            Elements::Leftmost32_8 => f.write_str("leftmost_32_8"),
            Elements::Leftmost64_1 => f.write_str("leftmost_64_1"),
            Elements::Leftmost64_16 => f.write_str("leftmost_64_16"),
            Elements::Leftmost64_2 => f.write_str("leftmost_64_2"),
            Elements::Leftmost64_32 => f.write_str("leftmost_64_32"),
            Elements::Leftmost64_4 => f.write_str("leftmost_64_4"),
            Elements::Leftmost64_8 => f.write_str("leftmost_64_8"),
            Elements::Leftmost8_1 => f.write_str("leftmost_8_1"),
            Elements::Leftmost8_2 => f.write_str("leftmost_8_2"),
            Elements::Leftmost8_4 => f.write_str("leftmost_8_4"),
            Elements::LinearCombination1 => f.write_str("linear_combination_1"),
            Elements::LinearVerify1 => f.write_str("linear_verify_1"),
            Elements::LockTime => f.write_str("lock_time"),
            Elements::Low1 => f.write_str("low_1"),
            Elements::Low16 => f.write_str("low_16"),
            Elements::Low32 => f.write_str("low_32"),
            Elements::Low64 => f.write_str("low_64"),
            Elements::Low8 => f.write_str("low_8"),
            Elements::Lt16 => f.write_str("lt_16"),
            Elements::Lt32 => f.write_str("lt_32"),
            Elements::Lt64 => f.write_str("lt_64"),
            Elements::Lt8 => f.write_str("lt_8"),
            Elements::Maj1 => f.write_str("maj_1"),
            Elements::Maj16 => f.write_str("maj_16"),
            Elements::Maj32 => f.write_str("maj_32"),
            Elements::Maj64 => f.write_str("maj_64"),
            Elements::Maj8 => f.write_str("maj_8"),
            Elements::Max16 => f.write_str("max_16"),
            Elements::Max32 => f.write_str("max_32"),
            Elements::Max64 => f.write_str("max_64"),
            Elements::Max8 => f.write_str("max_8"),
            Elements::Median16 => f.write_str("median_16"),
            Elements::Median32 => f.write_str("median_32"),
            Elements::Median64 => f.write_str("median_64"),
            Elements::Median8 => f.write_str("median_8"),
            Elements::Min16 => f.write_str("min_16"),
            Elements::Min32 => f.write_str("min_32"),
            Elements::Min64 => f.write_str("min_64"),
            Elements::Min8 => f.write_str("min_8"),
            Elements::Modulo16 => f.write_str("modulo_16"),
            Elements::Modulo32 => f.write_str("modulo_32"),
            Elements::Modulo64 => f.write_str("modulo_64"),
            Elements::Modulo8 => f.write_str("modulo_8"),
            Elements::Multiply16 => f.write_str("multiply_16"),
            Elements::Multiply32 => f.write_str("multiply_32"),
            Elements::Multiply64 => f.write_str("multiply_64"),
            Elements::Multiply8 => f.write_str("multiply_8"),
            Elements::Negate16 => f.write_str("negate_16"),
            Elements::Negate32 => f.write_str("negate_32"),
            Elements::Negate64 => f.write_str("negate_64"),
            Elements::Negate8 => f.write_str("negate_8"),
            Elements::NewIssuanceContract => f.write_str("new_issuance_contract"),
            Elements::NonceHash => f.write_str("nonce_hash"),
            Elements::NumInputs => f.write_str("num_inputs"),
            Elements::NumOutputs => f.write_str("num_outputs"),
            Elements::One16 => f.write_str("one_16"),
            Elements::One32 => f.write_str("one_32"),
            Elements::One64 => f.write_str("one_64"),
            Elements::One8 => f.write_str("one_8"),
            Elements::Or1 => f.write_str("or_1"),
            Elements::Or16 => f.write_str("or_16"),
            Elements::Or32 => f.write_str("or_32"),
            Elements::Or64 => f.write_str("or_64"),
            Elements::Or8 => f.write_str("or_8"),
            Elements::OutpointHash => f.write_str("outpoint_hash"),
            Elements::OutputAmount => f.write_str("output_amount"),
            Elements::OutputAmountsHash => f.write_str("output_amounts_hash"),
            Elements::OutputAsset => f.write_str("output_asset"),
            Elements::OutputHash => f.write_str("output_hash"),
            Elements::OutputIsFee => f.write_str("output_is_fee"),
            Elements::OutputNonce => f.write_str("output_nonce"),
            Elements::OutputNoncesHash => f.write_str("output_nonces_hash"),
            Elements::OutputNullDatum => f.write_str("output_null_datum"),
            Elements::OutputRangeProof => f.write_str("output_range_proof"),
            Elements::OutputRangeProofsHash => f.write_str("output_range_proofs_hash"),
            Elements::OutputScriptHash => f.write_str("output_script_hash"),
            Elements::OutputScriptsHash => f.write_str("output_scripts_hash"),
            Elements::OutputSurjectionProof => f.write_str("output_surjection_proof"),
            Elements::OutputSurjectionProofsHash => f.write_str("output_surjection_proofs_hash"),
            Elements::OutputsHash => f.write_str("outputs_hash"),
            Elements::ParseLock => f.write_str("parse_lock"),
            Elements::ParseSequence => f.write_str("parse_sequence"),
            Elements::PointVerify1 => f.write_str("point_verify_1"),
            Elements::ReissuanceBlinding => f.write_str("reissuance_blinding"),
            Elements::ReissuanceEntropy => f.write_str("reissuance_entropy"),
            Elements::RightExtend16_32 => f.write_str("right_extend_16_32"),
            Elements::RightExtend16_64 => f.write_str("right_extend_16_64"),
            Elements::RightExtend32_64 => f.write_str("right_extend_32_64"),
            Elements::RightExtend8_16 => f.write_str("right_extend_8_16"),
            Elements::RightExtend8_32 => f.write_str("right_extend_8_32"),
            Elements::RightExtend8_64 => f.write_str("right_extend_8_64"),
            Elements::RightPadHigh16_32 => f.write_str("right_pad_high_16_32"),
            Elements::RightPadHigh16_64 => f.write_str("right_pad_high_16_64"),
            Elements::RightPadHigh1_16 => f.write_str("right_pad_high_1_16"),
            Elements::RightPadHigh1_32 => f.write_str("right_pad_high_1_32"),
            Elements::RightPadHigh1_64 => f.write_str("right_pad_high_1_64"),
            Elements::RightPadHigh1_8 => f.write_str("right_pad_high_1_8"),
            Elements::RightPadHigh32_64 => f.write_str("right_pad_high_32_64"),
            Elements::RightPadHigh8_16 => f.write_str("right_pad_high_8_16"),
            Elements::RightPadHigh8_32 => f.write_str("right_pad_high_8_32"),
            Elements::RightPadHigh8_64 => f.write_str("right_pad_high_8_64"),
            Elements::RightPadLow16_32 => f.write_str("right_pad_low_16_32"),
            Elements::RightPadLow16_64 => f.write_str("right_pad_low_16_64"),
            Elements::RightPadLow1_16 => f.write_str("right_pad_low_1_16"),
            Elements::RightPadLow1_32 => f.write_str("right_pad_low_1_32"),
            Elements::RightPadLow1_64 => f.write_str("right_pad_low_1_64"),
            Elements::RightPadLow1_8 => f.write_str("right_pad_low_1_8"),
            Elements::RightPadLow32_64 => f.write_str("right_pad_low_32_64"),
            Elements::RightPadLow8_16 => f.write_str("right_pad_low_8_16"),
            Elements::RightPadLow8_32 => f.write_str("right_pad_low_8_32"),
            Elements::RightPadLow8_64 => f.write_str("right_pad_low_8_64"),
            Elements::RightRotate16 => f.write_str("right_rotate_16"),
            Elements::RightRotate32 => f.write_str("right_rotate_32"),
            Elements::RightRotate64 => f.write_str("right_rotate_64"),
            Elements::RightRotate8 => f.write_str("right_rotate_8"),
            Elements::RightShift16 => f.write_str("right_shift_16"),
            Elements::RightShift32 => f.write_str("right_shift_32"),
            Elements::RightShift64 => f.write_str("right_shift_64"),
            Elements::RightShift8 => f.write_str("right_shift_8"),
            Elements::RightShiftWith16 => f.write_str("right_shift_with_16"),
            Elements::RightShiftWith32 => f.write_str("right_shift_with_32"),
            Elements::RightShiftWith64 => f.write_str("right_shift_with_64"),
            Elements::RightShiftWith8 => f.write_str("right_shift_with_8"),
            Elements::Rightmost16_1 => f.write_str("rightmost_16_1"),
            Elements::Rightmost16_2 => f.write_str("rightmost_16_2"),
            Elements::Rightmost16_4 => f.write_str("rightmost_16_4"),
            Elements::Rightmost16_8 => f.write_str("rightmost_16_8"),
            Elements::Rightmost32_1 => f.write_str("rightmost_32_1"),
            Elements::Rightmost32_16 => f.write_str("rightmost_32_16"),
            Elements::Rightmost32_2 => f.write_str("rightmost_32_2"),
            Elements::Rightmost32_4 => f.write_str("rightmost_32_4"),
            Elements::Rightmost32_8 => f.write_str("rightmost_32_8"),
            Elements::Rightmost64_1 => f.write_str("rightmost_64_1"),
            Elements::Rightmost64_16 => f.write_str("rightmost_64_16"),
            Elements::Rightmost64_2 => f.write_str("rightmost_64_2"),
            Elements::Rightmost64_32 => f.write_str("rightmost_64_32"),
            Elements::Rightmost64_4 => f.write_str("rightmost_64_4"),
            Elements::Rightmost64_8 => f.write_str("rightmost_64_8"),
            Elements::Rightmost8_1 => f.write_str("rightmost_8_1"),
            Elements::Rightmost8_2 => f.write_str("rightmost_8_2"),
            Elements::Rightmost8_4 => f.write_str("rightmost_8_4"),
            Elements::ScalarAdd => f.write_str("scalar_add"),
            Elements::ScalarInvert => f.write_str("scalar_invert"),
            Elements::ScalarIsZero => f.write_str("scalar_is_zero"),
            Elements::ScalarMultiply => f.write_str("scalar_multiply"),
            Elements::ScalarMultiplyLambda => f.write_str("scalar_multiply_lambda"),
            Elements::ScalarNegate => f.write_str("scalar_negate"),
            Elements::ScalarNormalize => f.write_str("scalar_normalize"),
            Elements::ScalarSquare => f.write_str("scalar_square"),
            Elements::Scale => f.write_str("scale"),
            Elements::ScriptCMR => f.write_str("script_cmr"),
            Elements::Sha256Block => f.write_str("sha_256_block"),
            Elements::Sha256Ctx8Add1 => f.write_str("sha_256_ctx_8_add_1"),
            Elements::Sha256Ctx8Add128 => f.write_str("sha_256_ctx_8_add_128"),
            Elements::Sha256Ctx8Add16 => f.write_str("sha_256_ctx_8_add_16"),
            Elements::Sha256Ctx8Add2 => f.write_str("sha_256_ctx_8_add_2"),
            Elements::Sha256Ctx8Add256 => f.write_str("sha_256_ctx_8_add_256"),
            Elements::Sha256Ctx8Add32 => f.write_str("sha_256_ctx_8_add_32"),
            Elements::Sha256Ctx8Add4 => f.write_str("sha_256_ctx_8_add_4"),
            Elements::Sha256Ctx8Add512 => f.write_str("sha_256_ctx_8_add_512"),
            Elements::Sha256Ctx8Add64 => f.write_str("sha_256_ctx_8_add_64"),
            Elements::Sha256Ctx8Add8 => f.write_str("sha_256_ctx_8_add_8"),
            Elements::Sha256Ctx8AddBuffer511 => f.write_str("sha_256_ctx_8_add_buffer_511"),
            Elements::Sha256Ctx8Finalize => f.write_str("sha_256_ctx_8_finalize"),
            Elements::Sha256Ctx8Init => f.write_str("sha_256_ctx_8_init"),
            Elements::Sha256Iv => f.write_str("sha_256_iv"),
            Elements::SigAllHash => f.write_str("sig_all_hash"),
            Elements::Some1 => f.write_str("some_1"),
            Elements::Some16 => f.write_str("some_16"),
            Elements::Some32 => f.write_str("some_32"),
            Elements::Some64 => f.write_str("some_64"),
            Elements::Some8 => f.write_str("some_8"),
            Elements::Subtract16 => f.write_str("subtract_16"),
            Elements::Subtract32 => f.write_str("subtract_32"),
            Elements::Subtract64 => f.write_str("subtract_64"),
            Elements::Subtract8 => f.write_str("subtract_8"),
            Elements::Swu => f.write_str("swu"),
            Elements::TapEnvHash => f.write_str("tap_env_hash"),
            Elements::TapdataInit => f.write_str("tapdata_init"),
            Elements::TapleafHash => f.write_str("tapleaf_hash"),
            Elements::TapleafVersion => f.write_str("tapleaf_version"),
            Elements::Tappath => f.write_str("tappath"),
            Elements::TappathHash => f.write_str("tappath_hash"),
            Elements::TotalFee => f.write_str("total_fee"),
            Elements::TransactionId => f.write_str("transaction_id"),
            Elements::TxHash => f.write_str("tx_hash"),
            Elements::TxIsFinal => f.write_str("tx_is_final"),
            Elements::TxLockDistance => f.write_str("tx_lock_distance"),
            Elements::TxLockDuration => f.write_str("tx_lock_duration"),
            Elements::TxLockHeight => f.write_str("tx_lock_height"),
            Elements::TxLockTime => f.write_str("tx_lock_time"),
            Elements::Verify => f.write_str("verify"),
            Elements::Version => f.write_str("version"),
            Elements::Xor1 => f.write_str("xor_1"),
            Elements::Xor16 => f.write_str("xor_16"),
            Elements::Xor32 => f.write_str("xor_32"),
            Elements::Xor64 => f.write_str("xor_64"),
            Elements::Xor8 => f.write_str("xor_8"),
            Elements::XorXor1 => f.write_str("xor_xor_1"),
            Elements::XorXor16 => f.write_str("xor_xor_16"),
            Elements::XorXor32 => f.write_str("xor_xor_32"),
            Elements::XorXor64 => f.write_str("xor_xor_64"),
            Elements::XorXor8 => f.write_str("xor_xor_8"),
        }
    }
}

impl str::FromStr for Elements {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add_16" => Ok(Elements::Add16),
            "add_32" => Ok(Elements::Add32),
            "add_64" => Ok(Elements::Add64),
            "add_8" => Ok(Elements::Add8),
            "all_16" => Ok(Elements::All16),
            "all_32" => Ok(Elements::All32),
            "all_64" => Ok(Elements::All64),
            "all_8" => Ok(Elements::All8),
            "and_1" => Ok(Elements::And1),
            "and_16" => Ok(Elements::And16),
            "and_32" => Ok(Elements::And32),
            "and_64" => Ok(Elements::And64),
            "and_8" => Ok(Elements::And8),
            "annex_hash" => Ok(Elements::AnnexHash),
            "asset_amount_hash" => Ok(Elements::AssetAmountHash),
            "bip_0340_verify" => Ok(Elements::Bip0340Verify),
            "build_tapbranch" => Ok(Elements::BuildTapbranch),
            "build_tapleaf_simplicity" => Ok(Elements::BuildTapleafSimplicity),
            "build_taptweak" => Ok(Elements::BuildTaptweak),
            "calculate_asset" => Ok(Elements::CalculateAsset),
            "calculate_confidential_token" => Ok(Elements::CalculateConfidentialToken),
            "calculate_explicit_token" => Ok(Elements::CalculateExplicitToken),
            "calculate_issuance_entropy" => Ok(Elements::CalculateIssuanceEntropy),
            "ch_1" => Ok(Elements::Ch1),
            "ch_16" => Ok(Elements::Ch16),
            "ch_32" => Ok(Elements::Ch32),
            "ch_64" => Ok(Elements::Ch64),
            "ch_8" => Ok(Elements::Ch8),
            "check_lock_distance" => Ok(Elements::CheckLockDistance),
            "check_lock_duration" => Ok(Elements::CheckLockDuration),
            "check_lock_height" => Ok(Elements::CheckLockHeight),
            "check_lock_time" => Ok(Elements::CheckLockTime),
            "check_sig_verify" => Ok(Elements::CheckSigVerify),
            "complement_1" => Ok(Elements::Complement1),
            "complement_16" => Ok(Elements::Complement16),
            "complement_32" => Ok(Elements::Complement32),
            "complement_64" => Ok(Elements::Complement64),
            "complement_8" => Ok(Elements::Complement8),
            "current_amount" => Ok(Elements::CurrentAmount),
            "current_annex_hash" => Ok(Elements::CurrentAnnexHash),
            "current_asset" => Ok(Elements::CurrentAsset),
            "current_index" => Ok(Elements::CurrentIndex),
            "current_issuance_asset_amount" => Ok(Elements::CurrentIssuanceAssetAmount),
            "current_issuance_asset_proof" => Ok(Elements::CurrentIssuanceAssetProof),
            "current_issuance_token_amount" => Ok(Elements::CurrentIssuanceTokenAmount),
            "current_issuance_token_proof" => Ok(Elements::CurrentIssuanceTokenProof),
            "current_new_issuance_contract" => Ok(Elements::CurrentNewIssuanceContract),
            "current_pegin" => Ok(Elements::CurrentPegin),
            "current_prev_outpoint" => Ok(Elements::CurrentPrevOutpoint),
            "current_reissuance_blinding" => Ok(Elements::CurrentReissuanceBlinding),
            "current_reissuance_entropy" => Ok(Elements::CurrentReissuanceEntropy),
            "current_script_hash" => Ok(Elements::CurrentScriptHash),
            "current_script_sig_hash" => Ok(Elements::CurrentScriptSigHash),
            "current_sequence" => Ok(Elements::CurrentSequence),
            "decompress" => Ok(Elements::Decompress),
            "decrement_16" => Ok(Elements::Decrement16),
            "decrement_32" => Ok(Elements::Decrement32),
            "decrement_64" => Ok(Elements::Decrement64),
            "decrement_8" => Ok(Elements::Decrement8),
            "div_mod_128_64" => Ok(Elements::DivMod128_64),
            "div_mod_16" => Ok(Elements::DivMod16),
            "div_mod_32" => Ok(Elements::DivMod32),
            "div_mod_64" => Ok(Elements::DivMod64),
            "div_mod_8" => Ok(Elements::DivMod8),
            "divide_16" => Ok(Elements::Divide16),
            "divide_32" => Ok(Elements::Divide32),
            "divide_64" => Ok(Elements::Divide64),
            "divide_8" => Ok(Elements::Divide8),
            "divides_16" => Ok(Elements::Divides16),
            "divides_32" => Ok(Elements::Divides32),
            "divides_64" => Ok(Elements::Divides64),
            "divides_8" => Ok(Elements::Divides8),
            "eq_1" => Ok(Elements::Eq1),
            "eq_16" => Ok(Elements::Eq16),
            "eq_256" => Ok(Elements::Eq256),
            "eq_32" => Ok(Elements::Eq32),
            "eq_64" => Ok(Elements::Eq64),
            "eq_8" => Ok(Elements::Eq8),
            "fe_add" => Ok(Elements::FeAdd),
            "fe_invert" => Ok(Elements::FeInvert),
            "fe_is_odd" => Ok(Elements::FeIsOdd),
            "fe_is_zero" => Ok(Elements::FeIsZero),
            "fe_multiply" => Ok(Elements::FeMultiply),
            "fe_multiply_beta" => Ok(Elements::FeMultiplyBeta),
            "fe_negate" => Ok(Elements::FeNegate),
            "fe_normalize" => Ok(Elements::FeNormalize),
            "fe_square" => Ok(Elements::FeSquare),
            "fe_square_root" => Ok(Elements::FeSquareRoot),
            "full_add_16" => Ok(Elements::FullAdd16),
            "full_add_32" => Ok(Elements::FullAdd32),
            "full_add_64" => Ok(Elements::FullAdd64),
            "full_add_8" => Ok(Elements::FullAdd8),
            "full_decrement_16" => Ok(Elements::FullDecrement16),
            "full_decrement_32" => Ok(Elements::FullDecrement32),
            "full_decrement_64" => Ok(Elements::FullDecrement64),
            "full_decrement_8" => Ok(Elements::FullDecrement8),
            "full_increment_16" => Ok(Elements::FullIncrement16),
            "full_increment_32" => Ok(Elements::FullIncrement32),
            "full_increment_64" => Ok(Elements::FullIncrement64),
            "full_increment_8" => Ok(Elements::FullIncrement8),
            "full_left_shift_16_1" => Ok(Elements::FullLeftShift16_1),
            "full_left_shift_16_2" => Ok(Elements::FullLeftShift16_2),
            "full_left_shift_16_4" => Ok(Elements::FullLeftShift16_4),
            "full_left_shift_16_8" => Ok(Elements::FullLeftShift16_8),
            "full_left_shift_32_1" => Ok(Elements::FullLeftShift32_1),
            "full_left_shift_32_16" => Ok(Elements::FullLeftShift32_16),
            "full_left_shift_32_2" => Ok(Elements::FullLeftShift32_2),
            "full_left_shift_32_4" => Ok(Elements::FullLeftShift32_4),
            "full_left_shift_32_8" => Ok(Elements::FullLeftShift32_8),
            "full_left_shift_64_1" => Ok(Elements::FullLeftShift64_1),
            "full_left_shift_64_16" => Ok(Elements::FullLeftShift64_16),
            "full_left_shift_64_2" => Ok(Elements::FullLeftShift64_2),
            "full_left_shift_64_32" => Ok(Elements::FullLeftShift64_32),
            "full_left_shift_64_4" => Ok(Elements::FullLeftShift64_4),
            "full_left_shift_64_8" => Ok(Elements::FullLeftShift64_8),
            "full_left_shift_8_1" => Ok(Elements::FullLeftShift8_1),
            "full_left_shift_8_2" => Ok(Elements::FullLeftShift8_2),
            "full_left_shift_8_4" => Ok(Elements::FullLeftShift8_4),
            "full_multiply_16" => Ok(Elements::FullMultiply16),
            "full_multiply_32" => Ok(Elements::FullMultiply32),
            "full_multiply_64" => Ok(Elements::FullMultiply64),
            "full_multiply_8" => Ok(Elements::FullMultiply8),
            "full_right_shift_16_1" => Ok(Elements::FullRightShift16_1),
            "full_right_shift_16_2" => Ok(Elements::FullRightShift16_2),
            "full_right_shift_16_4" => Ok(Elements::FullRightShift16_4),
            "full_right_shift_16_8" => Ok(Elements::FullRightShift16_8),
            "full_right_shift_32_1" => Ok(Elements::FullRightShift32_1),
            "full_right_shift_32_16" => Ok(Elements::FullRightShift32_16),
            "full_right_shift_32_2" => Ok(Elements::FullRightShift32_2),
            "full_right_shift_32_4" => Ok(Elements::FullRightShift32_4),
            "full_right_shift_32_8" => Ok(Elements::FullRightShift32_8),
            "full_right_shift_64_1" => Ok(Elements::FullRightShift64_1),
            "full_right_shift_64_16" => Ok(Elements::FullRightShift64_16),
            "full_right_shift_64_2" => Ok(Elements::FullRightShift64_2),
            "full_right_shift_64_32" => Ok(Elements::FullRightShift64_32),
            "full_right_shift_64_4" => Ok(Elements::FullRightShift64_4),
            "full_right_shift_64_8" => Ok(Elements::FullRightShift64_8),
            "full_right_shift_8_1" => Ok(Elements::FullRightShift8_1),
            "full_right_shift_8_2" => Ok(Elements::FullRightShift8_2),
            "full_right_shift_8_4" => Ok(Elements::FullRightShift8_4),
            "full_subtract_16" => Ok(Elements::FullSubtract16),
            "full_subtract_32" => Ok(Elements::FullSubtract32),
            "full_subtract_64" => Ok(Elements::FullSubtract64),
            "full_subtract_8" => Ok(Elements::FullSubtract8),
            "ge_is_on_curve" => Ok(Elements::GeIsOnCurve),
            "ge_negate" => Ok(Elements::GeNegate),
            "gej_add" => Ok(Elements::GejAdd),
            "gej_double" => Ok(Elements::GejDouble),
            "gej_equiv" => Ok(Elements::GejEquiv),
            "gej_ge_add" => Ok(Elements::GejGeAdd),
            "gej_ge_add_ex" => Ok(Elements::GejGeAddEx),
            "gej_ge_equiv" => Ok(Elements::GejGeEquiv),
            "gej_infinity" => Ok(Elements::GejInfinity),
            "gej_is_infinity" => Ok(Elements::GejIsInfinity),
            "gej_is_on_curve" => Ok(Elements::GejIsOnCurve),
            "gej_negate" => Ok(Elements::GejNegate),
            "gej_normalize" => Ok(Elements::GejNormalize),
            "gej_rescale" => Ok(Elements::GejRescale),
            "gej_x_equiv" => Ok(Elements::GejXEquiv),
            "gej_y_is_odd" => Ok(Elements::GejYIsOdd),
            "generate" => Ok(Elements::Generate),
            "genesis_block_hash" => Ok(Elements::GenesisBlockHash),
            "hash_to_curve" => Ok(Elements::HashToCurve),
            "high_1" => Ok(Elements::High1),
            "high_16" => Ok(Elements::High16),
            "high_32" => Ok(Elements::High32),
            "high_64" => Ok(Elements::High64),
            "high_8" => Ok(Elements::High8),
            "increment_16" => Ok(Elements::Increment16),
            "increment_32" => Ok(Elements::Increment32),
            "increment_64" => Ok(Elements::Increment64),
            "increment_8" => Ok(Elements::Increment8),
            "input_amount" => Ok(Elements::InputAmount),
            "input_amounts_hash" => Ok(Elements::InputAmountsHash),
            "input_annex_hash" => Ok(Elements::InputAnnexHash),
            "input_annexes_hash" => Ok(Elements::InputAnnexesHash),
            "input_asset" => Ok(Elements::InputAsset),
            "input_hash" => Ok(Elements::InputHash),
            "input_outpoints_hash" => Ok(Elements::InputOutpointsHash),
            "input_pegin" => Ok(Elements::InputPegin),
            "input_prev_outpoint" => Ok(Elements::InputPrevOutpoint),
            "input_script_hash" => Ok(Elements::InputScriptHash),
            "input_script_sig_hash" => Ok(Elements::InputScriptSigHash),
            "input_script_sigs_hash" => Ok(Elements::InputScriptSigsHash),
            "input_scripts_hash" => Ok(Elements::InputScriptsHash),
            "input_sequence" => Ok(Elements::InputSequence),
            "input_sequences_hash" => Ok(Elements::InputSequencesHash),
            "input_utxo_hash" => Ok(Elements::InputUtxoHash),
            "input_utxos_hash" => Ok(Elements::InputUtxosHash),
            "inputs_hash" => Ok(Elements::InputsHash),
            "internal_key" => Ok(Elements::InternalKey),
            "is_one_16" => Ok(Elements::IsOne16),
            "is_one_32" => Ok(Elements::IsOne32),
            "is_one_64" => Ok(Elements::IsOne64),
            "is_one_8" => Ok(Elements::IsOne8),
            "is_zero_16" => Ok(Elements::IsZero16),
            "is_zero_32" => Ok(Elements::IsZero32),
            "is_zero_64" => Ok(Elements::IsZero64),
            "is_zero_8" => Ok(Elements::IsZero8),
            "issuance" => Ok(Elements::Issuance),
            "issuance_asset" => Ok(Elements::IssuanceAsset),
            "issuance_asset_amount" => Ok(Elements::IssuanceAssetAmount),
            "issuance_asset_amounts_hash" => Ok(Elements::IssuanceAssetAmountsHash),
            "issuance_asset_proof" => Ok(Elements::IssuanceAssetProof),
            "issuance_blinding_entropy_hash" => Ok(Elements::IssuanceBlindingEntropyHash),
            "issuance_entropy" => Ok(Elements::IssuanceEntropy),
            "issuance_hash" => Ok(Elements::IssuanceHash),
            "issuance_range_proofs_hash" => Ok(Elements::IssuanceRangeProofsHash),
            "issuance_token" => Ok(Elements::IssuanceToken),
            "issuance_token_amount" => Ok(Elements::IssuanceTokenAmount),
            "issuance_token_amounts_hash" => Ok(Elements::IssuanceTokenAmountsHash),
            "issuance_token_proof" => Ok(Elements::IssuanceTokenProof),
            "issuances_hash" => Ok(Elements::IssuancesHash),
            "lbtc_asset" => Ok(Elements::LbtcAsset),
            "le_16" => Ok(Elements::Le16),
            "le_32" => Ok(Elements::Le32),
            "le_64" => Ok(Elements::Le64),
            "le_8" => Ok(Elements::Le8),
            "left_extend_16_32" => Ok(Elements::LeftExtend16_32),
            "left_extend_16_64" => Ok(Elements::LeftExtend16_64),
            "left_extend_1_16" => Ok(Elements::LeftExtend1_16),
            "left_extend_1_32" => Ok(Elements::LeftExtend1_32),
            "left_extend_1_64" => Ok(Elements::LeftExtend1_64),
            "left_extend_1_8" => Ok(Elements::LeftExtend1_8),
            "left_extend_32_64" => Ok(Elements::LeftExtend32_64),
            "left_extend_8_16" => Ok(Elements::LeftExtend8_16),
            "left_extend_8_32" => Ok(Elements::LeftExtend8_32),
            "left_extend_8_64" => Ok(Elements::LeftExtend8_64),
            "left_pad_high_16_32" => Ok(Elements::LeftPadHigh16_32),
            "left_pad_high_16_64" => Ok(Elements::LeftPadHigh16_64),
            "left_pad_high_1_16" => Ok(Elements::LeftPadHigh1_16),
            "left_pad_high_1_32" => Ok(Elements::LeftPadHigh1_32),
            "left_pad_high_1_64" => Ok(Elements::LeftPadHigh1_64),
            "left_pad_high_1_8" => Ok(Elements::LeftPadHigh1_8),
            "left_pad_high_32_64" => Ok(Elements::LeftPadHigh32_64),
            "left_pad_high_8_16" => Ok(Elements::LeftPadHigh8_16),
            "left_pad_high_8_32" => Ok(Elements::LeftPadHigh8_32),
            "left_pad_high_8_64" => Ok(Elements::LeftPadHigh8_64),
            "left_pad_low_16_32" => Ok(Elements::LeftPadLow16_32),
            "left_pad_low_16_64" => Ok(Elements::LeftPadLow16_64),
            "left_pad_low_1_16" => Ok(Elements::LeftPadLow1_16),
            "left_pad_low_1_32" => Ok(Elements::LeftPadLow1_32),
            "left_pad_low_1_64" => Ok(Elements::LeftPadLow1_64),
            "left_pad_low_1_8" => Ok(Elements::LeftPadLow1_8),
            "left_pad_low_32_64" => Ok(Elements::LeftPadLow32_64),
            "left_pad_low_8_16" => Ok(Elements::LeftPadLow8_16),
            "left_pad_low_8_32" => Ok(Elements::LeftPadLow8_32),
            "left_pad_low_8_64" => Ok(Elements::LeftPadLow8_64),
            "left_rotate_16" => Ok(Elements::LeftRotate16),
            "left_rotate_32" => Ok(Elements::LeftRotate32),
            "left_rotate_64" => Ok(Elements::LeftRotate64),
            "left_rotate_8" => Ok(Elements::LeftRotate8),
            "left_shift_16" => Ok(Elements::LeftShift16),
            "left_shift_32" => Ok(Elements::LeftShift32),
            "left_shift_64" => Ok(Elements::LeftShift64),
            "left_shift_8" => Ok(Elements::LeftShift8),
            "left_shift_with_16" => Ok(Elements::LeftShiftWith16),
            "left_shift_with_32" => Ok(Elements::LeftShiftWith32),
            "left_shift_with_64" => Ok(Elements::LeftShiftWith64),
            "left_shift_with_8" => Ok(Elements::LeftShiftWith8),
            "leftmost_16_1" => Ok(Elements::Leftmost16_1),
            "leftmost_16_2" => Ok(Elements::Leftmost16_2),
            "leftmost_16_4" => Ok(Elements::Leftmost16_4),
            "leftmost_16_8" => Ok(Elements::Leftmost16_8),
            "leftmost_32_1" => Ok(Elements::Leftmost32_1),
            "leftmost_32_16" => Ok(Elements::Leftmost32_16),
            "leftmost_32_2" => Ok(Elements::Leftmost32_2),
            "leftmost_32_4" => Ok(Elements::Leftmost32_4),
            "leftmost_32_8" => Ok(Elements::Leftmost32_8),
            "leftmost_64_1" => Ok(Elements::Leftmost64_1),
            "leftmost_64_16" => Ok(Elements::Leftmost64_16),
            "leftmost_64_2" => Ok(Elements::Leftmost64_2),
            "leftmost_64_32" => Ok(Elements::Leftmost64_32),
            "leftmost_64_4" => Ok(Elements::Leftmost64_4),
            "leftmost_64_8" => Ok(Elements::Leftmost64_8),
            "leftmost_8_1" => Ok(Elements::Leftmost8_1),
            "leftmost_8_2" => Ok(Elements::Leftmost8_2),
            "leftmost_8_4" => Ok(Elements::Leftmost8_4),
            "linear_combination_1" => Ok(Elements::LinearCombination1),
            "linear_verify_1" => Ok(Elements::LinearVerify1),
            "lock_time" => Ok(Elements::LockTime),
            "low_1" => Ok(Elements::Low1),
            "low_16" => Ok(Elements::Low16),
            "low_32" => Ok(Elements::Low32),
            "low_64" => Ok(Elements::Low64),
            "low_8" => Ok(Elements::Low8),
            "lt_16" => Ok(Elements::Lt16),
            "lt_32" => Ok(Elements::Lt32),
            "lt_64" => Ok(Elements::Lt64),
            "lt_8" => Ok(Elements::Lt8),
            "maj_1" => Ok(Elements::Maj1),
            "maj_16" => Ok(Elements::Maj16),
            "maj_32" => Ok(Elements::Maj32),
            "maj_64" => Ok(Elements::Maj64),
            "maj_8" => Ok(Elements::Maj8),
            "max_16" => Ok(Elements::Max16),
            "max_32" => Ok(Elements::Max32),
            "max_64" => Ok(Elements::Max64),
            "max_8" => Ok(Elements::Max8),
            "median_16" => Ok(Elements::Median16),
            "median_32" => Ok(Elements::Median32),
            "median_64" => Ok(Elements::Median64),
            "median_8" => Ok(Elements::Median8),
            "min_16" => Ok(Elements::Min16),
            "min_32" => Ok(Elements::Min32),
            "min_64" => Ok(Elements::Min64),
            "min_8" => Ok(Elements::Min8),
            "modulo_16" => Ok(Elements::Modulo16),
            "modulo_32" => Ok(Elements::Modulo32),
            "modulo_64" => Ok(Elements::Modulo64),
            "modulo_8" => Ok(Elements::Modulo8),
            "multiply_16" => Ok(Elements::Multiply16),
            "multiply_32" => Ok(Elements::Multiply32),
            "multiply_64" => Ok(Elements::Multiply64),
            "multiply_8" => Ok(Elements::Multiply8),
            "negate_16" => Ok(Elements::Negate16),
            "negate_32" => Ok(Elements::Negate32),
            "negate_64" => Ok(Elements::Negate64),
            "negate_8" => Ok(Elements::Negate8),
            "new_issuance_contract" => Ok(Elements::NewIssuanceContract),
            "nonce_hash" => Ok(Elements::NonceHash),
            "num_inputs" => Ok(Elements::NumInputs),
            "num_outputs" => Ok(Elements::NumOutputs),
            "one_16" => Ok(Elements::One16),
            "one_32" => Ok(Elements::One32),
            "one_64" => Ok(Elements::One64),
            "one_8" => Ok(Elements::One8),
            "or_1" => Ok(Elements::Or1),
            "or_16" => Ok(Elements::Or16),
            "or_32" => Ok(Elements::Or32),
            "or_64" => Ok(Elements::Or64),
            "or_8" => Ok(Elements::Or8),
            "outpoint_hash" => Ok(Elements::OutpointHash),
            "output_amount" => Ok(Elements::OutputAmount),
            "output_amounts_hash" => Ok(Elements::OutputAmountsHash),
            "output_asset" => Ok(Elements::OutputAsset),
            "output_hash" => Ok(Elements::OutputHash),
            "output_is_fee" => Ok(Elements::OutputIsFee),
            "output_nonce" => Ok(Elements::OutputNonce),
            "output_nonces_hash" => Ok(Elements::OutputNoncesHash),
            "output_null_datum" => Ok(Elements::OutputNullDatum),
            "output_range_proof" => Ok(Elements::OutputRangeProof),
            "output_range_proofs_hash" => Ok(Elements::OutputRangeProofsHash),
            "output_script_hash" => Ok(Elements::OutputScriptHash),
            "output_scripts_hash" => Ok(Elements::OutputScriptsHash),
            "output_surjection_proof" => Ok(Elements::OutputSurjectionProof),
            "output_surjection_proofs_hash" => Ok(Elements::OutputSurjectionProofsHash),
            "outputs_hash" => Ok(Elements::OutputsHash),
            "parse_lock" => Ok(Elements::ParseLock),
            "parse_sequence" => Ok(Elements::ParseSequence),
            "point_verify_1" => Ok(Elements::PointVerify1),
            "reissuance_blinding" => Ok(Elements::ReissuanceBlinding),
            "reissuance_entropy" => Ok(Elements::ReissuanceEntropy),
            "right_extend_16_32" => Ok(Elements::RightExtend16_32),
            "right_extend_16_64" => Ok(Elements::RightExtend16_64),
            "right_extend_32_64" => Ok(Elements::RightExtend32_64),
            "right_extend_8_16" => Ok(Elements::RightExtend8_16),
            "right_extend_8_32" => Ok(Elements::RightExtend8_32),
            "right_extend_8_64" => Ok(Elements::RightExtend8_64),
            "right_pad_high_16_32" => Ok(Elements::RightPadHigh16_32),
            "right_pad_high_16_64" => Ok(Elements::RightPadHigh16_64),
            "right_pad_high_1_16" => Ok(Elements::RightPadHigh1_16),
            "right_pad_high_1_32" => Ok(Elements::RightPadHigh1_32),
            "right_pad_high_1_64" => Ok(Elements::RightPadHigh1_64),
            "right_pad_high_1_8" => Ok(Elements::RightPadHigh1_8),
            "right_pad_high_32_64" => Ok(Elements::RightPadHigh32_64),
            "right_pad_high_8_16" => Ok(Elements::RightPadHigh8_16),
            "right_pad_high_8_32" => Ok(Elements::RightPadHigh8_32),
            "right_pad_high_8_64" => Ok(Elements::RightPadHigh8_64),
            "right_pad_low_16_32" => Ok(Elements::RightPadLow16_32),
            "right_pad_low_16_64" => Ok(Elements::RightPadLow16_64),
            "right_pad_low_1_16" => Ok(Elements::RightPadLow1_16),
            "right_pad_low_1_32" => Ok(Elements::RightPadLow1_32),
            "right_pad_low_1_64" => Ok(Elements::RightPadLow1_64),
            "right_pad_low_1_8" => Ok(Elements::RightPadLow1_8),
            "right_pad_low_32_64" => Ok(Elements::RightPadLow32_64),
            "right_pad_low_8_16" => Ok(Elements::RightPadLow8_16),
            "right_pad_low_8_32" => Ok(Elements::RightPadLow8_32),
            "right_pad_low_8_64" => Ok(Elements::RightPadLow8_64),
            "right_rotate_16" => Ok(Elements::RightRotate16),
            "right_rotate_32" => Ok(Elements::RightRotate32),
            "right_rotate_64" => Ok(Elements::RightRotate64),
            "right_rotate_8" => Ok(Elements::RightRotate8),
            "right_shift_16" => Ok(Elements::RightShift16),
            "right_shift_32" => Ok(Elements::RightShift32),
            "right_shift_64" => Ok(Elements::RightShift64),
            "right_shift_8" => Ok(Elements::RightShift8),
            "right_shift_with_16" => Ok(Elements::RightShiftWith16),
            "right_shift_with_32" => Ok(Elements::RightShiftWith32),
            "right_shift_with_64" => Ok(Elements::RightShiftWith64),
            "right_shift_with_8" => Ok(Elements::RightShiftWith8),
            "rightmost_16_1" => Ok(Elements::Rightmost16_1),
            "rightmost_16_2" => Ok(Elements::Rightmost16_2),
            "rightmost_16_4" => Ok(Elements::Rightmost16_4),
            "rightmost_16_8" => Ok(Elements::Rightmost16_8),
            "rightmost_32_1" => Ok(Elements::Rightmost32_1),
            "rightmost_32_16" => Ok(Elements::Rightmost32_16),
            "rightmost_32_2" => Ok(Elements::Rightmost32_2),
            "rightmost_32_4" => Ok(Elements::Rightmost32_4),
            "rightmost_32_8" => Ok(Elements::Rightmost32_8),
            "rightmost_64_1" => Ok(Elements::Rightmost64_1),
            "rightmost_64_16" => Ok(Elements::Rightmost64_16),
            "rightmost_64_2" => Ok(Elements::Rightmost64_2),
            "rightmost_64_32" => Ok(Elements::Rightmost64_32),
            "rightmost_64_4" => Ok(Elements::Rightmost64_4),
            "rightmost_64_8" => Ok(Elements::Rightmost64_8),
            "rightmost_8_1" => Ok(Elements::Rightmost8_1),
            "rightmost_8_2" => Ok(Elements::Rightmost8_2),
            "rightmost_8_4" => Ok(Elements::Rightmost8_4),
            "scalar_add" => Ok(Elements::ScalarAdd),
            "scalar_invert" => Ok(Elements::ScalarInvert),
            "scalar_is_zero" => Ok(Elements::ScalarIsZero),
            "scalar_multiply" => Ok(Elements::ScalarMultiply),
            "scalar_multiply_lambda" => Ok(Elements::ScalarMultiplyLambda),
            "scalar_negate" => Ok(Elements::ScalarNegate),
            "scalar_normalize" => Ok(Elements::ScalarNormalize),
            "scalar_square" => Ok(Elements::ScalarSquare),
            "scale" => Ok(Elements::Scale),
            "script_cmr" => Ok(Elements::ScriptCMR),
            "sha_256_block" => Ok(Elements::Sha256Block),
            "sha_256_ctx_8_add_1" => Ok(Elements::Sha256Ctx8Add1),
            "sha_256_ctx_8_add_128" => Ok(Elements::Sha256Ctx8Add128),
            "sha_256_ctx_8_add_16" => Ok(Elements::Sha256Ctx8Add16),
            "sha_256_ctx_8_add_2" => Ok(Elements::Sha256Ctx8Add2),
            "sha_256_ctx_8_add_256" => Ok(Elements::Sha256Ctx8Add256),
            "sha_256_ctx_8_add_32" => Ok(Elements::Sha256Ctx8Add32),
            "sha_256_ctx_8_add_4" => Ok(Elements::Sha256Ctx8Add4),
            "sha_256_ctx_8_add_512" => Ok(Elements::Sha256Ctx8Add512),
            "sha_256_ctx_8_add_64" => Ok(Elements::Sha256Ctx8Add64),
            "sha_256_ctx_8_add_8" => Ok(Elements::Sha256Ctx8Add8),
            "sha_256_ctx_8_add_buffer_511" => Ok(Elements::Sha256Ctx8AddBuffer511),
            "sha_256_ctx_8_finalize" => Ok(Elements::Sha256Ctx8Finalize),
            "sha_256_ctx_8_init" => Ok(Elements::Sha256Ctx8Init),
            "sha_256_iv" => Ok(Elements::Sha256Iv),
            "sig_all_hash" => Ok(Elements::SigAllHash),
            "some_1" => Ok(Elements::Some1),
            "some_16" => Ok(Elements::Some16),
            "some_32" => Ok(Elements::Some32),
            "some_64" => Ok(Elements::Some64),
            "some_8" => Ok(Elements::Some8),
            "subtract_16" => Ok(Elements::Subtract16),
            "subtract_32" => Ok(Elements::Subtract32),
            "subtract_64" => Ok(Elements::Subtract64),
            "subtract_8" => Ok(Elements::Subtract8),
            "swu" => Ok(Elements::Swu),
            "tap_env_hash" => Ok(Elements::TapEnvHash),
            "tapdata_init" => Ok(Elements::TapdataInit),
            "tapleaf_hash" => Ok(Elements::TapleafHash),
            "tapleaf_version" => Ok(Elements::TapleafVersion),
            "tappath" => Ok(Elements::Tappath),
            "tappath_hash" => Ok(Elements::TappathHash),
            "total_fee" => Ok(Elements::TotalFee),
            "transaction_id" => Ok(Elements::TransactionId),
            "tx_hash" => Ok(Elements::TxHash),
            "tx_is_final" => Ok(Elements::TxIsFinal),
            "tx_lock_distance" => Ok(Elements::TxLockDistance),
            "tx_lock_duration" => Ok(Elements::TxLockDuration),
            "tx_lock_height" => Ok(Elements::TxLockHeight),
            "tx_lock_time" => Ok(Elements::TxLockTime),
            "verify" => Ok(Elements::Verify),
            "version" => Ok(Elements::Version),
            "xor_1" => Ok(Elements::Xor1),
            "xor_16" => Ok(Elements::Xor16),
            "xor_32" => Ok(Elements::Xor32),
            "xor_64" => Ok(Elements::Xor64),
            "xor_8" => Ok(Elements::Xor8),
            "xor_xor_1" => Ok(Elements::XorXor1),
            "xor_xor_16" => Ok(Elements::XorXor16),
            "xor_xor_32" => Ok(Elements::XorXor32),
            "xor_xor_64" => Ok(Elements::XorXor64),
            "xor_xor_8" => Ok(Elements::XorXor8),
            x => Err(crate::Error::InvalidJetName(x.to_owned())),
        }
    }
}
