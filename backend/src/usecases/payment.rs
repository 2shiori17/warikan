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
