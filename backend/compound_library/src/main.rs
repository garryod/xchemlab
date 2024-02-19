#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![doc=include_str!("../README.md")]

/// This module defines the structure and schema of the database tables
/// through various entity structs.
mod entities;
/// This module sets up the GraphQL schema, including queries, mutations,
/// and subscriptions. It defines how data is queried and mutated through the API.
mod graphql;
/// This module is responsible for defining and applying database migrations.
mod migrator;

use async_graphql::extensions::Tracing;
use axum::{routing::get, Router, Server};
use clap::Parser;
use graphql::{root_schema_builder, RootSchema};
use graphql_endpoints::{GraphQLHandler, GraphQLSubscription, GraphiQLHandler};
use opa_client::OPAClient;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr, TransactionError};
use sea_orm_migration::MigratorTrait;
use std::{
    fs::File,
    io::Write,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    path::PathBuf,
};
use url::Url;

/// A commnd line interface for Compound Library Service.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
enum Cli {
    /// Starts the web server.
    Serve(ServeArgs),
    /// Saves the schema to a file if a file path is provided,
    /// else prints it to the terminal
    Schema(SchemaArgs),
}

/// Arguments for the `serve` command.
#[derive(Debug, Parser)]
struct ServeArgs {
    /// Port number of the server.
    #[arg(short, long, default_value_t = 80)]
    port: u16,
    /// Base URL for the database.
    #[arg(long, env)]
    database_url: Url,
    /// Database path
    #[arg(long, env, default_value = "compound_library")]
    database_path: String,
    /// URL for the OPA server
    #[arg(long, env)]
    opa_url: Url,
}

/// Arguments for the `schema` command.
#[derive(Debug, Parser)]
struct SchemaArgs {
    /// Specifies an optional path to the file to save the schema.
    #[arg(short, long)]
    path: Option<PathBuf>,
}

/// Sets up the database connection and performs the migrations.
/// Returns a `Result` with a `DatabaseConnection` on success,
/// or a `TransactionError<DbErr>` if connecting to the database or running
/// migrations fails.
async fn setup_database(
    db_base: Url,
    db_path: String,
) -> Result<DatabaseConnection, TransactionError<DbErr>> {
    let db_url = format!("{}/{}", db_base, db_path);
    let db_options = ConnectOptions::new(db_url);
    let db = Database::connect(db_options).await?;
    migrator::Migrator::up(&db, None).await?;
    Ok(db)
}

/// Sets up the router for handling GraphQL queries and subscriptions.
/// Returns a `Router` configured with routes .
fn setup_router(schema: RootSchema) -> Router {
    /// The endpoint for handling GraphQL queries and mutations.
    const GRAPHQL_ENDPOINT: &str = "/";
    /// The endpoint for establishing WebSocket connections for GraphQL subscriptions.
    const SUBSCRIPTION_ENDPOINT: &str = "/ws";

    Router::new()
        .route(
            GRAPHQL_ENDPOINT,
            get(GraphiQLHandler::new(
                GRAPHQL_ENDPOINT,
                SUBSCRIPTION_ENDPOINT,
            ))
            .post(GraphQLHandler::new(schema.clone())),
        )
        .route_service(SUBSCRIPTION_ENDPOINT, GraphQLSubscription::new(schema))
}

/// Starts a web server to handle HTTP requests as defined in the provided `router`.
async fn serve(router: Router, port: u16) {
    let socket_addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port));
    println!("GraphiQL IDE: {}", socket_addr);
    Server::bind(&socket_addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let args = Cli::parse();
    let tracing_subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(tracing_subscriber).unwrap();

    match args {
        Cli::Serve(args) => {
            let db = setup_database(args.database_url, args.database_path)
                .await
                .unwrap();
            let opa_client = OPAClient::new(args.opa_url);
            let schema = root_schema_builder()
                .data(db)
                .data(opa_client)
                .extension(Tracing)
                .finish();
            let router = setup_router(schema);
            serve(router, args.port).await;
        }
        Cli::Schema(args) => {
            let schema = root_schema_builder().finish();
            let schema_string = schema.sdl();
            if let Some(path) = args.path {
                let mut file = File::create(path).unwrap();
                file.write_all(schema_string.as_bytes()).unwrap();
            } else {
                println!("{}", schema_string);
            }
        }
    }
}
