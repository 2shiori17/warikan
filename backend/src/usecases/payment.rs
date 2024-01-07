use crate::{
    entities::{AuthState, GroupID, Payment, PaymentID, UserID},
    usecases::{UseCase, UseCaseError},
};

impl UseCase {
    pub async fn create_payment(
        &self,
        title: String,
        group: GroupID,
        creditor: UserID,
        debtors: Vec<UserID>,
        auth: &AuthState,
    ) -> Result<Payment, Box<dyn std::error::Error + Send + Sync>> {
        if self.have_authority_group(&group, auth).await {
            let payment = Payment::new(title, group, creditor, debtors);
            let payment = self.repository.create_payment(payment).await?;
            Ok(payment)
        } else {
            Err(UseCaseError::UnAuthorized)?
        }
    }

    pub async fn delete_payment(
        &self,
        id: &PaymentID,
        auth: &AuthState,
    ) -> Result<PaymentID, Box<dyn std::error::Error + Send + Sync>> {
        if self.have_authority_payment(id, auth).await {
            self.repository.delete_payment(id).await?;
            Ok(id.clone())
        } else {
            Err(UseCaseError::UnAuthorized)?
        }
    }

    pub async fn get_payment_opt(
        &self,
        id: &PaymentID,
        auth: &AuthState,
    ) -> Result<Option<Payment>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(
            if let Some(payment) = self.repository.get_payment(id).await? {
                Some(
                    self.have_authority_group(&payment.group, auth)
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
            .get_payment_opt(id, auth)
            .await?
            .ok_or(UseCaseError::NotFound)?;
        Ok(payment)
    }

    pub async fn get_payments_by_group(
        &self,
        group: &GroupID,
        auth: &AuthState,
    ) -> Result<Vec<Payment>, Box<dyn std::error::Error + Send + Sync>> {
        if !self.have_authority_group(group, auth).await {
            return Err(UseCaseError::UnAuthorized)?;
        }
        let payments = self.repository.get_payments_by_group(group).await?;
        Ok(payments)
    }

    // TODO(2shiori17): `get_payment_opt`を使ったロジックに変更する
    pub async fn have_authority_payment(&self, id: &PaymentID, auth: &AuthState) -> bool {
        if let Ok(Some(payment)) = self.repository.get_payment(id).await {
            self.have_authority_group(&payment.group, auth).await
        } else {
            false
        }
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
    async fn create_payment_unauthorized_1() {
        let title: String = Faker.fake();
        let group: Group = Faker.fake();
        let creditor: UserID = Faker.fake();
        let debtors: Vec<UserID> = Faker.fake();

        let id = group.id.clone();
        let auth = AuthState::UnAuthorized;

        let mock = MockRepository::new();

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase
            .create_payment(title, id, creditor, debtors, &auth)
            .await
            .is_err());
    }

    #[tokio::test]
    async fn create_payment_unauthorized_2() {
        let claims: Claims = Faker.fake();
        let title: String = Faker.fake();
        let group: Group = Faker.fake();
        let creditor: UserID = Faker.fake();
        let debtors: Vec<UserID> = Faker.fake();

        let id = group.id.clone();
        let auth = AuthState::Authorized(claims);

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .return_once(move |_| Ok(Some(group)));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase
            .create_payment(title, id, creditor, debtors, &auth)
            .await
            .is_err());
    }

    #[tokio::test]
    async fn create_payment_authorized() {
        let mut claims: Claims = Faker.fake();
        let title: String = Faker.fake();
        let group: Group = Faker.fake();
        let creditor: UserID = Faker.fake();
        let debtors: Vec<UserID> = Faker.fake();

        claims.sub = group.participants[0].to_string();
        let id = group.id.clone();
        let auth = AuthState::Authorized(claims);

        let mut mock = MockRepository::new();
        mock.expect_create_payment()
            .return_once(move |payment| Ok(payment));
        mock.expect_get_group()
            .return_once(move |_| Ok(Some(group)));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase
            .create_payment(title, id, creditor, debtors, &auth)
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn delete_payment_unauthorized_1() {
        let payment: Payment = Faker.fake();

        let id = payment.id.clone();
        let auth = AuthState::UnAuthorized;

        let mut mock = MockRepository::new();
        mock.expect_get_payment()
            .return_once(move |_| Ok(Some(payment)));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.delete_payment(&id, &auth).await.is_err());
    }

    #[tokio::test]
    async fn delete_payment_unauthorized_2() {
        let group: Group = Faker.fake();
        let payment: Payment = Faker.fake();
        let claims: Claims = Faker.fake();

        let id = payment.id.clone();
        let auth = AuthState::Authorized(claims);

        let mut mock = MockRepository::new();
        mock.expect_get_payment()
            .return_once(move |_| Ok(Some(payment)));
        mock.expect_get_group()
            .return_once(move |_| Ok(Some(group)));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.delete_payment(&id, &auth).await.is_err());
    }

    #[tokio::test]
    async fn delete_payment_authorized() {
        let group: Group = Faker.fake();
        let payment: Payment = Faker.fake();
        let mut claims: Claims = Faker.fake();

        claims.sub = group.participants[0].to_string();
        let id = payment.id.clone();
        let auth = AuthState::Authorized(claims);

        let mut mock = MockRepository::new();
        mock.expect_delete_payment().return_once(move |_| Ok(()));
        mock.expect_get_payment()
            .return_once(move |_| Ok(Some(payment)));
        mock.expect_get_group()
            .return_once(move |_| Ok(Some(group)));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.delete_payment(&id, &auth).await.is_ok());
    }

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

    #[tokio::test]
    async fn get_payments_by_group_unauthorized() {
        let group: Group = Faker.fake();
        let claims: Claims = Faker.fake();

        let id = group.id.clone();
        let auth = AuthState::Authorized(claims);

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .return_once(move |_| Ok(Some(group)));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.get_payments_by_group(&id, &auth).await.is_err());
    }

    #[tokio::test]
    async fn get_payments_by_group_authorized() {
        let payments: Vec<Payment> = Faker.fake();
        let group: Group = Faker.fake();
        let mut claims: Claims = Faker.fake();

        claims.sub = group.participants[0].to_string();
        let id = group.id.clone();
        let auth = AuthState::Authorized(claims);

        let mut mock = MockRepository::new();
        mock.expect_get_payments_by_group()
            .return_once(move |_| Ok(payments));
        mock.expect_get_group()
            .return_once(move |_| Ok(Some(group)));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.get_payments_by_group(&id, &auth).await.is_ok());
    }

    #[tokio::test]
    async fn have_authority_payment_unauthorized_1() {
        let payment: Payment = Faker.fake();

        let id = payment.id.clone();
        let auth = AuthState::UnAuthorized;

        let mut mock = MockRepository::new();
        mock.expect_get_payment()
            .return_once(move |_| Ok(Some(payment)));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(!usecase.have_authority_payment(&id, &auth).await);
    }

    #[tokio::test]
    async fn have_authority_payment_unauthorized_2() {
        let group: Group = Faker.fake();
        let payment: Payment = Faker.fake();
        let claims: Claims = Faker.fake();

        let id = payment.id.clone();
        let auth = AuthState::Authorized(claims);

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .return_once(move |_| Ok(Some(group)));
        mock.expect_get_payment()
            .return_once(move |_| Ok(Some(payment)));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(!usecase.have_authority_payment(&id, &auth).await);
    }

    #[tokio::test]
    async fn have_authority_payment_authorized() {
        let group: Group = Faker.fake();
        let payment: Payment = Faker.fake();
        let mut claims: Claims = Faker.fake();

        claims.sub = group.participants[0].to_string();
        let id = payment.id.clone();
        let auth = AuthState::Authorized(claims);

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .return_once(move |_| Ok(Some(group)));
        mock.expect_get_payment()
            .return_once(move |_| Ok(Some(payment)));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.have_authority_payment(&id, &auth).await);
    }
}
