use crate::{
    entities::{AuthState, Group, Payment, PaymentID, User},
    usecases::UseCase,
};
use async_graphql::{Context, Object};
use chrono::{DateTime, Utc};

#[Object]
impl Payment {
    async fn id(&self) -> PaymentID {
        self.id.clone()
    }

    async fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    async fn group(&self, ctx: &Context<'_>) -> async_graphql::Result<Group> {
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        let group = usecase.get_group(&self.group, auth).await?;
        Ok(group)
    }

    async fn creditor(&self, ctx: &Context<'_>) -> async_graphql::Result<User> {
        let usecase = ctx.data::<UseCase>()?;
        let creditor = usecase.get_user(&self.creditor).await?;
        Ok(creditor)
    }

    async fn debtors(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<User>> {
        let usecase = ctx.data::<UseCase>()?;
        let debtors = usecase.get_users(&self.debtors).await?;
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
        let usecase = ctx.data::<UseCase>()?;
        let auth = ctx.data::<AuthState>()?;
        let payment = usecase.get_payment_opt(&id, auth).await?;
        Ok(payment)
    }
}
