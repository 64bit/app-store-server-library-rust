use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct TransactionInfoResponse {
    /// A customer’s in-app purchase transaction, signed by Apple, in JSON Web Signature (JWS) format.
    #[serde(rename = "signedTransactionInfo")]
    pub signed_transaction_info: Option<String>,
}
