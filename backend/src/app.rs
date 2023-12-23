use crate::{auth, graphql};
use async_graphql::{EmptySubscription, Schema};
use axum::{routing::get, Router};
use clap::Parser;
use jsonwebtoken::jwk::JwkSet;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use thiserror::Error;
use tokio::net::TcpListener;
use url::Url;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(long, env)]
    pub port: u16,

    #[arg(long, env)]
    pub auth0_issuer: Url,

    #[arg(long, env)]
    pub auth0_audience: String,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("io")]
    Io(#[from] std::io::Error),

    #[error("reqwest")]
    Reqwest(#[from] reqwest::Error),
}

#[derive(Clone)]
pub struct State {
    pub schema: Schema<graphql::Query, graphql::Mutation, EmptySubscription>,
    pub jwks: JwkSet,
    pub auth0_audience: String,
}

#[derive(Debug)]
pub struct App {
    pub args: Args,
}

impl App {
    pub fn new(args: Args) -> Self {
        App { args }
    }

    pub async fn serve(self) -> Result<(), Error> {
        let Args {
            port,
            auth0_issuer,
            auth0_audience,
        } = self.args;

        // Auth
        let jwks = auth::fetch_jwks(auth0_issuer).await?;

        // GraphQL
        let schema = Schema::build(
            graphql::Query::default(),
            graphql::Mutation::default(),
            EmptySubscription,
        )
        .finish();

        // Server
        let state = State {
            schema,
            jwks,
            auth0_audience,
        };
        let router = Router::new()
            .route("/", get(graphql::graphiql).post(graphql::graphql))
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
