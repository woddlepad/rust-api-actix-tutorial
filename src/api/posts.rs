use crate::models::{blog_post_store::BlogPostStore, post::BlogPost};

use actix_web::{get, http::StatusCode, post, web, HttpResponse, HttpResponseBuilder, Responder};
use actix_web_validator as validate;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, Clone, Validate, ToSchema)]
pub struct CreatePost {
    #[validate(length(min = 1, max = 280))]
    message: String,
}

#[derive(Deserialize, Validate)]
pub struct GetPostById {
    #[validate(length(min = 1, max = 280), custom = "validate_uuid")]
    post_id: String,
}

pub fn validate_uuid(uuid: &str) -> Result<(), validator::ValidationError> {
    return match Uuid::parse_str(uuid) {
        Ok(_) => Ok(()),
        Err(_) => Err(validator::ValidationError::new("Invalid UUID")),
    };
}

pub struct JSONErrorRepsonse {
    status: StatusCode,
    message: String,
}

impl JSONErrorRepsonse {
    pub fn new(status: StatusCode, message: &str) -> JSONErrorRepsonse {
        return JSONErrorRepsonse {
            status,
            message: message.to_string(),
        };
    }

    pub fn build(&self) -> HttpResponse {
        return HttpResponseBuilder::new(self.status)
            .status(self.status)
            .json(json!({ "error": self.message }));
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "Create a post")
    )
)]
#[post("/")]
pub(super) async fn create_post(
    post_store: web::Data<BlogPostStore>,
    form_data: validate::Json<CreatePost>,
) -> impl Responder {
    let post = BlogPost::new(form_data.message.clone());
    post_store.add_post(post);
    return HttpResponse::Ok().json(post_store.posts.lock().unwrap().clone());
}

#[utoipa::path(
    responses(
        (status = 200, description = "Get all posts")
    )
)]
#[get("/")]
pub(super) async fn get_posts(post_store: web::Data<BlogPostStore>) -> impl Responder {
    let posts = post_store.get_posts();
    return HttpResponse::Ok().json(posts);
}

#[utoipa::path(
    responses(
        (status = 200, description = "Get post by id"),
        (status = 404, description = "Post not found"),
    )
)]
#[get("/{post_id}")]
pub(super) async fn get_post_by_id(
    post_store: web::Data<BlogPostStore>,
    path_info: validate::Path<GetPostById>,
) -> impl Responder {
    let post_id = path_info.post_id.clone();

    let post = post_store.get_post(post_id);
    if post.is_none() {
        return JSONErrorRepsonse::new(StatusCode::NOT_FOUND, "Post not found").build();
    }
    return HttpResponse::Ok().json(post.unwrap());
}
