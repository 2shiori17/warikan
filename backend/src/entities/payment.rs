use crate::entities::{GroupID, UserID};
use async_graphql::{types::ID, NewType};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, NewType)]
pub struct PaymentID(pub ID);

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    pub id: PaymentID,
    pub created_at: DateTime<Local>,
    pub group: GroupID,
    pub creditor: UserID,
    pub debtors: Vec<UserID>,
}

impl PaymentID {
    pub fn new<T: ToString>(id: T) -> Self {
        PaymentID(ID(id.to_string()))
    }
}
