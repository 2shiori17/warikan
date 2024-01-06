use crate::{
    entities::{Group, GroupID, UserID},
    repositories::{GroupRepository, Mongo, MongoError, MONGO_COLLECTION_GROUPS},
};
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, Bson},
    options::IndexOptions,
    Collection, IndexModel,
};

impl From<GroupID> for Bson {
    fn from(value: GroupID) -> Self {
        Bson::String(value.0.to_string())
    }
}

impl Mongo {
    pub async fn create_group_index(&self) -> Result<(), MongoError> {
        {
            let model = IndexModel::builder()
                .keys(doc! {"id": 1})
                .options(IndexOptions::builder().unique(true).build())
                .build();

            self.database
                .collection::<Group>(MONGO_COLLECTION_GROUPS)
                .create_index(model, None)
                .await?;

            Ok(())
        }
    }
}

#[async_trait]
impl GroupRepository for Mongo {
    async fn create_group(
        &self,
        group: Group,
    ) -> Result<Group, Box<dyn std::error::Error + Send + Sync>> {
        let groups: Collection<Group> = self.database.collection(MONGO_COLLECTION_GROUPS);
        let _ = groups.insert_one(&group, None).await?;
        Ok(group)
    }

    async fn delete_group(
        &self,
        id: &GroupID,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let groups: Collection<Group> = self.database.collection(MONGO_COLLECTION_GROUPS);

        let filter = doc! { "id": id };
        let result = groups.delete_one(filter, None).await?;

        assert!(result.deleted_count == 1);
        Ok(())
    }

    async fn get_group(
        &self,
        id: &GroupID,
    ) -> Result<Option<Group>, Box<dyn std::error::Error + Send + Sync>> {
        let groups: Collection<Group> = self.database.collection(MONGO_COLLECTION_GROUPS);

        let filter = doc! { "id": id };
        let result = groups.find_one(filter, None).await?;

        Ok(result)
    }

    async fn get_groups_by_user(
        &self,
        id: &UserID,
    ) -> Result<Vec<Group>, Box<dyn std::error::Error + Send + Sync>> {
        let groups: Collection<Group> = self.database.collection(MONGO_COLLECTION_GROUPS);

        let filter = doc! { "participants": id };
        let result = groups.find(filter, None).await?.try_collect().await?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::MongoConfig;
    use fake::{Fake, Faker};

    #[tokio::test]
    async fn create_group() {
        let mongo = Mongo::new(MongoConfig {
            uri: "mongodb://localhost:27017",
            database: "warikan",
        })
        .await
        .unwrap();

        let group: Group = Faker.fake();

        let create = mongo.create_group(group).await.unwrap();
        let get = mongo.get_group(&create.id).await.unwrap();

        assert_eq!(Some(create), get);
    }

    #[tokio::test]
    async fn delete_group() {
        let mongo = Mongo::new(MongoConfig {
            uri: "mongodb://localhost:27017",
            database: "warikan",
        })
        .await
        .unwrap();

        let group: Group = Faker.fake();

        let create = mongo.create_group(group).await.unwrap();
        mongo.delete_group(&create.id).await.unwrap();
        let delete = mongo.get_group(&create.id).await.unwrap();

        assert_eq!(delete, None);
    }

    #[tokio::test]
    async fn get_groups_by_user() {
        let mongo = Mongo::new(MongoConfig {
            uri: "mongodb://localhost:27017",
            database: "warikan",
        })
        .await
        .unwrap();

        let user: UserID = Faker.fake();

        let mut group1: Group = Faker.fake();
        let mut group2: Group = Faker.fake();
        let group3: Group = Faker.fake();

        group1.participants.push(user.clone());
        group2.participants.push(user.clone());

        let _ = mongo.create_group(group1.clone()).await.unwrap();
        let _ = mongo.create_group(group2.clone()).await.unwrap();
        let _ = mongo.create_group(group3).await.unwrap();

        let groups = mongo.get_groups_by_user(&user).await.unwrap();

        assert_eq!(groups, vec![group1, group2]);
    }
}
