pub struct Html(pub String);

impl Html {
    pub fn from_string(html: String) -> Self {
        Self(html)
    }
}
