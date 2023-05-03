use crate::models::post::BlogPost;

#[derive(Debug, Clone)]
pub struct InMemoryState {
    pub posts: Vec<BlogPost>,
}

impl InMemoryState {
    pub fn new() -> Self {
        Self { posts: vec![] }
    }

    pub fn add_post(&mut self, post: BlogPost) {
        self.posts.push(post);
    }

    pub fn get_post(&self, id: String) -> Option<&BlogPost> {
        return self.posts.iter().find(|post| post.id == id);
    }
}
