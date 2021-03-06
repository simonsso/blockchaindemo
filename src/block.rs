use serde::{Deserialize, Serialize};

/// Struct containing a single block
#[derive(Serialize, Deserialize)]
pub struct Block {
    pub payload: Vec<Transaction>,
    /// Modified by miner to get a sha to satisfy difficulty
    pub nonce: u64,
    pub seq: u64,
    /// Cryptosum for last block
    pub prev_sha: [u8; 32],
    /// Cryptosum for this block
    pub sha: [u8; 32],
    /// Number of predefined bits in a successfully mined block
    pub difficulty: u32,
}

use crate::Hashable;

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend(self.nonce.to_be_bytes().iter());
        bytes.extend(
            self.payload
                .iter()
                .flat_map(|transaction| transaction.bytes())
                .collect::<Vec<u8>>(),
        );
        bytes.extend(self.prev_sha.iter());
        bytes.extend(self.seq.to_be_bytes().iter());
        bytes.extend(self.difficulty.to_be_bytes().iter());

        bytes
    }
}
use crate::Transaction;
impl Block {
    pub fn new(payload: Vec<Transaction>, nonce: u64, seq: u64, prev_sha: [u8; 32]) -> Self {
        Block {
            payload,
            nonce,
            seq,
            prev_sha,
            sha: [0xff; 32],
            difficulty: 0,
        }
    }
    /// Repeatedly calculating sha of the block until it fullfil requirements in difficulty
    pub fn mine(&mut self, difficulty: u32) {
        use std::convert::TryInto;
        self.difficulty = difficulty;
        self.sha = [0xff; 32]; // Force recalculate
        let difficulty = u128::MAX >> difficulty;
        while u128::from_be_bytes((self.sha[0..16]).try_into().unwrap()) > difficulty {
            self.nonce += 1;
            let hash = self.hash();
            self.sha = hash;
        }
    }
    /// Verify the block with claimed difficulty
    pub fn verify_difficulty(&self) -> bool {
        use std::convert::TryInto;
        u128::from_be_bytes((self.sha[0..16]).try_into().unwrap()) <= (u128::MAX >> self.difficulty)
    }
}

#[cfg(test)]
mod test {
    use crate::hashable::Hashable;
    #[test]
    fn test_calculate_hash() {
        let mut block = super::Block::new(vec![], 0, 0, [0; 32]);
        block.mine(8);

        // This is slower but a better proof of work
        // assert_eq!(block.sha, block.hash());
        // block.mine(24);

        assert_eq!(block.sha, block.hash());
    }
    #[test]
    fn test_serialize() {
        let mut block = super::Block::new(
            vec![super::Transaction::new("Alice", "Bob", 999)],
            0,
            0,
            [0; 32],
        );
        block.mine(10);
        let ser = rmp_serde::to_vec(&block);
        if let Err(e) = ser {
            eprintln!("Serializing failed: {}", e);
            assert!(false);
        }
    }
}
