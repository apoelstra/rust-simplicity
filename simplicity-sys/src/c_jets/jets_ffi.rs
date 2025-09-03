/* This file has been automatically generated. */

use crate::ffi::c_void;
use crate::bitcoin;
use crate::elements;
use crate::CFrameItem;

extern "C" {
    #[link_name = "rustsimplicity_0_6_c_add_16"]
    pub fn add_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_add_32"]
    pub fn add_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_add_64"]
    pub fn add_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_add_8"]
    pub fn add_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_all_16"]
    pub fn all_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_all_32"]
    pub fn all_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_all_64"]
    pub fn all_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_all_8"]
    pub fn all_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_and_1"]
    pub fn and_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_and_16"]
    pub fn and_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_and_32"]
    pub fn and_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_and_64"]
    pub fn and_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_and_8"]
    pub fn and_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_annex_hash"]
    pub fn bitcoin_annex_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_annex_hash"]
    pub fn annex_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_asset_amount_hash"]
    pub fn asset_amount_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bip_0340_verify"]
    pub fn bip_0340_verify(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_build_tapbranch"]
    pub fn bitcoin_build_tapbranch(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_build_tapbranch"]
    pub fn build_tapbranch(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_build_tapleaf_simplicity"]
    pub fn bitcoin_build_tapleaf_simplicity(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_build_tapleaf_simplicity"]
    pub fn build_tapleaf_simplicity(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_build_taptweak"]
    pub fn bitcoin_build_taptweak(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_build_taptweak"]
    pub fn build_taptweak(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_calculate_asset"]
    pub fn calculate_asset(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_calculate_confidential_token"]
    pub fn calculate_confidential_token(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_calculate_explicit_token"]
    pub fn calculate_explicit_token(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_calculate_issuance_entropy"]
    pub fn calculate_issuance_entropy(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_ch_1"]
    pub fn ch_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_ch_16"]
    pub fn ch_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_ch_32"]
    pub fn ch_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_ch_64"]
    pub fn ch_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_ch_8"]
    pub fn ch_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_check_lock_distance"]
    pub fn bitcoin_check_lock_distance(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_check_lock_distance"]
    pub fn check_lock_distance(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_check_lock_duration"]
    pub fn bitcoin_check_lock_duration(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_check_lock_duration"]
    pub fn check_lock_duration(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_check_lock_height"]
    pub fn bitcoin_check_lock_height(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_check_lock_height"]
    pub fn check_lock_height(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_check_lock_time"]
    pub fn bitcoin_check_lock_time(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_check_lock_time"]
    pub fn check_lock_time(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_check_sig_verify"]
    pub fn check_sig_verify(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_complement_1"]
    pub fn complement_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_complement_16"]
    pub fn complement_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_complement_32"]
    pub fn complement_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_complement_64"]
    pub fn complement_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_complement_8"]
    pub fn complement_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_current_amount"]
    pub fn current_amount(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_current_annex_hash"]
    pub fn bitcoin_current_annex_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_current_annex_hash"]
    pub fn current_annex_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_current_asset"]
    pub fn current_asset(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_current_index"]
    pub fn bitcoin_current_index(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_current_index"]
    pub fn current_index(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_current_issuance_asset_amount"]
    pub fn current_issuance_asset_amount(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_current_issuance_asset_proof"]
    pub fn current_issuance_asset_proof(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_current_issuance_token_amount"]
    pub fn current_issuance_token_amount(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_current_issuance_token_proof"]
    pub fn current_issuance_token_proof(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_current_new_issuance_contract"]
    pub fn current_new_issuance_contract(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_current_pegin"]
    pub fn current_pegin(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_current_prev_outpoint"]
    pub fn bitcoin_current_prev_outpoint(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_current_prev_outpoint"]
    pub fn current_prev_outpoint(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_current_reissuance_blinding"]
    pub fn current_reissuance_blinding(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_current_reissuance_entropy"]
    pub fn current_reissuance_entropy(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_current_script_hash"]
    pub fn bitcoin_current_script_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_current_script_hash"]
    pub fn current_script_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_current_script_sig_hash"]
    pub fn bitcoin_current_script_sig_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_current_script_sig_hash"]
    pub fn current_script_sig_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_current_sequence"]
    pub fn bitcoin_current_sequence(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_current_sequence"]
    pub fn current_sequence(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_current_value"]
    pub fn bitcoin_current_value(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_decompress"]
    pub fn decompress(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_decrement_16"]
    pub fn decrement_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_decrement_32"]
    pub fn decrement_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_decrement_64"]
    pub fn decrement_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_decrement_8"]
    pub fn decrement_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_div_mod_128_64"]
    pub fn div_mod_128_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_div_mod_16"]
    pub fn div_mod_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_div_mod_32"]
    pub fn div_mod_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_div_mod_64"]
    pub fn div_mod_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_div_mod_8"]
    pub fn div_mod_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_divide_16"]
    pub fn divide_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_divide_32"]
    pub fn divide_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_divide_64"]
    pub fn divide_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_divide_8"]
    pub fn divide_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_divides_16"]
    pub fn divides_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_divides_32"]
    pub fn divides_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_divides_64"]
    pub fn divides_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_divides_8"]
    pub fn divides_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_eq_1"]
    pub fn eq_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_eq_16"]
    pub fn eq_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_eq_256"]
    pub fn eq_256(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_eq_32"]
    pub fn eq_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_eq_64"]
    pub fn eq_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_eq_8"]
    pub fn eq_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_fe_add"]
    pub fn fe_add(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_fe_invert"]
    pub fn fe_invert(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_fe_is_odd"]
    pub fn fe_is_odd(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_fe_is_zero"]
    pub fn fe_is_zero(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_fe_multiply"]
    pub fn fe_multiply(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_fe_multiply_beta"]
    pub fn fe_multiply_beta(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_fe_negate"]
    pub fn fe_negate(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_fe_normalize"]
    pub fn fe_normalize(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_fe_square"]
    pub fn fe_square(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_fe_square_root"]
    pub fn fe_square_root(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_fee"]
    pub fn bitcoin_fee(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_add_16"]
    pub fn full_add_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_add_32"]
    pub fn full_add_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_add_64"]
    pub fn full_add_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_add_8"]
    pub fn full_add_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_decrement_16"]
    pub fn full_decrement_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_decrement_32"]
    pub fn full_decrement_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_decrement_64"]
    pub fn full_decrement_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_decrement_8"]
    pub fn full_decrement_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_increment_16"]
    pub fn full_increment_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_increment_32"]
    pub fn full_increment_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_increment_64"]
    pub fn full_increment_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_increment_8"]
    pub fn full_increment_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_left_shift_16_1"]
    pub fn full_left_shift_16_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_left_shift_16_2"]
    pub fn full_left_shift_16_2(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_left_shift_16_4"]
    pub fn full_left_shift_16_4(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_left_shift_16_8"]
    pub fn full_left_shift_16_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_left_shift_32_1"]
    pub fn full_left_shift_32_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_left_shift_32_16"]
    pub fn full_left_shift_32_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_left_shift_32_2"]
    pub fn full_left_shift_32_2(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_left_shift_32_4"]
    pub fn full_left_shift_32_4(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_left_shift_32_8"]
    pub fn full_left_shift_32_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_left_shift_64_1"]
    pub fn full_left_shift_64_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_left_shift_64_16"]
    pub fn full_left_shift_64_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_left_shift_64_2"]
    pub fn full_left_shift_64_2(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_left_shift_64_32"]
    pub fn full_left_shift_64_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_left_shift_64_4"]
    pub fn full_left_shift_64_4(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_left_shift_64_8"]
    pub fn full_left_shift_64_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_left_shift_8_1"]
    pub fn full_left_shift_8_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_left_shift_8_2"]
    pub fn full_left_shift_8_2(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_left_shift_8_4"]
    pub fn full_left_shift_8_4(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_multiply_16"]
    pub fn full_multiply_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_multiply_32"]
    pub fn full_multiply_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_multiply_64"]
    pub fn full_multiply_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_multiply_8"]
    pub fn full_multiply_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_right_shift_16_1"]
    pub fn full_right_shift_16_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_right_shift_16_2"]
    pub fn full_right_shift_16_2(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_right_shift_16_4"]
    pub fn full_right_shift_16_4(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_right_shift_16_8"]
    pub fn full_right_shift_16_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_right_shift_32_1"]
    pub fn full_right_shift_32_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_right_shift_32_16"]
    pub fn full_right_shift_32_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_right_shift_32_2"]
    pub fn full_right_shift_32_2(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_right_shift_32_4"]
    pub fn full_right_shift_32_4(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_right_shift_32_8"]
    pub fn full_right_shift_32_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_right_shift_64_1"]
    pub fn full_right_shift_64_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_right_shift_64_16"]
    pub fn full_right_shift_64_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_right_shift_64_2"]
    pub fn full_right_shift_64_2(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_right_shift_64_32"]
    pub fn full_right_shift_64_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_right_shift_64_4"]
    pub fn full_right_shift_64_4(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_right_shift_64_8"]
    pub fn full_right_shift_64_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_right_shift_8_1"]
    pub fn full_right_shift_8_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_right_shift_8_2"]
    pub fn full_right_shift_8_2(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_right_shift_8_4"]
    pub fn full_right_shift_8_4(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_subtract_16"]
    pub fn full_subtract_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_subtract_32"]
    pub fn full_subtract_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_subtract_64"]
    pub fn full_subtract_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_full_subtract_8"]
    pub fn full_subtract_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_ge_is_on_curve"]
    pub fn ge_is_on_curve(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_ge_negate"]
    pub fn ge_negate(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_gej_add"]
    pub fn gej_add(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_gej_double"]
    pub fn gej_double(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_gej_equiv"]
    pub fn gej_equiv(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_gej_ge_add"]
    pub fn gej_ge_add(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_gej_ge_add_ex"]
    pub fn gej_ge_add_ex(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_gej_ge_equiv"]
    pub fn gej_ge_equiv(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_gej_infinity"]
    pub fn gej_infinity(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_gej_is_infinity"]
    pub fn gej_is_infinity(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_gej_is_on_curve"]
    pub fn gej_is_on_curve(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_gej_negate"]
    pub fn gej_negate(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_gej_normalize"]
    pub fn gej_normalize(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_gej_rescale"]
    pub fn gej_rescale(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_gej_x_equiv"]
    pub fn gej_x_equiv(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_gej_y_is_odd"]
    pub fn gej_y_is_odd(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_generate"]
    pub fn generate(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_genesis_block_hash"]
    pub fn genesis_block_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_hash_to_curve"]
    pub fn hash_to_curve(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_high_1"]
    pub fn high_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_high_16"]
    pub fn high_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_high_32"]
    pub fn high_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_high_64"]
    pub fn high_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_high_8"]
    pub fn high_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_increment_16"]
    pub fn increment_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_increment_32"]
    pub fn increment_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_increment_64"]
    pub fn increment_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_increment_8"]
    pub fn increment_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_input_amount"]
    pub fn input_amount(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_input_amounts_hash"]
    pub fn input_amounts_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_input_annex_hash"]
    pub fn bitcoin_input_annex_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_input_annex_hash"]
    pub fn input_annex_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_input_annexes_hash"]
    pub fn bitcoin_input_annexes_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_input_annexes_hash"]
    pub fn input_annexes_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_input_asset"]
    pub fn input_asset(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_input_hash"]
    pub fn bitcoin_input_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_input_hash"]
    pub fn input_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_input_outpoints_hash"]
    pub fn bitcoin_input_outpoints_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_input_outpoints_hash"]
    pub fn input_outpoints_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_input_pegin"]
    pub fn input_pegin(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_input_prev_outpoint"]
    pub fn bitcoin_input_prev_outpoint(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_input_prev_outpoint"]
    pub fn input_prev_outpoint(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_input_script_hash"]
    pub fn bitcoin_input_script_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_input_script_hash"]
    pub fn input_script_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_input_script_sig_hash"]
    pub fn bitcoin_input_script_sig_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_input_script_sig_hash"]
    pub fn input_script_sig_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_input_script_sigs_hash"]
    pub fn bitcoin_input_script_sigs_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_input_script_sigs_hash"]
    pub fn input_script_sigs_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_input_scripts_hash"]
    pub fn bitcoin_input_scripts_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_input_scripts_hash"]
    pub fn input_scripts_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_input_sequence"]
    pub fn bitcoin_input_sequence(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_input_sequence"]
    pub fn input_sequence(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_input_sequences_hash"]
    pub fn bitcoin_input_sequences_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_input_sequences_hash"]
    pub fn input_sequences_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_input_utxo_hash"]
    pub fn bitcoin_input_utxo_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_input_utxo_hash"]
    pub fn input_utxo_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_input_utxos_hash"]
    pub fn bitcoin_input_utxos_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_input_utxos_hash"]
    pub fn input_utxos_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_input_value"]
    pub fn bitcoin_input_value(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_input_values_hash"]
    pub fn bitcoin_input_values_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_inputs_hash"]
    pub fn bitcoin_inputs_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_inputs_hash"]
    pub fn inputs_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_internal_key"]
    pub fn bitcoin_internal_key(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_internal_key"]
    pub fn internal_key(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_is_one_16"]
    pub fn is_one_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_is_one_32"]
    pub fn is_one_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_is_one_64"]
    pub fn is_one_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_is_one_8"]
    pub fn is_one_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_is_zero_16"]
    pub fn is_zero_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_is_zero_32"]
    pub fn is_zero_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_is_zero_64"]
    pub fn is_zero_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_is_zero_8"]
    pub fn is_zero_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_issuance"]
    pub fn issuance(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_issuance_asset"]
    pub fn issuance_asset(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_issuance_asset_amount"]
    pub fn issuance_asset_amount(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_issuance_asset_amounts_hash"]
    pub fn issuance_asset_amounts_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_issuance_asset_proof"]
    pub fn issuance_asset_proof(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_issuance_blinding_entropy_hash"]
    pub fn issuance_blinding_entropy_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_issuance_entropy"]
    pub fn issuance_entropy(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_issuance_hash"]
    pub fn issuance_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_issuance_range_proofs_hash"]
    pub fn issuance_range_proofs_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_issuance_token"]
    pub fn issuance_token(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_issuance_token_amount"]
    pub fn issuance_token_amount(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_issuance_token_amounts_hash"]
    pub fn issuance_token_amounts_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_issuance_token_proof"]
    pub fn issuance_token_proof(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_issuances_hash"]
    pub fn issuances_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_lbtc_asset"]
    pub fn lbtc_asset(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_le_16"]
    pub fn le_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_le_32"]
    pub fn le_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_le_64"]
    pub fn le_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_le_8"]
    pub fn le_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_extend_16_32"]
    pub fn left_extend_16_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_extend_16_64"]
    pub fn left_extend_16_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_extend_1_16"]
    pub fn left_extend_1_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_extend_1_32"]
    pub fn left_extend_1_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_extend_1_64"]
    pub fn left_extend_1_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_extend_1_8"]
    pub fn left_extend_1_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_extend_32_64"]
    pub fn left_extend_32_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_extend_8_16"]
    pub fn left_extend_8_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_extend_8_32"]
    pub fn left_extend_8_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_extend_8_64"]
    pub fn left_extend_8_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_pad_high_16_32"]
    pub fn left_pad_high_16_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_pad_high_16_64"]
    pub fn left_pad_high_16_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_pad_high_1_16"]
    pub fn left_pad_high_1_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_pad_high_1_32"]
    pub fn left_pad_high_1_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_pad_high_1_64"]
    pub fn left_pad_high_1_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_pad_high_1_8"]
    pub fn left_pad_high_1_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_pad_high_32_64"]
    pub fn left_pad_high_32_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_pad_high_8_16"]
    pub fn left_pad_high_8_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_pad_high_8_32"]
    pub fn left_pad_high_8_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_pad_high_8_64"]
    pub fn left_pad_high_8_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_pad_low_16_32"]
    pub fn left_pad_low_16_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_pad_low_16_64"]
    pub fn left_pad_low_16_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_pad_low_1_16"]
    pub fn left_pad_low_1_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_pad_low_1_32"]
    pub fn left_pad_low_1_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_pad_low_1_64"]
    pub fn left_pad_low_1_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_pad_low_1_8"]
    pub fn left_pad_low_1_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_pad_low_32_64"]
    pub fn left_pad_low_32_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_pad_low_8_16"]
    pub fn left_pad_low_8_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_pad_low_8_32"]
    pub fn left_pad_low_8_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_pad_low_8_64"]
    pub fn left_pad_low_8_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_rotate_16"]
    pub fn left_rotate_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_rotate_32"]
    pub fn left_rotate_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_rotate_64"]
    pub fn left_rotate_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_rotate_8"]
    pub fn left_rotate_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_shift_16"]
    pub fn left_shift_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_shift_32"]
    pub fn left_shift_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_shift_64"]
    pub fn left_shift_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_shift_8"]
    pub fn left_shift_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_shift_with_16"]
    pub fn left_shift_with_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_shift_with_32"]
    pub fn left_shift_with_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_shift_with_64"]
    pub fn left_shift_with_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_left_shift_with_8"]
    pub fn left_shift_with_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_leftmost_16_1"]
    pub fn leftmost_16_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_leftmost_16_2"]
    pub fn leftmost_16_2(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_leftmost_16_4"]
    pub fn leftmost_16_4(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_leftmost_16_8"]
    pub fn leftmost_16_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_leftmost_32_1"]
    pub fn leftmost_32_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_leftmost_32_16"]
    pub fn leftmost_32_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_leftmost_32_2"]
    pub fn leftmost_32_2(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_leftmost_32_4"]
    pub fn leftmost_32_4(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_leftmost_32_8"]
    pub fn leftmost_32_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_leftmost_64_1"]
    pub fn leftmost_64_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_leftmost_64_16"]
    pub fn leftmost_64_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_leftmost_64_2"]
    pub fn leftmost_64_2(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_leftmost_64_32"]
    pub fn leftmost_64_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_leftmost_64_4"]
    pub fn leftmost_64_4(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_leftmost_64_8"]
    pub fn leftmost_64_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_leftmost_8_1"]
    pub fn leftmost_8_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_leftmost_8_2"]
    pub fn leftmost_8_2(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_leftmost_8_4"]
    pub fn leftmost_8_4(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_linear_combination_1"]
    pub fn linear_combination_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_linear_verify_1"]
    pub fn linear_verify_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_lock_time"]
    pub fn bitcoin_lock_time(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_lock_time"]
    pub fn lock_time(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_low_1"]
    pub fn low_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_low_16"]
    pub fn low_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_low_32"]
    pub fn low_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_low_64"]
    pub fn low_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_low_8"]
    pub fn low_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_lt_16"]
    pub fn lt_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_lt_32"]
    pub fn lt_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_lt_64"]
    pub fn lt_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_lt_8"]
    pub fn lt_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_maj_1"]
    pub fn maj_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_maj_16"]
    pub fn maj_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_maj_32"]
    pub fn maj_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_maj_64"]
    pub fn maj_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_maj_8"]
    pub fn maj_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_max_16"]
    pub fn max_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_max_32"]
    pub fn max_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_max_64"]
    pub fn max_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_max_8"]
    pub fn max_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_median_16"]
    pub fn median_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_median_32"]
    pub fn median_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_median_64"]
    pub fn median_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_median_8"]
    pub fn median_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_min_16"]
    pub fn min_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_min_32"]
    pub fn min_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_min_64"]
    pub fn min_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_min_8"]
    pub fn min_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_modulo_16"]
    pub fn modulo_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_modulo_32"]
    pub fn modulo_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_modulo_64"]
    pub fn modulo_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_modulo_8"]
    pub fn modulo_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_multiply_16"]
    pub fn multiply_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_multiply_32"]
    pub fn multiply_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_multiply_64"]
    pub fn multiply_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_multiply_8"]
    pub fn multiply_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_negate_16"]
    pub fn negate_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_negate_32"]
    pub fn negate_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_negate_64"]
    pub fn negate_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_negate_8"]
    pub fn negate_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_new_issuance_contract"]
    pub fn new_issuance_contract(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_nonce_hash"]
    pub fn nonce_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_num_inputs"]
    pub fn bitcoin_num_inputs(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_num_inputs"]
    pub fn num_inputs(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_num_outputs"]
    pub fn bitcoin_num_outputs(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_num_outputs"]
    pub fn num_outputs(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_one_16"]
    pub fn one_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_one_32"]
    pub fn one_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_one_64"]
    pub fn one_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_one_8"]
    pub fn one_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_or_1"]
    pub fn or_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_or_16"]
    pub fn or_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_or_32"]
    pub fn or_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_or_64"]
    pub fn or_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_or_8"]
    pub fn or_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_outpoint_hash"]
    pub fn bitcoin_outpoint_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_outpoint_hash"]
    pub fn outpoint_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_output_amount"]
    pub fn output_amount(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_output_amounts_hash"]
    pub fn output_amounts_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_output_asset"]
    pub fn output_asset(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_output_hash"]
    pub fn bitcoin_output_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_output_hash"]
    pub fn output_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_output_is_fee"]
    pub fn output_is_fee(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_output_nonce"]
    pub fn output_nonce(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_output_nonces_hash"]
    pub fn output_nonces_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_output_null_datum"]
    pub fn output_null_datum(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_output_range_proof"]
    pub fn output_range_proof(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_output_range_proofs_hash"]
    pub fn output_range_proofs_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_output_script_hash"]
    pub fn bitcoin_output_script_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_output_script_hash"]
    pub fn output_script_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_output_scripts_hash"]
    pub fn bitcoin_output_scripts_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_output_scripts_hash"]
    pub fn output_scripts_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_output_surjection_proof"]
    pub fn output_surjection_proof(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_output_surjection_proofs_hash"]
    pub fn output_surjection_proofs_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_output_value"]
    pub fn bitcoin_output_value(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_output_values_hash"]
    pub fn bitcoin_output_values_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_outputs_hash"]
    pub fn bitcoin_outputs_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_outputs_hash"]
    pub fn outputs_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_parse_lock"]
    pub fn parse_lock(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_parse_sequence"]
    pub fn parse_sequence(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_point_verify_1"]
    pub fn point_verify_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_reissuance_blinding"]
    pub fn reissuance_blinding(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_reissuance_entropy"]
    pub fn reissuance_entropy(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_extend_16_32"]
    pub fn right_extend_16_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_extend_16_64"]
    pub fn right_extend_16_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_extend_32_64"]
    pub fn right_extend_32_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_extend_8_16"]
    pub fn right_extend_8_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_extend_8_32"]
    pub fn right_extend_8_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_extend_8_64"]
    pub fn right_extend_8_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_pad_high_16_32"]
    pub fn right_pad_high_16_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_pad_high_16_64"]
    pub fn right_pad_high_16_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_pad_high_1_16"]
    pub fn right_pad_high_1_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_pad_high_1_32"]
    pub fn right_pad_high_1_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_pad_high_1_64"]
    pub fn right_pad_high_1_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_pad_high_1_8"]
    pub fn right_pad_high_1_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_pad_high_32_64"]
    pub fn right_pad_high_32_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_pad_high_8_16"]
    pub fn right_pad_high_8_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_pad_high_8_32"]
    pub fn right_pad_high_8_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_pad_high_8_64"]
    pub fn right_pad_high_8_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_pad_low_16_32"]
    pub fn right_pad_low_16_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_pad_low_16_64"]
    pub fn right_pad_low_16_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_pad_low_1_16"]
    pub fn right_pad_low_1_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_pad_low_1_32"]
    pub fn right_pad_low_1_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_pad_low_1_64"]
    pub fn right_pad_low_1_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_pad_low_1_8"]
    pub fn right_pad_low_1_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_pad_low_32_64"]
    pub fn right_pad_low_32_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_pad_low_8_16"]
    pub fn right_pad_low_8_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_pad_low_8_32"]
    pub fn right_pad_low_8_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_pad_low_8_64"]
    pub fn right_pad_low_8_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_rotate_16"]
    pub fn right_rotate_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_rotate_32"]
    pub fn right_rotate_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_rotate_64"]
    pub fn right_rotate_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_rotate_8"]
    pub fn right_rotate_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_shift_16"]
    pub fn right_shift_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_shift_32"]
    pub fn right_shift_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_shift_64"]
    pub fn right_shift_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_shift_8"]
    pub fn right_shift_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_shift_with_16"]
    pub fn right_shift_with_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_shift_with_32"]
    pub fn right_shift_with_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_shift_with_64"]
    pub fn right_shift_with_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_right_shift_with_8"]
    pub fn right_shift_with_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_rightmost_16_1"]
    pub fn rightmost_16_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_rightmost_16_2"]
    pub fn rightmost_16_2(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_rightmost_16_4"]
    pub fn rightmost_16_4(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_rightmost_16_8"]
    pub fn rightmost_16_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_rightmost_32_1"]
    pub fn rightmost_32_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_rightmost_32_16"]
    pub fn rightmost_32_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_rightmost_32_2"]
    pub fn rightmost_32_2(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_rightmost_32_4"]
    pub fn rightmost_32_4(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_rightmost_32_8"]
    pub fn rightmost_32_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_rightmost_64_1"]
    pub fn rightmost_64_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_rightmost_64_16"]
    pub fn rightmost_64_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_rightmost_64_2"]
    pub fn rightmost_64_2(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_rightmost_64_32"]
    pub fn rightmost_64_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_rightmost_64_4"]
    pub fn rightmost_64_4(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_rightmost_64_8"]
    pub fn rightmost_64_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_rightmost_8_1"]
    pub fn rightmost_8_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_rightmost_8_2"]
    pub fn rightmost_8_2(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_rightmost_8_4"]
    pub fn rightmost_8_4(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_scalar_add"]
    pub fn scalar_add(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_scalar_invert"]
    pub fn scalar_invert(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_scalar_is_zero"]
    pub fn scalar_is_zero(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_scalar_multiply"]
    pub fn scalar_multiply(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_scalar_multiply_lambda"]
    pub fn scalar_multiply_lambda(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_scalar_negate"]
    pub fn scalar_negate(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_scalar_normalize"]
    pub fn scalar_normalize(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_scalar_square"]
    pub fn scalar_square(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_scale"]
    pub fn scale(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_script_cmr"]
    pub fn bitcoin_script_cmr(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_script_cmr"]
    pub fn script_cmr(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_sha_256_block"]
    pub fn sha_256_block(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_sha_256_ctx_8_add_1"]
    pub fn sha_256_ctx_8_add_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_sha_256_ctx_8_add_128"]
    pub fn sha_256_ctx_8_add_128(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_sha_256_ctx_8_add_16"]
    pub fn sha_256_ctx_8_add_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_sha_256_ctx_8_add_2"]
    pub fn sha_256_ctx_8_add_2(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_sha_256_ctx_8_add_256"]
    pub fn sha_256_ctx_8_add_256(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_sha_256_ctx_8_add_32"]
    pub fn sha_256_ctx_8_add_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_sha_256_ctx_8_add_4"]
    pub fn sha_256_ctx_8_add_4(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_sha_256_ctx_8_add_512"]
    pub fn sha_256_ctx_8_add_512(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_sha_256_ctx_8_add_64"]
    pub fn sha_256_ctx_8_add_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_sha_256_ctx_8_add_8"]
    pub fn sha_256_ctx_8_add_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_sha_256_ctx_8_add_buffer_511"]
    pub fn sha_256_ctx_8_add_buffer_511(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_sha_256_ctx_8_finalize"]
    pub fn sha_256_ctx_8_finalize(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_sha_256_ctx_8_init"]
    pub fn sha_256_ctx_8_init(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_sha_256_iv"]
    pub fn sha_256_iv(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_sig_all_hash"]
    pub fn bitcoin_sig_all_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_sig_all_hash"]
    pub fn sig_all_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_some_1"]
    pub fn some_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_some_16"]
    pub fn some_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_some_32"]
    pub fn some_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_some_64"]
    pub fn some_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_some_8"]
    pub fn some_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_subtract_16"]
    pub fn subtract_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_subtract_32"]
    pub fn subtract_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_subtract_64"]
    pub fn subtract_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_subtract_8"]
    pub fn subtract_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_swu"]
    pub fn swu(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_tap_env_hash"]
    pub fn bitcoin_tap_env_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_tap_env_hash"]
    pub fn tap_env_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_tapdata_init"]
    pub fn tapdata_init(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_tapleaf_hash"]
    pub fn bitcoin_tapleaf_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_tapleaf_hash"]
    pub fn tapleaf_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_tapleaf_version"]
    pub fn bitcoin_tapleaf_version(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_tapleaf_version"]
    pub fn tapleaf_version(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_tappath"]
    pub fn bitcoin_tappath(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_tappath"]
    pub fn tappath(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_tappath_hash"]
    pub fn bitcoin_tappath_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_tappath_hash"]
    pub fn tappath_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_total_fee"]
    pub fn total_fee(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_total_input_value"]
    pub fn bitcoin_total_input_value(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_total_output_value"]
    pub fn bitcoin_total_output_value(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_transaction_id"]
    pub fn bitcoin_transaction_id(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_transaction_id"]
    pub fn transaction_id(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_tx_hash"]
    pub fn bitcoin_tx_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_tx_hash"]
    pub fn tx_hash(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_tx_is_final"]
    pub fn bitcoin_tx_is_final(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_tx_is_final"]
    pub fn tx_is_final(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_tx_lock_distance"]
    pub fn bitcoin_tx_lock_distance(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_tx_lock_distance"]
    pub fn tx_lock_distance(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_tx_lock_duration"]
    pub fn bitcoin_tx_lock_duration(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_tx_lock_duration"]
    pub fn tx_lock_duration(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_tx_lock_height"]
    pub fn bitcoin_tx_lock_height(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_tx_lock_height"]
    pub fn tx_lock_height(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_tx_lock_time"]
    pub fn bitcoin_tx_lock_time(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_tx_lock_time"]
    pub fn tx_lock_time(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_verify"]
    pub fn verify(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_bitcoin_version"]
    pub fn bitcoin_version(dst: *mut CFrameItem, src: *const CFrameItem, env: *const bitcoin::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_version"]
    pub fn version(dst: *mut CFrameItem, src: *const CFrameItem, env: *const elements::CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_6_c_xor_1"]
    pub fn xor_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_xor_16"]
    pub fn xor_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_xor_32"]
    pub fn xor_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_xor_64"]
    pub fn xor_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_xor_8"]
    pub fn xor_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_xor_xor_1"]
    pub fn xor_xor_1(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_xor_xor_16"]
    pub fn xor_xor_16(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_xor_xor_32"]
    pub fn xor_xor_32(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_xor_xor_64"]
    pub fn xor_xor_64(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
    #[link_name = "rustsimplicity_0_6_c_xor_xor_8"]
    pub fn xor_xor_8(dst: *mut CFrameItem, src: *const CFrameItem, env: *const c_void) -> bool;
}
