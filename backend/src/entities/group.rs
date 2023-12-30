use crate::entities::UserID;
use async_graphql::{types::ID, NewType};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, NewType)]
pub struct GroupID(pub ID);

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub id: GroupID,
    pub created_at: DateTime<Local>,
    pub participants: Vec<UserID>,
}
