use std::path::Path;

use pulldown_cmark::{html::push_html, Parser};
use tera::{Context, Tera};
use tokio::{fs::File, io::AsyncReadExt};

pub async fn parse_markdown(path: impl AsRef<Path>) -> anyhow::Result<String> {
    let mut markdown = String::new();
    File::open(path)
        .await?
        .read_to_string(&mut markdown)
        .await?;

    let parser = Parser::new(&markdown);

    let mut html = String::new();
    push_html(&mut html, parser);

    Ok(html)
}

pub async fn render_template(
    dir: &str,
    name: &str,
    content: &str,
) -> anyhow::Result<String> {
    let mut context = Context::new();
    context.insert("content", &content);

    let tera = Tera::new(dir)?;
    let html = tera.render(name, &context)?;

    Ok(html)
}
