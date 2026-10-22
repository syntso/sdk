mod info;
mod written;

use crate::error::Error;
use crate::wit::synt::extension::documents;

pub use info::DocumentInfo;
pub use written::Written;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    pub info: DocumentInfo,
    pub body: Vec<u8>,
}

impl Document {
    pub fn list(project: &str) -> Result<Vec<DocumentInfo>, Error> {
        let found = documents::listing(project)?;

        Ok(found.into_iter().map(DocumentInfo::from).collect())
    }

    pub fn read(project: &str, path: &str) -> Result<Self, Error> {
        Ok(documents::read(project, path)?.into())
    }

    pub fn write(
        project: &str,
        path: &str,
        body: &[u8],
        base: Option<&str>,
    ) -> Result<Written, Error> {
        Ok(documents::write(project, path, body, base)?.into())
    }

    pub fn remove(project: &str, path: &str) -> Result<(), Error> {
        Ok(documents::remove(project, path)?)
    }

    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).into_owned()
    }
}

impl From<documents::Document> for Document {
    fn from(document: documents::Document) -> Self {
        Self {
            info: document.info.into(),
            body: document.body,
        }
    }
}
