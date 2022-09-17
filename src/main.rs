use std::path::Path;

use pulldown_cmark::{html::push_html, Parser};
use tera::{Context, Tera};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut markdown = String::new();
    File::open("markdown/content.md")
        .await?
        .read_to_string(&mut markdown)
        .await?;

    let parser = Parser::new(&markdown);

    let mut html_from_markdown = String::new();
    push_html(&mut html_from_markdown, parser);

    let mut context = Context::new();
    context.insert("content", &html_from_markdown);

    let tera = Tera::new("templates/**")?;
    let html = tera.render("base.html", &context)?;

    let output_file = Path::new("target/output.html").canonicalize()?;
    File::create(&output_file)
        .await?
        .write_all(html.as_bytes())
        .await?;

    Ok(())
}
