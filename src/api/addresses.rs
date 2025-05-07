use std::collections::HashMap;

use anyhow::Result;

use crate::Utxo;

impl crate::MempoolClient {
    pub async fn get_address_utxo(&self, address: &str) -> Result<Vec<Utxo>> {
        let url = format!("{}/address/{}/utxo", self.base_url, address);
        let response = self.client.get(&url).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }

    pub async fn get_batch_of_addresses_utxo(
        &self,
        addresses: &[&str],
    ) -> Result<HashMap<String, Vec<Utxo>>> {
        use futures::future::join_all;
        use tokio::time::{sleep, Duration};
        let mut futures = Vec::with_capacity(addresses.len());

        // avoid rate limit
        const DELAY_MS: u64 = 100;

        for (i, address) in addresses.iter().enumerate() {
            if i > 0 {
                sleep(Duration::from_millis(DELAY_MS)).await;
            }
            let future = async move { self.get_address_utxo(address).await };
            futures.push(future);
        }

        let responses = join_all(futures).await;

        let mut result: HashMap<String, Vec<Utxo>> = HashMap::new();

        for (index, response) in responses.into_iter().enumerate() {
            if let Ok(r) = response {
                let address = addresses[index].to_string();
                result.insert(address, r);
            }
        }

        Ok(result)
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

    #[tokio::test]
    async fn test_get_batch_of_addresses_utxo() {
        let client = MempoolClient::new(Network::Bitcoin);
        let addresses = vec!["1KFHE7w8BhaENAswwryaoccDb6qcT6DbYY", "123"];

        let result = client.get_batch_of_addresses_utxo(&addresses).await;
        assert!(
            result.is_ok(),
            "Batch UTXO fetch failed: {:?}",
            result.err()
        );

        let utxos = result.unwrap();
        assert_eq!(utxos.len(), 1, "Expected UTXOs for 1 address");

        for (i, address_utxos) in utxos.iter().enumerate() {
            println!(
                "Address {}: {} UTXOs found",
                addresses[i],
                address_utxos.1.len()
            );
        }
    }
}
