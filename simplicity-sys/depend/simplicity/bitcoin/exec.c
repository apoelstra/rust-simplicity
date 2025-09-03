#include <simplicity/bitcoin/exec.h>

#include <stdalign.h>
#include <string.h>
#include "primitive.h"
#include "txEnv.h"
#include "../deserialize.h"
#include "../eval.h"
#include "../limitations.h"
#include "../simplicity_alloc.h"
#include "../simplicity_assert.h"
#include "../typeInference.h"

/* Deserialize a Simplicity 'program' with its 'witness' data and execute it in the environment of the 'ix'th input of 'tx' with `taproot`.
 *
 * If at any time malloc fails then '*error' is set to 'SIMPLICITY_ERR_MALLOC' and 'false' is returned,
 * meaning we were unable to determine the result of the simplicity program.
 * Otherwise, 'true' is returned indicating that the result was successfully computed and returned in the '*error' value.
 *
 * If deserialization, analysis, or execution fails, then '*error' is set to some simplicity_err.
 *
 * If 'amr != NULL' and the annotated Merkle root of the decoded expression doesn't match 'amr' then '*error' is set to 'SIMPLICITY_ERR_AMR'.
 *
 * Otherwise '*error' is set to 'SIMPLICITY_NO_ERROR'.
 *
 * If 'ihr != NULL' and '*error' is set to 'SIMPLICITY_NO_ERROR', then the identity hash of the root of the decoded expression is written to 'ihr'.
 * Otherwise if 'ihr != NULL'  and '*error' is not set to 'SIMPLCITY_NO_ERROR', then 'ihr' may or may not be written to.
 *
 * Precondition: NULL != error;
 *               NULL != ihr implies unsigned char ihr[32]
 *               NULL != tx;
 *               NULL != taproot;
 *               0 <= minCost <= budget;
 *               NULL != amr implies unsigned char amr[32]
 *               unsigned char program[program_len]
 *               unsigned char witness[witness_len]
 */
extern bool rustsimplicity_0_6_bitcoin_execSimplicity( simplicity_err* error, unsigned char* ihr
                                              , const bitcoinTransaction* tx, uint_fast32_t ix, const bitcoinTapEnv* taproot
                                              , int64_t minCost, int64_t budget
                                              , const unsigned char* amr
                                              , const unsigned char* program, size_t program_len
                                              , const unsigned char* witness, size_t witness_len) {
  rustsimplicity_0_6_assert(NULL != error);
  rustsimplicity_0_6_assert(NULL != tx);
  rustsimplicity_0_6_assert(NULL != taproot);
  rustsimplicity_0_6_assert(0 <= minCost);
  rustsimplicity_0_6_assert(minCost <= budget);
  rustsimplicity_0_6_assert(NULL != program || 0 == program_len);
  rustsimplicity_0_6_assert(NULL != witness || 0 == witness_len);

  combinator_counters census;
  dag_node* dag = NULL;
  int_fast32_t dag_len;
  sha256_midstate amr_hash;

  if (amr) sha256_toMidstate(amr_hash.s, amr);

  {
    bitstream stream = initializeBitstream(program, program_len);
    dag_len = rustsimplicity_0_6_decodeMallocDag(&dag, rustsimplicity_0_6_bitcoin_decodeJet, &census, &stream);
    if (dag_len <= 0) {
      rustsimplicity_0_6_assert(dag_len < 0);
      *error = (simplicity_err)dag_len;
      return IS_PERMANENT(*error);
    }
    rustsimplicity_0_6_assert(NULL != dag);
    rustsimplicity_0_6_assert((uint_fast32_t)dag_len <= DAG_LEN_MAX);
    *error = rustsimplicity_0_6_closeBitstream(&stream);
  }

  if (IS_OK(*error)) {
    if (0 != memcmp(taproot->scriptCMR.s, dag[dag_len-1].cmr.s, sizeof(uint32_t[8]))) {
      *error = SIMPLICITY_ERR_CMR;
    }
  }

  if (IS_OK(*error)) {
    type* type_dag = NULL;
    *error = rustsimplicity_0_6_mallocTypeInference(&type_dag, rustsimplicity_0_6_bitcoin_mallocBoundVars, dag, (uint_fast32_t)dag_len, &census);
    if (IS_OK(*error)) {
      rustsimplicity_0_6_assert(NULL != type_dag);
      if (0 != dag[dag_len-1].sourceType || 0 != dag[dag_len-1].targetType) {
        *error = SIMPLICITY_ERR_TYPE_INFERENCE_NOT_PROGRAM;
      }
    }
    if (IS_OK(*error)) {
      bitstream witness_stream = initializeBitstream(witness, witness_len);
      *error = rustsimplicity_0_6_fillWitnessData(dag, type_dag, (uint_fast32_t)dag_len, &witness_stream);
      if (IS_OK(*error)) {
        *error = rustsimplicity_0_6_closeBitstream(&witness_stream);
        if (SIMPLICITY_ERR_BITSTREAM_TRAILING_BYTES == *error) *error = SIMPLICITY_ERR_WITNESS_TRAILING_BYTES;
        if (SIMPLICITY_ERR_BITSTREAM_ILLEGAL_PADDING == *error) *error = SIMPLICITY_ERR_WITNESS_ILLEGAL_PADDING;
      }
    }
    if (IS_OK(*error)) {
      sha256_midstate ihr_buf;
      *error = rustsimplicity_0_6_verifyNoDuplicateIdentityHashes(&ihr_buf, dag, type_dag, (uint_fast32_t)dag_len);
      if (IS_OK(*error) && ihr) sha256_fromMidstate(ihr, ihr_buf.s);
    }
    if (IS_OK(*error) && amr) {
      static_assert(DAG_LEN_MAX <= SIZE_MAX / sizeof(analyses), "analysis array too large.");
      static_assert(1 <= DAG_LEN_MAX, "DAG_LEN_MAX is zero.");
      static_assert(DAG_LEN_MAX - 1 <= UINT32_MAX, "analysis array index does nto fit in uint32_t.");
      analyses *analysis = rustsimplicity_0_6_malloc((size_t)dag_len * sizeof(analyses));
      if (analysis) {
        rustsimplicity_0_6_computeAnnotatedMerkleRoot(analysis, dag, type_dag, (uint_fast32_t)dag_len);
        if (0 != memcmp(amr_hash.s, analysis[dag_len-1].annotatedMerkleRoot.s, sizeof(uint32_t[8]))) {
          *error = SIMPLICITY_ERR_AMR;
        }
      } else {
        /* malloc failed which counts as a transient error. */
        *error = SIMPLICITY_ERR_MALLOC;
      }
      rustsimplicity_0_6_free(analysis);
    }
    if (IS_OK(*error)) {
      txEnv env = rustsimplicity_0_6_bitcoin_build_txEnv(tx, taproot, ix);
      static_assert(BUDGET_MAX <= UBOUNDED_MAX, "BUDGET_MAX doesn't fit in ubounded.");
      *error = evalTCOProgram( dag, type_dag, (size_t)dag_len
                             , minCost <= BUDGET_MAX ? (ubounded)minCost : BUDGET_MAX
                             , &(ubounded){budget <= BUDGET_MAX ? (ubounded)budget : BUDGET_MAX}
                             , &env);
    }
    rustsimplicity_0_6_free(type_dag);
  }

  rustsimplicity_0_6_free(dag);
  return IS_PERMANENT(*error);
}
