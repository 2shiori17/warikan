use async_graphql::{types::ID, NewType};
use serde::{Deserialize, Serialize};

#[cfg(test)]
use fake::{Dummy, Faker};
#[cfg(test)]
use rand::Rng;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NewType)]
pub struct UserID(pub ID);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Dummy))]
pub struct User {
    pub id: UserID,
}

impl UserID {
    pub fn new<T: ToString>(id: T) -> Self {
        UserID(ID(id.to_string()))
    }
}

impl ToString for UserID {
    fn to_string(&self) -> String {
        self.0 .0.to_string()
    }
}

#[cfg(test)]
impl Dummy<Faker> for UserID {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let s = String::dummy_with_rng(config, rng);
        UserID(ID(s))
    }
}
