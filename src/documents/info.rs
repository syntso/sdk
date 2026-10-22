use crate::wit::synt::extension::documents;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentInfo {
    pub project: String,
    pub path: String,
    pub kind: String,
    pub revision: String,
    pub size: u64,
}

impl From<documents::DocumentInfo> for DocumentInfo {
    fn from(info: documents::DocumentInfo) -> Self {
        Self {
            project: info.project,
            path: info.path,
            kind: info.kind,
            revision: info.revision,
            size: info.size,
        }
    }
}
