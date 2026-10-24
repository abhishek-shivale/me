use std::{env, fs, path::PathBuf};

use portfolio::app::{ServerApp, ServerAppProps};
use portfolio::utils::{blog::POSTS, resume::Resume};
use yew::LocalServerRenderer;

struct Page {
    path: String,
    title: String,
    description: String,
    kind: &'static str,
    extra_head: String,
}

fn main() {
    let out_dir = PathBuf::from(env::var("TRUNK_STAGING_DIR").unwrap_or_else(|_| "dist".into()));
    let template = fs::read_to_string(out_dir.join("index.html"))
        .expect("index.html missing; run trunk build first");
    let resume = Resume::parse().expect("invalid Resume.toml");
    let site = resume.personal.url.trim_end_matches('/');
    let name = &resume.personal.name;

    let mut pages = vec![
        Page {
            path: "/".into(),
            title: format!("{name} | {}", resume.personal.title),
            description: resume.personal.description.clone(),
            kind: "website",
            extra_head: person_json_ld(&resume),
        },
        Page {
            path: "/blog".into(),
            title: format!("Blog | {name}"),
            description: format!("Articles by {name} about backend systems, Node.js, databases, distributed systems, and Rust."),
            kind: "website",
            extra_head: String::new(),
        },
    ];
    for post in POSTS {
        pages.push(Page {
            path: format!("/blog/{}", post.slug),
            title: format!("{} | {name}", post.title),
            description: post.summary.into(),
            kind: "article",
            extra_head: format!(
                "<meta property=\"article:published_time\" content=\"{}\" />\n",
                escape(post.published_at)
            ),
        });
    }

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to start tokio runtime");
    for page in &pages {
        let props = ServerAppProps { path: page.path.clone().into() };
        let body = runtime.block_on(
            LocalServerRenderer::<ServerApp>::with_props(props)
                .hydratable(false)
                .render(),
        );
        let html = fill_template(&template, page, site, &body);
        let file = if page.path == "/" {
            out_dir.join("index.html")
        } else {
            out_dir.join(format!("{}.html", page.path.trim_start_matches('/')))
        };
        fs::create_dir_all(file.parent().unwrap()).expect("failed to create page directory");
        fs::write(&file, html).unwrap_or_else(|e| panic!("failed to write {}: {e}", file.display()));
    }

    let urls: String = pages
        .iter()
        .map(|p| format!("  <url><loc>{site}{}</loc></url>\n", p.path))
        .collect();
    fs::write(
        out_dir.join("sitemap.xml"),
        format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n{urls}</urlset>\n"),
    )
    .expect("failed to write sitemap.xml");
    fs::write(
        out_dir.join("robots.txt"),
        format!("User-agent: *\nAllow: /\n\nSitemap: {site}/sitemap.xml\n"),
    )
    .expect("failed to write robots.txt");
}

fn fill_template(template: &str, page: &Page, site: &str, body: &str) -> String {
    let url = format!("{site}{}", page.path);
    let title = escape(&page.title);
    let description = escape(&page.description);
    let head = format!(
        "<meta name=\"description\" content=\"{description}\" />\n\
         <link rel=\"canonical\" href=\"{url}\" />\n\
         <meta property=\"og:type\" content=\"{kind}\" />\n\
         <meta property=\"og:title\" content=\"{title}\" />\n\
         <meta property=\"og:description\" content=\"{description}\" />\n\
         <meta property=\"og:url\" content=\"{url}\" />\n\
         <meta name=\"twitter:card\" content=\"summary\" />\n\
         <meta name=\"twitter:title\" content=\"{title}\" />\n\
         <meta name=\"twitter:description\" content=\"{description}\" />\n\
         {extra}</head>",
        kind = page.kind,
        extra = page.extra_head,
    );
    let start = template.find("<title>").expect("template has no <title>");
    let end = template[start..].find("</title>").expect("template has no </title>") + start + "</title>".len();
    let html = format!("{}<title>{title}</title>{}", &template[..start], &template[end..]);
    html.replacen("</head>", &head, 1)
        .replacen("<body>", &format!("<body>{body}"), 1)
}

fn person_json_ld(resume: &Resume) -> String {
    let p = &resume.personal;
    let s = &resume.contact.social;
    let quote = |v: &str| format!("\"{}\"", v.replace('\\', "\\\\").replace('"', "\\\"").replace('<', "\\u003c"));
    format!(
        "<script type=\"application/ld+json\">{{\"@context\":\"https://schema.org\",\"@type\":\"Person\",\"name\":{},\"url\":{},\"jobTitle\":{},\"sameAs\":[{},{},{}]}}</script>\n",
        quote(&p.name),
        quote(&p.url),
        quote(&p.title),
        quote(&s.github.url),
        quote(&s.linkedin.url),
        quote(&s.x.url),
    )
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fills_title_meta_and_body() {
        let page = Page {
            path: "/blog/x".into(),
            title: "A & \"B\"".into(),
            description: "d".into(),
            kind: "article",
            extra_head: String::new(),
        };
        let out = fill_template("<head><title>old</title></head><body></body>", &page, "https://s.io", "<p>hi</p>");
        assert!(out.contains("<title>A &amp; &quot;B&quot;</title>"));
        assert!(out.contains("<link rel=\"canonical\" href=\"https://s.io/blog/x\" />"));
        assert!(out.contains("<body><p>hi</p></body>"));
        assert!(!out.contains("old"));
    }
}
