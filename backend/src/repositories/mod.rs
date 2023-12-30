mod mongo;

pub use mongo::*;

use crate::entities::{Group, GroupID, Payment, PaymentID, User, UserID};

pub trait Repository: GroupRepository + PaymentRepository + UserRepository {}

pub trait GroupRepository {
    type Error: std::error::Error + 'static + Send + Sync;

    async fn get_group(&self, id: &GroupID) -> Result<Option<Group>, Self::Error>;
}

pub trait PaymentRepository {
    type Error: std::error::Error + 'static + Send + Sync;

    async fn get_payment(&self, id: &PaymentID) -> Result<Option<Payment>, Self::Error>;
    async fn get_payments_by_group(&self, group: &GroupID) -> Result<Vec<Payment>, Self::Error>;
}

pub trait UserRepository {
    type Error: std::error::Error + 'static + Send + Sync;

    async fn get_user(&self, id: &UserID) -> Result<Option<User>, Self::Error>;
}
