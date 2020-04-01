/*

From: https://bitcointalk.org/index.php?topic=5162888.0

Welcome to our seeding event for Crash launching in Beta this week. We are seeding it similarly to
RHavar and others so it should be fairly straight forward.

To prove our fairness we have generated a chain of 10,000,000 SHA256 hashes where each hash is the
hash of the hexadecimal representation of the previous hash. The last hash in the chain is:
78a9757d3be42b74a3f70239078ad9317125fe9ee630d5bdada46de963e56752

The formula for generating the game result: Code: const gameHash = hashChain.pop()

const hmac = createHmac('sha256', gameHash);

// blockHash is the hash of bitcoin block 584,500

hmac.update(blockHash);

const hex = hmac.digest('hex').substr(0, 8); const int = parseInt(hex, 16);

// 0.01 will result in 1% house edge with a lowest crashpoint of 1

const crashpoint = Math.max(1, (2 ** 32 / (int + 1)) * (1 - 0.01))

blockHash used is Bitcoin block 584,500 which has not been mined at time of posting. Basically we
are using the hash of a future bitcoin block as a client seed so players can be certain that we did
not pick one in the house's favor. I'd appreciate it if someone could quote this post so this is
all set in stone.

Excited to show you all Crash very soon!

*/
use hex;
use hmac::{Hmac, Mac};
use sha2::digest::generic_array::typenum::*;
use sha2::digest::generic_array::GenericArray;
use sha2::{Digest, Sha256};
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Hash {
    value: GenericArray<u8, U32>,
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = &self.value[..];
        write!(f, "{}", hex::encode(bytes))
    }
}

impl Hash {
    pub fn new(value: GenericArray<u8, U32>) -> Hash {
        Hash { value }
    }
    fn digest(s: &str) -> Hash {
        Hash::new(Sha256::digest(s.as_bytes()))
    }
    pub fn from_hex(s: &str) -> Hash {
        let v = hex::decode(s).unwrap();
        Hash::new(GenericArray::clone_from_slice(&v[..]))
    }
    fn to_hex(&self) -> String {
        self.to_string()
    }
}

#[derive(Copy, Clone)]
pub struct Config {
    hash_chain_tip: Hash,
    block_hash: Hash,
    max_chain_length: usize,
}

impl Config {
    pub fn new(hash_chain_tip: Hash, block_hash: Hash, max_chain_length: usize) -> Config {
        Config {
            hash_chain_tip,
            block_hash,
            max_chain_length,
        }
    }
    pub fn for_stake() -> Config {
        // https://bitcointalk.org/index.php?topic=5162888.msg54134231#msg54134231
        let hash_chain_tip =
            Hash::from_hex("78a9757d3be42b74a3f70239078ad9317125fe9ee630d5bdada46de963e56752");
        let block_hash =
            Hash::from_hex("0000000000000000001b34dc6a1e86083f95500b096231436e9b25cbdd0075c4");
        let max_chain_length = 10_000_000;

        Config {
            hash_chain_tip,
            block_hash,
            max_chain_length,
        }
    }
}

#[derive(Copy, Clone)]
struct HashChain {
    hash: Hash,
}

impl HashChain {
    pub fn new(initial_hash: Hash) -> HashChain {
        HashChain { hash: initial_hash }
    }

    fn compute_next_hash(&self) -> Hash {
        Hash::new(Sha256::digest(&self.hash.value))
    }
}

impl std::iter::Iterator for HashChain {
    type Item = Hash;
    fn next(&mut self) -> Option<Self::Item> {
        // Some(self.next_float())
        let res = self.hash;
        self.hash = self.compute_next_hash();
        Some(res)
    }
}

type HmacSha256 = Hmac<Sha256>;

use std::convert::TryInto;

pub struct Outcome {
    crash_point: f64,
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Crash point: {}", self.crash_point)
    }
}

pub fn simulate(config: Config, game_hash: Hash) -> Outcome {
    // TODO: verify game hash
    let key = game_hash.to_hex();
    let input = config.block_hash.to_hex();
    let mut mac = HmacSha256::new_varkey(key.as_bytes())
        .expect("HMAC can take key of any size, never errors here");
    mac.input(input.as_bytes());
    let result = Hash::new(mac.result().code());

    // convert 4 first bytes of hash to u32
    let n = u32::from_be_bytes((&result.value[0..4]).try_into().unwrap());

    let n = n as f64;

    let crash_point = 1_f64.max((2_f64.powf(32.) / (n + 1.)) * (1. - 0.01));

    Outcome { crash_point }
}

// verify that the hash is really part of the hash chain
pub fn verify_hash(config: Config, game_hash: Hash) -> bool {
    let mut hash_chain = HashChain::new(game_hash);
    for _ in 0..config.max_chain_length {
        let h = hash_chain.next().unwrap();
        if h == config.hash_chain_tip {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_crash_hash_chain() {
        let hash_chain: Vec<_> = HashChain::new(Hash::digest("testing")).take(3).collect();
        assert_eq!(
            Hash::digest("testing").to_string(),
            "cf80cd8aed482d5d1527d7dc72fceff84e6326592848447d2dc0b0e87dfc9a90"
        );
        assert_eq!(
            hash_chain[0].to_string(),
            Hash::digest("testing").to_string()
        );
        assert_eq!(
            hash_chain[1].to_string(),
            Hash::new(Sha256::digest(&hash_chain[0].value[..])).to_string()
        );
        assert_eq!(
            hash_chain[2].to_string(),
            Hash::new(Sha256::digest(&hash_chain[1].value[..])).to_string()
        );
    }

    #[test]
    fn test_crash_simulate() {
        let hash_chain: Vec<_> = HashChain::new(Hash::digest("testing")).take(10).collect();
        let hash_chain_tip = *hash_chain.last().unwrap();
        let block_hash =
            Hash::from_hex("0000000000000000001b34dc6a1e86083f95500b096231436e9b25cbdd0075c4");
        let game_hash = hash_chain[2];
        let config = Config::new(hash_chain_tip, block_hash, hash_chain.len());
        let outcome = simulate(config, game_hash);
        assert_eq!(outcome.crash_point, 1.440106367685025);
        assert!(verify_hash(config, game_hash));
        let bad_game_hash =
            Hash::from_hex("deadbeefe7c270724bd4851c020d489257fa79a70e694a9b5099375464348698");
        assert!(!verify_hash(config, bad_game_hash), "bas_game_hash");
        let last_game_hash = hash_chain_tip;
        assert!(verify_hash(config, last_game_hash), "last_game_hash")
    }

    #[test]
    fn test_crash_simulate_2() {
        let hash_chain_tip =
            Hash::from_hex("0000000000000000001b34dc6a1e86083f95500b096231436e9b25cbdd0075c4");
        let block_hash =
            Hash::from_hex("0000000000000000001b34dc6a1e86083f95500b096231436e9b25cbdd0075c4");
        let config = Config::new(hash_chain_tip, block_hash, 0);
        let game_hash =
            Hash::from_hex("deadbeefe7c270724bd4851c020d489257fa79a70e694a9b5099375464348698");
        assert_eq!(simulate(config, game_hash).crash_point, 1.2897005203687084);
    }
}
