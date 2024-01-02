use crate::{
    entities::{Group, GroupID},
    repositories::{GroupRepository, Mongo, MongoError, MONGO_COLLECTION_GROUPS},
};
use async_trait::async_trait;
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

        let group1 = mongo.create_group(group).await.unwrap();
        let group2 = mongo.get_group(&group1.id).await.unwrap();

        assert_eq!(Some(group1), group2);
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

        let group1 = mongo.create_group(group).await.unwrap();
        mongo.delete_group(&group1.id).await.unwrap();
        let group2 = mongo.get_group(&group1.id).await.unwrap();

        assert_eq!(group2, None);
    }
}
