# blockchaindemo ![Rust](https://github.com/simonsso/blockchaindemo/workflows/Rust/badge.svg)
A simple blockchain demo in rust.

## Documentation
* [Application](blockchaindemo/index.html)
* [Datastructures](blockchaindemolib/index.html)

## Compile
Project can be compiled with rust stable toolchain
<pre>
cargo build
</pre>
The executable will be stored in target/debug/blockchaindemo

## Running the executable
Run from command line without argument to verify integrity of blockchain and with transaction to create and mine a transaction on the blockchain.
### Verify chain integrity
Run command without arguments:
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
