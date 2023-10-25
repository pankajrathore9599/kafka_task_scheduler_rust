use rdkafka::producer::{FutureProducer, FutureRecord};
use serde_json::Value;
use std::time::Duration;

pub async fn run_generator(brokers: &str) {
    let producer: FutureProducer = rdkafka::config::ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .create()
        .expect("Producer creation error");

    loop {
        // Simulating task generation (you can adjust this part as needed)
        let task = Value::String("This is a new task".into());

        // Convert task to string for sending as a message payload
        let task_str = serde_json::to_string(&task).expect("Failed to serialize task");

        let timeout = Duration::from_secs(5);  // Adjust as needed
        producer.send(FutureRecord::to("my_topic")
            .payload(&task_str)
            .key("some_key"), timeout)
            .await
            .expect("Failed to send message to Kafka");


        // Sleep to simulate a delay in generating tasks
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

#[tokio::main]
async fn main() {
    let brokers = "localhost:9092";
    run_generator(brokers).await;
}