use csv::ReaderBuilder;
use std::{collections::HashMap, fs::File, path::Path};

use crate::{
    dto::{
        client_account::ClientAccount,
        transaction::{Transaction, TransactionType},
    },
    errors::Error,
};

type Result<T> = std::result::Result<T, Error>;

pub fn process_csv<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut file = ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .delimiter(b',')
        .from_reader(File::open(path)?);

    let mut accounts: HashMap<u16, ClientAccount> = HashMap::new();
    let mut transactions: HashMap<u32, Transaction> = HashMap::new();

    for entry in file.deserialize() {
        let record: Transaction = entry?;
        let account = accounts.entry(record.client).or_default();

        match record.transaction_type {
            TransactionType::Deposit => {
                if let Some(amount) = record.amount {
                    account.deposit(amount);
                    transactions.insert(record.transaction_id, record);
                }
            }
            TransactionType::Withdrawal => {
                if let Some(amount) = record.amount {
                    if account.withdrawal(amount) {
                        transactions.insert(record.transaction_id, record);
                    }
                }
            }
            TransactionType::Dispute => {
                if let Some(tx) = transactions.get(&record.transaction_id) {
                    if let Some(amount) = tx.amount {
                        account.dispute(amount);
                    }
                }
            }
            TransactionType::Resolve => {
                if let Some(tx) = transactions.get(&record.transaction_id) {
                    if let Some(amount) = tx.amount {
                        account.resolve(amount);
                    }
                }
            }
            TransactionType::Chargeback => {
                if let Some(tx) = transactions.get(&record.transaction_id) {
                    if let Some(amount) = tx.amount {
                        account.chargeback(amount);
                    }
                }
            }
        }
    }
    print_report(accounts);

    Ok(())
}

fn print_report(accounts: HashMap<u16, ClientAccount>) {
    println!("client,available,held,total,locked");
    for (client, account) in &accounts {
        println!(
            "{},{:.4},{:.4},{:.4},{}",
            client, account.available, account.held, account.total, account.locked
        );
    }
}
