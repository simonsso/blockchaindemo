# blockchaindemo ![Rust](https://github.com/simonsso/blockchaindemo/workflows/Rust/badge.svg)
A simple blockchain demo in rust.
## Command line interface
Run from command line without argument to verify integrity of blockchain and with transaction to create and mine a transaction on the blockchain.
### Verify chain integrity
Run command without arguments to print sum of active ledger
<pre>
USAGE:
blockchaindemo
</pre>
### Add a transaction to the chain
<pre>
USAGE:
    blockchaindemo transaction [FLAGS] --amount <amount> --receiver <receiver> --sender <sender>

FLAGS:
    -d, --doubledifficultly    
    -h, --help                 Prints help information
    -V, --version              Prints version information

OPTIONS:
    -a, --amount <amount>        
    -r, --receiver <receiver>    Receiver of tokens
    -s, --sender <sender>        Sender of tokens
</pre>
    
## Documentation
https://simonsso.github.io/blockchaindemo/
