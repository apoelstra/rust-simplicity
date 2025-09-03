#include <stdlib.h>
#include <stdalign.h>
#include "simplicity/elements/env.h"
#include "simplicity/elements/txEnv.h"
#include "simplicity/elements/primitive.h"

typedef rawElementsBuffer rawBuffer;
typedef rawElementsBuffer rawBuffer;
typedef rawElementsOutput rawOutput;
typedef rawElementsInput rawInput;
typedef rawElementsTransaction rawTransaction;
typedef rawElementsTapEnv rawTapEnv;
 
const size_t rustsimplicity_0_6_c_sizeof_rawElementsBuffer = sizeof(rawElementsBuffer);
const size_t rustsimplicity_0_6_c_sizeof_rawElementsOutput = sizeof(rawElementsOutput);
const size_t rustsimplicity_0_6_c_sizeof_rawElementsInput = sizeof(rawElementsInput);
const size_t rustsimplicity_0_6_c_sizeof_rawElementsTransaction = sizeof(rawElementsTransaction);
const size_t rustsimplicity_0_6_c_sizeof_rawElementsTapEnv = sizeof(rawElementsTapEnv);
const size_t rustsimplicity_0_6_c_sizeof_elementsTxEnv = sizeof(txEnv);

const size_t rustsimplicity_0_6_c_alignof_rawElementsBuffer = alignof(rawElementsBuffer);
const size_t rustsimplicity_0_6_c_alignof_rawElementsOutput = alignof(rawElementsOutput);
const size_t rustsimplicity_0_6_c_alignof_rawElementsInput = alignof(rawElementsInput);
const size_t rustsimplicity_0_6_c_alignof_rawElementsTransaction = alignof(rawElementsTransaction);
const size_t rustsimplicity_0_6_c_alignof_rawElementsTapEnv = alignof(rawElementsTapEnv);
const size_t rustsimplicity_0_6_c_alignof_elementsTxEnv = alignof(txEnv);

void rustsimplicity_0_6_c_elements_set_txEnv(txEnv *result, const elementsTransaction *tx, const elementsTapEnv *taproot, const unsigned char *genesisHash, unsigned int ix)
{
    sha256_midstate genesis;
    sha256_toMidstate(genesis.s, genesisHash);
    *result = rustsimplicity_0_6_elements_build_txEnv(tx, taproot, &genesis, ix);
}
