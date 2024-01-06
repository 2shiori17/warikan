use crate::{
    entities::{AuthState, User, UserID},
    usecases::{UseCase, UseCaseError},
};
use futures::future::try_join_all;

impl UseCase {
    pub async fn create_user(
        &self,
        name: String,
        auth: &AuthState,
    ) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        if let AuthState::Authorized(claims) = auth {
            let user = User::new(name, claims);
            let user = self.repository.create_user(user).await?;
            Ok(user)
        } else {
            Err(UseCaseError::UnAuthorized)?
        }
    }

    pub async fn delete_user(
        &self,
        id: &UserID,
        auth: &AuthState,
    ) -> Result<UserID, Box<dyn std::error::Error + Send + Sync>> {
        if self.have_authority_user(id, auth).await {
            self.repository.delete_user(id).await?;
            Ok(id.clone())
        } else {
            Err(UseCaseError::UnAuthorized)?
        }
    }

    pub async fn get_user_opt(
        &self,
        id: &UserID,
        auth: &AuthState,
    ) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
        if let AuthState::Authorized(_) = auth {
            let user = self.repository.get_user(id).await?;
            Ok(user)
        } else {
            Err(UseCaseError::UnAuthorized)?
        }
    }

    pub async fn get_user(
        &self,
        id: &UserID,
        auth: &AuthState,
    ) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        let user = self
            .get_user_opt(id, auth)
            .await?
            .ok_or(UseCaseError::NotFound)?;
        Ok(user)
    }

    pub async fn get_users(
        &self,
        ids: &[UserID],
        auth: &AuthState,
    ) -> Result<Vec<User>, Box<dyn std::error::Error + Send + Sync>> {
        if let AuthState::Authorized(_) = auth {
            let participants =
                try_join_all(ids.iter().map(|id| async { self.get_user(id, auth).await })).await?;
            Ok(participants)
        } else {
            Err(UseCaseError::UnAuthorized)?
        }
    }

    // TODO(2shiori17): `get_user_opt`を使ったロジックに変更する
    pub async fn have_authority_user(&self, id: &UserID, auth: &AuthState) -> bool {
        if let AuthState::Authorized(claims) = auth {
            if let Ok(Some(user)) = self.repository.get_user(id).await {
                return user.id.to_string() == claims.sub;
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
    async fn create_user_unauthorized() {
        let name: String = Faker.fake();
        let auth = AuthState::UnAuthorized;

        let mock = MockRepository::new();

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.create_user(name, &auth).await.is_err());
    }

    #[tokio::test]
    async fn create_user_authorized() {
        let name: String = Faker.fake();
        let claims: Claims = Faker.fake();
        let auth = AuthState::Authorized(claims);

        let mut mock = MockRepository::new();
        mock.expect_create_user().return_once(move |user| Ok(user));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.create_user(name, &auth).await.is_ok());
    }

    #[tokio::test]
    async fn delete_user_unauthorized_1() {
        let user: User = Faker.fake();

        let id = user.id.clone();
        let auth = AuthState::UnAuthorized;

        let mock = MockRepository::new();

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.delete_user(&id, &auth).await.is_err());
    }

    #[tokio::test]
    async fn delete_user_unauthorized_2() {
        let user: User = Faker.fake();
        let claims: Claims = Faker.fake();

        let id = user.id.clone();
        let auth = AuthState::Authorized(claims);

        let mut mock = MockRepository::new();
        mock.expect_get_user().return_once(move |_| Ok(Some(user)));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.delete_user(&id, &auth).await.is_err());
    }

    #[tokio::test]
    async fn delete_user_authorized() {
        let user: User = Faker.fake();
        let mut claims: Claims = Faker.fake();

        claims.sub = user.id.to_string();
        let id = user.id.clone();
        let auth = AuthState::Authorized(claims);

        let mut mock = MockRepository::new();
        mock.expect_delete_user().return_once(move |_| Ok(()));
        mock.expect_get_user().return_once(move |_| Ok(Some(user)));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.delete_user(&id, &auth).await.is_ok());
    }

    #[tokio::test]
    async fn get_user_unauthorized() {
        let user: User = Faker.fake();

        let id = user.id.clone();
        let auth = AuthState::UnAuthorized;

        let mock = MockRepository::new();

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.get_user(&id, &auth).await.is_err());
    }

    #[tokio::test]
    async fn get_user_authorized_1() {
        let user: User = Faker.fake();
        let claims: Claims = Faker.fake();

        let id = user.id.clone();
        let auth = AuthState::Authorized(claims);

        let mut mock = MockRepository::new();
        mock.expect_get_user().return_once(move |_| Ok(Some(user)));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.get_user(&id, &auth).await.is_ok());
    }

    #[tokio::test]
    async fn get_user_authorized_2() {
        let user: User = Faker.fake();
        let mut claims: Claims = Faker.fake();

        claims.sub = user.id.to_string();
        let id = user.id.clone();
        let auth = AuthState::Authorized(claims);

        let mut mock = MockRepository::new();
        mock.expect_get_user().return_once(move |_| Ok(Some(user)));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.get_user(&id, &auth).await.is_ok());
    }

    #[tokio::test]
    async fn get_users_unauthorized() {
        let user: User = Faker.fake();

        let id = user.id.clone();
        let auth = AuthState::UnAuthorized;

        let mock = MockRepository::new();

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.get_users(&[id], &auth).await.is_err());
    }

    #[tokio::test]
    async fn get_users_authorized_1() {
        let user: User = Faker.fake();
        let claims: Claims = Faker.fake();

        let id = user.id.clone();
        let auth = AuthState::Authorized(claims);

        let mut mock = MockRepository::new();
        mock.expect_get_user().return_once(move |_| Ok(Some(user)));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.get_users(&[id], &auth).await.is_ok());
    }

    #[tokio::test]
    async fn get_users_authorized_2() {
        let user: User = Faker.fake();
        let mut claims: Claims = Faker.fake();

        claims.sub = user.id.to_string();
        let id = user.id.clone();
        let auth = AuthState::Authorized(claims);

        let mut mock = MockRepository::new();
        mock.expect_get_user().return_once(move |_| Ok(Some(user)));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.get_users(&[id], &auth).await.is_ok());
    }

    #[tokio::test]
    async fn have_authority_user_unauthorized_1() {
        let user: User = Faker.fake();

        let id = user.id.clone();
        let auth = AuthState::UnAuthorized;

        let mock = MockRepository::new();

        let usecase = UseCase::new(Arc::new(mock));
        assert!(!usecase.have_authority_user(&id, &auth).await);
    }

    #[tokio::test]
    async fn have_authority_user_unauthorized_2() {
        let user: User = Faker.fake();
        let claims: Claims = Faker.fake();

        let id = user.id.clone();
        let auth = AuthState::Authorized(claims);

        let mut mock = MockRepository::new();
        mock.expect_get_user().return_once(move |_| Ok(Some(user)));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(!usecase.have_authority_user(&id, &auth).await);
    }

    #[tokio::test]
    async fn have_authority_user_authorized() {
        let user: User = Faker.fake();
        let mut claims: Claims = Faker.fake();

        claims.sub = user.id.to_string();
        let id = user.id.clone();
        let auth = AuthState::Authorized(claims);

        let mut mock = MockRepository::new();
        mock.expect_get_user().return_once(move |_| Ok(Some(user)));

        let usecase = UseCase::new(Arc::new(mock));
        assert!(usecase.have_authority_user(&id, &auth).await);
    }
}
