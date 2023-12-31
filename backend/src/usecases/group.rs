use crate::{
    entities::{Group, GroupID},
    usecases::{UseCase, UseCaseError},
};

impl UseCase {
    pub async fn get_group_proper(
        &self,
        id: &GroupID,
    ) -> Result<Option<Group>, Box<dyn std::error::Error + Send + Sync>> {
        let group = self.repository.get_group(id).await?;
        Ok(group)
    }

    pub async fn get_group(
        &self,
        id: &GroupID,
    ) -> Result<Group, Box<dyn std::error::Error + Send + Sync>> {
        let group = self
            .repository
            .get_group(id)
            .await?
            .ok_or(UseCaseError::NotFound)?;
        Ok(group)
    }
}
