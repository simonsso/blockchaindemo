// extern crate serde;
extern crate hmac_sha256;
use blockchaindemolib::*;

fn main() {
    let mut v: Vec<Block> = vec![
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
            calculated == block.sha && block.prev_sha == lasthash
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
}
