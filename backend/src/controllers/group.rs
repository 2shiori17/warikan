use crate::{
    entities::{Group, GroupID, Payment, User},
    repositories::Mongo,
    usecases::UseCase,
};
use async_graphql::{Context, Object};
use chrono::{DateTime, Local};

#[Object]
impl Group {
    async fn id(&self) -> GroupID {
        self.id.clone()
    }

    async fn created_at(&self) -> DateTime<Local> {
        self.created_at
    }

    async fn participants(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<User>> {
        let usecase = ctx.data::<UseCase<Mongo>>()?;
        let participants = usecase.get_users(&self.participants).await?;
        Ok(participants)
    }

    async fn payments(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Payment>> {
        let usecase = ctx.data::<UseCase<Mongo>>()?;
        let payments = usecase.get_payments_by_group(&self.id).await?;
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
        let usecase = ctx.data::<UseCase<Mongo>>()?;
        let group = usecase.get_group_proper(&id).await?;
        Ok(group)
    }
}
