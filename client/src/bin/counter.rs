//! WIP client for vote contract.

use client::{args::get_args, trace::start_tracing};
use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::Address,
};
use std::str::FromStr;
use std::sync::Arc;
use tracing::{debug, info, warn};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args = get_args();

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
    info!("Counter number value = {num:?}");

    let pending = counter.increment();
    if let Some(receipt) = pending.send().await?.await? {
        debug!("Receipt = {:#?}", receipt);
    } else {
        warn!("No receipt received");
    }
    info!("Successfully incremented counter via a tx");

    let num = counter.number().call().await;
    info!("New counter number value = {:?}", num);

    Ok(())
}
