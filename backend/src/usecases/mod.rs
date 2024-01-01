mod group;
mod payment;
mod user;

use crate::repositories::Repository;
use std::sync::Arc;
use thiserror::Error;

pub struct UseCase {
    pub repository: Arc<dyn Repository>,
}

impl UseCase {
    pub fn new(repository: Arc<dyn Repository>) -> Self {
        Self { repository }
    }
}

#[derive(Debug, Error)]
pub enum UseCaseError {
    #[error("not found")]
    NotFound,

    #[error("unauthorized")]
    UnAuthorized,
}
