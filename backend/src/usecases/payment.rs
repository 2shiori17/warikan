use crate::{
    entities::{AuthState, GroupID, Payment, PaymentID},
    usecases::{UseCase, UseCaseError},
};

impl UseCase {
    pub async fn get_payment_proper(
        &self,
        id: &PaymentID,
        auth: &AuthState,
    ) -> Result<Option<Payment>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(
            if let Some(payment) = self.repository.get_payment(id).await? {
                Some(
                    self.have_authority(&payment.group, auth)
                        .await
                        .then_some(payment)
                        .ok_or(UseCaseError::UnAuthorized)?,
                )
            } else {
                None
            },
        )
    }

    pub async fn get_payment(
        &self,
        id: &PaymentID,
        auth: &AuthState,
    ) -> Result<Payment, Box<dyn std::error::Error + Send + Sync>> {
        let payment = self
            .get_payment_proper(id, auth)
            .await?
            .ok_or(UseCaseError::NotFound)?;
        Ok(payment)
    }

    pub async fn get_payments_by_group(
        &self,
        group: &GroupID,
        auth: &AuthState,
    ) -> Result<Vec<Payment>, Box<dyn std::error::Error + Send + Sync>> {
        if !self.have_authority(group, auth).await {
            return Err(UseCaseError::UnAuthorized)?;
        }
        let payments = self.repository.get_payments_by_group(group).await?;
        Ok(payments)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        entities::{Claims, Group},
        repositories::MockRepository,
    };
    use fake::{Fake, Faker};
    use std::sync::Arc;

    #[tokio::test]
    async fn get_payment_unauthorized() {
        let payment: Payment = Faker.fake();
        let group: Group = Faker.fake();
        let claims: Claims = Faker.fake();

        let id = payment.id.clone();
        let auth = AuthState::Authorized(claims);

        let mut mock = MockRepository::new();
        mock.expect_get_payment()
            .return_once(move |_| Ok(Some(payment)));
        mock.expect_get_group()
            .return_once(move |_| Ok(Some(group)));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.get_payment(&id, &auth).await.is_err());
    }

    #[tokio::test]
    async fn get_payment_authorized() {
        let payment: Payment = Faker.fake();
        let group: Group = Faker.fake();
        let mut claims: Claims = Faker.fake();
        claims.sub = group.participants[0].to_string();

        let id = payment.id.clone();
        let auth = AuthState::Authorized(claims);

        let mut mock = MockRepository::new();
        mock.expect_get_payment()
            .return_once(move |_| Ok(Some(payment)));
        mock.expect_get_group()
            .return_once(move |_| Ok(Some(group)));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.get_payment(&id, &auth).await.is_ok());
    }
}
