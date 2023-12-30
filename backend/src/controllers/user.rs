use crate::{
    entities::{User, UserID},
    mongo::Mongo,
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
        let mongo = ctx.data::<Mongo>()?;
        let user = mongo.get_user(&id).await?;
        Ok(user)
    }
}
