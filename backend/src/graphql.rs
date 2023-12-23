use crate::{app, auth};
use async_graphql::{http::GraphiQLSource, MergedObject, Object};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::{self, IntoResponse},
};

#[derive(Default, MergedObject)]
pub struct Query(Greet);

#[derive(Default, MergedObject)]
pub struct Mutation();

pub async fn graphql(
    State(state): State<app::State>,
    auth: auth::AuthState,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let req = req.into_inner();
    dbg!(auth);
    state.schema.execute(req).await.into()
}

pub async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[derive(Default)]
pub struct Greet;

#[Object]
impl Greet {
    async fn greet(&self, name: String) -> String {
        format!("Hello, {}!", name)
    }
}
