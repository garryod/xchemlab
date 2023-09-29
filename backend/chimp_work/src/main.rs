use std::str::FromStr;

use chimp_protocol::{Request, Response};
use clap::Parser;
use futures_lite::StreamExt;
use lapin::{
    options::{BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions},
    protocol::basic::AMQPProperties,
    types::FieldTable,
    ConnectionProperties,
};
use url::Url;
use uuid::Uuid;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    /// The URL of the RabbitMQ server.
    rabbitmq_url: Url,
    /// The RabbitMQ channel on which jobs are assigned.
    rabbitmq_channel: String,
    /// The number of jobs to submit.
    jobs: usize,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let args = Cli::parse();

    let rabbitmq_client =
        lapin::Connection::connect(args.rabbitmq_url.as_str(), ConnectionProperties::default())
            .await
            .unwrap();
    let rabbitmq_channel = rabbitmq_client.create_channel().await.unwrap();

    let reply_channel = Uuid::new_v4();
    rabbitmq_channel
        .queue_declare(
            &reply_channel.to_string(),
            QueueDeclareOptions {
                exclusive: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await
        .unwrap();
    println!("Reply on: {reply_channel}");

    for _ in 0..args.jobs {
        let job = Request {
            id: Uuid::new_v4(),
            download_url: Url::from_str("http://s3:4566/xchemlab-targeting/01234567-89ab-cdef-0123-456789abcdef/42?x-id=GetObject&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=%2F20230927%2Fundefined%2Fs3%2Faws4_request&X-Amz-Date=20230927T122113Z&X-Amz-Expires=600&X-Amz-SignedHeaders=host&X-Amz-Signature=5b3a9b4ed39df868f21cea5625cd92eac2d464bae5305eee3eab0051bbe339c6").unwrap(),
        };
        println!("Sending Job: {job:?}");
        rabbitmq_channel
            .basic_publish(
                "",
                &args.rabbitmq_channel,
                BasicPublishOptions::default(),
                &job.to_vec().unwrap(),
                AMQPProperties::default().with_reply_to(reply_channel.to_string().into()),
            )
            .await
            .unwrap()
            .await
            .unwrap();
    }

    let mut consumer = rabbitmq_channel
        .basic_consume(
            &reply_channel.to_string(),
            "work",
            BasicConsumeOptions {
                no_ack: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await
        .unwrap();

    for _ in 0..args.jobs {
        let delivery = consumer.next().await.unwrap().unwrap();
        let response = Response::from_slice(&delivery.data).unwrap();
        println!("Got Response: {response:?}");
    }
}
