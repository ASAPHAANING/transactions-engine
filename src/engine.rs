use crate::{Account, Action, ClientId, Transaction};
use std::collections::HashMap;
use std::io;

#[derive(Default)]
pub struct TransactionEngine {
    accounts: HashMap<ClientId, Account>,
}

impl TransactionEngine {
    pub fn print_balances(&self) {
        let mut wtr = csv::Writer::from_writer(io::stdout());
        self.accounts.iter().for_each(|(_cid, acc)| {
            wtr.serialize(&acc.balance).unwrap();
        });

        wtr.flush().unwrap();
    }

    pub fn debug(&self) {
        self.accounts
            .iter()
            .for_each(|(_cid, acc)| println!("{:?}", acc));
    }

    pub fn process_transactions(&mut self, txs: &[Transaction]) {
        txs.iter().for_each(|tx| {
            let acc = self
                .accounts
                .entry(tx.client)
                .or_insert_with(|| Account::new(tx.client));

            match tx.perform(acc) {
                Ok(_) => {}
                Err(err) => eprintln!("{:?}", err),
            }
        })
    }
}
