use crate::types::Wallet;
use once_cell::sync::Lazy;
use parking_lot::Mutex;

static WALLET: Lazy<Mutex<Wallet>> = Lazy::new(|| Mutex::new(Wallet { balance: 0 }));

pub fn initialize_wallet() {
    let mut wallet = WALLET.lock();
    wallet.balance = 0;
}

pub fn send_tokens(recipient: String, amount: u64) -> String {
    let mut wallet = WALLET.lock();
    if wallet.balance >= amount {
        wallet.balance -= amount;
        // Log the transaction or handle sending the tokens to the recipient
        format!("Sent {} tokens to {}", amount, recipient)
    } else {
        "Insufficient balance.".to_string()
    }
}

pub fn receive_tokens(amount: u64) {
    let mut wallet = WALLET.lock();
    wallet.balance += amount;
}

pub fn get_balance() -> u64 {
    let wallet = WALLET.lock();
    wallet.balance
}