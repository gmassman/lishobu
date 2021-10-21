use crate::{config::Conf, error::LSBError};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct LSBServer {
    pub server: Server,
}

impl LSBServer {
    pub async fn build(config: Conf) -> Result<Self, LSBError> {
        let db_pool = connect_db(&config.pg_conn).await?;

        let server = run_app(db_pool, &config.server_address).await?;
        Ok(LSBServer { server })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn connect_db(pg_conn: &String) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect(pg_conn)
        .await
}

async fn index(
    db_pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let result: (i32,) = sqlx::query_as("SELECT floor(random() * 10)::int")
        .fetch_one(db_pool.get_ref())
        .await?;
    let response = format!("I'm a random number: {}\n", result.0);
    Ok(HttpResponse::Ok().body(response))
}

async fn run_app(db_pool: Pool<Postgres>, server_address: &str) -> Result<Server, LSBError> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(db_pool.clone())
            .route("/", web::get().to(index))
    })
    .bind(server_address)?
    .run();

    Ok(server)
}