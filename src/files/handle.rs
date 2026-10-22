use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::wit::synt::extension::files;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Handle(String);

impl Handle {
    pub fn new(handle: impl Into<String>) -> Self {
        Self(handle.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn read(&self) -> Result<Vec<u8>, Error> {
        Ok(files::read(&self.0)?)
    }

    pub fn write(&self, body: &[u8]) -> Result<(), Error> {
        Ok(files::write(&self.0, body)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_from_params_as_plain_text() {
        let handle: Handle = serde_json::from_str(r#""f-1""#).unwrap();

        assert_eq!(handle, Handle::new("f-1"));
        assert_eq!(handle.as_str(), "f-1");
        assert_eq!(serde_json::to_string(&handle).unwrap(), r#""f-1""#);
    }
}
