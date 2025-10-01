// SPDX-License-Identifier: CC0-1.0

use crate::Cmr;

use bitcoin::hashes::Hash as _;
use bitcoin::taproot::ControlBlock;
use simplicity_sys::c_jets::c_env::bitcoin as c_bitcoin;

pub(super) fn new_tx(
    tx: &bitcoin::Transaction,
    in_utxos: &[bitcoin::TxOut],
) -> *mut c_bitcoin::CTransaction {
    let mut raw_inputs = Vec::with_capacity(tx.input.len());
    let mut raw_outputs = Vec::with_capacity(tx.output.len());

    for (inp, utxo) in tx
        .input
        .iter()
        .zip(in_utxos.iter())
    {
        raw_inputs.push(c_bitcoin::CRawInput {
            // FIXME actually pass the annex in; see https://github.com/BlockstreamResearch/simplicity/issues/311 for some difficulty here.
            annex: core::ptr::null(),
            prev_txid: inp.previous_output.txid.as_byte_array(),
            txo: c_bitcoin::CRawOutput {
                value: utxo.value.to_sat(),
                script_pubkey: c_bitcoin::CRawBuffer::new(utxo.script_pubkey.as_bytes()),
            },
            script_sig: c_bitcoin::CRawBuffer::new(inp.script_sig.as_bytes()),
            prev_txout_index: inp.previous_output.vout,
            sequence: inp.sequence.to_consensus_u32(),

        });
    }
    for out in tx.output.iter() {
        raw_outputs.push(c_bitcoin::CRawOutput {
            value: out.value.to_sat(),
            script_pubkey: c_bitcoin::CRawBuffer::new(out.script_pubkey.as_bytes()),
            
        });
    }
    let txid = tx.compute_txid();

    let c_raw_tx = c_bitcoin::CRawTransaction {
        txid: txid.as_byte_array(),
        inputs: raw_inputs.as_ptr(),
        outputs: raw_outputs.as_ptr(),
        n_inputs: raw_inputs.len().try_into().expect("sane length"),
        n_outputs: raw_outputs.len().try_into().expect("sane length"),
        version: tx.version.0 as u32, // in 1.87.0 can use .cast_unsigned
        locktime: tx.lock_time.to_consensus_u32(),
    };
    unsafe {
        // SAFETY: this is a FFI call and we constructed its argument correctly.
        c_bitcoin::simplicity_mallocTransaction(&c_raw_tx)
    }
}

pub(super) fn new_tap_env(
    control_block: &ControlBlock,
    script_cmr: Cmr,
) -> *mut c_bitcoin::CTapEnv {
    let cb_ser = control_block.serialize();
    let raw_tap_env = c_bitcoin::CRawTapEnv {
        control_block: cb_ser.as_ptr(),
        script_cmr: script_cmr.as_ref().as_ptr(),
        branch_len: control_block
            .merkle_branch
            .len()
            .try_into()
            .expect("sane length"),
    };

    unsafe {
        // SAFETY: this is a FFI call and we constructed its argument correctly.
        c_bitcoin::simplicity_mallocTapEnv(&raw_tap_env)
    }
}

pub(super) fn new_tx_env(
    tx: *const c_bitcoin::CTransaction,
    taproot: *const c_bitcoin::CTapEnv,
    ix: u32,
) -> c_bitcoin::CTxEnv {
    unsafe {
        let mut tx_env = std::mem::MaybeUninit::<c_bitcoin::CTxEnv>::uninit();
        c_bitcoin::c_set_txEnv(
            tx_env.as_mut_ptr(),
            tx,
            taproot,
            ix,
        );
        tx_env.assume_init()
    }
}

