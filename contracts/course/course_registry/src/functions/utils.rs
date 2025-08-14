use soroban_sdk::{Env, String};

pub fn generate_unique_id(env: &Env) -> String {
    let ts = env.ledger().timestamp();
    let rand1: u64 = env.prng().gen();
    let rand2: u64 = env.prng().gen();

    let rust_str = format!(
        "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
        ts,
        (rand1 & 0xFFFF) as u16,
        ((rand1 >> 16) & 0xFFFF) as u16,
        (rand2 & 0xFFFF) as u16,
        (rand2 >> 16) as u64
    );

    String::from_str(env, &rust_str)
}
