#[derive(Clone)]
pub struct Post(pub PostType, pub PostSummary);

#[derive(Clone)]
pub enum PostType {
    Blog,
    Article,
}

#[derive(Clone)]
pub struct PostSummary {
    pub title: String,
    pub slug: String,
    pub kind: PostType,
    pub date: Option<String>,
}
