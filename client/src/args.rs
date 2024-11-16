use clap::Parser;
use dotenv::dotenv;

use tracing::Level;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short, long, env, default_value = "info")]
    pub level: Level,

    #[arg(short, long, env("PRIVATE_KEY"))]
    pub key: String,

    #[arg(short, long, env("RPC_URL"))]
    pub rpc: String,

    #[arg(short, long, env("CONTRACT_ADDRESS"))]
    pub contract: String,

    #[arg(short, long, env)]
    pub api_key: String,
}

pub fn get_args() -> Args {
    dotenv().ok();
    Args::parse()
}
