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

    #[arg(long, env("CONTRACT_ADDRESS"))]
    pub contract: String,

    #[arg(long, env)]
    pub app_id: String,

    #[arg(long, env)]
    pub api_key: String,

    #[arg(long, env)]
    pub candidate: String,
}

pub fn get_args() -> Args {
    dotenv().ok();
    Args::parse()
}
