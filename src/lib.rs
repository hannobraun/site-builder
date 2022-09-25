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
) -> anyhow::Result<String> {
    let mut context = Context::new();
    context.insert("content", &content.0);

    let tera = Tera::new(dir)?;
    let html = tera.render(name, &context)?;

    Ok(html)
}

pub async fn write_html(
    file: impl AsRef<Path>,
    html: &str,
) -> anyhow::Result<()> {
    let output_file = file.as_ref().canonicalize()?;
    File::create(&output_file)
        .await?
        .write_all(html.as_bytes())
        .await?;

    let output_url = format!("file://{}", output_file.display());
    webbrowser::open(&output_url)?;

    Ok(())
}
