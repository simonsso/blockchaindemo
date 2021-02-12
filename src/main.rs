use blockchaindemolib::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // use crate::blockchain::BlockChainTrait;

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
    Ok({})
}

#[cfg(test)]
mod test {
    #[test]
    fn test_hello() {
        assert_eq!(super::main().is_ok(), true);
    }
}
