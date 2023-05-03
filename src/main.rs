mod api;
mod models;

use crate::api::posts::{create_post, get_post_by_id, get_posts};
use actix_web::{middleware, web, App, HttpServer};
use env_logger::Env;
use std::sync::{Arc, Mutex};

const PORT: u16 = 8080;
const HOST: &str = "127.0.0.1";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://{}:{}", HOST, PORT);

    let db = web::Data::new(Arc::new(Mutex::new(
        models::in_memory_state::InMemoryState::new(),
    )));

    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(db.clone())
            .service(
                web::scope("/posts")
                    .service(get_posts)
                    .service(get_post_by_id)
                    .service(create_post),
            )
    })
    .bind((HOST, PORT))?
    .run()
    .await
}
