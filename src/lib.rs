pub mod html;
pub mod markdown;

use std::path::Path;

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

pub async fn write_html(
    path: impl AsRef<Path>,
    html: &Html,
) -> anyhow::Result<()> {
    let html_file = html.write_to_file(path).await?;

    let output_url = format!("file://{}", html_file.path().display());
    webbrowser::open(&output_url)?;

    Ok(())
}
