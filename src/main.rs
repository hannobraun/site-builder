use std::fmt::Write;

use pulldown_cmark::{html::push_html, Parser};
use tokio::{fs::File, io::AsyncWriteExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let markdown = "Hello, world!";
    let parser = Parser::new(markdown);

    let mut html_from_markdown = String::new();
    push_html(&mut html_from_markdown, parser);

    let mut html = String::new();

    write!(
        html,
        "\
<!DOCTYPE html>
<html lang=\"en\">
    <head>
        <meta charset=\"UTF-8\" />
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />

        <title>Hello, world!</title>
    </head>
    <body>
        {html_from_markdown}
    </body>
</html>\
    "
    )?;

    File::create("target/output.html")
        .await?
        .write_all(html.as_bytes())
        .await?;

    Ok(())
}
