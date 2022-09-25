use site_builder::{markdown::parse_markdown, render_template, write_html};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let content = parse_markdown("markdown/content.md").await?;
    let html = render_template("templates/**", "base.html", &content).await?;
    write_html("target/output.html", &html).await?;

    Ok(())
}
