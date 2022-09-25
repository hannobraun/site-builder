pub mod html;
pub mod markdown;

use std::path::Path;

use html::{Html, HtmlFile};
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
    let html_file = HtmlFile(output_path);
    File::create(&html_file.0)
        .await?
        .write_all(html.as_str().as_bytes())
        .await?;

    let output_url = format!("file://{}", html_file.0.display());
    webbrowser::open(&output_url)?;

    Ok(())
}
