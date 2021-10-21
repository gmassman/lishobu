use lishobu::config::get_config;
use lishobu::server::LSBServer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("Failed to load configuration");

    let lsb_server = LSBServer::build(config).await.expect("Failed to build the application");
    lsb_server.run().await.expect("Failed while running the server");
    Ok(())
}
