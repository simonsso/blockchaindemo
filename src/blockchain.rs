use crate::hashable::Hashable;
use crate::Block;

pub type BlockChain = Vec<Block>;
use std::fs;

pub trait BlockChainTrait {
    fn verify(&self) -> bool;
    fn get_balance(&self) -> std::collections::BTreeMap<String, i128>;
    fn export(&self) -> Result<Vec<u8>, rmp_serde::encode::Error>;
    fn read_from_file() -> Result<BlockChain, Box<dyn std::error::Error>>;
    fn write_to_file(&self) -> Result<(), Box<dyn std::error::Error>>;
}

impl BlockChainTrait for BlockChain {
    fn read_from_file() -> Result<BlockChain, Box<dyn std::error::Error>> {
        let ser = fs::read("blockchain.blkchain")?;
        let clonechain: BlockChain = rmp_serde::from_read_ref(&ser)?;

        Ok(clonechain)
    }

    fn write_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let data: Vec<u8> = self.export()?;
        Ok(fs::write("blockchain.blkchain", &data)?)
    }

    fn export(&self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        rmp_serde::to_vec(&self)
    }

    fn get_balance(&self) -> std::collections::BTreeMap<String, i128> {
        let mut balance = std::collections::BTreeMap::new();
        for t in self.iter().flat_map(|b| b.payload.iter()) {
            // println!("{}->{} : {}",t.sender, t.receiver, t.amount);

            let sender = t.sender.clone();
            let receiver = t.receiver.clone();
            *balance.entry(receiver).or_insert(0) += t.amount as i128; // Transaction of unsigned 64 bit fits in signed 128
            *balance.entry(sender).or_insert(0) -= t.amount as i128;
        }
        balance
    }

    fn verify(&self) -> bool {
        // verify the blocks chain
        let mut lasthash = [0; 32];
        let mut lastdifficulty = 0;
        let mut expectedseq = 0;
        for block in self {
            // later blocks should be mined with a higher or equal difficulty
            if block.difficulty < lastdifficulty {
                return false;
            } else {
                lastdifficulty = block.difficulty;
            }
            if !block.verify_difficulty() {
                return false;
            }
            if block.seq != expectedseq {
                return false;
            }
            expectedseq += 1;
            let calculated = block.hash();

            // verify the hashes hash the block.
            if calculated != block.sha || block.prev_sha != lasthash {
                return false;
            }
            lasthash = calculated
        }
        true
    }
}

#[cfg(test)]
mod test {
    use crate::blockchain::BlockChainTrait;
    use crate::Block;
    use crate::BlockChain;
    use crate::Transaction;
    #[test]
    fn test_create_a_chain() {
        let mut v: BlockChain = vec![
            Block::new(
                vec![
                    Transaction::new("Alice", "Bob", 128),
                    Transaction::new("Alice", "Eve", 28),
                ],
                0,
                0,
                [0; 32],
            ),
            Block::new(vec![Transaction::new("Bob", "Eve", 108)], 0, 1, [0; 32]),
        ];

        let mut lasthash = [0; 32];
        for mut block in v.iter_mut() {
            block.prev_sha = lasthash;
            block.mine(8);
            lasthash = block.sha;
        }
    }
    #[test]
    fn test_serialize_a_raw_chain() {
        let mut chain: BlockChain = vec![
            Block::new(vec![Transaction::new("Alice", "Bob", 999)], 0, 0, [0; 32]),
            Block::new(vec![Transaction::new("Bob", "Eve", 20)], 0, 1, [0; 32]),
        ];
        assert!(!chain.verify());
        chain[0].mine(9);
        chain[1].prev_sha = chain[0].sha;
        chain[1].mine(10);
        assert!(chain.verify());

        let ser = rmp_serde::to_vec(&chain);

        match ser {
            Err(e) => {
                eprintln!("Serializing failed: {}", e);
                assert!(false);
            }
            Ok(ser) => {
                let clonechain: Option<BlockChain> = rmp_serde::from_read_ref(&ser).ok();
                if let Some(clonechain) = clonechain {
                    assert!(clonechain.verify());
                } else {
                    assert!(false); // clonechain was not read correctly
                }
            }
        }
    }
}
