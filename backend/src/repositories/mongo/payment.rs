use crate::{
    entities::{GroupID, Payment, PaymentID},
    repositories::{Mongo, MongoError, PaymentRepository, MONGO_COLLECTION_PAYMENTS},
};
use async_trait::async_trait;
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

#[async_trait]
impl PaymentRepository for Mongo {
    async fn create_payment(
        &self,
        payment: Payment,
    ) -> Result<Payment, Box<dyn std::error::Error + Send + Sync>> {
        let payments: Collection<Payment> = self.database.collection(MONGO_COLLECTION_PAYMENTS);
        let _ = payments.insert_one(&payment, None).await?;
        Ok(payment)
    }

    async fn delete_payment(
        &self,
        id: &PaymentID,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let payments: Collection<Payment> = self.database.collection(MONGO_COLLECTION_PAYMENTS);

        let filter = doc! { "id": id };
        let result = payments.delete_one(filter, None).await?;

        assert!(result.deleted_count == 1);
        Ok(())
    }

    async fn get_payment(
        &self,
        id: &PaymentID,
    ) -> Result<Option<Payment>, Box<dyn std::error::Error + Send + Sync>> {
        let payments: Collection<Payment> = self.database.collection(MONGO_COLLECTION_PAYMENTS);

        let filter = doc! { "id": id };
        let result = payments.find_one(filter, None).await?;

        Ok(result)
    }

    async fn get_payments_by_group(
        &self,
        group: &GroupID,
    ) -> Result<Vec<Payment>, Box<dyn std::error::Error + Send + Sync>> {
        let payments: Collection<Payment> = self.database.collection(MONGO_COLLECTION_PAYMENTS);

        let filter = doc! { "group": group };
        let result = payments.find(filter, None).await?.try_collect().await?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::MongoConfig;
    use fake::{Fake, Faker};

    #[tokio::test]
    async fn create_payment() {
        let mongo = Mongo::new(MongoConfig {
            uri: "mongodb://localhost:27017",
            database: "warikan",
        })
        .await
        .unwrap();

        let payment: Payment = Faker.fake();

        let create = mongo.create_payment(payment).await.unwrap();
        let get = mongo.get_payment(&create.id).await.unwrap();

        assert_eq!(Some(create), get);
    }

    #[tokio::test]
    async fn delete_payment() {
        let mongo = Mongo::new(MongoConfig {
            uri: "mongodb://localhost:27017",
            database: "warikan",
        })
        .await
        .unwrap();

        let payment: Payment = Faker.fake();

        let create = mongo.create_payment(payment).await.unwrap();
        mongo.delete_payment(&create.id).await.unwrap();
        let delete = mongo.get_payment(&create.id).await.unwrap();

        assert_eq!(delete, None);
    }

    #[tokio::test]
    async fn get_payments_by_group() {
        let mongo = Mongo::new(MongoConfig {
            uri: "mongodb://localhost:27017",
            database: "warikan",
        })
        .await
        .unwrap();

        let mut payment1: Payment = Faker.fake();
        let mut payment2: Payment = Faker.fake();
        let mut payment3: Payment = Faker.fake();

        let group: GroupID = Faker.fake();
        payment1.group = group.clone();
        payment2.group = group.clone();
        payment3.group = group.clone();

        mongo.create_payment(payment1.clone()).await.unwrap();
        mongo.create_payment(payment2.clone()).await.unwrap();
        mongo.create_payment(payment3.clone()).await.unwrap();

        let get = mongo.get_payments_by_group(&group).await.unwrap();

        assert_eq!(vec![payment1, payment2, payment3], get);
    }
}
