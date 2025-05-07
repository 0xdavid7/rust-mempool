use anyhow::Result;

use crate::Utxo;

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

    use crate::client::MempoolClient;

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
