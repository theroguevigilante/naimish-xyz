#[derive(Clone)]
pub struct Post {
    pub kind: PostType,
    pub summary: PostSummary,
    pub content: Option<String>,
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
    pub fn kind_route(&self) -> &'static str {
        match self.kind {
            PostType::Article => "articles/",
            PostType::Blog => "blog/",
        }
    }
}

#[derive(Clone)]
pub struct Project {
    pub title: String,
    pub slug: String,
    pub date: Option<String>,
    pub description: Option<String>,
    pub repo: Option<String>,
    pub docs: Option<String>,
    pub tech: Option<String>,
    pub content: Option<String>,
}

impl Project {
    pub fn date_str(&self) -> &str {
        self.date.as_deref().unwrap_or("")
    }
}
