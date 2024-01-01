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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{entities::Claims, repositories::MockRepository};
    use fake::{Fake, Faker};
    use std::sync::Arc;

    #[tokio::test]
    async fn get_group_unauthorized() {
        let group: Group = Faker.fake();
        let claims: Claims = Faker.fake();

        let id = group.id.clone();
        let auth = AuthState::Authorized(claims);

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .return_once(move |_| Ok(Some(group)));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.get_group(&id, &auth).await.is_err());
    }

    #[tokio::test]
    async fn get_group_authorized() {
        let group: Group = Faker.fake();
        let mut claims: Claims = Faker.fake();
        claims.sub = group.participants[0].to_string();

        let id = group.id.clone();
        let auth = AuthState::Authorized(claims);

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .return_once(move |_| Ok(Some(group)));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.get_group(&id, &auth).await.is_ok());
    }
}
