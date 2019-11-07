#!/usr/bin/env zsh
python3 -m pip install -U matplotlib  --user
python3 -m pip install -U seaborn  --user
python3 -m pip install -U pandas  --user
cargo build --release
cp target/release/DSLR .