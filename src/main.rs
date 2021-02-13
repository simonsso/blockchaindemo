use blockchaindemolib::*;

/// A simple demo
///
/// Run from command line without argument to verify integrity of blockchain and with transaction to create and mine a transaction on the blockchain.
/// ```text
/// USAGE:
/// blockchaindemo transaction --amount <amount> --receiver <receiver> --sender <sender>
///
/// FLAGS:
///     -h, --help       Prints help information
///     -V, --version    Prints version information
///
/// OPTIONS:
///     -a, --amount <amount>
///     -r, --receiver <receiver>    Receiver of tokens
///     -s, --sender <sender>        Sender of tokens
/// ```
///
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

    println!("Verify chain of {} blocks. Verifed: {}",chain.len() ,chain.verify());

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
