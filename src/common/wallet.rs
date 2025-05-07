use bip39::Mnemonic;
use bitcoin::{
    bip32::{ChildNumber, DerivationPath, Xpriv},
    key::Secp256k1,
    secp256k1::All,
    Address, CompressedPublicKey, Network, PrivateKey,
};

use super::AddressType;

#[derive(Debug, Clone)]
pub struct BitcoinWallet {
    pub network: Network,
    pub derivation_path: DerivationPath,
    pub address_type: AddressType,
    xpriv: Xpriv,
}

impl BitcoinWallet {
    pub fn new(
        mnemonic: &str,
        network: Network,
        address_type: AddressType,
    ) -> anyhow::Result<Self> {
        let mnemonic = Mnemonic::parse_normalized(mnemonic)
            .map_err(|e| anyhow::anyhow!("Invalid mnemonic: {}", e))?;

        let seed = mnemonic.to_seed("");
        let xpriv = Xpriv::new_master(network, &seed)
            .map_err(|e| anyhow::anyhow!("Failed to create master key: {}", e))?;

        let base_path = address_type.get_derivation_path();

        Ok(Self {
            network,
            derivation_path: base_path,
            address_type,
            xpriv,
        })
    }

    pub fn get_account(
        &self,
        secp: &Secp256k1<All>,
        index: u32,
    ) -> anyhow::Result<(PrivateKey, Address)> {
        let private_key = self.derive_private_key(secp, index)?;
        let public_key = CompressedPublicKey::from_private_key(secp, &private_key)
            .map_err(|e| anyhow::anyhow!("Failed to derive public key: {}", e))?;
        let address = self
            .address_type
            .generate_address(&public_key, self.network)?;
        Ok((private_key, address))
    }

    fn derive_private_key(&self, secp: &Secp256k1<All>, index: u32) -> anyhow::Result<PrivateKey> {
        let child_path = self
            .derivation_path
            .child(ChildNumber::from_normal_idx(index)?);
        let child_xprv = self
            .xpriv
            .derive_priv(secp, &child_path)
            .map_err(|e| anyhow::anyhow!("Failed to derive child key: {}", e))?;

        Ok(PrivateKey::new(child_xprv.private_key, self.network))
    }
}

#[cfg(test)]
mod tests {
    use crate::common::wallet::BitcoinWallet;

    use super::*;
    use bitcoin::hex::DisplayHex;
    use bitcoin::Network;

    #[test]
    fn test_wallet_setup() {
        let mnemonic =  "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

        let wallet = BitcoinWallet::new(&mnemonic, Network::Testnet4, AddressType::P2WPKH).unwrap();
        let secp = bitcoin::secp256k1::Secp256k1::new();

        let (privkey, address) = wallet.get_account(&secp, 0).unwrap();

        println!("Private key: {}", privkey.to_bytes().to_lower_hex_string());
        println!("Address: {}", address.to_string());
    }
}
