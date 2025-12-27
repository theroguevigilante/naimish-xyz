# naimish-xyz

**naimish-xyz** is the crate powering the public website at `naimish.xyz`.  
It is a self-hosted, markdown-driven site engine built with:

- **Axum** for routing and HTTP handling
- **Askama** for server-side HTML templates
- **Comrak** for Markdown → HTML conversion
- **RSS feed support** (planned full feed for articles + blogs)
- **Multiple client targets** (HTML, terminal/cURL mode planned)
- **Slug routing for `/blog/:slug` and `/article/:slug`**

The goal is to provide a clean, Unix-style architecture: minimal dependencies, readable structure, and an engine that is small enough to understand but flexible enough to extend.


## Features (Current & In-Progress)

- Markdown content with front matter
- Render to HTML using Askama
- Structured post lookup by slug
- Separation of _engine code_ vs _personal content_
- Retro CRT aesthetic via pure CSS
- Planned cURL/terminal-friendly output mode
- Planned `/rss.xml` full-feed generation

### Planned:
- **Docker container image** for deployment
- Fast “static site export” mode
- JSON output endpoints
- Optional caching layer for Markdown parsing

> The Docker plan will allow users to run the entire site with:
> ```bash
> docker run -p 3000:3000 naimish-xyz
> ```
> The container will ship with the engine, templates, and configuration, but the `/content` directory will remain mountable from outside to preserve personal ownership of written work.

## Licensing

This project uses a split-license model.

| Component                          | License                             |
|------------------------------------|--------------------------------------|
| Engine code & crate                | **AGPL-3.0** (free & copyleft)      |
| Templates / routing / utilities    | **AGPL-3.0**                        |
| CSS / static files                 | **AGPL-3.0** unless stated otherwise|
| `content/` directory (your posts)  | **Personal Copyright / All Rights Reserved** |

The **engine is copyleft**, but the **content is not**.  
You may host and modify the engine freely under AGPL, but the written content (`/content`) remains owned by its author.

## Running Locally

```bash
cargo run
