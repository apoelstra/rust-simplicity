/* This file has been automatically generated. */

use crate::bitcoin;
use crate::elements;
use crate::CFrameItem;
use super::jets_ffi;

pub fn add_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::add_16(dst, &src, std::ptr::null()) }
}

pub fn add_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::add_32(dst, &src, std::ptr::null()) }
}

pub fn add_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::add_64(dst, &src, std::ptr::null()) }
}

pub fn add_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::add_8(dst, &src, std::ptr::null()) }
}

pub fn all_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::all_16(dst, &src, std::ptr::null()) }
}

pub fn all_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::all_32(dst, &src, std::ptr::null()) }
}

pub fn all_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::all_64(dst, &src, std::ptr::null()) }
}

pub fn all_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::all_8(dst, &src, std::ptr::null()) }
}

pub fn and_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::and_1(dst, &src, std::ptr::null()) }
}

pub fn and_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::and_16(dst, &src, std::ptr::null()) }
}

pub fn and_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::and_32(dst, &src, std::ptr::null()) }
}

pub fn and_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::and_64(dst, &src, std::ptr::null()) }
}

pub fn and_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::and_8(dst, &src, std::ptr::null()) }
}

pub fn bitcoin_annex_hash(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_annex_hash(dst, &src, env) }
}

pub fn annex_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::annex_hash(dst, &src, env) }
}

pub fn asset_amount_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::asset_amount_hash(dst, &src, env) }
}

pub fn bip_0340_verify<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::bip_0340_verify(dst, &src, std::ptr::null()) }
}

pub fn bitcoin_build_tapbranch(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_build_tapbranch(dst, &src, env) }
}

pub fn build_tapbranch(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::build_tapbranch(dst, &src, env) }
}

pub fn bitcoin_build_tapleaf_simplicity(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_build_tapleaf_simplicity(dst, &src, env) }
}

pub fn build_tapleaf_simplicity(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::build_tapleaf_simplicity(dst, &src, env) }
}

pub fn bitcoin_build_taptweak(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_build_taptweak(dst, &src, env) }
}

pub fn build_taptweak(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::build_taptweak(dst, &src, env) }
}

pub fn calculate_asset(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::calculate_asset(dst, &src, env) }
}

pub fn calculate_confidential_token(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::calculate_confidential_token(dst, &src, env) }
}

pub fn calculate_explicit_token(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::calculate_explicit_token(dst, &src, env) }
}

pub fn calculate_issuance_entropy(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::calculate_issuance_entropy(dst, &src, env) }
}

pub fn ch_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::ch_1(dst, &src, std::ptr::null()) }
}

pub fn ch_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::ch_16(dst, &src, std::ptr::null()) }
}

pub fn ch_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::ch_32(dst, &src, std::ptr::null()) }
}

pub fn ch_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::ch_64(dst, &src, std::ptr::null()) }
}

pub fn ch_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::ch_8(dst, &src, std::ptr::null()) }
}

pub fn bitcoin_check_lock_distance(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_check_lock_distance(dst, &src, env) }
}

pub fn check_lock_distance(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::check_lock_distance(dst, &src, env) }
}

pub fn bitcoin_check_lock_duration(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_check_lock_duration(dst, &src, env) }
}

pub fn check_lock_duration(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::check_lock_duration(dst, &src, env) }
}

pub fn bitcoin_check_lock_height(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_check_lock_height(dst, &src, env) }
}

pub fn check_lock_height(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::check_lock_height(dst, &src, env) }
}

pub fn bitcoin_check_lock_time(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_check_lock_time(dst, &src, env) }
}

pub fn check_lock_time(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::check_lock_time(dst, &src, env) }
}

pub fn check_sig_verify<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::check_sig_verify(dst, &src, std::ptr::null()) }
}

pub fn complement_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::complement_1(dst, &src, std::ptr::null()) }
}

pub fn complement_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::complement_16(dst, &src, std::ptr::null()) }
}

pub fn complement_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::complement_32(dst, &src, std::ptr::null()) }
}

pub fn complement_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::complement_64(dst, &src, std::ptr::null()) }
}

pub fn complement_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::complement_8(dst, &src, std::ptr::null()) }
}

pub fn current_amount(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::current_amount(dst, &src, env) }
}

pub fn bitcoin_current_annex_hash(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_current_annex_hash(dst, &src, env) }
}

pub fn current_annex_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::current_annex_hash(dst, &src, env) }
}

pub fn current_asset(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::current_asset(dst, &src, env) }
}

pub fn bitcoin_current_index(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_current_index(dst, &src, env) }
}

pub fn current_index(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::current_index(dst, &src, env) }
}

pub fn current_issuance_asset_amount(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::current_issuance_asset_amount(dst, &src, env) }
}

pub fn current_issuance_asset_proof(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::current_issuance_asset_proof(dst, &src, env) }
}

pub fn current_issuance_token_amount(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::current_issuance_token_amount(dst, &src, env) }
}

pub fn current_issuance_token_proof(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::current_issuance_token_proof(dst, &src, env) }
}

pub fn current_new_issuance_contract(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::current_new_issuance_contract(dst, &src, env) }
}

pub fn current_pegin(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::current_pegin(dst, &src, env) }
}

pub fn bitcoin_current_prev_outpoint(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_current_prev_outpoint(dst, &src, env) }
}

pub fn current_prev_outpoint(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::current_prev_outpoint(dst, &src, env) }
}

pub fn current_reissuance_blinding(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::current_reissuance_blinding(dst, &src, env) }
}

pub fn current_reissuance_entropy(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::current_reissuance_entropy(dst, &src, env) }
}

pub fn bitcoin_current_script_hash(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_current_script_hash(dst, &src, env) }
}

pub fn current_script_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::current_script_hash(dst, &src, env) }
}

pub fn bitcoin_current_script_sig_hash(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_current_script_sig_hash(dst, &src, env) }
}

pub fn current_script_sig_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::current_script_sig_hash(dst, &src, env) }
}

pub fn bitcoin_current_sequence(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_current_sequence(dst, &src, env) }
}

pub fn current_sequence(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::current_sequence(dst, &src, env) }
}

pub fn bitcoin_current_value(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_current_value(dst, &src, env) }
}

pub fn decompress<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::decompress(dst, &src, std::ptr::null()) }
}

pub fn decrement_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::decrement_16(dst, &src, std::ptr::null()) }
}

pub fn decrement_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::decrement_32(dst, &src, std::ptr::null()) }
}

pub fn decrement_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::decrement_64(dst, &src, std::ptr::null()) }
}

pub fn decrement_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::decrement_8(dst, &src, std::ptr::null()) }
}

pub fn div_mod_128_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::div_mod_128_64(dst, &src, std::ptr::null()) }
}

pub fn div_mod_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::div_mod_16(dst, &src, std::ptr::null()) }
}

pub fn div_mod_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::div_mod_32(dst, &src, std::ptr::null()) }
}

pub fn div_mod_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::div_mod_64(dst, &src, std::ptr::null()) }
}

pub fn div_mod_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::div_mod_8(dst, &src, std::ptr::null()) }
}

pub fn divide_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::divide_16(dst, &src, std::ptr::null()) }
}

pub fn divide_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::divide_32(dst, &src, std::ptr::null()) }
}

pub fn divide_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::divide_64(dst, &src, std::ptr::null()) }
}

pub fn divide_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::divide_8(dst, &src, std::ptr::null()) }
}

pub fn divides_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::divides_16(dst, &src, std::ptr::null()) }
}

pub fn divides_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::divides_32(dst, &src, std::ptr::null()) }
}

pub fn divides_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::divides_64(dst, &src, std::ptr::null()) }
}

pub fn divides_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::divides_8(dst, &src, std::ptr::null()) }
}

pub fn eq_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::eq_1(dst, &src, std::ptr::null()) }
}

pub fn eq_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::eq_16(dst, &src, std::ptr::null()) }
}

pub fn eq_256<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::eq_256(dst, &src, std::ptr::null()) }
}

pub fn eq_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::eq_32(dst, &src, std::ptr::null()) }
}

pub fn eq_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::eq_64(dst, &src, std::ptr::null()) }
}

pub fn eq_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::eq_8(dst, &src, std::ptr::null()) }
}

pub fn fe_add<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::fe_add(dst, &src, std::ptr::null()) }
}

pub fn fe_invert<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::fe_invert(dst, &src, std::ptr::null()) }
}

pub fn fe_is_odd<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::fe_is_odd(dst, &src, std::ptr::null()) }
}

pub fn fe_is_zero<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::fe_is_zero(dst, &src, std::ptr::null()) }
}

pub fn fe_multiply<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::fe_multiply(dst, &src, std::ptr::null()) }
}

pub fn fe_multiply_beta<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::fe_multiply_beta(dst, &src, std::ptr::null()) }
}

pub fn fe_negate<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::fe_negate(dst, &src, std::ptr::null()) }
}

pub fn fe_normalize<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::fe_normalize(dst, &src, std::ptr::null()) }
}

pub fn fe_square<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::fe_square(dst, &src, std::ptr::null()) }
}

pub fn fe_square_root<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::fe_square_root(dst, &src, std::ptr::null()) }
}

pub fn bitcoin_fee(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_fee(dst, &src, env) }
}

pub fn full_add_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_add_16(dst, &src, std::ptr::null()) }
}

pub fn full_add_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_add_32(dst, &src, std::ptr::null()) }
}

pub fn full_add_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_add_64(dst, &src, std::ptr::null()) }
}

pub fn full_add_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_add_8(dst, &src, std::ptr::null()) }
}

pub fn full_decrement_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_decrement_16(dst, &src, std::ptr::null()) }
}

pub fn full_decrement_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_decrement_32(dst, &src, std::ptr::null()) }
}

pub fn full_decrement_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_decrement_64(dst, &src, std::ptr::null()) }
}

pub fn full_decrement_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_decrement_8(dst, &src, std::ptr::null()) }
}

pub fn full_increment_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_increment_16(dst, &src, std::ptr::null()) }
}

pub fn full_increment_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_increment_32(dst, &src, std::ptr::null()) }
}

pub fn full_increment_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_increment_64(dst, &src, std::ptr::null()) }
}

pub fn full_increment_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_increment_8(dst, &src, std::ptr::null()) }
}

pub fn full_left_shift_16_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_left_shift_16_1(dst, &src, std::ptr::null()) }
}

pub fn full_left_shift_16_2<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_left_shift_16_2(dst, &src, std::ptr::null()) }
}

pub fn full_left_shift_16_4<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_left_shift_16_4(dst, &src, std::ptr::null()) }
}

pub fn full_left_shift_16_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_left_shift_16_8(dst, &src, std::ptr::null()) }
}

pub fn full_left_shift_32_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_left_shift_32_1(dst, &src, std::ptr::null()) }
}

pub fn full_left_shift_32_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_left_shift_32_16(dst, &src, std::ptr::null()) }
}

pub fn full_left_shift_32_2<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_left_shift_32_2(dst, &src, std::ptr::null()) }
}

pub fn full_left_shift_32_4<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_left_shift_32_4(dst, &src, std::ptr::null()) }
}

pub fn full_left_shift_32_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_left_shift_32_8(dst, &src, std::ptr::null()) }
}

pub fn full_left_shift_64_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_left_shift_64_1(dst, &src, std::ptr::null()) }
}

pub fn full_left_shift_64_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_left_shift_64_16(dst, &src, std::ptr::null()) }
}

pub fn full_left_shift_64_2<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_left_shift_64_2(dst, &src, std::ptr::null()) }
}

pub fn full_left_shift_64_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_left_shift_64_32(dst, &src, std::ptr::null()) }
}

pub fn full_left_shift_64_4<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_left_shift_64_4(dst, &src, std::ptr::null()) }
}

pub fn full_left_shift_64_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_left_shift_64_8(dst, &src, std::ptr::null()) }
}

pub fn full_left_shift_8_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_left_shift_8_1(dst, &src, std::ptr::null()) }
}

pub fn full_left_shift_8_2<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_left_shift_8_2(dst, &src, std::ptr::null()) }
}

pub fn full_left_shift_8_4<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_left_shift_8_4(dst, &src, std::ptr::null()) }
}

pub fn full_multiply_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_multiply_16(dst, &src, std::ptr::null()) }
}

pub fn full_multiply_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_multiply_32(dst, &src, std::ptr::null()) }
}

pub fn full_multiply_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_multiply_64(dst, &src, std::ptr::null()) }
}

pub fn full_multiply_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_multiply_8(dst, &src, std::ptr::null()) }
}

pub fn full_right_shift_16_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_right_shift_16_1(dst, &src, std::ptr::null()) }
}

pub fn full_right_shift_16_2<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_right_shift_16_2(dst, &src, std::ptr::null()) }
}

pub fn full_right_shift_16_4<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_right_shift_16_4(dst, &src, std::ptr::null()) }
}

pub fn full_right_shift_16_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_right_shift_16_8(dst, &src, std::ptr::null()) }
}

pub fn full_right_shift_32_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_right_shift_32_1(dst, &src, std::ptr::null()) }
}

pub fn full_right_shift_32_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_right_shift_32_16(dst, &src, std::ptr::null()) }
}

pub fn full_right_shift_32_2<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_right_shift_32_2(dst, &src, std::ptr::null()) }
}

pub fn full_right_shift_32_4<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_right_shift_32_4(dst, &src, std::ptr::null()) }
}

pub fn full_right_shift_32_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_right_shift_32_8(dst, &src, std::ptr::null()) }
}

pub fn full_right_shift_64_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_right_shift_64_1(dst, &src, std::ptr::null()) }
}

pub fn full_right_shift_64_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_right_shift_64_16(dst, &src, std::ptr::null()) }
}

pub fn full_right_shift_64_2<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_right_shift_64_2(dst, &src, std::ptr::null()) }
}

pub fn full_right_shift_64_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_right_shift_64_32(dst, &src, std::ptr::null()) }
}

pub fn full_right_shift_64_4<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_right_shift_64_4(dst, &src, std::ptr::null()) }
}

pub fn full_right_shift_64_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_right_shift_64_8(dst, &src, std::ptr::null()) }
}

pub fn full_right_shift_8_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_right_shift_8_1(dst, &src, std::ptr::null()) }
}

pub fn full_right_shift_8_2<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_right_shift_8_2(dst, &src, std::ptr::null()) }
}

pub fn full_right_shift_8_4<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_right_shift_8_4(dst, &src, std::ptr::null()) }
}

pub fn full_subtract_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_subtract_16(dst, &src, std::ptr::null()) }
}

pub fn full_subtract_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_subtract_32(dst, &src, std::ptr::null()) }
}

pub fn full_subtract_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_subtract_64(dst, &src, std::ptr::null()) }
}

pub fn full_subtract_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::full_subtract_8(dst, &src, std::ptr::null()) }
}

pub fn ge_is_on_curve<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::ge_is_on_curve(dst, &src, std::ptr::null()) }
}

pub fn ge_negate<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::ge_negate(dst, &src, std::ptr::null()) }
}

pub fn gej_add<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::gej_add(dst, &src, std::ptr::null()) }
}

pub fn gej_double<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::gej_double(dst, &src, std::ptr::null()) }
}

pub fn gej_equiv<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::gej_equiv(dst, &src, std::ptr::null()) }
}

pub fn gej_ge_add<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::gej_ge_add(dst, &src, std::ptr::null()) }
}

pub fn gej_ge_add_ex<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::gej_ge_add_ex(dst, &src, std::ptr::null()) }
}

pub fn gej_ge_equiv<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::gej_ge_equiv(dst, &src, std::ptr::null()) }
}

pub fn gej_infinity<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::gej_infinity(dst, &src, std::ptr::null()) }
}

pub fn gej_is_infinity<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::gej_is_infinity(dst, &src, std::ptr::null()) }
}

pub fn gej_is_on_curve<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::gej_is_on_curve(dst, &src, std::ptr::null()) }
}

pub fn gej_negate<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::gej_negate(dst, &src, std::ptr::null()) }
}

pub fn gej_normalize<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::gej_normalize(dst, &src, std::ptr::null()) }
}

pub fn gej_rescale<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::gej_rescale(dst, &src, std::ptr::null()) }
}

pub fn gej_x_equiv<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::gej_x_equiv(dst, &src, std::ptr::null()) }
}

pub fn gej_y_is_odd<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::gej_y_is_odd(dst, &src, std::ptr::null()) }
}

pub fn generate<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::generate(dst, &src, std::ptr::null()) }
}

pub fn genesis_block_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::genesis_block_hash(dst, &src, env) }
}

pub fn hash_to_curve<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::hash_to_curve(dst, &src, std::ptr::null()) }
}

pub fn high_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::high_1(dst, &src, std::ptr::null()) }
}

pub fn high_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::high_16(dst, &src, std::ptr::null()) }
}

pub fn high_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::high_32(dst, &src, std::ptr::null()) }
}

pub fn high_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::high_64(dst, &src, std::ptr::null()) }
}

pub fn high_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::high_8(dst, &src, std::ptr::null()) }
}

pub fn increment_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::increment_16(dst, &src, std::ptr::null()) }
}

pub fn increment_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::increment_32(dst, &src, std::ptr::null()) }
}

pub fn increment_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::increment_64(dst, &src, std::ptr::null()) }
}

pub fn increment_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::increment_8(dst, &src, std::ptr::null()) }
}

pub fn input_amount(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::input_amount(dst, &src, env) }
}

pub fn input_amounts_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::input_amounts_hash(dst, &src, env) }
}

pub fn bitcoin_input_annex_hash(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_input_annex_hash(dst, &src, env) }
}

pub fn input_annex_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::input_annex_hash(dst, &src, env) }
}

pub fn bitcoin_input_annexes_hash(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_input_annexes_hash(dst, &src, env) }
}

pub fn input_annexes_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::input_annexes_hash(dst, &src, env) }
}

pub fn input_asset(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::input_asset(dst, &src, env) }
}

pub fn bitcoin_input_hash(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_input_hash(dst, &src, env) }
}

pub fn input_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::input_hash(dst, &src, env) }
}

pub fn bitcoin_input_outpoints_hash(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_input_outpoints_hash(dst, &src, env) }
}

pub fn input_outpoints_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::input_outpoints_hash(dst, &src, env) }
}

pub fn input_pegin(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::input_pegin(dst, &src, env) }
}

pub fn bitcoin_input_prev_outpoint(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_input_prev_outpoint(dst, &src, env) }
}

pub fn input_prev_outpoint(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::input_prev_outpoint(dst, &src, env) }
}

pub fn bitcoin_input_script_hash(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_input_script_hash(dst, &src, env) }
}

pub fn input_script_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::input_script_hash(dst, &src, env) }
}

pub fn bitcoin_input_script_sig_hash(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_input_script_sig_hash(dst, &src, env) }
}

pub fn input_script_sig_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::input_script_sig_hash(dst, &src, env) }
}

pub fn bitcoin_input_script_sigs_hash(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_input_script_sigs_hash(dst, &src, env) }
}

pub fn input_script_sigs_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::input_script_sigs_hash(dst, &src, env) }
}

pub fn bitcoin_input_scripts_hash(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_input_scripts_hash(dst, &src, env) }
}

pub fn input_scripts_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::input_scripts_hash(dst, &src, env) }
}

pub fn bitcoin_input_sequence(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_input_sequence(dst, &src, env) }
}

pub fn input_sequence(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::input_sequence(dst, &src, env) }
}

pub fn bitcoin_input_sequences_hash(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_input_sequences_hash(dst, &src, env) }
}

pub fn input_sequences_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::input_sequences_hash(dst, &src, env) }
}

pub fn bitcoin_input_utxo_hash(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_input_utxo_hash(dst, &src, env) }
}

pub fn input_utxo_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::input_utxo_hash(dst, &src, env) }
}

pub fn bitcoin_input_utxos_hash(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_input_utxos_hash(dst, &src, env) }
}

pub fn input_utxos_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::input_utxos_hash(dst, &src, env) }
}

pub fn bitcoin_input_value(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_input_value(dst, &src, env) }
}

pub fn bitcoin_input_values_hash(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_input_values_hash(dst, &src, env) }
}

pub fn bitcoin_inputs_hash(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_inputs_hash(dst, &src, env) }
}

pub fn inputs_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::inputs_hash(dst, &src, env) }
}

pub fn bitcoin_internal_key(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_internal_key(dst, &src, env) }
}

pub fn internal_key(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::internal_key(dst, &src, env) }
}

pub fn is_one_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::is_one_16(dst, &src, std::ptr::null()) }
}

pub fn is_one_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::is_one_32(dst, &src, std::ptr::null()) }
}

pub fn is_one_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::is_one_64(dst, &src, std::ptr::null()) }
}

pub fn is_one_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::is_one_8(dst, &src, std::ptr::null()) }
}

pub fn is_zero_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::is_zero_16(dst, &src, std::ptr::null()) }
}

pub fn is_zero_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::is_zero_32(dst, &src, std::ptr::null()) }
}

pub fn is_zero_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::is_zero_64(dst, &src, std::ptr::null()) }
}

pub fn is_zero_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::is_zero_8(dst, &src, std::ptr::null()) }
}

pub fn issuance(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::issuance(dst, &src, env) }
}

pub fn issuance_asset(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::issuance_asset(dst, &src, env) }
}

pub fn issuance_asset_amount(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::issuance_asset_amount(dst, &src, env) }
}

pub fn issuance_asset_amounts_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::issuance_asset_amounts_hash(dst, &src, env) }
}

pub fn issuance_asset_proof(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::issuance_asset_proof(dst, &src, env) }
}

pub fn issuance_blinding_entropy_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::issuance_blinding_entropy_hash(dst, &src, env) }
}

pub fn issuance_entropy(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::issuance_entropy(dst, &src, env) }
}

pub fn issuance_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::issuance_hash(dst, &src, env) }
}

pub fn issuance_range_proofs_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::issuance_range_proofs_hash(dst, &src, env) }
}

pub fn issuance_token(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::issuance_token(dst, &src, env) }
}

pub fn issuance_token_amount(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::issuance_token_amount(dst, &src, env) }
}

pub fn issuance_token_amounts_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::issuance_token_amounts_hash(dst, &src, env) }
}

pub fn issuance_token_proof(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::issuance_token_proof(dst, &src, env) }
}

pub fn issuances_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::issuances_hash(dst, &src, env) }
}

pub fn lbtc_asset(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::lbtc_asset(dst, &src, env) }
}

pub fn le_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::le_16(dst, &src, std::ptr::null()) }
}

pub fn le_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::le_32(dst, &src, std::ptr::null()) }
}

pub fn le_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::le_64(dst, &src, std::ptr::null()) }
}

pub fn le_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::le_8(dst, &src, std::ptr::null()) }
}

pub fn left_extend_16_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_extend_16_32(dst, &src, std::ptr::null()) }
}

pub fn left_extend_16_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_extend_16_64(dst, &src, std::ptr::null()) }
}

pub fn left_extend_1_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_extend_1_16(dst, &src, std::ptr::null()) }
}

pub fn left_extend_1_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_extend_1_32(dst, &src, std::ptr::null()) }
}

pub fn left_extend_1_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_extend_1_64(dst, &src, std::ptr::null()) }
}

pub fn left_extend_1_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_extend_1_8(dst, &src, std::ptr::null()) }
}

pub fn left_extend_32_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_extend_32_64(dst, &src, std::ptr::null()) }
}

pub fn left_extend_8_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_extend_8_16(dst, &src, std::ptr::null()) }
}

pub fn left_extend_8_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_extend_8_32(dst, &src, std::ptr::null()) }
}

pub fn left_extend_8_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_extend_8_64(dst, &src, std::ptr::null()) }
}

pub fn left_pad_high_16_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_pad_high_16_32(dst, &src, std::ptr::null()) }
}

pub fn left_pad_high_16_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_pad_high_16_64(dst, &src, std::ptr::null()) }
}

pub fn left_pad_high_1_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_pad_high_1_16(dst, &src, std::ptr::null()) }
}

pub fn left_pad_high_1_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_pad_high_1_32(dst, &src, std::ptr::null()) }
}

pub fn left_pad_high_1_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_pad_high_1_64(dst, &src, std::ptr::null()) }
}

pub fn left_pad_high_1_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_pad_high_1_8(dst, &src, std::ptr::null()) }
}

pub fn left_pad_high_32_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_pad_high_32_64(dst, &src, std::ptr::null()) }
}

pub fn left_pad_high_8_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_pad_high_8_16(dst, &src, std::ptr::null()) }
}

pub fn left_pad_high_8_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_pad_high_8_32(dst, &src, std::ptr::null()) }
}

pub fn left_pad_high_8_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_pad_high_8_64(dst, &src, std::ptr::null()) }
}

pub fn left_pad_low_16_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_pad_low_16_32(dst, &src, std::ptr::null()) }
}

pub fn left_pad_low_16_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_pad_low_16_64(dst, &src, std::ptr::null()) }
}

pub fn left_pad_low_1_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_pad_low_1_16(dst, &src, std::ptr::null()) }
}

pub fn left_pad_low_1_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_pad_low_1_32(dst, &src, std::ptr::null()) }
}

pub fn left_pad_low_1_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_pad_low_1_64(dst, &src, std::ptr::null()) }
}

pub fn left_pad_low_1_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_pad_low_1_8(dst, &src, std::ptr::null()) }
}

pub fn left_pad_low_32_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_pad_low_32_64(dst, &src, std::ptr::null()) }
}

pub fn left_pad_low_8_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_pad_low_8_16(dst, &src, std::ptr::null()) }
}

pub fn left_pad_low_8_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_pad_low_8_32(dst, &src, std::ptr::null()) }
}

pub fn left_pad_low_8_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_pad_low_8_64(dst, &src, std::ptr::null()) }
}

pub fn left_rotate_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_rotate_16(dst, &src, std::ptr::null()) }
}

pub fn left_rotate_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_rotate_32(dst, &src, std::ptr::null()) }
}

pub fn left_rotate_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_rotate_64(dst, &src, std::ptr::null()) }
}

pub fn left_rotate_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_rotate_8(dst, &src, std::ptr::null()) }
}

pub fn left_shift_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_shift_16(dst, &src, std::ptr::null()) }
}

pub fn left_shift_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_shift_32(dst, &src, std::ptr::null()) }
}

pub fn left_shift_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_shift_64(dst, &src, std::ptr::null()) }
}

pub fn left_shift_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_shift_8(dst, &src, std::ptr::null()) }
}

pub fn left_shift_with_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_shift_with_16(dst, &src, std::ptr::null()) }
}

pub fn left_shift_with_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_shift_with_32(dst, &src, std::ptr::null()) }
}

pub fn left_shift_with_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_shift_with_64(dst, &src, std::ptr::null()) }
}

pub fn left_shift_with_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::left_shift_with_8(dst, &src, std::ptr::null()) }
}

pub fn leftmost_16_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::leftmost_16_1(dst, &src, std::ptr::null()) }
}

pub fn leftmost_16_2<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::leftmost_16_2(dst, &src, std::ptr::null()) }
}

pub fn leftmost_16_4<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::leftmost_16_4(dst, &src, std::ptr::null()) }
}

pub fn leftmost_16_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::leftmost_16_8(dst, &src, std::ptr::null()) }
}

pub fn leftmost_32_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::leftmost_32_1(dst, &src, std::ptr::null()) }
}

pub fn leftmost_32_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::leftmost_32_16(dst, &src, std::ptr::null()) }
}

pub fn leftmost_32_2<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::leftmost_32_2(dst, &src, std::ptr::null()) }
}

pub fn leftmost_32_4<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::leftmost_32_4(dst, &src, std::ptr::null()) }
}

pub fn leftmost_32_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::leftmost_32_8(dst, &src, std::ptr::null()) }
}

pub fn leftmost_64_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::leftmost_64_1(dst, &src, std::ptr::null()) }
}

pub fn leftmost_64_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::leftmost_64_16(dst, &src, std::ptr::null()) }
}

pub fn leftmost_64_2<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::leftmost_64_2(dst, &src, std::ptr::null()) }
}

pub fn leftmost_64_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::leftmost_64_32(dst, &src, std::ptr::null()) }
}

pub fn leftmost_64_4<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::leftmost_64_4(dst, &src, std::ptr::null()) }
}

pub fn leftmost_64_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::leftmost_64_8(dst, &src, std::ptr::null()) }
}

pub fn leftmost_8_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::leftmost_8_1(dst, &src, std::ptr::null()) }
}

pub fn leftmost_8_2<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::leftmost_8_2(dst, &src, std::ptr::null()) }
}

pub fn leftmost_8_4<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::leftmost_8_4(dst, &src, std::ptr::null()) }
}

pub fn linear_combination_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::linear_combination_1(dst, &src, std::ptr::null()) }
}

pub fn linear_verify_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::linear_verify_1(dst, &src, std::ptr::null()) }
}

pub fn bitcoin_lock_time(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_lock_time(dst, &src, env) }
}

pub fn lock_time(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::lock_time(dst, &src, env) }
}

pub fn low_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::low_1(dst, &src, std::ptr::null()) }
}

pub fn low_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::low_16(dst, &src, std::ptr::null()) }
}

pub fn low_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::low_32(dst, &src, std::ptr::null()) }
}

pub fn low_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::low_64(dst, &src, std::ptr::null()) }
}

pub fn low_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::low_8(dst, &src, std::ptr::null()) }
}

pub fn lt_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::lt_16(dst, &src, std::ptr::null()) }
}

pub fn lt_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::lt_32(dst, &src, std::ptr::null()) }
}

pub fn lt_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::lt_64(dst, &src, std::ptr::null()) }
}

pub fn lt_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::lt_8(dst, &src, std::ptr::null()) }
}

pub fn maj_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::maj_1(dst, &src, std::ptr::null()) }
}

pub fn maj_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::maj_16(dst, &src, std::ptr::null()) }
}

pub fn maj_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::maj_32(dst, &src, std::ptr::null()) }
}

pub fn maj_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::maj_64(dst, &src, std::ptr::null()) }
}

pub fn maj_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::maj_8(dst, &src, std::ptr::null()) }
}

pub fn max_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::max_16(dst, &src, std::ptr::null()) }
}

pub fn max_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::max_32(dst, &src, std::ptr::null()) }
}

pub fn max_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::max_64(dst, &src, std::ptr::null()) }
}

pub fn max_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::max_8(dst, &src, std::ptr::null()) }
}

pub fn median_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::median_16(dst, &src, std::ptr::null()) }
}

pub fn median_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::median_32(dst, &src, std::ptr::null()) }
}

pub fn median_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::median_64(dst, &src, std::ptr::null()) }
}

pub fn median_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::median_8(dst, &src, std::ptr::null()) }
}

pub fn min_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::min_16(dst, &src, std::ptr::null()) }
}

pub fn min_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::min_32(dst, &src, std::ptr::null()) }
}

pub fn min_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::min_64(dst, &src, std::ptr::null()) }
}

pub fn min_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::min_8(dst, &src, std::ptr::null()) }
}

pub fn modulo_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::modulo_16(dst, &src, std::ptr::null()) }
}

pub fn modulo_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::modulo_32(dst, &src, std::ptr::null()) }
}

pub fn modulo_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::modulo_64(dst, &src, std::ptr::null()) }
}

pub fn modulo_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::modulo_8(dst, &src, std::ptr::null()) }
}

pub fn multiply_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::multiply_16(dst, &src, std::ptr::null()) }
}

pub fn multiply_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::multiply_32(dst, &src, std::ptr::null()) }
}

pub fn multiply_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::multiply_64(dst, &src, std::ptr::null()) }
}

pub fn multiply_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::multiply_8(dst, &src, std::ptr::null()) }
}

pub fn negate_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::negate_16(dst, &src, std::ptr::null()) }
}

pub fn negate_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::negate_32(dst, &src, std::ptr::null()) }
}

pub fn negate_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::negate_64(dst, &src, std::ptr::null()) }
}

pub fn negate_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::negate_8(dst, &src, std::ptr::null()) }
}

pub fn new_issuance_contract(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::new_issuance_contract(dst, &src, env) }
}

pub fn nonce_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::nonce_hash(dst, &src, env) }
}

pub fn bitcoin_num_inputs(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_num_inputs(dst, &src, env) }
}

pub fn num_inputs(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::num_inputs(dst, &src, env) }
}

pub fn bitcoin_num_outputs(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_num_outputs(dst, &src, env) }
}

pub fn num_outputs(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::num_outputs(dst, &src, env) }
}

pub fn one_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::one_16(dst, &src, std::ptr::null()) }
}

pub fn one_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::one_32(dst, &src, std::ptr::null()) }
}

pub fn one_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::one_64(dst, &src, std::ptr::null()) }
}

pub fn one_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::one_8(dst, &src, std::ptr::null()) }
}

pub fn or_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::or_1(dst, &src, std::ptr::null()) }
}

pub fn or_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::or_16(dst, &src, std::ptr::null()) }
}

pub fn or_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::or_32(dst, &src, std::ptr::null()) }
}

pub fn or_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::or_64(dst, &src, std::ptr::null()) }
}

pub fn or_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::or_8(dst, &src, std::ptr::null()) }
}

pub fn bitcoin_outpoint_hash(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_outpoint_hash(dst, &src, env) }
}

pub fn outpoint_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::outpoint_hash(dst, &src, env) }
}

pub fn output_amount(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::output_amount(dst, &src, env) }
}

pub fn output_amounts_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::output_amounts_hash(dst, &src, env) }
}

pub fn output_asset(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::output_asset(dst, &src, env) }
}

pub fn bitcoin_output_hash(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_output_hash(dst, &src, env) }
}

pub fn output_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::output_hash(dst, &src, env) }
}

pub fn output_is_fee(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::output_is_fee(dst, &src, env) }
}

pub fn output_nonce(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::output_nonce(dst, &src, env) }
}

pub fn output_nonces_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::output_nonces_hash(dst, &src, env) }
}

pub fn output_null_datum(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::output_null_datum(dst, &src, env) }
}

pub fn output_range_proof(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::output_range_proof(dst, &src, env) }
}

pub fn output_range_proofs_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::output_range_proofs_hash(dst, &src, env) }
}

pub fn bitcoin_output_script_hash(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_output_script_hash(dst, &src, env) }
}

pub fn output_script_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::output_script_hash(dst, &src, env) }
}

pub fn bitcoin_output_scripts_hash(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_output_scripts_hash(dst, &src, env) }
}

pub fn output_scripts_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::output_scripts_hash(dst, &src, env) }
}

pub fn output_surjection_proof(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::output_surjection_proof(dst, &src, env) }
}

pub fn output_surjection_proofs_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::output_surjection_proofs_hash(dst, &src, env) }
}

pub fn bitcoin_output_value(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_output_value(dst, &src, env) }
}

pub fn bitcoin_output_values_hash(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_output_values_hash(dst, &src, env) }
}

pub fn bitcoin_outputs_hash(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_outputs_hash(dst, &src, env) }
}

pub fn outputs_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::outputs_hash(dst, &src, env) }
}

pub fn parse_lock<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::parse_lock(dst, &src, std::ptr::null()) }
}

pub fn parse_sequence<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::parse_sequence(dst, &src, std::ptr::null()) }
}

pub fn point_verify_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::point_verify_1(dst, &src, std::ptr::null()) }
}

pub fn reissuance_blinding(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::reissuance_blinding(dst, &src, env) }
}

pub fn reissuance_entropy(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::reissuance_entropy(dst, &src, env) }
}

pub fn right_extend_16_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_extend_16_32(dst, &src, std::ptr::null()) }
}

pub fn right_extend_16_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_extend_16_64(dst, &src, std::ptr::null()) }
}

pub fn right_extend_32_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_extend_32_64(dst, &src, std::ptr::null()) }
}

pub fn right_extend_8_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_extend_8_16(dst, &src, std::ptr::null()) }
}

pub fn right_extend_8_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_extend_8_32(dst, &src, std::ptr::null()) }
}

pub fn right_extend_8_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_extend_8_64(dst, &src, std::ptr::null()) }
}

pub fn right_pad_high_16_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_pad_high_16_32(dst, &src, std::ptr::null()) }
}

pub fn right_pad_high_16_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_pad_high_16_64(dst, &src, std::ptr::null()) }
}

pub fn right_pad_high_1_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_pad_high_1_16(dst, &src, std::ptr::null()) }
}

pub fn right_pad_high_1_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_pad_high_1_32(dst, &src, std::ptr::null()) }
}

pub fn right_pad_high_1_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_pad_high_1_64(dst, &src, std::ptr::null()) }
}

pub fn right_pad_high_1_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_pad_high_1_8(dst, &src, std::ptr::null()) }
}

pub fn right_pad_high_32_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_pad_high_32_64(dst, &src, std::ptr::null()) }
}

pub fn right_pad_high_8_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_pad_high_8_16(dst, &src, std::ptr::null()) }
}

pub fn right_pad_high_8_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_pad_high_8_32(dst, &src, std::ptr::null()) }
}

pub fn right_pad_high_8_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_pad_high_8_64(dst, &src, std::ptr::null()) }
}

pub fn right_pad_low_16_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_pad_low_16_32(dst, &src, std::ptr::null()) }
}

pub fn right_pad_low_16_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_pad_low_16_64(dst, &src, std::ptr::null()) }
}

pub fn right_pad_low_1_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_pad_low_1_16(dst, &src, std::ptr::null()) }
}

pub fn right_pad_low_1_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_pad_low_1_32(dst, &src, std::ptr::null()) }
}

pub fn right_pad_low_1_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_pad_low_1_64(dst, &src, std::ptr::null()) }
}

pub fn right_pad_low_1_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_pad_low_1_8(dst, &src, std::ptr::null()) }
}

pub fn right_pad_low_32_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_pad_low_32_64(dst, &src, std::ptr::null()) }
}

pub fn right_pad_low_8_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_pad_low_8_16(dst, &src, std::ptr::null()) }
}

pub fn right_pad_low_8_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_pad_low_8_32(dst, &src, std::ptr::null()) }
}

pub fn right_pad_low_8_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_pad_low_8_64(dst, &src, std::ptr::null()) }
}

pub fn right_rotate_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_rotate_16(dst, &src, std::ptr::null()) }
}

pub fn right_rotate_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_rotate_32(dst, &src, std::ptr::null()) }
}

pub fn right_rotate_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_rotate_64(dst, &src, std::ptr::null()) }
}

pub fn right_rotate_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_rotate_8(dst, &src, std::ptr::null()) }
}

pub fn right_shift_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_shift_16(dst, &src, std::ptr::null()) }
}

pub fn right_shift_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_shift_32(dst, &src, std::ptr::null()) }
}

pub fn right_shift_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_shift_64(dst, &src, std::ptr::null()) }
}

pub fn right_shift_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_shift_8(dst, &src, std::ptr::null()) }
}

pub fn right_shift_with_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_shift_with_16(dst, &src, std::ptr::null()) }
}

pub fn right_shift_with_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_shift_with_32(dst, &src, std::ptr::null()) }
}

pub fn right_shift_with_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_shift_with_64(dst, &src, std::ptr::null()) }
}

pub fn right_shift_with_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::right_shift_with_8(dst, &src, std::ptr::null()) }
}

pub fn rightmost_16_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::rightmost_16_1(dst, &src, std::ptr::null()) }
}

pub fn rightmost_16_2<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::rightmost_16_2(dst, &src, std::ptr::null()) }
}

pub fn rightmost_16_4<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::rightmost_16_4(dst, &src, std::ptr::null()) }
}

pub fn rightmost_16_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::rightmost_16_8(dst, &src, std::ptr::null()) }
}

pub fn rightmost_32_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::rightmost_32_1(dst, &src, std::ptr::null()) }
}

pub fn rightmost_32_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::rightmost_32_16(dst, &src, std::ptr::null()) }
}

pub fn rightmost_32_2<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::rightmost_32_2(dst, &src, std::ptr::null()) }
}

pub fn rightmost_32_4<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::rightmost_32_4(dst, &src, std::ptr::null()) }
}

pub fn rightmost_32_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::rightmost_32_8(dst, &src, std::ptr::null()) }
}

pub fn rightmost_64_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::rightmost_64_1(dst, &src, std::ptr::null()) }
}

pub fn rightmost_64_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::rightmost_64_16(dst, &src, std::ptr::null()) }
}

pub fn rightmost_64_2<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::rightmost_64_2(dst, &src, std::ptr::null()) }
}

pub fn rightmost_64_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::rightmost_64_32(dst, &src, std::ptr::null()) }
}

pub fn rightmost_64_4<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::rightmost_64_4(dst, &src, std::ptr::null()) }
}

pub fn rightmost_64_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::rightmost_64_8(dst, &src, std::ptr::null()) }
}

pub fn rightmost_8_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::rightmost_8_1(dst, &src, std::ptr::null()) }
}

pub fn rightmost_8_2<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::rightmost_8_2(dst, &src, std::ptr::null()) }
}

pub fn rightmost_8_4<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::rightmost_8_4(dst, &src, std::ptr::null()) }
}

pub fn scalar_add<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::scalar_add(dst, &src, std::ptr::null()) }
}

pub fn scalar_invert<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::scalar_invert(dst, &src, std::ptr::null()) }
}

pub fn scalar_is_zero<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::scalar_is_zero(dst, &src, std::ptr::null()) }
}

pub fn scalar_multiply<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::scalar_multiply(dst, &src, std::ptr::null()) }
}

pub fn scalar_multiply_lambda<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::scalar_multiply_lambda(dst, &src, std::ptr::null()) }
}

pub fn scalar_negate<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::scalar_negate(dst, &src, std::ptr::null()) }
}

pub fn scalar_normalize<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::scalar_normalize(dst, &src, std::ptr::null()) }
}

pub fn scalar_square<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::scalar_square(dst, &src, std::ptr::null()) }
}

pub fn scale<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::scale(dst, &src, std::ptr::null()) }
}

pub fn bitcoin_script_cmr(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_script_cmr(dst, &src, env) }
}

pub fn script_cmr(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::script_cmr(dst, &src, env) }
}

pub fn sha_256_block<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::sha_256_block(dst, &src, std::ptr::null()) }
}

pub fn sha_256_ctx_8_add_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::sha_256_ctx_8_add_1(dst, &src, std::ptr::null()) }
}

pub fn sha_256_ctx_8_add_128<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::sha_256_ctx_8_add_128(dst, &src, std::ptr::null()) }
}

pub fn sha_256_ctx_8_add_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::sha_256_ctx_8_add_16(dst, &src, std::ptr::null()) }
}

pub fn sha_256_ctx_8_add_2<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::sha_256_ctx_8_add_2(dst, &src, std::ptr::null()) }
}

pub fn sha_256_ctx_8_add_256<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::sha_256_ctx_8_add_256(dst, &src, std::ptr::null()) }
}

pub fn sha_256_ctx_8_add_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::sha_256_ctx_8_add_32(dst, &src, std::ptr::null()) }
}

pub fn sha_256_ctx_8_add_4<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::sha_256_ctx_8_add_4(dst, &src, std::ptr::null()) }
}

pub fn sha_256_ctx_8_add_512<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::sha_256_ctx_8_add_512(dst, &src, std::ptr::null()) }
}

pub fn sha_256_ctx_8_add_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::sha_256_ctx_8_add_64(dst, &src, std::ptr::null()) }
}

pub fn sha_256_ctx_8_add_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::sha_256_ctx_8_add_8(dst, &src, std::ptr::null()) }
}

pub fn sha_256_ctx_8_add_buffer_511<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::sha_256_ctx_8_add_buffer_511(dst, &src, std::ptr::null()) }
}

pub fn sha_256_ctx_8_finalize<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::sha_256_ctx_8_finalize(dst, &src, std::ptr::null()) }
}

pub fn sha_256_ctx_8_init<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::sha_256_ctx_8_init(dst, &src, std::ptr::null()) }
}

pub fn sha_256_iv<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::sha_256_iv(dst, &src, std::ptr::null()) }
}

pub fn bitcoin_sig_all_hash(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_sig_all_hash(dst, &src, env) }
}

pub fn sig_all_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::sig_all_hash(dst, &src, env) }
}

pub fn some_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::some_1(dst, &src, std::ptr::null()) }
}

pub fn some_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::some_16(dst, &src, std::ptr::null()) }
}

pub fn some_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::some_32(dst, &src, std::ptr::null()) }
}

pub fn some_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::some_64(dst, &src, std::ptr::null()) }
}

pub fn some_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::some_8(dst, &src, std::ptr::null()) }
}

pub fn subtract_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::subtract_16(dst, &src, std::ptr::null()) }
}

pub fn subtract_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::subtract_32(dst, &src, std::ptr::null()) }
}

pub fn subtract_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::subtract_64(dst, &src, std::ptr::null()) }
}

pub fn subtract_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::subtract_8(dst, &src, std::ptr::null()) }
}

pub fn swu<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::swu(dst, &src, std::ptr::null()) }
}

pub fn bitcoin_tap_env_hash(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_tap_env_hash(dst, &src, env) }
}

pub fn tap_env_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::tap_env_hash(dst, &src, env) }
}

pub fn tapdata_init<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::tapdata_init(dst, &src, std::ptr::null()) }
}

pub fn bitcoin_tapleaf_hash(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_tapleaf_hash(dst, &src, env) }
}

pub fn tapleaf_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::tapleaf_hash(dst, &src, env) }
}

pub fn bitcoin_tapleaf_version(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_tapleaf_version(dst, &src, env) }
}

pub fn tapleaf_version(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::tapleaf_version(dst, &src, env) }
}

pub fn bitcoin_tappath(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_tappath(dst, &src, env) }
}

pub fn tappath(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::tappath(dst, &src, env) }
}

pub fn bitcoin_tappath_hash(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_tappath_hash(dst, &src, env) }
}

pub fn tappath_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::tappath_hash(dst, &src, env) }
}

pub fn total_fee(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::total_fee(dst, &src, env) }
}

pub fn bitcoin_total_input_value(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_total_input_value(dst, &src, env) }
}

pub fn bitcoin_total_output_value(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_total_output_value(dst, &src, env) }
}

pub fn bitcoin_transaction_id(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_transaction_id(dst, &src, env) }
}

pub fn transaction_id(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::transaction_id(dst, &src, env) }
}

pub fn bitcoin_tx_hash(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_tx_hash(dst, &src, env) }
}

pub fn tx_hash(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::tx_hash(dst, &src, env) }
}

pub fn bitcoin_tx_is_final(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_tx_is_final(dst, &src, env) }
}

pub fn tx_is_final(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::tx_is_final(dst, &src, env) }
}

pub fn bitcoin_tx_lock_distance(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_tx_lock_distance(dst, &src, env) }
}

pub fn tx_lock_distance(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::tx_lock_distance(dst, &src, env) }
}

pub fn bitcoin_tx_lock_duration(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_tx_lock_duration(dst, &src, env) }
}

pub fn tx_lock_duration(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::tx_lock_duration(dst, &src, env) }
}

pub fn bitcoin_tx_lock_height(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_tx_lock_height(dst, &src, env) }
}

pub fn tx_lock_height(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::tx_lock_height(dst, &src, env) }
}

pub fn bitcoin_tx_lock_time(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_tx_lock_time(dst, &src, env) }
}

pub fn tx_lock_time(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::tx_lock_time(dst, &src, env) }
}

pub fn verify<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::verify(dst, &src, std::ptr::null()) }
}

pub fn bitcoin_version(dst: &mut CFrameItem, src: CFrameItem, env: &bitcoin::CTxEnv) -> bool {
    unsafe { jets_ffi::bitcoin_version(dst, &src, env) }
}

pub fn version(dst: &mut CFrameItem, src: CFrameItem, env: &elements::CTxEnv) -> bool {
    unsafe { jets_ffi::version(dst, &src, env) }
}

pub fn xor_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::xor_1(dst, &src, std::ptr::null()) }
}

pub fn xor_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::xor_16(dst, &src, std::ptr::null()) }
}

pub fn xor_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::xor_32(dst, &src, std::ptr::null()) }
}

pub fn xor_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::xor_64(dst, &src, std::ptr::null()) }
}

pub fn xor_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::xor_8(dst, &src, std::ptr::null()) }
}

pub fn xor_xor_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::xor_xor_1(dst, &src, std::ptr::null()) }
}

pub fn xor_xor_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::xor_xor_16(dst, &src, std::ptr::null()) }
}

pub fn xor_xor_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::xor_xor_32(dst, &src, std::ptr::null()) }
}

pub fn xor_xor_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::xor_xor_64(dst, &src, std::ptr::null()) }
}

pub fn xor_xor_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { jets_ffi::xor_xor_8(dst, &src, std::ptr::null()) }
}

