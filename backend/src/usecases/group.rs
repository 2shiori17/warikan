use futures::future::try_join_all;

use crate::{
    entities::{AuthState, Group, GroupID, UserID},
    usecases::{UseCase, UseCaseError},
};

impl UseCase {
    pub async fn create_group(
        &self,
        title: String,
        auth: &AuthState,
    ) -> Result<Group, Box<dyn std::error::Error + Send + Sync>> {
        if let AuthState::Authorized(claims) = auth {
            let group = Group::new(title, claims);
            let group = self.repository.create_group(group).await?;
            Ok(group)
        } else {
            Err(UseCaseError::UnAuthorized)?
        }
    }

    pub async fn delete_group(
        &self,
        id: &GroupID,
        auth: &AuthState,
    ) -> Result<GroupID, Box<dyn std::error::Error + Send + Sync>> {
        if self.have_authority_group(id, auth).await {
            try_join_all(
                self.repository
                    .get_payments_by_group(id)
                    .await?
                    .iter()
                    .map(|payment| async { self.repository.delete_payment(&payment.id).await }),
            )
            .await?;
            self.repository.delete_group(id).await?;
            Ok(id.clone())
        } else {
            Err(UseCaseError::UnAuthorized)?
        }
    }

    pub async fn get_group_opt(
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
            .get_group_opt(id, auth)
            .await?
            .ok_or(UseCaseError::NotFound)?;
        Ok(group)
    }

    pub async fn get_groups_by_user(
        &self,
        auth: &AuthState,
    ) -> Result<Vec<Group>, Box<dyn std::error::Error + Send + Sync>> {
        if let AuthState::Authorized(claims) = auth {
            let groups = self
                .repository
                .get_groups_by_user(&UserID::new(&claims.sub))
                .await?;
            Ok(groups)
        } else {
            Err(UseCaseError::UnAuthorized)?
        }
    }

    // TODO(2shiori17): `get_group_opt`を使ったロジックに変更する
    pub async fn have_authority_group(&self, id: &GroupID, auth: &AuthState) -> bool {
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
    use crate::{
        entities::{Claims, Payment},
        repositories::MockRepository,
    };
    use fake::{Fake, Faker};
    use std::sync::Arc;

    #[tokio::test]
    async fn create_group_unauthorized() {
        let title: String = Faker.fake();
        let auth = AuthState::UnAuthorized;

        let mock = MockRepository::new();

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.create_group(title, &auth).await.is_err());
    }

    #[tokio::test]
    async fn create_group_authorized() {
        let claims: Claims = Faker.fake();
        let title: String = Faker.fake();
        let auth = AuthState::Authorized(claims);

        let mut mock = MockRepository::new();
        mock.expect_create_group()
            .return_once(move |group| Ok(group));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.create_group(title, &auth).await.is_ok());
    }

    #[tokio::test]
    async fn delete_group_unauthorized_1() {
        let group: Group = Faker.fake();

        let id = group.id.clone();
        let auth = AuthState::UnAuthorized;

        let mock = MockRepository::new();

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.delete_group(&id, &auth).await.is_err());
    }

    #[tokio::test]
    async fn delete_group_unauthorized_2() {
        let group: Group = Faker.fake();
        let claims: Claims = Faker.fake();

        let id = group.id.clone();
        let auth = AuthState::Authorized(claims);

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .return_once(move |_| Ok(Some(group)));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.delete_group(&id, &auth).await.is_err());
    }

    #[tokio::test]
    async fn delete_group_authorized() {
        let group: Group = Faker.fake();
        let mut payments: Vec<Payment> = vec![Faker.fake()];
        let mut claims: Claims = Faker.fake();

        payments[0].group = group.id.clone();
        claims.sub = group.participants[0].to_string();
        let id = group.id.clone();
        let auth = AuthState::Authorized(claims);

        let mut mock = MockRepository::new();
        mock.expect_delete_payment().return_once(move |_| Ok(()));
        mock.expect_delete_group().return_once(move |_| Ok(()));
        mock.expect_get_group()
            .return_once(move |_| Ok(Some(group)));
        mock.expect_get_payments_by_group()
            .return_once(move |_| Ok(payments));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.delete_group(&id, &auth).await.is_ok());
    }

    #[tokio::test]
    async fn get_group_unauthorized_1() {
        let group: Group = Faker.fake();

        let id = group.id.clone();
        let auth = AuthState::UnAuthorized;

        let mock = MockRepository::new();

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.get_group(&id, &auth).await.is_err());
    }

    #[tokio::test]
    async fn get_group_unauthorized_2() {
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

    #[tokio::test]
    async fn get_groups_by_user_unauthorized() {
        let auth = AuthState::UnAuthorized;

        let mock = MockRepository::new();

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.get_groups_by_user(&auth).await.is_err());
    }

    #[tokio::test]
    async fn get_groups_by_user_authorized() {
        let groups: Vec<Group> = vec![Faker.fake()];
        let mut claims: Claims = Faker.fake();

        claims.sub = groups[0].participants[0].to_string();
        let auth = AuthState::Authorized(claims);

        let mut mock = MockRepository::new();
        mock.expect_get_groups_by_user()
            .return_once(move |_| Ok(groups));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.get_groups_by_user(&auth).await.is_ok());
    }

    #[tokio::test]
    async fn have_authority_group_unauthorized_1() {
        let group: Group = Faker.fake();

        let id = group.id.clone();
        let auth = AuthState::UnAuthorized;

        let mock = MockRepository::new();

        let usecase = UseCase::new(Arc::new(mock));
        assert!(!usecase.have_authority_group(&id, &auth).await);
    }

    #[tokio::test]
    async fn have_authority_group_unauthorized_2() {
        let group: Group = Faker.fake();
        let claims: Claims = Faker.fake();

        let id = group.id.clone();
        let auth = AuthState::Authorized(claims);

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .return_once(move |_| Ok(Some(group)));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(!usecase.have_authority_group(&id, &auth).await);
    }

    #[tokio::test]
    async fn have_authority_group_authorized() {
        let group: Group = Faker.fake();
        let mut claims: Claims = Faker.fake();

        claims.sub = group.participants[0].to_string();
        let id = group.id.clone();
        let auth = AuthState::Authorized(claims);

        let mut mock = MockRepository::new();
        mock.expect_get_group()
            .return_once(move |_| Ok(Some(group)));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.have_authority_group(&id, &auth).await);
    }
}
