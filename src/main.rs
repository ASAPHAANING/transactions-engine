use std::env;
use std::error::Error;

use transactions_engine::{Parser, TransactionEngine};

fn main() -> Result<(), Box<dyn Error>> {
    let input = env::args().nth(1);
    let mut csv_parser = Parser::from_path(input)?;
    let txs = Parser::extract_transactions(&mut csv_parser);

    let mut txe = TransactionEngine::default();

    txe.process_transactions(&txs);
    txe.print_balances();

    Ok(())
}
