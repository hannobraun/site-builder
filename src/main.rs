use site_builder::{markdown::Markdown, render_template};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let content = Markdown::read("markdown/content.md").await?.parse().await?;
    let html = render_template("templates/**", "base.html", &content).await?;
    let html_file = html.write_to_file("target/output.html").await?;
    html_file.open_in_browser()?;

    Ok(())
}
