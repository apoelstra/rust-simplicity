// SPDX-License-Identifier: CC0-1.0

//! Serialization of Policy as Simplicity

use crate::jet::{Elements, Jet};
use crate::merkle::cmr::ConstructibleCmr;
use crate::node::{CoreConstructible, JetConstructible, WitnessConstructible};
use crate::types;
use crate::FailEntropy;
use crate::{Cmr, ConstructNode, ToXOnlyPubkey};

use hashes::Hash;

use crate::value::Word;
use std::convert::TryFrom;
use std::sync::Arc;

/// Constructors for the assembly fragment.
pub trait AssemblyConstructible<'brand>: Sized {
    /// Construct the assembly fragment with the given CMR.
    ///
    /// The construction fails if the CMR alone is not enough information to construct the object.
    fn assembly(inference_context: &types::Context<'brand>, cmr: Cmr) -> Option<Self>;
}

impl<'brand> AssemblyConstructible<'brand> for ConstructibleCmr<'brand> {
    fn assembly(inference_context: &types::Context<'brand>, cmr: Cmr) -> Option<Self> {
        Some(ConstructibleCmr {
            cmr,
            inference_context: inference_context.shallow_clone(),
        })
    }
}

impl<'brand, J: Jet> AssemblyConstructible<'brand> for Arc<ConstructNode<'brand, J>> {
    fn assembly(_: &types::Context, _cmr: Cmr) -> Option<Self> {
        None
    }
}

pub fn unsatisfiable<'brand, N>(
    inference_context: &types::Context<'brand>,
    entropy: FailEntropy,
) -> N
where
    N: CoreConstructible<'brand>,
{
    N::fail(inference_context, entropy)
}

pub fn trivial<'brand, N>(inference_context: &types::Context<'brand>) -> N
where
    N: CoreConstructible<'brand>,
{
    N::unit(inference_context)
}

pub fn key<'brand, Pk, N, W>(inference_context: &types::Context<'brand>, key: &Pk, witness: W) -> N
where
    Pk: ToXOnlyPubkey,
    N: CoreConstructible<'brand>
        + JetConstructible<'brand, Elements>
        + WitnessConstructible<'brand, W>,
{
    let key_value = Word::u256(key.to_x_only_pubkey().serialize());
    let const_key = N::const_word(inference_context, key_value);
    let sighash_all = N::jet(inference_context, Elements::SigAllHash);
    let pair_key_msg = N::pair(&const_key, &sighash_all).expect("consistent types");
    let witness = N::witness(inference_context, witness);
    let pair_key_msg_sig = N::pair(&pair_key_msg, &witness).expect("consistent types");
    let bip_0340_verify = N::jet(inference_context, Elements::Bip0340Verify);

    N::comp(&pair_key_msg_sig, &bip_0340_verify).expect("consistent types")
}

pub fn after<'brand, N>(inference_context: &types::Context<'brand>, n: u32) -> N
where
    N: CoreConstructible<'brand> + JetConstructible<'brand, Elements>,
{
    let n_value = Word::u32(n);
    let const_n = N::const_word(inference_context, n_value);
    let check_lock_height = N::jet(inference_context, Elements::CheckLockHeight);

    N::comp(&const_n, &check_lock_height).expect("consistent types")
}

pub fn older<'brand, N>(inference_context: &types::Context<'brand>, n: u16) -> N
where
    N: CoreConstructible<'brand> + JetConstructible<'brand, Elements>,
{
    let n_value = Word::u16(n);
    let const_n = N::const_word(inference_context, n_value);
    let check_lock_distance = N::jet(inference_context, Elements::CheckLockDistance);

    N::comp(&const_n, &check_lock_distance).expect("consistent types")
}

pub fn compute_sha256<'brand, N>(witness256: &N) -> N
where
    N: CoreConstructible<'brand> + JetConstructible<'brand, Elements>,
{
    let ctx = N::jet(witness256.inference_context(), Elements::Sha256Ctx8Init);
    let pair_ctx_witness = N::pair(&ctx, witness256).expect("consistent types");
    let add256 = N::jet(witness256.inference_context(), Elements::Sha256Ctx8Add32);
    let digest_ctx = N::comp(&pair_ctx_witness, &add256).expect("consistent types");
    let finalize = N::jet(witness256.inference_context(), Elements::Sha256Ctx8Finalize);
    N::comp(&digest_ctx, &finalize).expect("consistent types")
}

pub fn verify_bexp<'brand, N>(input: &N, bexp: &N) -> N
where
    N: CoreConstructible<'brand> + JetConstructible<'brand, Elements>,
{
    assert_eq!(
        input.inference_context(),
        bexp.inference_context(),
        "cannot compose policy fragments with different type inference contexts",
    );
    let computed_bexp = N::comp(input, bexp).expect("consistent types");
    let verify = N::jet(input.inference_context(), Elements::Verify);
    N::comp(&computed_bexp, &verify).expect("consistent types")
}

pub fn sha256<'brand, Pk, N, W>(
    inference_context: &types::Context<'brand>,
    hash: &Pk::Sha256,
    witness: W,
) -> N
where
    Pk: ToXOnlyPubkey,
    N: CoreConstructible<'brand>
        + JetConstructible<'brand, Elements>
        + WitnessConstructible<'brand, W>,
{
    let hash_value = Word::u256(Pk::to_sha256(hash).to_byte_array());
    let const_hash = N::const_word(inference_context, hash_value);
    let witness256 = N::witness(inference_context, witness);
    let computed_hash = compute_sha256(&witness256);
    let pair_hash_computed_hash = N::pair(&const_hash, &computed_hash).expect("consistent types");
    let eq256 = N::jet(inference_context, Elements::Eq256);

    verify_bexp(&pair_hash_computed_hash, &eq256)
}

pub fn and<'brand, N>(left: &N, right: &N) -> N
where
    N: CoreConstructible<'brand>,
{
    N::comp(left, right).expect("consistent types")
}

pub fn selector<'brand, N, W>(inference_context: &types::Context<'brand>, witness_bit: W) -> N
where
    N: CoreConstructible<'brand> + WitnessConstructible<'brand, W>,
{
    let witness = N::witness(inference_context, witness_bit);
    let unit = N::unit(inference_context);
    N::pair(&witness, &unit).expect("consistent types")
}

pub fn or<'brand, N, W>(left: &N, right: &N, witness_bit: W) -> N
where
    N: CoreConstructible<'brand> + WitnessConstructible<'brand, W>,
{
    assert_eq!(
        left.inference_context(),
        right.inference_context(),
        "cannot compose policy fragments with different type inference contexts",
    );
    let drop_left = N::drop_(left);
    let drop_right = N::drop_(right);
    let case_left_right = N::case(&drop_left, &drop_right).expect("consistent types");
    let selector = selector(left.inference_context(), witness_bit);

    N::comp(&selector, &case_left_right).expect("consistent types")
}

/// child: 1 → 1
///
/// summand(child): 1 → 2^32
pub fn thresh_summand<'brand, N, W>(child: &N, witness_bit: W) -> N
where
    N: CoreConstructible<'brand> + WitnessConstructible<'brand, W>,
{
    // 1 → 2 x 1
    let selector = selector(child.inference_context(), witness_bit);

    // 1 → 2^32
    let const_one = N::const_word(child.inference_context(), Word::u32(1));
    // 1 → 2^32
    let child_one = N::comp(child, &const_one).expect("consistent types");
    // 1 → 2^32
    let const_zero = N::const_word(child.inference_context(), Word::u32(0));

    // 1 × 1 → 2^32
    let drop_left = N::drop_(&const_zero);
    // 1 × 1 → 2^32
    let drop_right = N::drop_(&child_one);
    // 2 x 1 → 2^32
    let child_one_or_zero = N::case(&drop_left, &drop_right).expect("consistent types");

    // 1 → 2^32
    N::comp(&selector, &child_one_or_zero).expect("consistent types")
}

/// acc: 1 → 2^32, summand: 1 → 2^32
///
/// add(sum, summand): 1 → 2^32
pub fn thresh_add<'brand, N>(sum: &N, summand: &N) -> N
where
    N: CoreConstructible<'brand> + JetConstructible<'brand, Elements>,
{
    assert_eq!(
        sum.inference_context(),
        summand.inference_context(),
        "cannot compose policy fragments with different type inference contexts",
    );
    // 1 → 2^32 × 2^32
    let pair_sum_summand = N::pair(sum, summand).expect("consistent types");
    // 2^32 × 2^32 → 2 × 2^32
    let add32 = N::jet(sum.inference_context(), Elements::Add32);
    // 1 → 2 x 2^32
    let full_sum = N::comp(&pair_sum_summand, &add32).expect("consistent types");
    // 2^32 → 2^32
    let iden = N::iden(sum.inference_context());
    // 2 × 2^32 → 2^32
    let drop_iden = N::drop_(&iden);

    // Discard the overflow bit.
    // FIXME: enforce that sum of weights is less than 2^32
    // 1 → 2^32
    N::comp(&full_sum, &drop_iden).expect("consistent types")
}

/// verify(sum): 1 → 1
pub fn thresh_verify<'brand, N>(sum: &N, k: u32) -> N
where
    N: CoreConstructible<'brand> + JetConstructible<'brand, Elements>,
{
    // 1 → 2^32
    let const_k = N::const_word(sum.inference_context(), Word::u32(k));
    // 1 → 2^32 × 2^32
    let pair_k_sum = N::pair(&const_k, sum).expect("consistent types");
    // 2^32 × 2^32 → 2
    let eq32 = N::jet(sum.inference_context(), Elements::Eq32);

    // 1 → 1
    verify_bexp(&pair_k_sum, &eq32)
}

pub fn threshold<'brand, N, W>(k: u32, subs: &[N], witness_bits: &[W]) -> N
where
    N: CoreConstructible<'brand>
        + JetConstructible<'brand, Elements>
        + WitnessConstructible<'brand, W>,
    W: Clone,
{
    let n = u32::try_from(subs.len()).expect("can have at most 2^32 children in a threshold");
    assert!(k <= n, "threshold must be <= number of children");
    assert!(!subs.is_empty(), "cannot have empty threshold");

    let mut sum = thresh_summand(&subs[0], witness_bits[0].clone());
    for (sub, witness_bit) in subs[1..].iter().zip(witness_bits[1..].iter()) {
        let summand = thresh_summand(sub, witness_bit.clone());
        sum = thresh_add(&sum, &summand);
    }

    thresh_verify(&sum, k)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jet::elements::ElementsEnv;
    use crate::node::SimpleFinalizer;
    use crate::policy::Policy;
    use crate::{BitMachine, CommitNode, FailEntropy, Value};
    use elements::bitcoin::key::XOnlyPublicKey;
    use elements::locktime::Height;
    use elements::secp256k1_zkp;
    use hashes::{sha256, Hash};
    use std::sync::Arc;

    fn compile(
        policy: Policy<XOnlyPublicKey>,
    ) -> (
        Arc<CommitNode<Elements>>,
        ElementsEnv<impl core::borrow::Borrow<elements::Transaction>>,
    ) {
        let commit = policy.commit().expect("no asm");
        let env = ElementsEnv::dummy();

        (commit, env)
    }

    fn execute_successful(
        commit: &CommitNode<Elements>,
        witness: Vec<Value>,
        env: &ElementsEnv<impl core::borrow::Borrow<elements::Transaction>>,
    ) -> bool {
        let finalized = commit
            .finalize(&mut SimpleFinalizer::new(witness.into_iter()))
            .expect("finalize");
        let mut mac = match BitMachine::for_program(&finalized) {
            Ok(mac) => mac,
            Err(_) => return false,
        };

        match mac.exec(&finalized, env) {
            Ok(output) => output == Value::unit(),
            Err(_) => false,
        }
    }

    #[test]
    fn execute_unsatisfiable() {
        let (commit, env) = compile(Policy::Unsatisfiable(FailEntropy::ZERO));
        assert!(!execute_successful(&commit, vec![], &env));
    }

    #[test]
    fn execute_trivial() {
        let (commit, env) = compile(Policy::Trivial);
        assert!(execute_successful(&commit, vec![], &env));
    }

    #[test]
    fn execute_pk() {
        let env = ElementsEnv::dummy();

        let sighash = env.c_tx_env().sighash_all();
        let secp = secp256k1_zkp::Secp256k1::new();
        let keypair = secp256k1_zkp::Keypair::new(&secp, &mut secp256k1_zkp::rand::rngs::OsRng);
        let message = secp256k1_zkp::Message::from_digest(sighash.to_byte_array());
        let signature = keypair.sign_schnorr(message);

        let (xonly, _) = keypair.x_only_public_key();
        let commit = Policy::Key(xonly).commit().expect("no asm");

        assert!(execute_successful(
            &commit,
            vec![Value::u512(signature.serialize())],
            &env
        ));
    }

    #[test]
    fn execute_after() {
        let height = Height::from_consensus(42).unwrap();
        let env =
            ElementsEnv::dummy_with(elements::LockTime::Blocks(height), elements::Sequence::ZERO);

        let commit = Policy::<XOnlyPublicKey>::After(41)
            .commit()
            .expect("no asm");
        assert!(execute_successful(&commit, vec![], &env));

        let commit = Policy::<XOnlyPublicKey>::After(42)
            .commit()
            .expect("no asm");
        assert!(execute_successful(&commit, vec![], &env));

        let commit = Policy::<XOnlyPublicKey>::After(43)
            .commit()
            .expect("no asm");
        assert!(!execute_successful(&commit, vec![], &env));
    }

    #[test]
    fn execute_older() {
        let env = ElementsEnv::dummy_with(
            elements::LockTime::ZERO,
            elements::Sequence::from_consensus(42),
        );

        let commit = Policy::<XOnlyPublicKey>::Older(41)
            .commit()
            .expect("no asm");
        assert!(execute_successful(&commit, vec![], &env));

        let commit = Policy::<XOnlyPublicKey>::Older(42)
            .commit()
            .expect("no asm");
        assert!(execute_successful(&commit, vec![], &env));

        let commit = Policy::<XOnlyPublicKey>::Older(43)
            .commit()
            .expect("no asm");
        assert!(!execute_successful(&commit, vec![], &env));
    }

    #[test]
    fn execute_sha256() {
        let preimage = [1; 32];
        let image = sha256::Hash::hash(&preimage);
        let (commit, env) = compile(Policy::Sha256(image));

        let valid_witness = vec![Value::u256(preimage)];
        assert!(execute_successful(&commit, valid_witness, &env));

        let invalid_witness = vec![Value::u256([0; 32])];
        assert!(!execute_successful(&commit, invalid_witness, &env));
    }

    #[test]
    fn execute_and() {
        let preimage0 = [1; 32];
        let image0 = sha256::Hash::hash(&preimage0);
        let preimage1 = [2; 32];
        let image1 = sha256::Hash::hash(&preimage1);

        let (commit, env) = compile(Policy::And {
            left: Arc::new(Policy::Sha256(image0)),
            right: Arc::new(Policy::Sha256(image1)),
        });

        let valid_witness = vec![Value::u256(preimage0), Value::u256(preimage1)];
        assert!(execute_successful(&commit, valid_witness, &env));

        let invalid_witness = vec![Value::u256(preimage0), Value::u256([0; 32])];
        assert!(!execute_successful(&commit, invalid_witness, &env));

        let invalid_witness = vec![Value::u256([0; 32]), Value::u256(preimage1)];
        assert!(!execute_successful(&commit, invalid_witness, &env));
    }

    #[test]
    fn execute_and_true() {
        let preimage0 = [1; 32];
        let image0 = sha256::Hash::hash(&preimage0);

        let (commit, env) = compile(Policy::And {
            left: Arc::new(Policy::Sha256(image0)),
            right: Arc::new(Policy::Trivial),
        });

        let valid_witness = vec![Value::u256(preimage0)];
        assert!(execute_successful(&commit, valid_witness, &env));

        let invalid_witness = vec![Value::u256([0; 32])];
        assert!(!execute_successful(&commit, invalid_witness, &env));
    }

    #[test]
    fn execute_or() {
        let preimage0 = [1; 32];
        let image0 = sha256::Hash::hash(&preimage0);
        let preimage1 = [2; 32];
        let image1 = sha256::Hash::hash(&preimage1);

        let (commit, env) = compile(Policy::Or {
            left: Arc::new(Policy::Sha256(image0)),
            right: Arc::new(Policy::Sha256(image1)),
        });

        let valid_witness = vec![Value::u1(0), Value::u256(preimage0), Value::u256([0; 32])];
        assert!(execute_successful(&commit, valid_witness, &env));
        let valid_witness = vec![Value::u1(1), Value::u256([0; 32]), Value::u256(preimage1)];
        assert!(execute_successful(&commit, valid_witness, &env));

        let invalid_witness = vec![Value::u1(0), Value::u256([0; 32]), Value::u256(preimage1)];
        assert!(!execute_successful(&commit, invalid_witness, &env));
        let invalid_witness = vec![Value::u1(1), Value::u256(preimage0), Value::u256([0; 32])];
        assert!(!execute_successful(&commit, invalid_witness, &env));
    }

    #[test]
    fn execute_threshold() {
        let preimage0 = [1; 32];
        let image0 = sha256::Hash::hash(&preimage0);
        let preimage1 = [2; 32];
        let image1 = sha256::Hash::hash(&preimage1);
        let preimage2 = [3; 32];
        let image2 = sha256::Hash::hash(&preimage2);

        let (commit, env) = compile(Policy::Threshold(
            2,
            vec![
                Policy::Sha256(image0),
                Policy::Sha256(image1),
                Policy::Sha256(image2),
            ],
        ));

        let valid_witness = vec![
            Value::u1(1),
            Value::u256(preimage0),
            Value::u1(1),
            Value::u256(preimage1),
            Value::u1(0),
            Value::u256([0; 32]),
        ];
        assert!(execute_successful(&commit, valid_witness, &env));

        let valid_witness = vec![
            Value::u1(1),
            Value::u256(preimage0),
            Value::u1(0),
            Value::u256([0; 32]),
            Value::u1(1),
            Value::u256(preimage2),
        ];
        assert!(execute_successful(&commit, valid_witness, &env));

        let valid_witness = vec![
            Value::u1(0),
            Value::u256([0; 32]),
            Value::u1(1),
            Value::u256(preimage1),
            Value::u1(1),
            Value::u256(preimage2),
        ];
        assert!(execute_successful(&commit, valid_witness, &env));

        let invalid_witness = vec![
            Value::u1(1),
            Value::u256(preimage0),
            Value::u1(1),
            Value::u256(preimage1),
            Value::u1(1),
            Value::u256(preimage2),
        ];
        assert!(!execute_successful(&commit, invalid_witness, &env));

        let invalid_witness = vec![
            Value::u1(1),
            Value::u256(preimage1),
            Value::u1(1),
            Value::u256(preimage0),
            Value::u1(0),
            Value::u256([0; 32]),
        ];
        assert!(!execute_successful(&commit, invalid_witness, &env));

        let invalid_witness = vec![
            Value::u1(1),
            Value::u256(preimage0),
            Value::u1(0),
            Value::u256([0; 32]),
            Value::u1(0),
            Value::u256([0; 32]),
        ];
        assert!(!execute_successful(&commit, invalid_witness, &env));
    }
}
