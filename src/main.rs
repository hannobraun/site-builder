use site_builder::{markdown::Markdown, render_template, write_html};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let content = Markdown::read("markdown/content.md").await?.parse().await?;
    let html = render_template("templates/**", "base.html", &content.0).await?;
    write_html("target/output.html", &html).await?;

    Ok(())
}
