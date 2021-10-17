use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer};
use env_logger::Env;

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("I'm a response\n")
}

fn server_address() -> String {
    std::env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".into())
}

async fn connect_db() -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    let pg_conn =
        std::env::var("PG_CONN").expect("PG_CONN must be present to connect to the database");

    sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect(&pg_conn)
        .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let pool = connect_db()
        .await
        .expect("Failed to connect to the database");

    HttpServer::new(|| {
        App::new().wrap(Logger::default()).service(index)
        //.data(data.clone())
        //.service(show_count)
        //.service(add_one)
    })
    .bind(server_address())?
    .run()
    .await
}
