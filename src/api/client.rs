use bitcoin::Network;
use reqwest::Client;

pub struct MempoolClient {
    pub client: Client,
    pub base_url: String,
}

impl MempoolClient {
    pub fn new(network: Network) -> Self {
        let base_url = match network {
            Network::Bitcoin => "https://mempool.space/api",
            Network::Testnet => "https://mempool.space/testnet/api",
            Network::Signet => "https://mempool.space/signet/api",
            Network::Regtest => "https://mempool.space/regtest/api",
            Network::Testnet4 => "https://mempool.space/testnet4/api",
            _ => panic!("Unsupported network"),
        }
        .to_string();

        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub fn new_with_endpoint(endpoint: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: endpoint.to_string(),
        }
    }
}
