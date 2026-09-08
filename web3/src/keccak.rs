use patterns_macros::because;
use sha3::{Digest, Keccak256};

pub fn hashed(bytes: &[u8]) -> Vec<u8> {
    Keccak256::digest(bytes).to_vec()
}
because!(hashed, "the digest the chain uses for topics and checksums, kept behind one call so no binding names the algorithm and the library that computes it can change in one place");
