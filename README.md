# rust-mempool

⚠️: The library is under development, it is still missing a lot of features that will come soon. ⚠️

[![crates.io](https://img.shields.io/crates/v/rust-mempool.svg)](https://crates.io/crates/rust-mempool)
[![Documentation](https://docs.rs/rust-mempool/badge.svg)](https://docs.rs/rust-mempool)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/rust-mempool.svg)](./LICENSE.txt)
[![CI](https://github.com/0xdavid7/rust-mempool/actions/workflows/ci.yml/badge.svg)](https://github.com/0xdavid7/rust-mempool/actions/workflows/ci.yml)
[![Issues](https://img.shields.io/github/issues/0xdavid7/rust-mempool)](https://img.shields.io/github/issues/0xdavid7/rust-mempool)

An ergonomic, [Mempool](https://mempool.space/) API Client for Rust.

- [Changelog](CHANGELOG.md)

## Example

```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
rust-mempool = "0.1"
```

And then the code:

```rust,norun
use rust_mempool::MempoolClient;

#[tokio::main]
async fn main() {
    let client = MempoolClient::new(Network::Bitcoin);
    let utxos = client
      .get_address_utxo("1KFHE7w8BhaENAswwryaoccDb6qcT6DbYY")
      .await
      .unwrap();
      println!("{:?}", utxos);
}

```

## Key features

- General
  - [ ] GET difficulty adjustment
- Address
  - [ ] GET address
  - [ ] GET address transactions
  - [ ] GET address transactions chain
  - [ ] GET address transactions mempool
  - [x] GET address UTXO
- Blocks
  - [ ] GET block
  - [ ] GET block header
  - [ ] GET block height
  - [ ] GET block raw
  - [ ] GET block status
  - [ ] GET block tip height
  - [ ] GET block tip hash
  - [ ] GET block transaction ID
  - [ ] GET block transaction IDs
  - [ ] GET block transactions
  - [ ] GET blocks
- Mining
  - [ ] GET mining pools
  - [ ] GET mining pool
  - [ ] GET mining pool hashrates
  - [ ] GET mining pool hashrate
  - [ ] GET mining pool blocks
  - [ ] GET hashrate
  - [ ] GET reward stats
  - [ ] GET block fees
  - [ ] GET block rewards
  - [ ] GET block feerates
  - [ ] GET block sizes and weights
- Fees
  - [ ] GET mempool blocks fees
  - [ ] GET recommended fees
- Mempool
  - [ ] GET mempool
  - [ ] GET mempool transaction IDs
  - [ ] GET mempool recent
- Transactions
  - [ ] GET children pay for parent
  - [ ] GET transaction
  - [ ] GET transaction hex
  - [ ] GET transaction merkleblock proof
  - [ ] GET transaction merkle proof
  - [ ] GET transaction outspend
  - [ ] GET transaction outspends
  - [ ] GET transaction raw
  - [ ] GET transaction status
  - [x] POST transaction

## Additional features:
- Address
  - [ ] Get batch of addresses UTXO
- Wallet
  - [ ] Derive addresses, private keys from mnemonic

## License

Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
