pub trait Subset {
    fn subset_left(&self, pattern: &str) -> &str;
    fn subset_right(&self, pattern: &str) -> &str;
    fn subset(&self, pattern: &str) -> &str;
}

impl Subset for str {
    fn subset_right(&self, pattern: &str) -> &str {
        if let Some(index) = self.rfind(pattern) {
            return &self[..index];
        } else {
            return self;
        }
    }

    fn subset_left(&self, pattern: &str) -> &str {
        if let Some(index) = self.find(pattern) {
            return &self[(index + pattern.len())..];
        } else {
            return self;
        }
    }

    fn subset(&self, pattern: &str) -> &str {
        return self.subset_left(pattern).subset_right(pattern);
    }
}
