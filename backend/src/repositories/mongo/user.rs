use crate::{
    entities::{User, UserID},
    repositories::{Mongo, MongoError, UserRepository, MONGO_COLLECTION_USERS},
};
use async_trait::async_trait;
use mongodb::{
    bson::{doc, Bson},
    options::IndexOptions,
    Collection, IndexModel,
};

impl From<UserID> for Bson {
    fn from(value: UserID) -> Self {
        Bson::String(value.0.to_string())
    }
}

impl Mongo {
    pub async fn create_user_index(&self) -> Result<(), MongoError> {
        {
            let model = IndexModel::builder()
                .keys(doc! {"id": 1})
                .options(IndexOptions::builder().unique(true).build())
                .build();

            self.database
                .collection::<User>(MONGO_COLLECTION_USERS)
                .create_index(model, None)
                .await?;

            Ok(())
        }
    }
}

#[async_trait]
impl UserRepository for Mongo {
    async fn create_user(
        &self,
        user: User,
    ) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        let users: Collection<User> = self.database.collection(MONGO_COLLECTION_USERS);
        let _ = users.insert_one(&user, None).await?;
        Ok(user)
    }

    async fn delete_user(
        &self,
        id: &UserID,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let users: Collection<User> = self.database.collection(MONGO_COLLECTION_USERS);

        let filter = doc! { "id": id };
        let result = users.delete_one(filter, None).await?;

        assert!(result.deleted_count == 1);
        Ok(())
    }

    async fn get_user(
        &self,
        id: &UserID,
    ) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
        let users: Collection<User> = self.database.collection(MONGO_COLLECTION_USERS);

        let filter = doc! { "id": id };
        let result = users.find_one(filter, None).await?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::MongoConfig;
    use fake::{Fake, Faker};

    #[tokio::test]
    async fn create_user() {
        let mongo = Mongo::new(MongoConfig {
            uri: "mongodb://localhost:27017",
            database: "warikan",
        })
        .await
        .unwrap();

        let user: User = Faker.fake();

        let create = mongo.create_user(user).await.unwrap();
        let get = mongo.get_user(&create.id).await.unwrap();

        assert_eq!(Some(create), get);
    }

    #[tokio::test]
    async fn delete_user() {
        let mongo = Mongo::new(MongoConfig {
            uri: "mongodb://localhost:27017",
            database: "warikan",
        })
        .await
        .unwrap();

        let user: User = Faker.fake();

        let create = mongo.create_user(user).await.unwrap();
        mongo.delete_user(&create.id).await.unwrap();
        let delete = mongo.get_user(&create.id).await.unwrap();

        assert_eq!(delete, None);
    }
}
