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
                )
                .arg(
                    clap::Arg::with_name("doubledifficultly")
                        .required(false)
                        .short("d")
                        .long("doubledifficultly")
                        .takes_value(false),
                ),
        )
        .get_matches();

    // load or create am empty if it fails.
    let mut chain: BlockChain = BlockChain::read_from_file().unwrap_or(vec![]);

    // If chain len is 0 either from loading an empty chain or from vec![] above - add and mine a genesis block
    if chain.len() == 0 {
        chain.push(Block::new(
            vec![
                Transaction::new("Alice", "Bob", 128),
                Transaction::new("Satoshi", "Eve", 28),
            ],
            0,
            0,
            [0; 32],
        ));
        chain[0].prev_sha = [0; 32];
        chain[0].mine(8);
    }

    if let Some(transaction) = matches.subcommand_matches("transaction") {
        // Internal Error should never happen, as clap will catch usage errors
        let sender = transaction
            .value_of("sender")
            .ok_or(BlockChainDemoError::InternalError)?;
        let receiver = transaction
            .value_of("receiver")
            .ok_or(BlockChainDemoError::InternalError)?;
        let amount = transaction
            .value_of("amount")
            .ok_or(BlockChainDemoError::InternalError)?
            .parse::<u64>()?;

        let t = Transaction::new(sender, receiver, amount);

        // At this point there should always be a genesis block
        let lastsha = chain.last().ok_or(BlockChainDemoError::GenesisError)?.sha;
        let mut block = Block::new(vec![t], 0, chain.len() as u64, lastsha);
        let difficulty = chain
            .last()
            .ok_or(BlockChainDemoError::GenesisError)?
            .difficulty;
        let difficulty = if transaction.is_present("doubledifficultly") {
            difficulty + 1
        } else {
            difficulty
        };

        println!(
            "Mining block {} with difficulty {} bits",
            block.seq, difficulty
        );
        block.mine(difficulty);
        chain.push(block);
    }

    println!(
        "Verify chain of {} blocks. Verified: {}",
        chain.len(),
        chain.verify()
    );

    let balance = chain.get_balance();
    println!();
    println!("Dump balance:");
    for (user, cash) in balance {
        println!("{}  {}", user, cash);
    }
    println!();

    if chain.verify() {
        println!("Save file");
        chain.write_to_file()
    } else {
        Err(BlockChainDemoError::VerifyError.into())
    }
}
