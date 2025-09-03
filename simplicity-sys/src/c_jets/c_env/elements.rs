// SPDX-License-Identifier: CC0-1.0

use hashes::{sha256, Hash};

use crate::ffi::sha256::CSha256Midstate;
use crate::ffi::{c_size_t, c_uchar, c_uint, c_uint_fast32_t};

#[derive(Debug)]
#[repr(C)]
pub struct CRawBuffer {
    pub ptr: *const c_uchar,
    pub len: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct CRawOutput<'raw> {
    pub asset: Option<&'raw [c_uchar; 33]>,
    pub value: *const c_uchar,
    pub nonce: Option<&'raw [c_uchar; 33]>,
    pub script_pubkey: CRawBuffer,
    pub surjection_proof: CRawBuffer,
    pub range_proof: CRawBuffer,
}

#[repr(C)]
pub struct CRawInputIssuance<'raw> {
    pub blinding_nonce: Option<&'raw [c_uchar; 32]>,
    pub asset_entropy: Option<&'raw [c_uchar; 32]>,
    pub amount: *const c_uchar,
    pub inflation_keys: *const c_uchar,
    pub amount_range_proof: CRawBuffer,
    pub inflation_keys_range_proof: CRawBuffer,
}

impl<'raw> CRawInputIssuance<'raw> {
    /// Constructs a raw input issuance structure corresponding to "no issuance".
    pub fn no_issuance() -> Self {
        Self {
            blinding_nonce: None,
            asset_entropy: None,
            amount: core::ptr::null(),
            inflation_keys: core::ptr::null(),
            amount_range_proof: CRawBuffer::new(&[]),
            inflation_keys_range_proof: CRawBuffer::new(&[]),
        }
    }
}

#[repr(C)]
pub struct CRawInputTxo<'raw> {
    pub asset: Option<&'raw [c_uchar; 33]>,
    pub value: *const c_uchar,
    pub script_pubkey: CRawBuffer,
}

#[repr(C)]
pub struct CRawInput<'raw> {
    pub annex: *const CRawBuffer,
    pub prev_txid: &'raw [c_uchar; 32],
    pub pegin: Option<&'raw [c_uchar; 32]>,
    pub issuance: CRawInputIssuance<'raw>,
    pub txo: CRawInputTxo<'raw>,
    pub script_sig: CRawBuffer,
    pub prev_txout_index: u32,
    pub sequence: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct CRawTransaction<'raw> {
    pub txid: &'raw [c_uchar; 32],
    pub inputs: *const CRawInput<'raw>,
    pub outputs: *const CRawOutput<'raw>,
    pub n_inputs: u32,
    pub n_outputs: u32,
    pub version: u32,
    pub locktime: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct CRawTapEnv {
    pub control_block: *const c_uchar,
    pub script_cmr: *const c_uchar,
    pub branch_len: u8,
}

#[repr(C)]
pub struct CTransaction {
    _data: (),
}

#[derive(Debug)]
#[repr(C)]
pub struct CTxEnv {
    tx: *const CTransaction,
    taproot: *const CTapEnv,
    genesis_hash: CSha256Midstate,
    sighash_all: CSha256Midstate,
    ix: c_uint_fast32_t,
}

#[repr(C)]
pub struct CTapEnv {
    _data: (),
}

extern "C" {
    #[link_name = "rustsimplicity_0_6_c_sizeof_rawElementsBuffer"]
    pub static c_sizeof_rawBuffer: c_size_t;
    #[link_name = "rustsimplicity_0_6_c_sizeof_rawElementsOutput"]
    pub static c_sizeof_rawOutput: c_size_t;
    #[link_name = "rustsimplicity_0_6_c_sizeof_rawElementsInput"]
    pub static c_sizeof_rawInput: c_size_t;
    #[link_name = "rustsimplicity_0_6_c_sizeof_rawElementsTransaction"]
    pub static c_sizeof_rawTransaction: c_size_t;
    #[link_name = "rustsimplicity_0_6_c_sizeof_rawElementsTapEnv"]
    pub static c_sizeof_rawTapEnv: c_size_t;
    #[link_name = "rustsimplicity_0_6_c_sizeof_elementsTxEnv"]
    pub static c_sizeof_txEnv: c_size_t;

    #[link_name = "rustsimplicity_0_6_c_alignof_rawElementsBuffer"]
    pub static c_alignof_rawBuffer: c_size_t;
    #[link_name = "rustsimplicity_0_6_c_alignof_rawElementsOutput"]
    pub static c_alignof_rawOutput: c_size_t;
    #[link_name = "rustsimplicity_0_6_c_alignof_rawElementsInput"]
    pub static c_alignof_rawInput: c_size_t;
    #[link_name = "rustsimplicity_0_6_c_alignof_rawElementsTransaction"]
    pub static c_alignof_rawTransaction: c_size_t;
    #[link_name = "rustsimplicity_0_6_c_alignof_rawElementsTapEnv"]
    pub static c_alignof_rawTapEnv: c_size_t;
    #[link_name = "rustsimplicity_0_6_c_alignof_elementsTxEnv"]
    pub static c_alignof_txEnv: c_size_t;

    #[link_name = "rustsimplicity_0_6_c_elements_set_txEnv"]
    pub fn c_set_txEnv(
        result: *mut CTxEnv,
        tx: *const CTransaction,
        taproot: *const CTapEnv,
        genesisHash: *const c_uchar,
        ix: c_uint,
    );
    #[link_name = "rustsimplicity_0_6_elements_mallocTapEnv"]
    pub fn simplicity_mallocTapEnv(rawEnv: *const CRawTapEnv) -> *mut CTapEnv;
    #[link_name = "rustsimplicity_0_6_elements_mallocTransaction"]
    pub fn simplicity_mallocTransaction(rawTx: *const CRawTransaction) -> *mut CTransaction;
}
impl CTxEnv {
    pub fn sighash_all(&self) -> sha256::Hash {
        let midstate: sha256::Midstate = self.sighash_all.into();
        sha256::Hash::from_byte_array(midstate.to_byte_array())
    }
}

// Pointer must be manually free after dropping
impl Drop for CTxEnv {
    fn drop(&mut self) {
        unsafe {
            crate::alloc::rust_0_6_free(self.tx as *mut u8);
            crate::alloc::rust_0_6_free(self.taproot as *mut u8);
        }
    }
}

impl CRawBuffer {
    pub fn new(buf: &[c_uchar]) -> Self {
        Self {
            ptr: buf.as_ptr(),
            len: buf.len().try_into().expect("sane buffer lengths"),
        }
    }
}

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use crate::c_jets::frame_ffi::{c_alignof_frameItem, c_sizeof_frameItem, CFrameItem};

    use super::*;

    #[test]
    fn test_sizes() {
        unsafe {
            assert_eq!(size_of::<CFrameItem>(), c_sizeof_frameItem);
            assert_eq!(size_of::<CRawBuffer>(), c_sizeof_rawBuffer);
            assert_eq!(size_of::<CRawInput>(), c_sizeof_rawInput);
            assert_eq!(size_of::<CRawOutput>(), c_sizeof_rawOutput);
            assert_eq!(size_of::<CRawTransaction>(), c_sizeof_rawTransaction);
            assert_eq!(size_of::<CRawTapEnv>(), c_sizeof_rawTapEnv);
            assert_eq!(size_of::<CTxEnv>(), c_sizeof_txEnv);
        }
    }

    #[test]
    fn test_aligns() {
        unsafe {
            assert_eq!(align_of::<CFrameItem>(), c_alignof_frameItem);
            assert_eq!(align_of::<CRawBuffer>(), c_alignof_rawBuffer);
            assert_eq!(align_of::<CRawInput>(), c_alignof_rawInput);
            assert_eq!(align_of::<CRawOutput>(), c_alignof_rawOutput);
            assert_eq!(align_of::<CRawTransaction>(), c_alignof_rawTransaction);
            assert_eq!(align_of::<CRawTapEnv>(), c_alignof_rawTapEnv);
            assert_eq!(align_of::<CTxEnv>(), c_alignof_txEnv);
        }
    }
}
