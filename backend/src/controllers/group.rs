use crate::{
    entities::{Group, GroupID, Payment, User},
    repositories::{GroupRepository, Mongo, MongoError, PaymentRepository, UserRepository},
};
use async_graphql::{Context, Object};
use chrono::{DateTime, Local};
use futures::future::try_join_all;

#[Object]
impl Group {
    async fn id(&self) -> GroupID {
        self.id.clone()
    }

    async fn created_at(&self) -> DateTime<Local> {
        self.created_at
    }

    async fn participants(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<User>> {
        let mongo = ctx.data::<Mongo>()?;
        let participants = try_join_all(self.participants.iter().map(|id| async {
            mongo
                .get_user(id)
                .await
                .and_then(|x| x.ok_or(MongoError::NotFound))
        }))
        .await?;
        Ok(participants)
    }

    async fn payments(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Payment>> {
        let mongo = ctx.data::<Mongo>()?;
        let payments = mongo.get_payments_by_group(&self.id).await?;
        Ok(payments)
    }
}

#[derive(Default)]
pub struct GroupQuery;

#[Object]
impl GroupQuery {
    async fn get_group(
        &self,
        ctx: &Context<'_>,
        id: GroupID,
    ) -> async_graphql::Result<Option<Group>> {
        let mongo = ctx.data::<Mongo>()?;
        let group = mongo.get_group(&id).await?;
        Ok(group)
    }
}
