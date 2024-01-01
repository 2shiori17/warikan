use crate::entities::{GroupID, UserID};
use async_graphql::{types::ID, NewType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(test)]
use fake::{Dummy, Faker};
#[cfg(test)]
use rand::Rng;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NewType)]
pub struct PaymentID(pub ID);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Dummy))]
pub struct Payment {
    pub id: PaymentID,
    pub created_at: DateTime<Utc>,
    pub group: GroupID,
    pub creditor: UserID,
    #[cfg_attr(test, dummy(faker = "(Faker, 1..10)"))]
    pub debtors: Vec<UserID>,
}

impl PaymentID {
    pub fn new<T: ToString>(id: T) -> Self {
        PaymentID(ID(id.to_string()))
    }
}

impl ToString for PaymentID {
    fn to_string(&self) -> String {
        self.0 .0.to_string()
    }
}

#[cfg(test)]
impl Dummy<Faker> for PaymentID {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let s = String::dummy_with_rng(config, rng);
        PaymentID(ID(s))
    }
}
