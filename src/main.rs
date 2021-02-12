use blockchaindemolib::*;
use std::fs;

fn read_chain() -> Result<BlockChain, Box<dyn std::error::Error>> {
    let ser = fs::read("blockchain.blkchain")?;
    let clonechain: BlockChain = rmp_serde::from_read_ref(&ser)?;

    Ok(clonechain)
}

fn write_chain(chain:BlockChain)-> Result<(),Box<dyn std::error::Error>>{
    let data:Vec<u8> = chain.export()?;
    Ok(fs::write("blockchain.blkchain", &data)?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // use crate::blockchain::BlockChainTrait;

    let mut chain: BlockChain = read_chain()
        .or_else(|_| -> Result<BlockChain, Box<dyn std::error::Error>> {
            Ok(vec![
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
            ])
        })
        ?;
        chain.push(Block::new(vec![Transaction::new("Bob", "Eve", 99)], 0, chain.len() as u64, [0; 32]));

    let mut lasthash = [0; 32];
    for mut block in chain.iter_mut() {
        block.prev_sha = lasthash;
        block.mine(8);
        lasthash = block.sha;
    }

    println!("Hello {}",chain.len());
    println!("Verify {}",chain.verify());
    if chain.verify() {
        println!("Save file");
        write_chain(chain)?
    }
    Ok({})
}

#[cfg(test)]
mod test {
    // Todo make proper test of this now it is only test if it compiles
    #[test]
    fn test_ledger() {
        use super::*;
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
        // verify the block chain
        let mut lasthash = [0; 32];
        for block in &v {
            let calculated = block.hash();
            println!(
                "Checking {} N={} {}",
                block.seq,
                block.nonce,
                calculated == block.sha && block.prev_sha == lasthash
            );
            lasthash = calculated
        }

        let balance = v.get_balance();

        println!("Dump balance:");
        for (user, cash) in balance {
            println!("{}  {}", user, cash);
        }
        // Verify the chain again just to make sure we have not lost ownership
        println!("Final verify: {}", v.verify());
    }
}
