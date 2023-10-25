# Kafka Task Scheduler in Rust

This project demonstrates a simple Kafka task scheduler implemented in Rust, leveraging the `rdkafka` crate. It consists of two main components:

1. **Generator**: Generates tasks and pushes them to a Kafka topic.
2. **Processor**: Consumes tasks from the Kafka topic and processes them.

## Prerequisites

- Rust (latest stable version)
- Kafka broker running on `localhost:9092` (or modify the broker string as needed)

## Setup

1. Clone the repository:
    ```bash
    git clone https://github.com/pankajrathore9599/kafka_task_scheduler_rust
    cd kafka_task_scheduler_rust
    ```

2. Build the project:
    ```bash
    cargo build
    ```

## Running the Components

1. Run the generator:
    ```bash
    cargo run --bin generator
    ```

2. Run the processor:
    ```bash
    cargo run --bin processor
    ```

## Testing

### Sending Tasks to Kafka

You can manually send tasks (in JSON format) to the `my_topic` Kafka topic using the Kafka console producer tool. For instance:

```/path/to/kafka/bin/kafka-console-producer.sh --broker-list localhost:9092 --topic my_topic ```



Replace /path/to/kafka with the path to your Kafka installation.

Then, input a task in JSON format, e.g.:

`{"task": "demo_task", "priority": "high"}`

## Monitoring Task Processing
While the processor is running, it will pick up and process the tasks you've sent via the Kafka console producer. The output will be displayed in the terminal where the processor is running.