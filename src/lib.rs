use ark_bls12_381::Bls12_381;
use ark_ff::PrimeField;
use ark_groth16::{
    Groth16, Proof as ArkProof, ProvingKey as ArkProvingKey, VerifyingKey as ArkVerifyingKey,
};
use ark_relations::{
    shielder::{
        WithdrawRelationWithFullInput, WithdrawRelationWithPublicInput,
        WithdrawRelationWithoutInput,
    },
    xor::{XorRelationWithFullInput, XorRelationWithoutInput},
};
use ark_snark::SNARK;
use ark_std::test_rng;
use jf_primitives::merkle_tree::{
    prelude::RescueSparseMerkleTree, MerkleTreeScheme, UniversalMerkleTreeScheme, MerkleCommitment,
};
use jf_relations::{
    generate_srs,
    shielder_types::{compute_note, convert_array},
    withdraw::{WithdrawPrivateInput,WithdrawRelation, WithdrawPublicInput},
    PlonkKzgSnark, Proof as JfProof, ProvingKey as JfProvingKey, StandardTranscript,
    VerifyingKey as JfVerifyingKey, Relation as _, PublicInput, Curve, UniversalSNARK,
};
use num_bigint::BigUint;

pub enum ArkRelation {
    Xor,
    Withdraw,
}

impl From<&str> for ArkRelation {
    fn from(value: &str) -> Self {
        match value {
            "xor" => Self::Xor,
            "withdraw" => Self::Withdraw,
            _ => unreachable!("Unknown relation"),
        }
    }
}

impl ArkRelation {
    pub fn generate_keys(&self) -> (ArkProvingKey<Bls12_381>, ArkVerifyingKey<Bls12_381>) {
        let mut rng = test_rng();

        let (pk, vk) = match self {
            ArkRelation::Xor => Groth16::<Bls12_381>::circuit_specific_setup(
                XorRelationWithoutInput::new(2),
                &mut rng,
            ),
            ArkRelation::Withdraw => Groth16::<Bls12_381>::circuit_specific_setup(
                WithdrawRelationWithoutInput::new(16),
                &mut rng,
            ),
        }
        .unwrap();

        (pk, vk)
    }

    pub fn generate_proof(&self, pk: ArkProvingKey<Bls12_381>) -> ArkProof<Bls12_381> {
        let mut rng = test_rng();
        match self {
            ArkRelation::Xor => {
                Groth16::<Bls12_381>::prove(&pk, XorRelationWithFullInput::new(2, 1, 3), &mut rng)
            }
            ArkRelation::Withdraw => {
                let circuit = WithdrawRelationWithFullInput::new(
                    16,
                    10,
                    [
                        212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214,
                        130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
                    ],
                    0,
                    [1919191919; 4],
                    [1, 2, 3, 4],
                    100,
                    [0, 0, 0, 0],
                    [17171717171717; 4],
                    [181818181818; 4],
                    [41414141414141; 4],
                    vec![
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                    ],
                    65537,
                    [0, 0, 0, 0],
                    200,
                    100,
                );
                Groth16::<Bls12_381>::prove(&pk, circuit, &mut rng)
            }
        }
        .unwrap()
    }

    pub fn verify_proof(&self, proof: &ArkProof<Bls12_381>, vk: &ArkVerifyingKey<Bls12_381>) {
        match self {
            ArkRelation::Xor => (),
            ArkRelation::Withdraw => {
                let public_input = WithdrawRelationWithPublicInput::new(
                    16,
                    10,
                    [
                        212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214,
                        130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
                    ],
                    0,
                    [1919191919; 4],
                    [1, 2, 3, 4],
                    100,
                    [0, 0, 0, 0],
                )
                .serialize_public_input();
                Groth16::<Bls12_381>::verify(vk, &public_input, proof).unwrap();
            }
        };
    }
}

pub enum JfRelation {
    Withdraw,
}

impl JfRelation {
    pub fn generate_keys(&self) -> (JfProvingKey<Curve>, JfVerifyingKey<Curve>) {
        let rng = &mut jf_utils::test_rng();
        let srs = generate_srs(17_000, rng).unwrap();

        WithdrawRelation::generate_keys(&srs).unwrap()
    }

    pub fn generate_proof(&self, pk: JfProvingKey<Curve>) -> JfProof<Curve> {
        let rng = &mut jf_utils::test_rng();
        let relation = relation();
        relation.generate_proof(&pk, rng).unwrap()
    }

    pub fn verify_proof(
        &self,
        proof: &JfProof<Curve>,
        vk: &JfVerifyingKey<Curve>,
    ) {
        let input = relation().public_input();
        assert!(
            PlonkKzgSnark::<Curve>::verify::<StandardTranscript>(&vk, &input, &proof, None,)
                .is_ok()
        )
    }
}

fn relation() -> WithdrawRelation {
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
