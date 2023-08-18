use {
    archive::{config, error, log},
    dotenv::dotenv,
    tokio::sync::broadcast,
};

#[tokio::main]
async fn main() -> error::Result<()> {
    let logger = log::Logger::init().expect("Failed to start logging");

    let (_signal, shutdown) = broadcast::channel(1);
    dotenv().ok();
    let config = config::get_config().expect(
        "Failed to load configuration, please ensure that all environment variables are defined.",
    );

    let options = archive::Options::default();
    let result = archive::bootstrap(shutdown, config, options).await;

    logger.stop();

    result
}
