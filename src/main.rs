use site_builder::{markdown::Markdown, render_template, template::Template};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let content = Markdown::read("markdown/content.md").await?.parse().await?;
    let template = Template {
        directory: "templates/**",
        name: "base.html",
    };
    let html = render_template(&template, &content).await?;
    let html_file = html.write_to_file("target/output.html").await?;
    html_file.open_in_browser()?;

    Ok(())
}
