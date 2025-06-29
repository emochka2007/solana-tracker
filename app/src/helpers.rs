use solana_sdk::signature::Keypair;
use std::env;

/**
    Example usage -> "[150,12, 123...]"
*/
pub fn from_bytes_to_key_pair(env: String) -> Keypair {
    let bytes: Vec<u8> = env
        .trim_matches(&['[', ']'][..])
        .split(',')
        .map(|s| s.trim().parse::<u8>().expect("Error converting to bytes"))
        .collect();
    Keypair::from_bytes(&bytes).unwrap()
}
