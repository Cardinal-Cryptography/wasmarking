use ark_bls12_381::Bls12_381;
use ark_groth16::Groth16;
use ark_serialize::CanonicalDeserialize;
use ark_snark::SNARK;
use ark_std::test_rng;
use relations::{serialize, WithdrawRelation, XorRelationWithFullInput, XorRelationWithoutInput};

pub enum Relation {
    Xor,
    Withdraw,
}

impl From<&str> for Relation {
    fn from(value: &str) -> Self {
        match value {
            "xor" => Self::Xor,
            "withdraw" => Self::Withdraw,
            _ => unreachable!("Unknown relation"),
        }
    }
}

impl Relation {
    pub fn generate_keys(&self) -> Vec<u8> {
        let mut rng = test_rng();

        let (pk, _vk) = match self {
            Relation::Xor => Groth16::<Bls12_381>::circuit_specific_setup(
                XorRelationWithoutInput::new(2),
                &mut rng,
            ),
            Relation::Withdraw => Groth16::<Bls12_381>::circuit_specific_setup(
                WithdrawRelation::without_input(16),
                &mut rng,
            ),
        }
        .unwrap();

        serialize(&pk)
    }

    pub fn generate_proof(&self, pk: Vec<u8>) -> Vec<u8> {
        let pk = CanonicalDeserialize::deserialize(&*pk).unwrap();

        let mut rng = test_rng();
        let proof = match self {
            Relation::Xor => {
                Groth16::<Bls12_381>::prove(&pk, XorRelationWithFullInput::new(2, 1, 3), &mut rng)
            }
            Relation::Withdraw => {
                let circuit = WithdrawRelation::with_full_input(
                    16,
                    10,
                    [
                        212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214,
                        130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
                    ],
                    0,
                    1919191919,
                    [1, 2, 3, 4],
                    100,
                    [0, 0, 0, 0],
                    17171717171717,
                    181818181818,
                    41414141414141,
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
        .unwrap();

        serialize(&proof)
    }
}
