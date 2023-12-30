use crate::{
    entities::{Group, GroupID},
    repositories::{GroupRepository, Mongo, MongoError, MONGO_COLLECTION_GROUPS},
};
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

impl GroupRepository for Mongo {
    type Error = MongoError;

    async fn get_group(&self, id: &GroupID) -> Result<Option<Group>, Self::Error> {
        let groups: Collection<Group> = self.database.collection(MONGO_COLLECTION_GROUPS);

        let filter = doc! { "id": id };
        let result = groups.find_one(filter, None).await?;

        Ok(result)
    }
}
