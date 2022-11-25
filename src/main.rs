use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use std::{env, io};

mod api;
use api::task::get_task;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new().wrap(logger).service(get_task)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
