use blockchaindemolib::*;

fn main() {
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

    // First attempt to create a ledger
    let mut balance = std::collections::BTreeMap::new();
    for t in v.iter().flat_map(|b| b.payload.iter()) {
        println!("{}->{} : {}",t.sender, t.receiver, t.amount);

        let sender = t.sender.clone();
        let receiver = t.receiver.clone();
        *balance.entry(receiver).or_insert(0) += t.amount as i128; // Transaction of unsigned 64 bit fits in signed 128
        *balance.entry(sender).or_insert(0) -= t.amount as i128;
    };

    println!("Dump balance:");
    for (user,cash) in balance {
        println!("{}  {}",user, cash);
    }

    // Verify the chain again just to make sure we have not lost ownership 
    println!("Final verify: {}",v.verify());


}

#[cfg(test)]
mod test {
    #[test]
    fn test_hello() {
        assert_eq!(super::main(), {})
    }
}
