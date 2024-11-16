//! WIP client for vote contract.

use client::{args::get_args, invoke::invoke_counter, trace::start_tracing};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args = get_args();

    start_tracing(args.level)?;

    invoke_counter(args).await?;

    Ok(())
}
