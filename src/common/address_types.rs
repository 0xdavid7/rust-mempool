use std::str::FromStr;

use bitcoin::{bip32::DerivationPath, Address, CompressedPublicKey, Network};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AddressType {
    P2PKH,  // Legacy addresses (1...)
    P2WPKH, // Native SegWit addresses (bc1q...)
    P2TR,   // Taproot addresses (bc1p...)
}

impl AddressType {
    pub fn generate_address(
        &self,
        public_key: &CompressedPublicKey,
        network: Network,
    ) -> anyhow::Result<Address> {
        let address = match self {
            AddressType::P2PKH => Address::p2pkh(public_key, network),
            AddressType::P2WPKH => Address::p2wpkh(public_key, network),
            AddressType::P2TR => {
                let secp = bitcoin::secp256k1::Secp256k1::new();
                let xonly_pubkey = public_key.0.x_only_public_key();
                let xonly_pubkey = xonly_pubkey.0;
                Address::p2tr(&secp, xonly_pubkey, None, network)
            }
        };
        Ok(address)
    }

    pub fn get_derivation_path(&self) -> DerivationPath {
        match self {
            AddressType::P2PKH => DerivationPath::from_str("m/44'/0'/0'/0").unwrap(), // BIP44
            AddressType::P2WPKH => DerivationPath::from_str("m/84'/0'/0'/0").unwrap(), // BIP84
            AddressType::P2TR => DerivationPath::from_str("m/86'/0'/0'/0").unwrap(),  // BIP86
        }
    }
}
