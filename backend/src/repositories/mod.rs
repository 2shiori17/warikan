mod mongo;

pub use mongo::*;

use crate::entities::{Group, GroupID, Payment, PaymentID, User, UserID};
use async_trait::async_trait;
use shaku::Interface;

#[async_trait]
pub trait Repository: GroupRepository + PaymentRepository + UserRepository {}

impl<T: GroupRepository + PaymentRepository + UserRepository> Repository for T {}

#[async_trait]
pub trait GroupRepository: Interface {
    async fn get_group(
        &self,
        id: &GroupID,
    ) -> Result<Option<Group>, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
pub trait PaymentRepository: Interface {
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
pub trait UserRepository: Interface {
    async fn get_user(
        &self,
        id: &UserID,
    ) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>>;
}
