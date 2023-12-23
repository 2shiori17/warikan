use async_graphql::{Object, SimpleObject};
use chrono::{DateTime, Local};

#[derive(Debug, SimpleObject)]
pub struct User {
    pub id: String,
    pub created_at: DateTime<Local>,
    pub name: String,
    pub photo: String,
}

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn get_user(&self) -> User {
        todo!()
    }
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn create_user(&self) -> User {
        todo!()
    }

    async fn delete_user(&self) -> User {
        todo!()
    }
}
