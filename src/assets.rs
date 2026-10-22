pub struct Assets {
    files: &'static [(&'static str, &'static [u8])],
}

impl Assets {
    pub const fn new(files: &'static [(&'static str, &'static [u8])]) -> Self {
        Self { files }
    }

    pub fn get(&self, path: &str) -> Option<Vec<u8>> {
        self.files
            .iter()
            .find(|(name, _)| *name == path)
            .map(|(_, bytes)| bytes.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::Assets;

    const BUNDLE: Assets = Assets::new(&[("index.html", b"<html></html>"), ("app.js", b"go()")]);

    #[test]
    fn serves_the_files_it_was_built_with() {
        assert_eq!(BUNDLE.get("app.js"), Some(b"go()".to_vec()));
        assert_eq!(BUNDLE.get("missing.css"), None);
    }
}
