use std::{env, fs, path::Path};

use gray_matter::{Matter, Pod, engine::YAML};
use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd, html};

const CONTENT_DIR: &str = "content";

struct FrontMatter {
    title: String,
    published_at: String,
    summary: String,
}

fn main() {
    println!("cargo:rerun-if-changed={CONTENT_DIR}");

    let mut posts = Vec::new();
    for entry in fs::read_dir(CONTENT_DIR).expect("missing content/ directory") {
        let path = entry.expect("unreadable content/ entry").path();
        if path.extension().is_none_or(|ext| ext != "mdx") {
            continue;
        }
        let slug = path.file_stem().unwrap().to_string_lossy().into_owned();
        let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("{slug}: {e}"));
        let parsed = Matter::<YAML>::new()
            .parse::<Pod>(&text)
            .unwrap_or_else(|e| panic!("{slug}: bad front matter: {e}"));
        let data = parsed
            .data
            .unwrap_or_else(|| panic!("{slug}: missing front matter"));
        let field = |key: &str| {
            data[key]
                .as_string()
                .unwrap_or_else(|_| panic!("{slug}: front matter needs a `{key}` string"))
        };
        let meta = FrontMatter {
            title: field("title"),
            published_at: field("publishedAt"),
            summary: field("summary"),
        };
        let body = to_html(&parsed.content, &slug);
        posts.push((meta, slug, body));
    }
    posts.sort_by(|a, b| b.0.published_at.cmp(&a.0.published_at));

    let mut out = String::from("pub const POSTS: &[Post] = &[\n");
    for (meta, slug, body) in &posts {
        out += &format!(
            "    Post {{ slug: {slug:?}, title: {:?}, published_at: {:?}, summary: {:?}, html: {body:?} }},\n",
            meta.title, meta.published_at, meta.summary
        );
    }
    out += "];\n";
    let dest = Path::new(&env::var("OUT_DIR").unwrap()).join("posts.rs");
    fs::write(dest, out).expect("failed to write posts.rs");
}

fn to_html(markdown: &str, slug: &str) -> String {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_FOOTNOTES;
    let mut events = Vec::new();
    let mut mermaid: Option<String> = None;
    for event in Parser::new_ext(markdown, options) {
        if let Some(source) = mermaid.as_mut() {
            match event {
                Event::Text(text) => source.push_str(&text),
                Event::End(TagEnd::CodeBlock) => {
                    let svg = mermaid_rs_renderer::render(source)
                        .unwrap_or_else(|e| panic!("{slug}: bad mermaid diagram: {e}"));
                    events.push(Event::Html(CowStr::from(format!(
                        "<figure class=\"mermaid\">{svg}</figure>"
                    ))));
                    mermaid = None;
                }
                _ => {}
            }
            continue;
        }
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref lang))) if &**lang == "mermaid" => {
                mermaid = Some(String::new());
            }
            other => events.push(other),
        }
    }

    let mut body = String::from("<div class=\"prose\">");
    html::push_html(&mut body, events.into_iter());
    body.push_str("</div>");
    body
}
