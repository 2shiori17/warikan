mod group;
mod payment;
mod user;

use crate::repositories::Repository;
use thiserror::Error;

#[derive(Debug)]
pub struct UseCase<R: Repository> {
    pub repository: R,
}

pub type UseCaseResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Error)]
pub enum UseCaseError {
    #[error("not found")]
    NotFound,
}
