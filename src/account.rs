use crate::{ClientId, Transaction, TransactionId};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Account {
    pub(crate) _client: ClientId,
    pub(crate) transactions: HashMap<TransactionId, Transaction>,
    pub(crate) disputes: HashMap<TransactionId, Transaction>,
    pub(crate) balance: Balance,
}

impl Account {
    pub fn new(id: ClientId) -> Self {
        Account {
            _client: id,
            transactions: Default::default(),
            disputes: Default::default(),
            balance: Balance::new(id),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Balance {
    pub(crate) client: ClientId,
    pub(crate) available: Decimal,
    pub(crate) held: Decimal,
    pub(crate) total: Decimal,
    pub(crate) locked: bool,
}

impl Balance {
    fn new(cid: ClientId) -> Self {
        Balance {
            client: cid,
            ..Balance::default()
        }
    }
}
