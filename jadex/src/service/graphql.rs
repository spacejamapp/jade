//! GraphQL service for the Jadex runtime

use crate::config::{Cors, Graphql};
use async_graphql::{http::GraphiQLSource, ObjectType, Schema, SubscriptionType};
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{
    http::{header, HeaderValue, Method},
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use std::{any::Any, net::SocketAddr, time::Duration};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{Any as CorsAny, CorsLayer};

/// Start the GraphQL service
pub async fn start<Query, Mutation, Subscription, Data>(
    query: Query,
    mutation: Mutation,
    subscription: Subscription,
    data: Data,
    graphql: &Graphql,
) -> anyhow::Result<()>
where
    Query: ObjectType + 'static,
    Mutation: ObjectType + 'static,
    Subscription: SubscriptionType + 'static,
    Data: Send + Sync + Any,
{
    let schema = Schema::build(query, mutation, subscription)
        .data(data)
        .finish();

    // Configure CORS using the provided configuration
    let cors = graphql.cors.layer();
    let app = Router::new()
        .route(
            "/",
            get(graphiql).post_service(GraphQL::new(schema.clone())),
        )
        .route_service("/ws", GraphQLSubscription::new(schema))
        .layer(ServiceBuilder::new().layer(cors).into_inner());

    axum::serve(TcpListener::bind(graphql.graphql).await?, app).await?;
    Ok(())
}

async fn graphiql() -> impl IntoResponse {
    response::Html(
        GraphiQLSource::build()
            .endpoint("/")
            .subscription_endpoint("/ws")
            .finish(),
    )
}
