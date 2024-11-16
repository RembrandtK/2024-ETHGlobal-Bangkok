use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::Address,
};
use std::str::FromStr;
use std::sync::Arc;
use tracing::*;

use crate::{abi::Contract, args::Args};

pub async fn invoke_counter(args: Args) -> eyre::Result<()> {
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
