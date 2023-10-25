use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::{BorrowedMessage, Message};
use serde_json::Value;
use futures::stream::StreamExt; 

pub async fn run_processor(brokers: &str) { 
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "task_processor_group")
        .set("bootstrap.servers", brokers)
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(&["my_topic"])
        .expect("Can't subscribe to specified topic");

    let mut message_stream = consumer.stream(); 
    while let Some(message) = message_stream.next().await {
        match message {
            Ok(message) => handle_message(&message),
            Err(err) => eprintln!("Error while receiving from Kafka: {:?}", err),
        }
    }
}

fn handle_message(message: &BorrowedMessage) {
    if let Some(payload) = message.payload_view::<str>() {
        match payload {
            Ok(payload_str) => {
                let task: Value = serde_json::from_str(payload_str).expect("Failed to deserialize task");
                // Process the task (dummy logic for now)
                println!("Received Task: {:?}", task);
            }
            Err(e) => println!("Error decoding message: {:?}", e),
        }
    }
}

#[tokio::main]
async fn main() {
    let brokers = "localhost:9092";
    run_processor(brokers).await;
}