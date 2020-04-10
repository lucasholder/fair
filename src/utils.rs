use sha2::{Digest, Sha256};

use hex;

pub fn hash_server_seed(server_seed: &str) -> String {
    hex::encode(Sha256::digest(server_seed.as_bytes()))
}

