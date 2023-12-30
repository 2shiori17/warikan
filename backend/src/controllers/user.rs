use crate::{
    entities::{User, UserID},
    repositories::Mongo,
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
        let usecase = ctx.data::<UseCase<Mongo>>()?;
        let user = usecase.get_user_proper(&id).await?;
        Ok(user)
    }
}
