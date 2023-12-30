use crate::entities::{Group, GroupID, Payment, PaymentID, User, UserID};
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, Bson},
    options::IndexOptions,
    Client, Collection, Database, IndexModel,
};
use thiserror::Error;

const MONGO_COLLECTION_GROUPS: &str = "groups";
const MONGO_COLLECTION_PAYMENTS: &str = "payments";
const MONGO_COLLECTION_USERS: &str = "users";

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
        {
            // group

            let model = IndexModel::builder()
                .keys(doc! {"id": 1})
                .options(IndexOptions::builder().unique(true).build())
                .build();

            self.database
                .collection::<Group>(MONGO_COLLECTION_GROUPS)
                .create_index(model, None)
                .await?;
        }

        {
            // payment

            let model = IndexModel::builder()
                .keys(doc! {"id": 1})
                .options(IndexOptions::builder().unique(true).build())
                .build();

            self.database
                .collection::<Payment>(MONGO_COLLECTION_PAYMENTS)
                .create_index(model, None)
                .await?;
        }

        {
            // user

            let model = IndexModel::builder()
                .keys(doc! {"id": 1})
                .options(IndexOptions::builder().unique(true).build())
                .build();

            self.database
                .collection::<User>(MONGO_COLLECTION_USERS)
                .create_index(model, None)
                .await?;
        }

        Ok(())
    }
}

// =============================================================================
// group
// =============================================================================

impl From<GroupID> for Bson {
    fn from(value: GroupID) -> Self {
        Bson::String(value.0.to_string())
    }
}

impl Mongo {
    pub async fn get_group(&self, id: &GroupID) -> Result<Option<Group>, MongoError> {
        let groups: Collection<Group> = self.database.collection(MONGO_COLLECTION_GROUPS);

        let filter = doc! { "id": id };
        let result = groups.find_one(filter, None).await?;

        Ok(result)
    }
}

// =============================================================================
// payment
// =============================================================================

impl From<PaymentID> for Bson {
    fn from(value: PaymentID) -> Self {
        Bson::String(value.0.to_string())
    }
}

impl Mongo {
    pub async fn get_payment(&self, id: &PaymentID) -> Result<Option<Payment>, MongoError> {
        let payments: Collection<Payment> = self.database.collection(MONGO_COLLECTION_PAYMENTS);

        let filter = doc! { "id": id };
        let result = payments.find_one(filter, None).await?;

        Ok(result)
    }

    pub async fn get_payments_by_group(&self, group: &GroupID) -> Result<Vec<Payment>, MongoError> {
        let payments: Collection<Payment> = self.database.collection(MONGO_COLLECTION_PAYMENTS);

        let filter = doc! { "group": group };
        let result = payments.find(filter, None).await?.try_collect().await?;

        Ok(result)
    }
}

// =============================================================================
// user
// =============================================================================

impl From<UserID> for Bson {
    fn from(value: UserID) -> Self {
        Bson::String(value.0.to_string())
    }
}

impl Mongo {
    pub async fn get_user(&self, id: &UserID) -> Result<Option<User>, MongoError> {
        let users: Collection<User> = self.database.collection(MONGO_COLLECTION_USERS);

        let filter = doc! { "id": id };
        let result = users.find_one(filter, None).await?;

        Ok(result)
    }
}
