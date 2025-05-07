use anyhow::Result;

impl crate::MempoolClient {
    pub async fn broadcast_transaction(&self, tx: &str) -> Result<String> {
        let url = format!("{}/tx", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "text/plain")
            .body(tx.to_string())
            .send()
            .await?;

        if response.status().is_success() {
            let tx_id = response.text().await?;
            Ok(tx_id)
        } else {
            let error_message = response.text().await?;
            Err(anyhow::anyhow!(
                "Failed to broadcast transaction: {}",
                error_message
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    // use bitcoin::Network;

    // use crate::MempoolClient;

    // #[tokio::test]
    // async fn test_send_transaction() {
    //     let client = MempoolClient::new(Network::Testnet4);

    //     let tx_id = client
    //         .broadcast_transaction("02000000000101ff36f2beed51000d45aad607184cf381aa08e38df383006411b9e43aa0cc8ea10100000000fdffffff020000000000000000386a365343344c3452000141005045504500000000000000000000000000000000000000000000000000000000000000000000000000000000d70400000000000016001450dceca158a9c872eb405d52293d351110572c9e074032f724a7aed7ad34b435f4903f8718a64553452cd141768e4c80cc8573749801959e6a03cd2e7bcd637a3ab80e65eda83d58cd216784002d8a33bf67f26ce03a0040a770a33698fe97849d1665ab78bbcd71d708cde24dcf790bdb5fb9a1d2d450f0e847b61fc06e34a05f2faac43b3c9583fa06163766006b1ed80a354256b678f04092441a373d52678d4ef15045b7256cfc4ccd7282e17cc58cfb359b3d7859feb692d56011c61a6e3e9c071e421fa3b776a37b1d274f6cd1ddf17579cab870423a00ac2015da913b3e87b4932b1e1b87d9667c28e7250aa0ed60b3a31095f541e1641488ac20594e78c0a2968210d9c1550d4ad31b03d5e4b9659cf2f67842483bb3c2bb7811ba20b59e575cef873ea95273afd55956c84590507200d410e693e4b079a426cc6102ba20e2d226cfdaec93903c3f3b81a01a81b19137627cb26e621a0afb7bcd6efbcfffba20f0f3d9beaf7a3945bcaa147e041ae1d5ca029bde7e40d8251f0783d6ecbe8fb5ba53a221c050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac000000000")
    //         .await
    //         .unwrap();
    //     println!("{:?}", tx_id);
    //     // 370d4452fb63424c45ca75c4092f0e33bc0396a5bba7231ace2c9ac8522b75ca
    // }
}
