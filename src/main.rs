use blockchaindemolib::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap::App::new("Blockchain demo")
        .author("Fredrik SIMONSSON")
        .about("Simple ledger on ablockchain")
        .version("0.5")
        .subcommand(
            clap::SubCommand::with_name("transaction")
                .arg(
                    clap::Arg::with_name("sender")
                        .short("s")
                        .required(true)
                        .long("sender")
                        .takes_value(true)
                        .help("Sender of tokens"),
                )
                .arg(
                    clap::Arg::with_name("receiver")
                        .required(true)
                        .short("r")
                        .long("receiver")
                        .takes_value(true)
                        .help("Receiver of tokens"),
                )
                .arg(
                    clap::Arg::with_name("amount")
                        .required(true)
                        .short("a")
                        .long("amount")
                        .takes_value(true),
                ),
        )
        .get_matches();

    // load or genesis
    let mut chain: BlockChain = BlockChain::read_from_file().or_else(
        |_| -> Result<BlockChain, Box<dyn std::error::Error>> {
            Ok(vec![Block::new(
                vec![
                    Transaction::new("Alice", "Bob", 128),
                    Transaction::new("Satoshi", "Eve", 28),
                ],
                0,
                0,
                [0; 32],
            )])
        },
    )?;

    if let Some(transaction) = matches.subcommand_matches("transaction") {
        let sender = transaction.value_of("sender");
        let receiver = transaction.value_of("receiver");
        let amount = transaction.value_of("amount");

        let amount = amount.unwrap_or("0").parse::<u64>();
        if sender.is_none() || receiver.is_none() || amount.is_err() {
            return Ok({}); //TODO return an error here - should be unreachable
        };
        let t = Transaction::new(sender.unwrap(), receiver.unwrap(), amount.unwrap());

        let lastsha = if let Some(prev) = chain.last() {
            prev.sha
        } else {
            [0; 32]
        };
        let mut block = Block::new(vec![t], 0, chain.len() as u64, lastsha);
        block.mine(8);
        chain.push(block);
    }
    let mut lasthash = [0; 32];
    for mut block in chain.iter_mut() {
        block.prev_sha = lasthash;
        block.mine(8);
        lasthash = block.sha;
    }

    println!("Hello {}", chain.len());
    println!("Verify {}", chain.verify());

    let balance = chain.get_balance();

    println!("Dump balance:");
    for (user, cash) in balance {
        println!("{}  {}", user, cash);
    }

    if chain.verify() {
        println!("Save file");
        chain.write_to_file()?
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
