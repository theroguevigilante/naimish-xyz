use super::types::*;
use std::fs;

fn sort_by_date(mut posts: Vec<Post>) -> Vec<Post> {
    posts.sort_by(|a, b| b.summary.date.cmp(&a.summary.date)); // newest first
    posts
}

pub fn strip_front_matter(content: &str) -> &str {
    let parts: Vec<&str> = content.splitn(3, "---").collect();

    if parts.len() >= 3 {
        parts[2].trim_start()
    } else {
        content
    }
}

pub fn get_post(slug: &str) -> Option<Post> {
    let (kind, clean_slug) = slug.split_once('/').unwrap_or(("", slug));

    let path = format!("content/posts/{}/{}.md", kind, clean_slug);
    println!("loading file: {}", path);

    let posts = match kind {
        "articles" => recent_articles(),
        "blog"     => recent_blogs(),
        _ => return None,
    };

    let mut post = posts
        .into_iter()
        .inspect(|p| println!("checking slug: {}", p.summary.slug))
        .find(|p| p.summary.slug == clean_slug)?;
    let content = std::fs::read_to_string(&path).ok()?;
    post.content = Some(content);

    Some(post)
}


fn extract_field(text: &str, key: &str) -> Option<String> {
    text.lines()
        .find(|line| line.trim_start().starts_with(key))
        .map(|line| line.replacen(key, "", 1).trim().to_string())
}

pub fn recent_posts() -> Vec<Post> {
    let mut posts = recent_blogs();
    posts.extend(recent_articles());
    sort_by_date(posts)
}

pub fn recent_articles() -> Vec<Post> {
    let mut posts = Vec::new();

    if let Ok(entries) = fs::read_dir("content/posts/articles") {
        for entry in entries.flatten() {
            if entry.path().extension().and_then(|e| e.to_str()) == Some("md") {
                let slug = entry
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                let text = fs::read_to_string(entry.path()).unwrap_or_default();

                let title = extract_field(&text, "title:").unwrap_or_else(|| slug.clone());
                let date = extract_field(&text, "date:");

                posts.push(Post {
                    kind: PostType::Article,
                    summary: PostSummary { title, slug, date },
                    content: None,
                });
            }
        }
    }

    sort_by_date(posts)
}

pub fn recent_blogs() -> Vec<Post> {
    let mut posts = Vec::new();

    //blogs
    if let Ok(entries) = fs::read_dir("content/posts/blog") {
        for entry in entries.flatten() {
            if entry.path().extension().and_then(|e| e.to_str()) == Some("md") {
                let slug = entry
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                let text = fs::read_to_string(entry.path()).unwrap_or_default();

                let title = extract_field(&text, "title:").unwrap_or_else(|| slug.clone());
                let date = extract_field(&text, "date:");

                posts.push(Post {
                    kind: PostType::Blog,
                    summary: PostSummary { title, slug, date },
                    content: None,
                });
            }
        }
    }

    sort_by_date(posts)
}
