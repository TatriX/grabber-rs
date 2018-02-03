# grabber-rs
Pluggable rust grabber example. It can be used to easily make http grabbers.
# Usage
See example:
```sh
cargo run --example amazon
```

# Tests
Lib tests should be run with `--test-threads=1` (for mockito). You may use
```sh
./tests.sh`
```
script to do it easily.

Also you can run `amazon` example test:
```sh
cargo test --example amazon
```
