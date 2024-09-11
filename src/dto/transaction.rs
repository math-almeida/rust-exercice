use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
    pub client: u16,
    #[serde(rename = "tx")]
    pub transaction_id: u32,
    pub amount: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Withdrawal,
    Deposit,
    Dispute,
    Resolve,
    Chargeback,
}
