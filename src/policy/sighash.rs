// SPDX-License-Identifier: CC0-1.0

use std::borrow::Borrow;
use std::ops::Deref;

use hashes::sha256;

use crate::jet::elements::ElementsUtxo;
use crate::Cmr;

pub struct SighashCache<T: Deref<Target = elements::Transaction>> {
    tx: T,
    fallback: elements::sighash::SighashCache<T>,
}

impl<T: Deref<Target = elements::Transaction> + Clone> SighashCache<T> {
    pub fn new(tx: T) -> Self {
        Self {
            tx: tx.clone(),
            fallback: elements::sighash::SighashCache::new(tx),
        }
    }

    pub fn taproot_key_spend_signature_hash<O>(
        &mut self,
        input_index: usize,
        prevouts: &elements::sighash::Prevouts<O>,
        sighash_type: elements::sighash::SchnorrSighashType,
        genesis_hash: elements::BlockHash,
    ) -> Result<elements::taproot::TapSighashHash, elements::sighash::Error>
    where
        O: Borrow<elements::TxOut>,
    {
        self.fallback.taproot_key_spend_signature_hash(
            input_index,
            prevouts,
            sighash_type,
            genesis_hash,
        )
    }

    pub fn taproot_script_spend_signature_hash<S, O>(
        &mut self,
        input_index: usize,
        prevouts: &elements::sighash::Prevouts<O>,
        leaf_hash: S,
        sighash_type: elements::sighash::SchnorrSighashType,
        genesis_hash: elements::BlockHash,
    ) -> Result<elements::taproot::TapSighashHash, elements::sighash::Error>
    where
        S: Into<elements::taproot::TapLeafHash>,
        O: Borrow<elements::TxOut>,
    {
        self.fallback.taproot_script_spend_signature_hash(
            input_index,
            prevouts,
            leaf_hash,
            sighash_type,
            genesis_hash,
        )
    }

    pub fn simplicity_spend_signature_hash<O>(
        &mut self,
        input_index: usize,
        prevouts: &elements::sighash::Prevouts<O>,
        script_cmr: Cmr,
        control_block: elements::taproot::ControlBlock,
        genesis_hash: elements::BlockHash,
    ) -> Result<sha256::Hash, elements::sighash::Error>
    where
        O: Borrow<elements::TxOut>,
    {
        let all = match prevouts {
            elements::sighash::Prevouts::All(prevouts) => *prevouts,
            _ => return Err(elements::sighash::Error::PrevoutKind),
        };
        let utxos: Vec<_> = all
            .iter()
            .map(|o| ElementsUtxo::from(O::borrow(o).clone()))
            .collect();

        let simplicity_env = crate::jet::elements::ElementsEnv::new(
            (*self.tx).borrow(),
            utxos,
            input_index as u32,
            script_cmr,
            control_block,
            None,
            genesis_hash,
        );
        let simplicity_sighash = simplicity_env.c_tx_env().sighash_all();

        Ok(simplicity_sighash)
    }
}
