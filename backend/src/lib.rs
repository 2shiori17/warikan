#![allow(async_fn_in_trait)]

pub mod app;
pub mod auth;
pub mod controllers;
pub mod entities;
pub mod repositories;
pub mod usecases;

pub use app::{App, Args, Error};
