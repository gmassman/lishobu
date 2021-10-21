use lishobu::config::get_config;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};

async fn index(
    db_pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let result: (i32,) = sqlx::query_as("SELECT floor(random() * 10)::int")
        .fetch_one(db_pool.get_ref())
        .await?;
    let response = format!("I'm a random number: {}\n", result.0);
    Ok(HttpResponse::Ok().body(response))
}

async fn connect_db(pg_conn: &String) -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect(pg_conn)
        .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("Failed to load configuration");

    let db_pool = connect_db(&config.pg_conn)
        .await
        .expect("Failed to connect to the database");
    let db_pool = web::Data::new(db_pool);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(db_pool.clone())
            .route("/", web::get().to(index))
    })
    .bind(&config.server_address)?
    .run()
    .await
}
