use std::path::Path;

use site_builder::parse_markdown;
use tera::{Context, Tera};
use tokio::{fs::File, io::AsyncWriteExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let content = parse_markdown("markdown/content.md").await?;

    let mut context = Context::new();
    context.insert("content", &content);

    let tera = Tera::new("templates/**")?;
    let html = tera.render("base.html", &context)?;

    let output_file = Path::new("target/output.html").canonicalize()?;
    File::create(&output_file)
        .await?
        .write_all(html.as_bytes())
        .await?;

    let output_url = format!("file://{}", output_file.display());
    webbrowser::open(&output_url)?;

    Ok(())
}
