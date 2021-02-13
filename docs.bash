#!/bin/bash

cargo fmt
cargo test || exit -1
cargo doc || exit -1
(cd target/doc; tar c *) | (cd docs ; tar x)
cd docs
git add `find blockchaindemo*`
git commit -m "Auto update documentation"
git push

