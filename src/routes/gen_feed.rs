use crate::app::loader::{recent_posts, md_to_html, strip_front_matter};
use axum::{http::header, response::IntoResponse};
use chrono::NaiveDate;
use rss::{ChannelBuilder, GuidBuilder, ItemBuilder};

pub async fn handler() -> impl IntoResponse {
    let posts = recent_posts(); 

    let items: Vec<rss::Item> = posts
        .into_iter()
        .map(|post| {
            let link = format!(
                "https://naimish.xyz/{}{}", 
                post.kind_route(), 
                post.summary.slug
            );

            let guid = GuidBuilder::default()
                .value(link.clone())
                .permalink(true)
                .build();

            let mut item_builder = ItemBuilder::default();
            item_builder
                .title(Some(post.summary.title.clone()))
                .link(Some(link))
                .guid(Some(guid));

            if let Some(date) = post.summary.date.clone().and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()) {
                let datetime = date.and_hms_opt(0, 0, 0).unwrap().and_utc();
                item_builder.pub_date(Some(datetime.to_rfc2822()));
            }

            let file_path = format!("content/posts/{}{}.md", post.kind_route(), post.summary.slug);
            if let Ok(raw_text) = std::fs::read_to_string(&file_path) {
                let markdown_body = strip_front_matter(&raw_text);
                let html_content = md_to_html(markdown_body);
                
                item_builder.content(Some(html_content));
            }

            item_builder.build()
        })
        .collect();

    let channel = ChannelBuilder::default()
        .title("Naimish's site")
        .link("https://naimish.xyz")
        .description("No man, for any considerable period, can wear one face to himself and another to the multitude, without finally getting bewildered as to which may be the true")
        .items(items)
        .build();

    (
        [(header::CONTENT_TYPE, "application/rss+xml; charset=utf-8")],
        channel.to_string(),
    )
}
