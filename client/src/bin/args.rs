use client::{args::get_args, trace::start_tracing};
use tracing::info;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args = get_args();

    start_tracing(args.level)?;

    info!("{args:#?}");

    Ok(())
}
