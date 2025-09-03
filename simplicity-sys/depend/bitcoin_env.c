#include <stdlib.h>
#include <stdalign.h>
#include "simplicity/bitcoin/env.h"
#include "simplicity/bitcoin/txEnv.h"
#include "simplicity/bitcoin/primitive.h"

typedef rawBitcoinBuffer rawBuffer;
typedef rawBitcoinBuffer rawBuffer;
typedef rawBitcoinOutput rawOutput;
typedef rawBitcoinInput rawInput;
typedef rawBitcoinTransaction rawTransaction;
typedef rawBitcoinTapEnv rawTapEnv;
 
const size_t rustsimplicity_0_6_c_sizeof_rawBitcoinBuffer = sizeof(rawBitcoinBuffer);
const size_t rustsimplicity_0_6_c_sizeof_rawBitcoinOutput = sizeof(rawBitcoinOutput);
const size_t rustsimplicity_0_6_c_sizeof_rawBitcoinInput = sizeof(rawBitcoinInput);
const size_t rustsimplicity_0_6_c_sizeof_rawBitcoinTransaction = sizeof(rawBitcoinTransaction);
const size_t rustsimplicity_0_6_c_sizeof_rawBitcoinTapEnv = sizeof(rawBitcoinTapEnv);
const size_t rustsimplicity_0_6_c_sizeof_bitcoinTxEnv = sizeof(txEnv);

const size_t rustsimplicity_0_6_c_alignof_rawBitcoinBuffer = alignof(rawBitcoinBuffer);
const size_t rustsimplicity_0_6_c_alignof_rawBitcoinOutput = alignof(rawBitcoinOutput);
const size_t rustsimplicity_0_6_c_alignof_rawBitcoinInput = alignof(rawBitcoinInput);
const size_t rustsimplicity_0_6_c_alignof_rawBitcoinTransaction = alignof(rawBitcoinTransaction);
const size_t rustsimplicity_0_6_c_alignof_rawBitcoinTapEnv = alignof(rawBitcoinTapEnv);
const size_t rustsimplicity_0_6_c_alignof_bitcoinTxEnv = alignof(txEnv);

void rustsimplicity_0_6_c_bitcoin_set_txEnv(txEnv *result, const bitcoinTransaction *tx, const bitcoinTapEnv *taproot, unsigned int ix)
{
    *result = rustsimplicity_0_6_bitcoin_build_txEnv(tx, taproot, ix);
}
