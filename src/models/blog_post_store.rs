use std::sync::Mutex;

use crate::models::post::BlogPost;

#[derive(Debug)]
pub struct BlogPostStore {
    pub posts: Mutex<Vec<BlogPost>>,
}

impl BlogPostStore {
    pub fn new() -> Self {
        Self {
            posts: Mutex::new(vec![]),
        }
    }

    pub fn add_post(&self, post: BlogPost) {
        self.posts.lock().unwrap().push(post);
    }

    pub fn get_post(&self, id: String) -> Option<BlogPost> {
        return self
            .posts
            .lock()
            .unwrap()
            .iter()
            .find(|post| post.id == id)
            .map(|post| post.clone());
    }

    pub fn get_posts(&self) -> Vec<BlogPost> {
        return self.posts.lock().unwrap().clone();
    }
}
