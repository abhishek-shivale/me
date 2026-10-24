# portfolio

Personal portfolio and blog of Abhishek Shivale, built with [Yew](https://yew.rs) (Rust + WebAssembly) and live at [abhishek.win](https://abhishek.win).

## Stack

- **Yew** + **yew-router**: client-side rendered UI compiled to WASM
- **Trunk**: build and dev server
- **build.rs**: turns `content/*.mdx` into blog posts at compile time (pulldown-cmark, Mermaid diagrams rendered to SVG)
- **prerender** binary: server-side renders every route into static HTML after each build, for SEO and fast first paint

## Layout

```
Resume.toml   all site data: personal info, navbar, experience, projects, skills
content/      blog posts (.mdx with front matter)
src/          Yew app: components/, pages/, utils/, renderer/
src/bin/      prerender binary (ssr feature)
web/          index.html and style.css
build.rs      blog content pipeline
```

## Develop

Requires Rust, the `wasm32-unknown-unknown` target, and Trunk.

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk serve        # http://127.0.0.1:8000
```

## Build

```sh
trunk build --release
```

Output goes to `dist/`. Trunk's post-build hook runs `cargo run --features ssr --bin prerender`, which writes prerendered HTML for each page into the output.

## Edit content

- **Profile, experience, projects:** edit `Resume.toml`. `{{YOE}}` in `personal.description` is replaced with computed years of experience.
- **Blog post:** add `content/<slug>.mdx` with front matter:

  ```yaml
  ---
  title: "My Post"
  publishedAt: "2026-01-01"
  summary: "One-line summary."
  ---
  ```

  The post is served at `/blog/<slug>`.
