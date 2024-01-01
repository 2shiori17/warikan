use crate::{
    entities::{AuthState, Group, GroupID, UserID},
    usecases::{UseCase, UseCaseError},
};

impl UseCase {
    pub async fn get_group_proper(
        &self,
        id: &GroupID,
        auth: &AuthState,
    ) -> Result<Option<Group>, Box<dyn std::error::Error + Send + Sync>> {
        if let AuthState::Authorized(claims) = auth {
            let group = self
                .repository
                .get_group(id)
                .await?
                .map(|group| {
                    group
                        .participants
                        .contains(&UserID::new(&claims.sub))
                        .then_some(group)
                        .ok_or(UseCaseError::UnAuthorized)
                })
                .transpose()?;
            Ok(group)
        } else {
            Err(UseCaseError::UnAuthorized)?
        }
    }

    pub async fn get_group(
        &self,
        id: &GroupID,
        auth: &AuthState,
    ) -> Result<Group, Box<dyn std::error::Error + Send + Sync>> {
        let group = self
            .get_group_proper(id, auth)
            .await?
            .ok_or(UseCaseError::NotFound)?;
        Ok(group)
    }

    // TODO(2shiori17): `get_group_proper`を使ったロジックに変更する
    pub async fn have_authority(&self, id: &GroupID, auth: &AuthState) -> bool {
        if let AuthState::Authorized(claims) = auth {
            if let Ok(Some(group)) = self.repository.get_group(id).await {
                return group.participants.contains(&UserID::new(&claims.sub));
            }
        }
        false
    }
}
