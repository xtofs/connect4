```
cargo build --release --example auto && time ./target/release/examples/auto
```

```sh
cargo build --release --example stats && time ./target/release/examples/stats 1000000
```

```sh
CARGO_PROFILE_RELEASE_DEBUG=true sudo cargo flamegraph --example stats -- 100000 && open ./flamegraph.svg
```
