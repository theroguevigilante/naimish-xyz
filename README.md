# naimish.xyz

Personal website powered by Rust, Axum, Askama, and Comrak.

## Quick Start

```sh
echo "PORT=6432" > .env
cargo run
```

Open `http://localhost:6432`.

## Content

All content is markdown with front matter:

```
content/
├── description.txt      # site tagline (editable at runtime)
├── about.md             # homepage about section (editable at runtime)
├── posts/
│   ├── blog/            # blog posts
│   └── articles/        # articles
└── projects/            # project pages
```

Add a `.md` file to any post directory and it appears on the site — no recompile needed. `about.md` and `description.txt` are also read at runtime.

## Deploy

Compile once, run anywhere:

```sh
cargo build --release
./target/release/naimish-xyz
```

Set `PORT` environment variable to change the port (default: 6432).
