use crate::hashable::Hashable;
use crate::Block;

pub type BlockChain = Vec<Block>;

trait BlockChainTrait {
    fn verify(&self) -> bool;
}
impl BlockChainTrait for BlockChain {
    fn verify(&self) -> bool {
        // verify the block chain
        let mut lasthash = [0; 32];
        for block in self {
            let calculated = block.hash();
            println!(
                "Checking {} N={} {}",
                block.seq,
                block.nonce,
                calculated == block.sha && block.prev_sha == lasthash
            );
            lasthash = calculated
        }
        true
    }
}

#[cfg(test)]
mod test {
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
}
