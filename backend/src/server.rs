use crate::{config::Conf, error::LSBError};
use actix_session::CookieSession;
//use actix_session::{CookieSession, Session};
use actix_web::dev::Server;
use actix_web::middleware::{Condition, DefaultHeaders, Logger};
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct LSBServer {
    pub server: Server,
}

impl LSBServer {
    pub async fn build(config: &Conf) -> Result<Self, LSBError> {
        let db_pool = connect_db(&config.pg_conn).await?;

        let server = run_app(db_pool, config).await?;
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

async fn index(db_pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let result: Result<(i32,), sqlx::Error> =
        sqlx::query_as("SELECT floor((random() * 10) + 1)::int")
            .fetch_one(db_pool.get_ref())
            .await;
    match result {
        Ok(num) => HttpResponse::Ok().body(format!("random number is {}", num.0)),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

async fn run_app(db_pool: Pool<Postgres>, conf: &Conf) -> Result<Server, LSBError> {
    let dev_env = conf.rust_env == "development";
    let db_pool = web::Data::new(db_pool);

    let mut cookie_secret = vec![0; 32];
    cookie_secret.clone_from_slice(&*conf.cookie_secret);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Condition::new(
                dev_env,
                DefaultHeaders::new().header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
            ))
            .wrap(CookieSession::private(&cookie_secret).secure(!dev_env))
            .app_data(db_pool.clone())
            .route("/", web::get().to(index))
    })
    .bind(&conf.server_address)?
    .run();

    Ok(server)
}
