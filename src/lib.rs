pub mod html;
pub mod markdown;

use html::Html;
use tera::{Context, Tera};

pub async fn render_template(
    dir: &str,
    name: &str,
    content: &Html,
) -> anyhow::Result<Html> {
    let mut context = Context::new();
    context.insert("content", content.as_str());

    let tera = Tera::new(dir)?;
    let html = tera.render(name, &context)?;

    Ok(Html::from_string(html))
}
