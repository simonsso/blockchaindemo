use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64, // Unsigned reverse sender and receiver to reverse transactions
}

impl Transaction {
    pub fn new<S: Into<String>, R: Into<String>>(s: S, r: R, amount: u64) -> Self {
        Transaction {
            sender: s.into(),
            receiver: r.into(),
            amount,
        }
    }
}

use crate::hashable::Hashable;
impl Hashable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut v = vec![];
        v.extend(self.sender.as_bytes());
        v.extend(self.receiver.as_bytes());
        v.extend(self.amount.to_be_bytes().iter());
        v
    }
}

