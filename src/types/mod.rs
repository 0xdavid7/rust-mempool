use serde::Deserialize;

/// Represents the status of a UTXO
#[derive(Debug, Deserialize)]
pub struct UtxoStatus {
    /// Whether the UTXO has been confirmed in a block
    pub confirmed: bool,
    /// The height of the block containing the UTXO, if confirmed
    pub block_height: Option<u32>,
    /// The hash of the block containing the UTXO, if confirmed
    pub block_hash: Option<String>,
    /// The timestamp of the block containing the UTXO, if confirmed
    pub block_time: Option<u64>,
}

/// Represents an unspent transaction output (UTXO)
#[derive(Debug, Deserialize)]
pub struct Utxo {
    /// The transaction ID
    pub txid: String,
    /// The output index
    pub vout: u32,
    /// The status of the UTXO
    pub status: UtxoStatus,
    /// The value of the UTXO in satoshis
    pub value: u64,
}
