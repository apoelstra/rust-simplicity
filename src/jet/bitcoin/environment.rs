// SPDX-License-Identifier: CC0-1.0

use simplicity_sys::c_jets::c_env::bitcoin as c_bitcoin;

/// Environment for Bitcoin Simplicity
pub struct BitcoinEnv<T> {
    pub tx: T,
}

impl<T> BitcoinEnv<T>
where
    T: core::borrow::Borrow<bitcoin::Transaction>,
{
    pub fn new(tx: T) -> Self {
        BitcoinEnv { tx }
    }

    pub fn c_tx_env(&self) -> &c_bitcoin::CTxEnv {
        unimplemented!()
    }
}
