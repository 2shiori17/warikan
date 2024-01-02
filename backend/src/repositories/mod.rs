mod mongo;

pub use mongo::*;

use crate::entities::{Group, GroupID, Payment, PaymentID, User, UserID};
use async_trait::async_trait;
use shaku::Interface;

#[cfg(test)]
use mockall::predicate::*;
#[cfg(test)]
use mockall::*;

#[async_trait]
pub trait Repository: GroupRepository + PaymentRepository + UserRepository {}

impl<T: GroupRepository + PaymentRepository + UserRepository> Repository for T {}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait GroupRepository: Interface {
    async fn create_group(
        &self,
        group: Group,
    ) -> Result<Group, Box<dyn std::error::Error + Send + Sync>>;

    async fn delete_group(
        &self,
        id: &GroupID,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    async fn get_group(
        &self,
        id: &GroupID,
    ) -> Result<Option<Group>, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait PaymentRepository: Interface {
    async fn create_payment(
        &self,
        payment: Payment,
    ) -> Result<Payment, Box<dyn std::error::Error + Send + Sync>>;

    async fn delete_payment(
        &self,
        id: &PaymentID,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    async fn get_payment(
        &self,
        id: &PaymentID,
    ) -> Result<Option<Payment>, Box<dyn std::error::Error + Send + Sync>>;

    async fn get_payments_by_group(
        &self,
        group: &GroupID,
    ) -> Result<Vec<Payment>, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait UserRepository: Interface {
    async fn create_user(
        &self,
        user: User,
    ) -> Result<User, Box<dyn std::error::Error + Send + Sync>>;

    async fn delete_user(
        &self,
        id: &UserID,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    async fn get_user(
        &self,
        id: &UserID,
    ) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>>;
}

#[cfg(test)]
mock! {
    pub Repository {}

    #[async_trait]
    impl GroupRepository for Repository {
        async fn create_group(
            &self,
            group: Group
        ) -> Result<Group, Box<dyn std::error::Error + Send + Sync>>;

        async fn delete_group(
            &self,
            id: &GroupID
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

        async fn get_group(
            &self,
            id: &GroupID,
        ) -> Result<Option<Group>, Box<dyn std::error::Error + Send + Sync>>;
    }

    #[async_trait]
    impl PaymentRepository for Repository {
        async fn create_payment(
            &self,
            payment: Payment,
        ) -> Result<Payment, Box<dyn std::error::Error + Send + Sync>>;

        async fn delete_payment(
            &self,
            id: &PaymentID,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

        async fn get_payment(
            &self,
            id: &PaymentID,
        ) -> Result<Option<Payment>, Box<dyn std::error::Error + Send + Sync>>;

        async fn get_payments_by_group(
            &self,
            group: &GroupID,
        ) -> Result<Vec<Payment>, Box<dyn std::error::Error + Send + Sync>>;
    }

    #[async_trait]
    impl UserRepository for Repository {
        async fn create_user(
            &self,
            user: User,
        ) -> Result<User, Box<dyn std::error::Error + Send + Sync>>;

        async fn delete_user(
            &self,
            id: &UserID,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

        async fn get_user(
            &self,
            id: &UserID,
        ) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>>;
    }
}
