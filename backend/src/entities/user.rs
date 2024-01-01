use async_graphql::{types::ID, NewType};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, NewType)]
pub struct UserID(pub ID);

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: UserID,
}

impl UserID {
    pub fn new<T: ToString>(id: T) -> Self {
        UserID(ID(id.to_string()))
    }
}
