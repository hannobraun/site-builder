use std::path::Path;

use site_builder::{parse_markdown, render_template};
use tokio::{fs::File, io::AsyncWriteExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let content = parse_markdown("markdown/content.md").await?;
    let html = render_template("templates/**", "base.html", &content).await?;

    let output_file = Path::new("target/output.html").canonicalize()?;
    File::create(&output_file)
        .await?
        .write_all(html.as_bytes())
        .await?;

    let output_url = format!("file://{}", output_file.display());
    webbrowser::open(&output_url)?;

    Ok(())
}
