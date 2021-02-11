// use blockchaindemolib::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Block {
    pub payload: Vec<Transaction>,
    pub nonce: u64,
    pub seq: u64,
    pub prev_sha: [u8; 32],
    pub sha: [u8; 32],
}

use crate::Hashable;

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(std::mem::size_of::<Block>());

        bytes.extend(self.nonce.to_be_bytes().iter());
        bytes.extend(
            self.payload
                .iter()
                .flat_map(|transaction| transaction.bytes())
                .collect::<Vec<u8>>(),
        );
        bytes.extend(self.prev_sha.iter());
        bytes.extend(self.seq.to_be_bytes().iter());
        // bytes.extend(self.nonce.to_be_bytes());
        // bytes.extend(self.nonce.to_be_bytes());

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
        }
    }
    pub fn mine(&mut self, difficulty: u32) {
        use std::convert::TryInto;
        let difficulty = u128::MAX >> difficulty;
        while u128::from_be_bytes((self.sha[0..16]).try_into().unwrap()) > difficulty {
            self.nonce += 7;
            let hash = self.hash();
            self.sha = hash;
        }
    }
}

#[cfg(test)]
mod test {

    // use super::blockchaindemolib::*;
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
