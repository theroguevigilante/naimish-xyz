#[derive(Clone)]
pub struct Post {
    pub kind: PostType,
    pub summary: PostSummary,
}

#[derive(Clone)]
pub enum PostType {
    Blog,
    Article,
}

#[derive(Clone)]
pub struct PostSummary {
    pub title: String,
    pub slug: String,
    pub date: Option<String>,
}
