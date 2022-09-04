use pulldown_cmark::{Parser, html::push_html};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let markdown = "Hello, world!";
    let parser = Parser::new(markdown);

    let mut html_from_markdown = String::new();
    push_html(&mut html_from_markdown, parser);

    let html = format!("\
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
    ");

    println!("{}", html);

    Ok(())
}
