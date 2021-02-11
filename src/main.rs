// extern crate serde;
extern crate hmac_sha256;


use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: u64, // Unsigned reverse sender and receiver to reverse transactions
}

impl Transaction {

    fn new<S:Into<String>,R:Into<String>>(s:S,r:R,amount:u64)->Self
    {
        Transaction{sender:s.into(),receiver:r.into(),amount}
    }
}

#[derive(Serialize, Deserialize)]
struct Block {
    payload: Vec<Transaction>,
    nonce: u64,
    seq: u64,
    prev_sha: [u8; 32],
    sha: [u8; 32],
}

trait Hashable {
    fn bytes(&self) -> Vec<u8>;
    fn hash(&self) -> [u8; 32] {
        hmac_sha256::Hash::hash(&self.bytes())
    }
}

impl Hashable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut v = vec![];
        v.extend(self.sender.as_bytes());
        v.extend(self.receiver.as_bytes());
        v.extend(self.amount.to_be_bytes().iter());
        v
    }
}

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

impl Block {
    fn new(payload: Vec<Transaction>, nonce: u64, seq: u64, prev_sha: [u8; 32]) -> Self {
        Block {
            payload,
            nonce,
            seq,
            prev_sha,
            sha: [0xff; 32],
        }
    }
    fn mine(&mut self, difficulty: u64) {
        use std::convert::TryInto;
        let mut d = u64::from_be_bytes((self.sha[0..8]).try_into().unwrap());
        while d > difficulty {
            self.nonce += 7;
            let hash = self.hash();
            self.sha = hash;
            d = u64::from_be_bytes((self.sha[0..8]).try_into().unwrap());
        }
    }
}
fn main() {
    let mut v: Vec<Block> = vec![
        Block::new(vec![Transaction::new("Alice", "Bob", 128),Transaction::new("Alice", "Eve", 28)], 0, 0, [0; 32]),
        Block::new(vec![Transaction::new("Bob", "Eve", 108)], 0, 1, [0; 32]),
    ];

    let mut lasthash = [0; 32];
    for mut block in v.iter_mut() {
        block.prev_sha = lasthash;
        block.mine(0x00FFFFFFFFFFFFFF);
        lasthash = block.sha;
    }
    // verify the block chain
    let mut lasthash = [0; 32];
    for block in v {
        let calculated = block.hash();
        println!(
            "Checking {} N={} {}",
            block.seq,
            block.nonce,
            calculated == block.sha &&
            block.prev_sha == lasthash
        );
        lasthash = calculated
    }
    println!("Done");
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
        let mut block = super::Block::new(vec![], 0, 0, [0; 32]);
        block.mine(0x00FFFFFFFFFFFFFF);

        // This is slower but a better proof of work
        // assert_eq!(block.sha, block.hash());
        // block.mine(0x000FFFFFFFFFFFFFFFFFFFFFFFFFFFFF);

        assert_eq!(block.sha, block.hash());
    }
    #[test]
    fn test_serialize(){
        use super::Hashable;
        let mut block = super::Block::new(vec![super::Transaction::new("Alice", "Bob", 999)], 0, 0, [0; 32]);
        block.mine(0x00FFFFFFFFFFFFFF);
        let ser = rmp_serde::to_vec(&block);
        if let Ok(ser) = ser {
            print!("");
        }else{
            assert!(false);
        }
        // if let Some(desr) = rmp_serde::decode::read_slice(ser,ser.len()) {

        // }else{
        //     assert!(false);
        // }

    }
}
