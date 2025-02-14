use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UtxoStatus {
    pub confirmed: bool,
    pub block_height: Option<u32>,
    pub block_hash: Option<String>,
    pub block_time: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct Utxo {
    pub txid: String,
    pub vout: u32,
    pub status: UtxoStatus,
    pub value: u64,
}

impl crate::MempoolClient {
    pub async fn get_address_utxo(&self, address: &str) -> Result<Vec<Utxo>> {
        let url = format!("{}/address/{}/utxo", self.base_url, address);
        let response = self.client.get(&url).send().await?.error_for_status()?;

        Ok(response.json().await?)
    }
}

#[cfg(test)]
mod tests {
    use bitcoin::Network;

    use crate::MempoolClient;

    #[tokio::test]
    async fn test_get_address_utxo() {
        let client = MempoolClient::new(Network::Bitcoin);

        let utxos = client
            .get_address_utxo("1KFHE7w8BhaENAswwryaoccDb6qcT6DbYY")
            .await
            .unwrap();
        println!("{:?}", utxos);
    }
}
