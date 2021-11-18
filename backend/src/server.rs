use actix_files::{Files, NamedFile};
use actix_session::{CookieSession, Session};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};

use crate::config::Conf;
use crate::error::{LSBError, Result};

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

async fn gen_rand(db: &PgPool) -> Result<i32> {
    let result: (i32,) = sqlx::query_as("SELECT floor((random() * 10) + 1)::int")
        .fetch_optional(db) // this works but fetch_one doesn't, wtf?
        .await?
        .unwrap();

    Ok(result.0)
}

fn update_count(session: Session, num: i32) -> Result<i32, LSBError> {
    if let Some(count) = session.get::<i32>("count")? {
        let total = count + num;
        session.insert("count", total)?;
        Ok(total)
    } else {
        session.insert("count", num)?;
        Ok(num)
    }
}

async fn api_index(session: Session, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = gen_rand(db_pool.get_ref()).await;
    match result {
        Ok(num) => {
            let sess = update_count(session, num);
            match sess {
                Ok(total) => HttpResponse::Ok()
                    .body(format!("random number is {}, total is {}\n", num, total)),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

async fn run_app(db_pool: Pool<Postgres>, conf: &Conf) -> Result<Server, LSBError> {
    let dev_env = conf.rust_env == "development";
    let db_pool = web::Data::new(db_pool);
    let cookie_expiry = conf.cookie_expiry;
    let frontend_path = conf.frontend_path.clone();

    let mut cookie_secret = vec![0; 32];
    cookie_secret.clone_from_slice(&*conf.cookie_secret);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                CookieSession::private(&cookie_secret)
                    .expires_in(cookie_expiry.clone())
                    .secure(!dev_env),
            )
            .wrap(Logger::default())
            .app_data(db_pool.clone())
            .route("/api", web::get().to(api_index))
            .service(
                Files::new("/", &frontend_path.clone())
                    .redirect_to_slash_directory()
                    .index_file("index.html")
                    .default_handler(
                        NamedFile::open(format!("{}/404.html", frontend_path)).unwrap(),
                    ),
            )
    })
    .bind(&conf.server_address)?
    .run();

    Ok(server)
}
