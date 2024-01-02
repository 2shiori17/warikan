use crate::entities::UserID;
use async_graphql::{types::ID, NewType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(test)]
use fake::{Dummy, Faker};
#[cfg(test)]
use rand::Rng;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NewType)]
pub struct GroupID(pub ID);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Dummy))]
pub struct Group {
    pub id: GroupID,
    pub created_at: DateTime<Utc>,
    #[cfg_attr(test, dummy(faker = "(Faker, 1..10)"))]
    pub participants: Vec<UserID>,
}

impl GroupID {
    pub fn new<T: ToString>(id: T) -> Self {
        GroupID(ID(id.to_string()))
    }
}

impl ToString for GroupID {
    fn to_string(&self) -> String {
        self.0 .0.to_string()
    }
}

#[cfg(test)]
impl Dummy<Faker> for GroupID {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let s = String::dummy_with_rng(config, rng);
        GroupID(ID(s))
    }
}