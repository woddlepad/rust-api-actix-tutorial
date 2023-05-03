use crate::models::{in_memory_state::InMemoryState, post::BlogPost};

use actix_web::{get, http::StatusCode, post, web, HttpResponse, HttpResponseBuilder, Responder};
use actix_web_validator as validate;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, Clone, Validate)]
pub struct Createpost {
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

#[post("/")]
pub async fn create_post(
    memdb: web::Data<Arc<Mutex<InMemoryState>>>,
    form_data: validate::Json<Createpost>,
) -> impl Responder {
    let post = BlogPost::new(form_data.message.clone());
    let mut guard = memdb.lock().unwrap();
    guard.add_post(post);
    return HttpResponse::Ok().json(guard.posts.clone());
}

#[get("/")]
pub async fn get_posts(memdb: web::Data<Arc<Mutex<InMemoryState>>>) -> impl Responder {
    let guard = memdb.lock().unwrap();
    return HttpResponse::Ok().json(&guard.posts);
}

#[get("/{post_id}")]
pub async fn get_post_by_id(
    memdb: web::Data<Arc<Mutex<InMemoryState>>>,
    path_info: validate::Path<GetPostById>,
) -> impl Responder {
    let guard = memdb.lock().unwrap();
    let post_id = path_info.post_id.clone();

    let post = guard.get_post(post_id);
    if post.is_none() {
        return JSONErrorRepsonse::new(StatusCode::NOT_FOUND, "post not found").build();
    }
    return HttpResponse::Ok().json(post.unwrap());
}
