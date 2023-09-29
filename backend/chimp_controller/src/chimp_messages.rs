use chimp_protocol::{Request, Response};
use futures_util::{Stream, StreamExt, TryStreamExt};
use lapin::{
    message::Delivery,
    options::{BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions},
    protocol::basic::AMQPProperties,
    types::FieldTable,
    Channel, Connection, ConnectionProperties, Consumer,
};
use url::Url;
use uuid::Uuid;

pub async fn setup_rabbitmq_clients(
    rabbitmq_url: Url,
    job_channel: String,
) -> Result<(AMQPPublisher, PredictionConsumer), anyhow::Error> {
    let connection =
        Connection::connect(rabbitmq_url.as_str(), ConnectionProperties::default()).await?;

    let channel = connection.create_channel().await?;

    let reply_queue_id = Uuid::new_v4();
    channel
        .queue_declare(
            &reply_queue_id.to_string(),
            QueueDeclareOptions {
                exclusive: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    let consumer = channel
        .basic_consume(
            &reply_queue_id.to_string(),
            "chimp_controller",
            BasicConsumeOptions {
                no_ack: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    Ok((
        AMQPPublisher {
            channel,
            job_channel,
            reply_queue_id,
        },
        PredictionConsumer { consumer },
    ))
}

#[derive(Debug)]
pub struct AMQPPublisher {
    channel: Channel,
    job_channel: String,
    reply_queue_id: Uuid,
}

impl AMQPPublisher {
    pub async fn publish(&self, request: Request) -> Result<(), anyhow::Error> {
        self.channel
            .basic_publish(
                "",
                &self.job_channel,
                BasicPublishOptions::default(),
                &request.to_vec()?,
                AMQPProperties::default().with_reply_to(self.reply_queue_id.to_string().into()),
            )
            .await?
            .await?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct PredictionConsumer {
    consumer: Consumer,
}

impl PredictionConsumer {
    pub fn into_response_stream(self) -> impl Stream<Item = Result<Response, anyhow::Error>> {
        fn into_response(
            delivery: Result<Delivery, lapin::Error>,
        ) -> Result<Response, anyhow::Error> {
            let data = delivery?.data;
            println!("Delivery: {}", String::from_utf8(data.clone()).unwrap());
            Ok(Response::from_slice(&data)?)
        }

        self.consumer.into_stream().map(into_response)
    }
}
