//! Stores header data in postgres

use async_graphql::{Context, EmptyMutation, EmptySubscription, Object};
use jadex::{
    config::{Config, Graphql, Node},
    service,
};
use sqlx::{Executor, PgPool};
use tracing_subscriber::EnvFilter;

/// My runtime hook for block headers
#[derive(Clone)]
struct HeaderHook(PgPool);

impl runtime::Hook for HeaderHook {
    async fn on_finalized_block(&self, block: score::Block) -> anyhow::Result<()> {
        self.0
            .execute(sqlx::query!(
                "INSERT INTO headers (hash) VALUES ($1)",
                hex::encode(block.header.hash()?)
            ))
            .await?;
        Ok(())
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn headers(&self, ctx: &Context<'_>) -> Vec<String> {
        let pool = ctx.data::<HeaderHook>().unwrap();
        let headers: Vec<String> = sqlx::query_scalar!("SELECT hash FROM headers")
            .fetch_all(&pool.0)
            .await
            .unwrap();
        headers
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Config {
        node: Node {
            quic: "0.0.0.0:6888".parse()?,
            spec: None,
            data: "headers".into(),
        },
        graphql: Graphql {
            graphql: "0.0.0.0:3000".parse()?,
            cors: Default::default(),
        },
    };

    let pool = PgPool::connect("postgres://postgres:postgres@localhost/headers").await?;
    let hook = HeaderHook(pool);

    tokio::select! {
        r = service::node::dev(&config, hook.clone()) => r,
        r = service::graphql::start(
            QueryRoot,
            EmptyMutation,
            EmptySubscription,
            hook,
            &config.graphql,
        ) => r,
        _ = tokio::signal::ctrl_c() => Ok(()),
    }
}
