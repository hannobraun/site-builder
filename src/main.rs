use pulldown_cmark::{html::push_html, Parser};
use tera::{Context, Tera};
use tokio::{fs::File, io::AsyncWriteExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let markdown = "Hello, world!";
    let parser = Parser::new(markdown);

    let mut html_from_markdown = String::new();
    push_html(&mut html_from_markdown, parser);

    let mut context = Context::new();
    context.insert("content", &html_from_markdown);

    let tera = Tera::new("templates/**")?;
    let html = tera.render("base.html", &context)?;

    File::create("target/output.html")
        .await?
        .write_all(html.as_bytes())
        .await?;

    Ok(())
}
