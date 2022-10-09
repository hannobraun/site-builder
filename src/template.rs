pub struct Template {
    pub directory: &'static str,
    pub name: &'static str,
}

impl Template {
    pub fn new(directory: &'static str, name: &'static str) -> Self {
        Self { directory, name }
    }
}
