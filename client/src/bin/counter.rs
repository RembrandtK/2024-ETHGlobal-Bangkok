//! Example on how to interact with a deployed `stylus-hello-world` contract using defaults.
//! This example uses ethers-rs to instantiate the contract using a Solidity ABI.
//! Then, it attempts to check the current counter value, increment it via a tx,
//! and check the value again. The deployed contract is fully written in Rust and compiled to WASM
//! but with Stylus, it is accessible just as a normal Solidity smart contract is via an ABI.

use client::trace::start_tracing;
use dotenv::dotenv;
use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::Address,
};
use std::str::FromStr;
use std::sync::Arc;
use tracing::Level;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long, env, default_value = "info")]
    level: Level,

    #[arg(short, long, env("PRIVATE_KEY"))]
    key: String,

    #[arg(short, long, env("RPC_URL"))]
    rpc: String,

    #[arg(short, long, env("CONTRACT_ADDRESS"))]
    contract: String,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();
    let args = Args::parse();

    start_tracing(args.level)?;

    abigen!(
        Contract,
        r#"[
            function number() external view returns (uint256)
            function setNumber(uint256 number) external
            function increment() external
        ]"#
    );

    let provider = Provider::<Http>::try_from(args.rpc)?;
    let address: Address = args.contract.parse()?;

    let wallet = LocalWallet::from_str(&args.key)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.clone().with_chain_id(chain_id),
    ));

    let counter = Contract::new(address, client);
    let num = counter.number().call().await;
    println!("Counter number value = {:?}", num);

    let pending = counter.increment();
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:#?}", receipt);
    }
    println!("Successfully incremented counter via a tx");

    let num = counter.number().call().await;
    println!("New counter number value = {:?}", num);
    Ok(())
}
