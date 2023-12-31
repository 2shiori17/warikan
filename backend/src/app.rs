use crate::{
    auth,
    controllers::{graphiql, graphql, Query},
    repositories::{Mongo, MongoConfig, MongoError},
    usecases::UseCase,
};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::{routing::get, Router};
use clap::Parser;
use jsonwebtoken::jwk::JwkSet;
use shaku::{module, HasComponent};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use thiserror::Error;
use tokio::net::TcpListener;
use url::Url;

module! {
    pub Module {
        components = [Mongo],
        providers = [],
    }
}

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(long, env)]
    pub port: u16,

    #[arg(long, env)]
    pub auth0_issuer: Url,

    #[arg(long, env)]
    pub auth0_audience: String,

    #[arg(long, env)]
    pub mongo_uri: String,

    #[arg(long, env)]
    pub mongo_db: String,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("io")]
    Io(#[from] std::io::Error),

    #[error("reqwest")]
    Reqwest(#[from] reqwest::Error),

    #[error("mongo")]
    Mongo(#[from] MongoError),
}

#[derive(Clone)]
pub struct State {
    pub schema: Schema<Query, EmptyMutation, EmptySubscription>,
    pub jwks: JwkSet,
    pub auth0_audience: String,
}

pub struct App {
    pub args: Args,
}

impl App {
    pub fn new(args: Args) -> Self {
        Self { args }
    }

    pub async fn serve(self) -> Result<(), Error> {
        let Args {
            port,
            auth0_issuer,
            auth0_audience,
            mongo_uri,
            mongo_db,
        } = self.args;

        // Module
        let mongo = Mongo::new(MongoConfig {
            uri: &mongo_uri,
            database: &mongo_db,
        })
        .await?;
        let module = Module::builder()
            .with_component_override(Box::new(mongo))
            .build();

        // UseCase
        let usecase = UseCase::new(module.resolve());

        // Auth
        let jwks = auth::fetch_jwks(auth0_issuer).await?;

        // GraphQL
        let schema = Schema::build(Query::default(), EmptyMutation, EmptySubscription)
            .data(usecase)
            .finish();

        // Server
        let state = State {
            schema,
            jwks,
            auth0_audience,
        };
        let router = Router::new()
            .route("/", get(graphiql).post(graphql))
            .with_state(state);
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port);
        axum::serve(TcpListener::bind(addr).await?, router).await?;
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        let args = Args::parse();
        Self::new(args)
    }
}
