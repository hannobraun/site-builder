use std::path::Path;

use pulldown_cmark::{html::push_html, Parser};
use tokio::{fs::File, io::AsyncReadExt};

pub struct Markdown(String);

impl Markdown {
    pub async fn read(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let mut markdown = String::new();
        File::open(path)
            .await?
            .read_to_string(&mut markdown)
            .await?;

        Ok(Self(markdown))
    }
}

pub async fn parse_markdown(path: impl AsRef<Path>) -> anyhow::Result<String> {
    let markdown = Markdown::read(path).await?;

    let parser = Parser::new(&markdown.0);

    let mut html = String::new();
    push_html(&mut html, parser);

    Ok(html)
}
