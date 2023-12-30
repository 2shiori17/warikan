use crate::{
    entities::{GroupID, Payment, PaymentID},
    repositories::{Mongo, MongoError, PaymentRepository, MONGO_COLLECTION_PAYMENTS},
};
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, Bson},
    options::IndexOptions,
    Collection, IndexModel,
};

impl From<PaymentID> for Bson {
    fn from(value: PaymentID) -> Self {
        Bson::String(value.0.to_string())
    }
}

impl Mongo {
    pub async fn create_payment_index(&self) -> Result<(), MongoError> {
        {
            let model = IndexModel::builder()
                .keys(doc! {"id": 1})
                .options(IndexOptions::builder().unique(true).build())
                .build();

            self.database
                .collection::<Payment>(MONGO_COLLECTION_PAYMENTS)
                .create_index(model, None)
                .await?;

            Ok(())
        }
    }
}

impl PaymentRepository for Mongo {
    type Error = MongoError;

    async fn get_payment(&self, id: &PaymentID) -> Result<Option<Payment>, Self::Error> {
        let payments: Collection<Payment> = self.database.collection(MONGO_COLLECTION_PAYMENTS);

        let filter = doc! { "id": id };
        let result = payments.find_one(filter, None).await?;

        Ok(result)
    }

    async fn get_payments_by_group(&self, group: &GroupID) -> Result<Vec<Payment>, Self::Error> {
        let payments: Collection<Payment> = self.database.collection(MONGO_COLLECTION_PAYMENTS);

        let filter = doc! { "group": group };
        let result = payments.find(filter, None).await?.try_collect().await?;

        Ok(result)
    }
}
