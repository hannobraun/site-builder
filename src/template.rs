use tera::Tera;

use crate::html::Html;

pub struct Template {
    pub directory: &'static str,
    pub name: &'static str,
}

impl Template {
    pub fn new(directory: &'static str, name: &'static str) -> Self {
        Self { directory, name }
    }

    pub async fn render(self, content: &Html) -> anyhow::Result<Html> {
        let mut context = tera::Context::new();
        context.insert("content", content.as_str());

        let tera = Tera::new(self.directory)?;
        let html = tera.render(self.name, &context)?;

        Ok(Html::from_string(html))
    }
}
