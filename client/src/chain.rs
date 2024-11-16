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