//use actix_web::{get, web, App, HttpServer, Responder};
//use std::cell::Cell;
//use std::sync::atomic::{AtomicUsize, Ordering};
//use std::sync::Arc;

//#[derive(Clone)]
//struct AppState {
//local_count: Cell<usize>,
//global_count: Arc<AtomicUsize>,
//}

//#[get("/add")]
//async fn add_one(data: web::Data<AppState>) -> impl Responder {
//data.global_count.fetch_add(1, Ordering::Relaxed);

//let local_count = data.local_count.get();
//data.local_count.set(local_count + 1);

//println!("adding...");
//format!(
//"global_count: {}\nlocal_count: {}",
//data.global_count.load(Ordering::Relaxed),
//data.local_count.get()
//)
//}

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    log::debug!("recompiling!");
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
