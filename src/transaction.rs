use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

pub type TransactionId = u32;
pub type ClientId = u16;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Transaction {
    #[serde(rename = "tx")]
    pub(crate) id: TransactionId,
    #[serde(rename = "type")]
    pub(crate) ttype: TransactionType,
    pub(crate) client: ClientId,
    pub(crate) amount: Option<Decimal>,
}
