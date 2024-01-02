use crate::{
    entities::{AuthState, Group, GroupID, Payment, User},
    usecases::UseCase,
};
use async_graphql::{Context, Object};
use chrono::{DateTime, Utc};

#[Object]
impl Group {
    async fn id(&self) -> GroupID {
        self.id.clone()
    }

    async fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    async fn participants(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<User>> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        Ok(usecase.get_users(&self.participants, auth).await?)
    }

    async fn payments(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Payment>> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        Ok(usecase.get_payments_by_group(&self.id, auth).await?)
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
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        Ok(usecase.get_group_opt(&id, auth).await?)
    }
}

#[derive(Default)]
pub struct GroupMutation;

#[Object]
impl GroupMutation {
    async fn create_group(&self, ctx: &Context<'_>) -> async_graphql::Result<Group> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        Ok(usecase.create_group(auth).await?)
    }

    async fn delete_group(&self, ctx: &Context<'_>, id: GroupID) -> async_graphql::Result<GroupID> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        Ok(usecase.delete_group(&id, auth).await?)
    }
}
