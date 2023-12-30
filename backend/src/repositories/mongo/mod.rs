mod group;
mod payment;
mod user;

use crate::repositories::Repository;
use mongodb::{Client, Database};
use thiserror::Error;

pub const MONGO_COLLECTION_GROUPS: &str = "groups";
pub const MONGO_COLLECTION_PAYMENTS: &str = "payments";
pub const MONGO_COLLECTION_USERS: &str = "users";

#[derive(Debug)]
pub struct Mongo {
    pub database: Database,
}

#[derive(Debug, Error)]
pub enum MongoError {
    #[error("mongodb error")]
    Mongo(#[from] mongodb::error::Error),

    #[error("not found")]
    NotFound,
}

#[derive(Debug)]
pub struct MongoConfig {
    pub uri: String,
    pub database: String,
}

impl Mongo {
    pub async fn new(config: MongoConfig) -> Result<Self, MongoError> {
        let client = Client::with_uri_str(&config.uri).await?;
        let database = client.database(&config.database);
        let mongo = Mongo { database };
        mongo.create_index().await?;
        Ok(mongo)
    }

    pub async fn create_index(&self) -> Result<(), MongoError> {
        self.create_group_index().await?;
        self.create_payment_index().await?;
        self.create_user_index().await?;

        Ok(())
    }
}

impl Repository for Mongo {}
