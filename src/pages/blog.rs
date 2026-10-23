use yew::prelude::*;

use crate::utils::blog::{self, POSTS};

pub fn blog() -> Html {
    html! {
        <section id="blog">
            <p class="section-heading">
                <i class="fa-solid fa-book" aria-hidden="true"></i>
                { "Blog" }
            </p>
            <div class="blog-list">
                { for POSTS.iter().map(|post| html! {
                    <a class="blog-card" href={format!("/blog/{}", post.slug)}>
                        <p class="blog-title">{ post.title }</p>
                        <p class="blog-summary">{ post.summary }</p>
                        <p class="blog-date">{ post.published_at }</p>
                    </a>
                }) }
            </div>
        </section>
    }
}

pub fn post(slug: &str) -> Html {
    let Some(post) = blog::find(slug) else {
        return html! { <p class="section-heading">{ "Post not found" }</p> };
    };
    html! {
        <article class="post">
            <a class="post-back" href="/blog">
                <i class="fa-solid fa-arrow-left" aria-hidden="true"></i>
                { "Back to blog" }
            </a>
            <h1 class="post-title">{ post.title }</h1>
            <p class="blog-date">{ post.published_at }</p>
            { Html::from_html_unchecked(AttrValue::Static(post.html)) }
        </article>
    }
}
