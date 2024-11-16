#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args = client::args::get_args();

    client::trace::start_tracing(args.level)?;

    client::action::vote::approve_vote(args).await?;

    Ok(())
}
