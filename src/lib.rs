use std::path::Path;

use pulldown_cmark::{html::push_html, Parser};
use tokio::{fs::File, io::AsyncReadExt};

pub async fn parse_markdown(path: impl AsRef<Path>) -> anyhow::Result<String> {
    let mut markdown = String::new();
    File::open(path)
        .await?
        .read_to_string(&mut markdown)
        .await?;

    let parser = Parser::new(&markdown);

    let mut html_from_markdown = String::new();
    push_html(&mut html_from_markdown, parser);

    Ok(html_from_markdown)
}
