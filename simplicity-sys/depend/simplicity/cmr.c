#include "cmr.h"

#include "limitations.h"
#include "simplicity_alloc.h"
#include "simplicity_assert.h"

/* Deserialize a Simplicity 'program' and compute its CMR.
 *
 * Caution: no typechecking is performed, only a well-formedness check.
 *
 * If at any time malloc fails then '*error' is set to 'SIMPLICITY_ERR_MALLOC' and 'false' is returned,
 * Otherwise, 'true' is returned indicating that the result was successfully computed and returned in the '*error' value.
 *
 * If the operation completes successfully then '*error' is set to 'SIMPLICITY_NO_ERROR', and the 'cmr' array is filled in with the program's computed CMR.
 *
 * Precondition: NULL != error;
 *               unsigned char cmr[32]
 *               unsigned char program[program_len]
 */
bool rustsimplicity_0_6_computeCmr( simplicity_err* error, unsigned char* cmr, rustsimplicity_0_6_callback_decodeJet decodeJet
                          , const unsigned char* program, size_t program_len) {
  rustsimplicity_0_6_assert(NULL != error);
  rustsimplicity_0_6_assert(NULL != cmr);
  rustsimplicity_0_6_assert(NULL != program || 0 == program_len);

  bitstream stream = initializeBitstream(program, program_len);
  dag_node* dag = NULL;
  int_fast32_t dag_len = rustsimplicity_0_6_decodeMallocDag(&dag, decodeJet, NULL, &stream);
  if (dag_len <= 0) {
    rustsimplicity_0_6_assert(dag_len < 0);
    *error = (simplicity_err)dag_len;
  } else {
    rustsimplicity_0_6_assert(NULL != dag);
    rustsimplicity_0_6_assert((uint_fast32_t)dag_len <= DAG_LEN_MAX);
    *error = rustsimplicity_0_6_closeBitstream(&stream);
    sha256_fromMidstate(cmr, dag[dag_len-1].cmr.s);
  }

  rustsimplicity_0_6_free(dag);
  return IS_PERMANENT(*error);
}
