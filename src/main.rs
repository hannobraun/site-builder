use site_builder::{markdown::Markdown, template::Template};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let content = Markdown::read("markdown/content.md").await?.parse().await?;
    let template = Template::new("templates/**", "base.html");
    let html = template.render_template(&content).await?;
    let html_file = html.write_to_file("target/output.html").await?;
    html_file.open_in_browser()?;

    Ok(())
}
