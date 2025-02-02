pub struct Bookmark {
    name: String,
    path: String,
}

impl Bookmark {
    pub fn new(name: String, path: String) -> Self {
        Bookmark { name, path }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn path(&self) -> &str {
        &self.path
    }
}
