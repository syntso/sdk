use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Entry {
    pub name: String,
    pub kind: String,
    pub size: Option<u64>,
    pub mode: Option<String>,
    pub modified: Option<u32>,
}

impl Entry {
    pub fn is_dir(&self) -> bool {
        self.kind == "dir"
    }
}
