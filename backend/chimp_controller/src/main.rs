#![forbid(unsafe_code)]
#![doc=include_str!("../README.md")]

mod chimp_messages;
mod queries;
pub mod schemas;
mod spawner;

use crate::{
    chimp_messages::setup_rabbitmq_clients,
    queries::{
        create_prediction::{
            CreatePredictionMutation, CreatePredictionVariables, CrystalInput, DropInput, WellInput,
        },
        image_created::Image,
    },
    spawner::Spawner,
};
use async_tungstenite::tungstenite::{client::IntoClientRequest, http::HeaderValue, Message};
use chimp_protocol::{Request, Response};
use clap::Parser;
use cynic::{http::ReqwestExt, MutationBuilder, SubscriptionBuilder};
use futures_util::StreamExt;
use graphql_ws_client::{graphql::Cynic, AsyncWebsocketClient, CynicClientBuilder};
use queries::image_created::ImageCreatedSubscription;
use reqwest::{Client, Method};
use tokio::select;
use url::Url;

async fn setup_targeting_subscription_client(
    targeting_url: Url,
) -> Result<AsyncWebsocketClient<Cynic, Message>, anyhow::Error> {
    let mut request = targeting_url.into_client_request()?;
    request.headers_mut().insert(
        "Sec-WebSocket-Protocol",
        HeaderValue::from_static("graphql-transport-ws"),
    );
    request.headers_mut().insert(
        "Authorization",
        HeaderValue::from_static("Bearer ValidToken"),
    );

    let (connection, _) = async_tungstenite::tokio::connect_async(request).await?;
    let (sink, stream) = connection.split();

    Ok(CynicClientBuilder::new()
        .build(stream, sink, Spawner::new())
        .await?)
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    /// The URL of the Targeting service GraphQL query endpoint.
    targeting_url: Url,
    /// The URL of the Targeting service GraphQL subscription endpoint.
    targeting_subscription_url: Url,
    /// The URL of the RabbitMQ server.
    rabbitmq_url: Url,
    /// The RabbitMQ queue on which jobs are assigned.
    rabbitmq_channel: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let args = Cli::parse();

    let mut targeting_subscription_client =
        setup_targeting_subscription_client(args.targeting_subscription_url.clone())
            .await
            .unwrap();
    let subscription = ImageCreatedSubscription::build(());
    let mut image_creation_stream = targeting_subscription_client
        .streaming_operation(subscription)
        .await
        .unwrap();

    let (job_publsiher, job_consumer) =
        setup_rabbitmq_clients(args.rabbitmq_url, args.rabbitmq_channel)
            .await
            .unwrap();

    let mut response_stream = job_consumer.into_response_stream();

    let targeting_client = Client::new();

    loop {
        select! {
            Some(image_created) = image_creation_stream.next() => {
                println!("Image created! {image_created:?}");
                let Image {plate, well, download_url} = image_created.unwrap().data.unwrap().image_created;
                let request = Request {
                    plate,
                    well,
                    download_url,
                };
                job_publsiher.publish(request).await.unwrap();
            },

            Some(response) = response_stream.next() => {
                println!("Response recieved: {response:?}");
                let response = response.unwrap();
                if let Response::Success {plate, well, insertion_point, well_location, drop, crystals} = response {
                    let variables = CreatePredictionVariables {
                        plate: WellInput { plate , well},
                        well_centroid: well_location.center.try_into().unwrap(),
                        well_radius: well_location.radius,
                        drops: vec![DropInput {
                            crystals: crystals.into_iter().map(|crystal| CrystalInput {
                                bounding_box: crystal.try_into().unwrap()
                            }).collect(),
                            bounding_box: drop.try_into().unwrap(),
                            insertion_point: insertion_point.try_into().unwrap()
                        }],
                    };
                    let mutation = CreatePredictionMutation::build(variables);
                    let response = targeting_client.request(Method::POST, args.targeting_url.clone()).header("Authorization", "Bearer ValidToken").run_graphql(mutation).await.unwrap();
                    if let Some(errs) = response.errors {
                        panic!("Targeting service returned error(s): {errs:?}");
                    }
                }
            }
        }
    }
}
