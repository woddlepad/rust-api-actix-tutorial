mod api;
mod models;

use crate::api::posts::{create_post, get_post_by_id, get_posts, CreatePost};
use crate::models::post::BlogPost;
use actix_web::{middleware, web, App, HttpServer};
use env_logger::Env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

const PORT: u16 = 8080;
const HOST: &str = "127.0.0.1";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://{}:{}", HOST, PORT);

    let db = web::Data::new(models::blog_post_store::BlogPostStore::new());

    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    #[derive(OpenApi)]
    #[openapi(
        paths(
            api::posts::get_posts,
            api::posts::get_post_by_id,
            api::posts::create_post
        ),
        components(schemas(BlogPost, CreatePost))
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(db.clone())
            .service(
                web::scope("/api/v1").service(
                    web::scope("/posts")
                        .service(get_posts)
                        .service(get_post_by_id)
                        .service(create_post),
                ),
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind((HOST, PORT))?
    .run()
    .await
}
