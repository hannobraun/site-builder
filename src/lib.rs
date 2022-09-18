use std::path::Path;

use pulldown_cmark::{html::push_html, Parser};
use tera::{Context, Tera};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

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
