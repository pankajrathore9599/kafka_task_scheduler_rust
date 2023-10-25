mod generator;
mod processor;

#[tokio::main]
async fn main() {
    let brokers = "localhost:9092";

    // Using join! to await completion of both generator and processor.
    // This ensures the main function doesn't exit until both tasks are complete.
    tokio::join!(
        async {
            generator::run_generator(brokers).await;
        },
        async {
            processor::run_processor(brokers).await;
        }
    );
}
