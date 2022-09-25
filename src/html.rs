use std::path::{Path, PathBuf};

use tokio::{fs::File, io::AsyncWriteExt};

pub struct Html(String);

impl Html {
    pub fn from_string(html: String) -> Self {
        Self(html)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub async fn write_to_file(
        &self,
        path: impl AsRef<Path>,
    ) -> anyhow::Result<HtmlFile> {
        let file = HtmlFile::create(self, path).await?;
        Ok(file)
    }
}

pub struct HtmlFile(pub PathBuf);

impl HtmlFile {
    pub async fn create(
        html: &Html,
        path: impl AsRef<Path>,
    ) -> anyhow::Result<Self> {
        let path = path.as_ref().canonicalize()?;

        File::create(&path)
            .await?
            .write_all(html.as_str().as_bytes())
            .await?;

        Ok(Self(path))
    }

    pub fn path(&self) -> &Path {
        &self.0
    }
}
