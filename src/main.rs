use portfolio::app::App;

fn main() {
    if let Some(body) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.body())
    {
        body.set_inner_html("");
    }
    yew::Renderer::<App>::new().render();
}
