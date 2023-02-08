use std::time::Instant;

mod relation;

use relation::Relation;

fn main() {
    let xor_pk = generate_keys("xor");
    generate_proof("xor", xor_pk);

    let withdraw_pk = generate_keys("withdraw");
    generate_proof("withdraw", withdraw_pk);
}

fn now() -> Instant {
    std::time::Instant::now()
}

fn generate_keys(relation_name: &str) -> Vec<u8> {
    let start = now();

    let relation = Relation::from(relation_name);
    let pk = relation.generate_keys();

    let elapsed = now().duration_since(start);

    println!(
        "Generating keys for {:?} took {:?}. Key has length: {:?}",
        relation_name,
        elapsed,
        pk.len()
    );

    pk
}

pub fn generate_proof(relation_name: &str, pk: Vec<u8>) {
    let start = now();

    let relation = Relation::from(relation_name);
    let proof = relation.generate_proof(pk);

    let elapsed = now().duration_since(start);

    println!(
        "Generating proof for `{:?}` took {:?}. Proof has length: {:?}",
        relation_name,
        elapsed,
        proof.len()
    );
}
