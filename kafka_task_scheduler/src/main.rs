mod generator;
mod processor;

#[tokio::main]
async fn main() {
    let brokers = "localhost:9092";
    
    tokio::join!(
        async {
            generator::run_generator(brokers).await;
        },
        async {
            processor::run_processor(brokers).await;
        }
    );
}
