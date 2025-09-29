use simplicity::bitcoin::{Amount, OutPoint, Transaction, Txid, TxIn, TxOut, ScriptBuf, Sequence, Witness};
use simplicity::bitcoin::key::{TapTweak as _, XOnlyPublicKey};
use simplicity::bitcoin::locktime::absolute::LockTime;
use simplicity::bitcoin::taproot::ControlBlock;
use simplicity::bitcoin::transaction;
use simplicity::hashes::Hash;
use simplicity::jet::bitcoin::BitcoinEnv;
use simplicity::Cmr;
use std::sync::Arc;

pub struct EnvSampling {
    /// Number of inputs in the transaction
    n_in: usize,
    /// Number of outputs in the transaction
    n_out: usize,
}

impl EnvSampling {
    /// Null Bitcoin transaction environment.
    ///
    /// For jets that do not use the environment, you may use this as-is. If the
    /// jet does need the environment, you should call [`Self::n_inputs`] and
    /// [`Self::n_outputs`] to set nonzero input and output counts.
    pub fn null() -> Self {
        EnvSampling {
            n_in: 0,
            n_out: 0,
        }
    }

    /// Attach a number of inputs to the environment
    pub fn n_inputs(self, n_in: usize) -> Self {
        EnvSampling { n_in, ..self }
    }

    /// Attach a number of inputs to the environment
    pub fn n_outputs(self, n_out: usize) -> Self {
        EnvSampling { n_out, ..self }
    }

    /// Obtain a random environment from the sampler.
    pub fn env(&self) -> BitcoinEnv<Arc<Transaction>> {
        let mut tx = Transaction {
            version: transaction::Version::TWO,
            lock_time: LockTime::ZERO,
            input: Vec::new(),
            output: Vec::new(),
        };
        let mut utxos = Vec::with_capacity(self.n_in);

        // Add inputs
        for _ in 0..self.n_in {
            let (txin, spent_utxo) = random_input();
            tx.input.push(txin);
            utxos.push(spent_utxo);
        }
        // Add outputs
        for _ in 0..self.n_out {
            let txout = random_output();
            tx.output.push(txout);
        }

        // Build txenv
        let ctrl_blk: [u8; 33] = [
            0xc0, 0xeb, 0x04, 0xb6, 0x8e, 0x9a, 0x26, 0xd1, 0x16, 0x04, 0x6c, 0x76, 0xe8, 0xff, 0x47,
            0x33, 0x2f, 0xb7, 0x1d, 0xda, 0x90, 0xff, 0x4b, 0xef, 0x53, 0x70, 0xf2, 0x52, 0x26, 0xd3,
            0xbc, 0x09, 0xfc,
        ];
        BitcoinEnv::new(
            Arc::new(tx),
            &utxos,
            u32::default(),
            Cmr::from_byte_array([0xab; 32]), // dummy values
            ControlBlock::decode(&ctrl_blk).unwrap(),
        )
    }
}

fn random_output() -> TxOut {
    // random 64 bit value
    // Appx random as we take mode over 21 mil, but really does
    // not matter
    let value = rand::random::<u64>() % Amount::MAX.to_sat();
    let xpk = "21ddd128bfa066994cb4be6880f0589075996203d8a7e88f136eec662b8912b8";
    let output_key = xpk
        .parse::<XOnlyPublicKey>()
        .unwrap()
        .dangerous_assume_tweaked();
    TxOut {
        value: Amount::from_sat(value),
        script_pubkey: ScriptBuf::new_p2tr_tweaked(output_key),
    }
}

fn random_input() -> (TxIn, TxOut) {
    let txout = random_output();
    let tx_bytes = rand::random::<[u8; 32]>();
    let vout = rand::random::<u16>() as u32;

    let mut witness = Witness::new();
    witness.push(b"witness data");
    witness.push(b"leaf script i.e. simplicity program");
    witness.push(b"control block");
    if rand::random() {
        witness.push(b"annex");
    }
    
    let txin = TxIn {
        previous_output: OutPoint::new(Txid::from_byte_array(tx_bytes), vout),
        script_sig: ScriptBuf::new(),
        sequence: Sequence::MAX,
        witness,
    };
    (txin, txout)
}
