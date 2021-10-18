use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use env_logger::Env;

async fn index(
    db_pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let result: (i32,) = sqlx::query_as("SELECT floor(random() * 10)::int")
        .fetch_one(db_pool.get_ref())
        .await?;
    let response = format!("I'm a random number: {}\n", result.0);
    Ok(HttpResponse::Ok().body(response))
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

    let db_pool = connect_db()
        .await
        .expect("Failed to connect to the database");
    let db_pool = web::Data::new(db_pool);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(db_pool.clone())
            .route("/", web::get().to(index))
        //.data(data.clone())
        //.service(show_count)
        //.service(add_one)
    })
    .bind(server_address())?
    .run()
    .await
}
