pub mod html;
pub mod markdown;

use std::path::Path;

use html::Html;
use tera::{Context, Tera};
use tokio::{fs::File, io::AsyncWriteExt};

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
    let output_path = path.as_ref().canonicalize()?;
    File::create(&output_path)
        .await?
        .write_all(html.as_str().as_bytes())
        .await?;

    let output_url = format!("file://{}", output_path.display());
    webbrowser::open(&output_url)?;

    Ok(())
}
