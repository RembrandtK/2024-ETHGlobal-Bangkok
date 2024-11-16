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
use eyre::eyre;
use std::str::FromStr;
use std::sync::Arc;

/// Your private key file path.
const ENV_PRIVATE_KEY: &str = "PRIVATE_KEY";

/// Stylus RPC endpoint url.
const ENV_RPC_URL: &str = "RPC_URL";

/// Deployed pragram address.
const ENV_CONTRACT_ADDRESS: &str = "CONTRACT_ADDRESS";


use clap::Parser;


#[derive(Parser)]
#[command(name = "Protocol View Export")]
#[command(about = "Protocol views for GIPs", long_about = None)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,

    #[arg(short, long)]
    view: Vec<PView>,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    start_tracing(filter_level)?;

    dotenv().ok();
    let private_key =
        std::env::var(ENV_PRIVATE_KEY).map_err(|_| eyre!("No {} env var set", ENV_PRIVATE_KEY))?;
    let rpc_url =
        std::env::var(ENV_RPC_URL).map_err(|_| eyre!("No {} env var set", ENV_RPC_URL))?;
    let contract_address = std::env::var(ENV_CONTRACT_ADDRESS)
        .map_err(|_| eyre!("No {} env var set", ENV_CONTRACT_ADDRESS))?;
    abigen!(
        Contract,
        r#"[
            function number() external view returns (uint256)
            function setNumber(uint256 number) external
            function increment() external
        ]"#
    );

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let address: Address = contract_address.parse()?;

    let wallet = LocalWallet::from_str(&private_key)?;
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
