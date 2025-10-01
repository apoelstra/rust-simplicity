// SPDX-License-Identifier: CC0-1.0

use crate::Cmr;

use bitcoin::taproot::ControlBlock;
use simplicity_sys::c_jets::c_env::bitcoin as c_bitcoin;

use super::c_env;

/// Environment for Bitcoin Simplicity
pub struct BitcoinEnv<T> {
    /// The CTxEnv struct
    c_tx_env: c_bitcoin::CTxEnv,
    tx: T,
    ix: u32,
}

impl<T> BitcoinEnv<T>
where
    T: core::borrow::Borrow<bitcoin::Transaction>,
{
    pub fn new(
        tx: T,
        utxos: &[bitcoin::TxOut],
        ix: u32,
        script_cmr: Cmr,
        control_block: ControlBlock,
    ) -> Self {
        let c_tx = c_env::new_tx(tx.borrow(), utxos);
        let c_tap_env = c_env::new_tap_env(&control_block, script_cmr);
        let c_tx_env = c_env::new_tx_env(c_tx, c_tap_env, ix);

        BitcoinEnv {
            c_tx_env,
            tx,
            ix
        }
    }

    /// The transaction of this environment
    pub fn tx(&self) -> &bitcoin::Transaction {
        self.tx.borrow()
    }

    /// The input index of this environment
    pub fn ix(&self) -> u32 {
        self.ix
    }

    /// A version of this environment used by the C implementation of the Bit Machine.
    pub fn c_tx_env(&self) -> &c_bitcoin::CTxEnv {
        &self.c_tx_env
    }
}
