#[derive(Clone)]
pub struct Post {
    pub kind: PostType,
    pub summary: PostSummary,
    pub content: Option<String>
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

impl PostSummary {
    pub fn date_str(&self) -> &str {
        self.date.as_deref().unwrap_or("")
    }
}

impl Post {
    pub fn kind_prefix(&self) -> &'static str {
        match self.kind {
            PostType::Article => "* ",
            PostType::Blog => "",
        }
    }
}
