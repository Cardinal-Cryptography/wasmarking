use wasm_bindgen::prelude::*;
use js_sys::Date;

#[cfg(feature = "parallel")]
pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen]
pub fn generate_proof() -> JsValue {
    let relation = JfRelation::Withdraw;
    let srs = relation.generate_srs();
    let (pk, _) = relation.generate_keys(&srs);

    let start = Date::now();
    relation.generate_proof(pk);
    (Date::now() - start).into()
}

use ark_ff::PrimeField;
use jf_primitives::{
    merkle_tree::{
        prelude::RescueSparseMerkleTree, MerkleCommitment, MerkleTreeScheme,
        UniversalMerkleTreeScheme,
    },
    pcs::prelude::UnivariateUniversalParams,
};
use jf_relations::{
    shielder_types::{compute_note, convert_array},
    withdraw::{WithdrawPrivateInput, WithdrawPublicInput, WithdrawRelation},
    Curve, Proof as JfProof, ProvingKey as JfProvingKey, Relation as _,
    VerifyingKey as JfVerifyingKey,
};
use num_bigint::BigUint;

pub enum JfRelation {
    Withdraw,
}

impl JfRelation {
    pub fn generate_srs(&self) -> UnivariateUniversalParams<Curve> {
        let rng = &mut jf_utils::test_rng();
        jf_relations::generate_srs(17_000, rng).unwrap()
    }

    pub fn generate_keys(
        &self,
        srs: &UnivariateUniversalParams<Curve>,
    ) -> (JfProvingKey<Curve>, JfVerifyingKey<Curve>) {
        WithdrawRelation::generate_keys(srs).unwrap()
    }

    pub fn generate_proof(&self, pk: JfProvingKey<Curve>) -> JfProof<Curve> {
        let rng = &mut jf_utils::test_rng();
        let relation = jf_relation();
        relation.generate_proof(&pk, rng).unwrap()
    }
}

fn jf_relation() -> WithdrawRelation {
    let token_id = 1;
    let whole_token_amount = 10;
    let spend_trapdoor = [1; 4];
    let spend_nullifier = [2; 4];
    let spend_note = compute_note(
        token_id,
        whole_token_amount,
        spend_trapdoor,
        spend_nullifier,
    );

    let deposit_token_amount = 7;
    let deposit_trapdoor = [3; 4];
    let deposit_nullifier = [4; 4];
    let deposit_note = compute_note(
        token_id,
        deposit_token_amount,
        deposit_trapdoor,
        deposit_nullifier,
    );

    let leaf_index = 0u64;
    let uid = BigUint::from(leaf_index);
    let elem = convert_array(spend_note);
    let mt = RescueSparseMerkleTree::from_kv_set(11, &[(uid.clone(), elem)]).unwrap();
    let (retrieved_elem, merkle_proof) = mt.lookup(&uid).expect_ok().unwrap();
    assert_eq!(retrieved_elem, elem);
    assert!(mt.verify(&uid, merkle_proof.clone()).expect("succeed"));
    let merkle_root = mt.commitment().digest().into_bigint().0;

    let public_input = WithdrawPublicInput {
        fee: 1,
        recipient: [7; 32],
        token_id,
        spend_nullifier,
        token_amount_out: 3,
        merkle_root,
        deposit_note,
    };

    let private_input = WithdrawPrivateInput {
        spend_trapdoor,
        deposit_trapdoor,
        deposit_nullifier,
        merkle_proof,
        leaf_index,
        spend_note,
        whole_token_amount,
        deposit_token_amount,
    };

    WithdrawRelation::new(public_input, private_input)
}
