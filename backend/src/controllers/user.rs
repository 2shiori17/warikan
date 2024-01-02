use crate::{
    entities::{AuthState, User, UserID},
    usecases::UseCase,
};
use async_graphql::{Context, Object};

#[Object]
impl User {
    async fn id(&self) -> UserID {
        self.id.clone()
    }
}

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn get_user(&self, ctx: &Context<'_>, id: UserID) -> async_graphql::Result<Option<User>> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        let user = usecase.get_user_opt(&id, auth).await?;
        Ok(user)
    }
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn create_user(&self, ctx: &Context<'_>) -> async_graphql::Result<User> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        Ok(usecase.create_user(auth).await?)
    }

    async fn delete_user(&self, ctx: &Context<'_>, id: UserID) -> async_graphql::Result<UserID> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        Ok(usecase.delete_user(&id, auth).await?)
    }
}
