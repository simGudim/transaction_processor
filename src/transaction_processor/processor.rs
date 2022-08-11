use std::collections::HashMap;

use super::client_account::ClientAccount;
use super::transaction_state::{TransactionState};
use super::transaction::{Transaction, TransactionType};

pub struct Proccessor {
    pub transactions_map: HashMap<u32, TransactionState>,
    pub accounts_map: HashMap<u16, ClientAccount> 
}


impl Proccessor {
    pub fn new() -> Self {
        Self { 
            transactions_map: HashMap::new(),
            accounts_map: HashMap::new()
         }
    }

    pub fn insert_transaction_history(&mut self, transaction: &Transaction) {
        self.transactions_map
            .entry(transaction.tx)
            .or_insert_with(|| {
                TransactionState {
                    client: transaction.client,
                    transaction_type: transaction.transaction_type,
                    amount: transaction.amount
                }
            }
        );
    }

    fn check_valid_amount(&self, transaction: &Transaction) -> f64 {
        if let Some(amount) = transaction.amount {
           amount
        } else {
            0.0
        }
    }

    fn insert_deposit_transaction_into_account(&mut self, transaction: &Transaction) {
        let amount = self.check_valid_amount(&transaction);
        self.accounts_map
            .entry(transaction.client)
            .and_modify(|account| {
                account.deposit(amount)
            }).or_insert_with(|| {
                let mut account: ClientAccount = ClientAccount::new(transaction.client);
                account.deposit(amount);
                account
            });
    }

    fn insert_withdrawl_transaction_into_account(&mut self, transaction: &Transaction) {
        let amount = self.check_valid_amount(&transaction);
        self.accounts_map
            .entry(transaction.client)
            .and_modify(|account| {
                account.withdrawl(amount)
            }).or_insert_with(|| {
                let account: ClientAccount = ClientAccount::new(transaction.client);
                account
            });
    }

    fn handle_dipsute_transactions(&mut self, transaction: &Transaction) {
        let transaction_state = self.transactions_map.get(&transaction.tx);
        match transaction_state {
            Some(state) => {
                // let amount = self.check_valid_amount(&state);
                let amount = if let Some(amount) = state.amount { amount } else { 0.0 };
                self.accounts_map
                    .entry(transaction.client)
                    .and_modify(|client| {
                        match transaction.transaction_type {
                            TransactionType::Dispute => {
                                client.dispute(amount)
                            },
                            TransactionType::Resolve => client.resolve(amount),
                            TransactionType::Chargeback => client.chargeback(amount),
                            _ => eprintln!("something is terribly wrong with your code....")
                        }
                    }
                ).or_insert_with(|| ClientAccount::new(transaction.client));
            },
            None => eprintln!("transaction doesn't exist, maybe something went wrong....")
        }
    }

    pub fn process_transaction(&mut self, transaction: Transaction) {
        match transaction.transaction_type {
            TransactionType::Deposit => {
                self.insert_transaction_history(&transaction);
                self.insert_deposit_transaction_into_account(&transaction);
            },
            TransactionType::Withdrawal => {
                self.insert_transaction_history(&transaction);
                self.insert_withdrawl_transaction_into_account(&transaction);
            },
            _  => self.handle_dipsute_transactions(&transaction)
        }

    }
}