// SPDX-License-Identifier: CC0-1.0

use crate::merkle::cmr::Cmr;
use elements::confidential;
use elements::taproot::ControlBlock;
use simplicity_sys::c_jets::c_env::elements as c_elements;
use std::borrow::Borrow;

use super::c_env;

/// An Elements UTXO
// This is not a complete TxOut as it does not contain the nonce that
// is sent to the recipient.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ElementsUtxo {
    /// The 'scriptpubkey' (hash of Simplicity program)
    pub script_pubkey: elements::Script,
    /// The explicit or confidential asset
    pub asset: confidential::Asset,
    /// The explict or confidential value
    pub value: confidential::Value,
}

impl From<elements::TxOut> for ElementsUtxo {
    fn from(txout: elements::TxOut) -> Self {
        ElementsUtxo {
            script_pubkey: txout.script_pubkey,
            asset: txout.asset,
            value: txout.value,
        }
    }
}

/// Environment for Elements Simplicity
///
/// # Note
/// The order of `utxos` must be same as of the order of inputs in the
/// transaction.
#[derive(Debug)]
pub struct ElementsEnv<T: Borrow<elements::Transaction>> {
    /// The CTxEnv struct
    c_tx_env: c_elements::CTxEnv,
    /// The elements transaction
    tx: T,
    /// the current index of the input
    ix: u32,
    /// Control block used to spend this leaf script
    control_block: ControlBlock,
    /// Optional Annex.
    annex: Option<Vec<u8>>,
    /// Genesis block hash
    genesis_hash: elements::BlockHash,
}

impl<T> ElementsEnv<T>
where
    T: Borrow<elements::Transaction>,
{
    pub fn new(
        tx: T,
        utxos: Vec<ElementsUtxo>,
        ix: u32,
        script_cmr: Cmr,
        control_block: ControlBlock,
        annex: Option<Vec<u8>>,
        genesis_hash: elements::BlockHash,
    ) -> Self {
        let c_tx = c_env::new_tx(tx.borrow(), &utxos);
        let c_tap_env = c_env::new_tap_env(&control_block, script_cmr);
        let c_tx_env = c_env::new_tx_env(c_tx, c_tap_env, genesis_hash, ix);
        ElementsEnv {
            c_tx_env,
            tx,
            ix,
            control_block,
            annex,
            genesis_hash,
        }
    }

    /// Obtains the FFI compatible CTxEnv from self
    pub fn c_tx_env(&self) -> &c_elements::CTxEnv {
        &self.c_tx_env
    }

    /// Returns the transaction of this environment
    pub fn tx(&self) -> &elements::Transaction {
        self.tx.borrow()
    }

    /// Returns the input index of this environment
    pub fn ix(&self) -> u32 {
        self.ix
    }

    /// Returns a reference to the control block of this [`ElementsEnv`].
    pub fn control_block(&self) -> &ControlBlock {
        &self.control_block
    }

    /// Returns the annex of this [`ElementsEnv`].
    pub fn annex(&self) -> Option<&Vec<u8>> {
        self.annex.as_ref()
    }

    /// Returns the genesis hash of this [`ElementsEnv`].
    pub fn genesis_hash(&self) -> elements::BlockHash {
        self.genesis_hash
    }
}

#[cfg(test)]
impl ElementsEnv<std::sync::Arc<elements::Transaction>> {
    /// Return a dummy Elements environment
    pub fn dummy() -> Self {
        Self::dummy_with(elements::LockTime::ZERO, elements::Sequence::MAX)
    }

    /// Return a dummy Elements environment with given locktime
    pub fn dummy_with(lock_time: elements::LockTime, sequence: elements::Sequence) -> Self {
        use elements::AssetIssuance;
        use hashes::Hash;

        let ctrl_blk: [u8; 33] = [
            0xc0, 0xeb, 0x04, 0xb6, 0x8e, 0x9a, 0x26, 0xd1, 0x16, 0x04, 0x6c, 0x76, 0xe8, 0xff,
            0x47, 0x33, 0x2f, 0xb7, 0x1d, 0xda, 0x90, 0xff, 0x4b, 0xef, 0x53, 0x70, 0xf2, 0x52,
            0x26, 0xd3, 0xbc, 0x09, 0xfc,
        ];

        ElementsEnv::new(
            std::sync::Arc::new(elements::Transaction {
                version: 2,
                lock_time,
                // Enable locktime in dummy txin
                input: vec![elements::TxIn {
                    previous_output: elements::OutPoint::default(),
                    is_pegin: false,
                    script_sig: elements::Script::new(),
                    sequence,
                    asset_issuance: AssetIssuance::default(),
                    witness: elements::TxInWitness::default(),
                }],
                output: Vec::default(),
            }),
            vec![ElementsUtxo {
                script_pubkey: elements::Script::new(),
                asset: confidential::Asset::Null,
                value: confidential::Value::Null,
            }],
            0,
            Cmr::from_byte_array([0; 32]),
            ControlBlock::from_slice(&ctrl_blk).unwrap(),
            None,
            elements::BlockHash::all_zeros(),
        )
    }
}
