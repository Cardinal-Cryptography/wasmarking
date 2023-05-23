use ark_bls12_381::Bls12_381;
use ark_groth16::{Groth16, Proof, ProvingKey, VerifyingKey};
use ark_snark::SNARK;
use ark_std::test_rng;
use ark_relations::{
    shielder::{
        WithdrawRelationWithFullInput, WithdrawRelationWithPublicInput,
        WithdrawRelationWithoutInput,
    },
    xor::{XorRelationWithFullInput, XorRelationWithoutInput},
};

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
    pub fn generate_keys(&self) -> (ProvingKey<Bls12_381>, VerifyingKey<Bls12_381>) {
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

    pub fn generate_proof(&self, pk: ProvingKey<Bls12_381>) -> Proof<Bls12_381> {
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

    pub fn verify_proof(&self, proof: &Proof<Bls12_381>, vk: &VerifyingKey<Bls12_381>) {
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
