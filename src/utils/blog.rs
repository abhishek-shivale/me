pub struct Post {
    pub slug: &'static str,
    pub title: &'static str,
    pub published_at: &'static str,
    pub summary: &'static str,
    pub html: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/posts.rs"));

pub fn find(slug: &str) -> Option<&'static Post> {
    POSTS.iter().find(|post| post.slug == slug)
}
