use crate::{
    entities::{Group, Payment, PaymentID, User},
    mongo::{Mongo, MongoError},
};
use async_graphql::{Context, Object};
use chrono::{DateTime, Local};
use futures::future::try_join_all;

#[Object]
impl Payment {
    async fn id(&self) -> PaymentID {
        self.id.clone()
    }

    async fn created_at(&self) -> DateTime<Local> {
        self.created_at
    }

    async fn group(&self, ctx: &Context<'_>) -> async_graphql::Result<Group> {
        let mongo = ctx.data::<Mongo>()?;
        let group = mongo
            .get_group(&self.group)
            .await?
            .ok_or(MongoError::NotFound)?;
        Ok(group)
    }

    async fn creditor(&self, ctx: &Context<'_>) -> async_graphql::Result<User> {
        let mongo = ctx.data::<Mongo>()?;
        let creditor = mongo
            .get_user(&self.creditor)
            .await?
            .ok_or(MongoError::NotFound)?;
        Ok(creditor)
    }

    async fn debtors(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<User>> {
        let mongo = ctx.data::<Mongo>()?;
        let debtors = try_join_all(self.debtors.iter().map(|id| async {
            mongo
                .get_user(id)
                .await
                .and_then(|x| x.ok_or(MongoError::NotFound))
        }))
        .await?;
        Ok(debtors)
    }
}

#[derive(Default)]
pub struct PaymentQuery;

#[Object]
impl PaymentQuery {
    async fn get_payment(
        &self,
        ctx: &Context<'_>,
        id: PaymentID,
    ) -> async_graphql::Result<Option<Payment>> {
        let mongo = ctx.data::<Mongo>()?;
        let payment = mongo.get_payment(&id).await?;
        Ok(payment)
    }
}
