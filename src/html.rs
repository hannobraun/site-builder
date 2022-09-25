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
        let html_file = HtmlFile::from_path(path)?;
        File::create(html_file.path())
            .await?
            .write_all(self.as_str().as_bytes())
            .await?;
        Ok(html_file)
    }
}

pub struct HtmlFile(pub PathBuf);

impl HtmlFile {
    pub fn from_path(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref().canonicalize()?;
        Ok(Self(path))
    }

    pub fn path(&self) -> &Path {
        &self.0
    }
}
