use async_graphql::{types::ID, NewType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, NewType)]
pub struct UserID(pub ID);

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: UserID,
}
