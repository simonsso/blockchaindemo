struct Block {
    payload: [u8; 4096],
    nonce: u64,
    seq: u64,
    prev_sha: [u8; 32],
    sha: [u8; 32],
}

extern crate hmac_sha256;
trait Hashable {
    fn bytes(&self) -> Vec<u8>;
    fn hash(&self) -> [u8; 32] {
        hmac_sha256::Hash::hash(&self.bytes())
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(std::mem::size_of::<Block>());

        bytes.extend(self.nonce.to_be_bytes().iter());
        bytes.extend(self.payload.iter());
        bytes.extend(self.prev_sha.iter());
        bytes.extend(self.seq.to_be_bytes().iter());
        // bytes.extend(self.nonce.to_be_bytes());
        // bytes.extend(self.nonce.to_be_bytes());

        bytes
    }
}

impl Block {
    fn new(payload: [u8; 4096], nonce: u64, seq: u64, prev_sha: [u8; 32]) -> Self {
        Block {
            payload,
            nonce,
            seq,
            prev_sha,
            sha: [0xff; 32],
        }
    }
    fn mine(&mut self, difficulty: u128) {
        use std::convert::TryInto;
        let mut d = u128::from_be_bytes((self.sha[0..16]).try_into().unwrap());
        // while u128::from_be_bytes((self.sha[0..16]).try_into().unwrap()) < difficulty
        while d > difficulty {
            self.nonce += 7;
            let hash = self.hash();
            self.sha = hash;
            d = u128::from_be_bytes((self.sha[0..16]).try_into().unwrap());
        }
    }
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[test]
    fn test_hello() {
        assert_eq!(super::main(), {})
    }
    #[test]
    fn test_calculate_hash() {
        use super::Hashable;
        let mut block = super::Block::new([0; 4096], 0, 0, [0; 32]);
        block.mine(0x00FFFFFFFFFFFFFFFFFFFFFFFFFFFFFF);

        // This is slower
        // assert_eq!(block.sha, block.hash());
        // block.mine(0x000FFFFFFFFFFFFFFFFFFFFFFFFFFFFF);

        assert_eq!(block.sha, block.hash());
    }
}
