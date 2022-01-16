use crate::Transaction;
use csv::{Reader, Trim};
use std::fs::File;

pub struct Parser {}

impl Parser {
    pub fn from_path(input: Option<String>) -> Result<Reader<File>, csv::Error> {
        csv::ReaderBuilder::new()
            .trim(Trim::All)
            .flexible(true)
            .from_path(input.expect("Expected argument but found none"))
    }

    pub fn extract_transactions(
        reader: &mut Reader<File>,
    ) -> impl Iterator<Item = Transaction> + '_ {
        reader
            .deserialize()
            .filter_map(|tx: Result<Transaction, csv::Error>| match tx.ok() {
                None => {
                    eprintln!("Encountered faulty rune while deserializing, skipping instruction");
                    None
                }
                Some(t) => Some(t),
            })
    }
}
