use std::path::{Path, PathBuf};

pub struct Html(String);

impl Html {
    pub fn from_string(html: String) -> Self {
        Self(html)
    }

    pub fn as_str(&self) -> &str {
        &self.0
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
