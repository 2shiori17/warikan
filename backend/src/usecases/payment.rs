use crate::{
    entities::{GroupID, Payment, PaymentID},
    repositories::Repository,
    usecases::{UseCase, UseCaseError},
};

impl<R: Repository> UseCase<R> {
    pub async fn get_payment_proper(
        &self,
        id: &PaymentID,
    ) -> Result<Option<Payment>, Box<dyn std::error::Error + Send + Sync>> {
        let payment = self.repository.get_payment(id).await?;
        Ok(payment)
    }

    pub async fn get_payment(
        &self,
        id: &PaymentID,
    ) -> Result<Payment, Box<dyn std::error::Error + Send + Sync>> {
        let payment = self
            .repository
            .get_payment(id)
            .await?
            .ok_or(UseCaseError::NotFound)?;
        Ok(payment)
    }

    pub async fn get_payments_by_group(
        &self,
        group: &GroupID,
    ) -> Result<Vec<Payment>, Box<dyn std::error::Error + Send + Sync>> {
        let payments = self.repository.get_payments_by_group(group).await?;
        Ok(payments)
    }
}
