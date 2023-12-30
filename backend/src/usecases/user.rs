use crate::{
    entities::{User, UserID},
    repositories::Repository,
    usecases::{UseCase, UseCaseError, UseCaseResult},
};
use futures::future::try_join_all;

impl<R: Repository> UseCase<R> {
    pub async fn get_user_proper(&self, id: &UserID) -> UseCaseResult<Option<User>> {
        let user = self.repository.get_user(id).await?;
        Ok(user)
    }

    pub async fn get_user(&self, id: &UserID) -> UseCaseResult<User> {
        let user = self
            .repository
            .get_user(id)
            .await?
            .ok_or(UseCaseError::NotFound)?;
        Ok(user)
    }

    pub async fn get_users(&self, ids: &[UserID]) -> UseCaseResult<Vec<User>> {
        let participants =
            try_join_all(ids.iter().map(|id| async { self.get_user(id).await })).await?;
        Ok(participants)
    }
}
