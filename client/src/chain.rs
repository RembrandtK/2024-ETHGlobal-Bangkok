use ethers::types::H256;

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum, Default, strum_macros::Display)]
pub enum Chain {
    /// Arbitrum mainnet chain.
    Arbitrum,

    /// Arbitrum testnet chain.
    ArbitrumSepolia,

    /// Local testnet.
    #[default]
    Local,
}

impl Chain {
    pub fn chain_id(&self) -> u64 {
        match self {
            Self::Arbitrum => 42161,
            Self::ArbitrumSepolia => 421614,
            Self::Local => 412346,
        }
    }

    pub fn rpc(&self) -> &str {
        match self {
            Self::Arbitrum => "https://arb1.arbitrum.io/rpc",
            Self::ArbitrumSepolia => "https://sepolia-rpc.arbitrum.io",
            Self::Local => "http://localhost:8545",
        }
    }

    /// Chain explorer base URL.
    pub fn explorer(&self) -> &str {
        match self {
            Self::Arbitrum => "https://arbitrum.blockscout.com",
            Self::ArbitrumSepolia => "https://sepolia-explorer.arbitrum.io",
            Self::Local => "https://sepolia-explorer.arbitrum.io",
        }
    }

    /// Explorer transaction URL.
    pub fn tx_url(&self, hash: &H256) -> String {
        format!("{explorer}/tx/{hash:?}", explorer = self.explorer())
    }
}