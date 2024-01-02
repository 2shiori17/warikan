use crate::{
    entities::{User, UserID},
    usecases::{UseCase, UseCaseError},
};
use futures::future::try_join_all;

impl UseCase {
    pub async fn get_user_opt(
        &self,
        id: &UserID,
    ) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
        let user = self.repository.get_user(id).await?;
        Ok(user)
    }

    pub async fn get_user(
        &self,
        id: &UserID,
    ) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        let user = self
            .repository
            .get_user(id)
            .await?
            .ok_or(UseCaseError::NotFound)?;
        Ok(user)
    }

    pub async fn get_users(
        &self,
        ids: &[UserID],
    ) -> Result<Vec<User>, Box<dyn std::error::Error + Send + Sync>> {
        let participants =
            try_join_all(ids.iter().map(|id| async { self.get_user(id).await })).await?;
        Ok(participants)
    }
}
