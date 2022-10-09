pub mod html;
pub mod markdown;
pub mod template;

use html::Html;
use template::Template;
use tera::{Context, Tera};

pub async fn render_template(
    template: &Template,
    content: &Html,
) -> anyhow::Result<Html> {
    let mut context = Context::new();
    context.insert("content", content.as_str());

    let tera = Tera::new(template.directory)?;
    let html = tera.render(template.name, &context)?;

    Ok(Html::from_string(html))
}
