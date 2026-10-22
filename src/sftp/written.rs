use data_encoding::BASE64;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::error::Error;
use crate::host::call;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Written {
    pub path: String,
    pub bytes: u64,
}

pub fn write(host: &str, path: &str, body: &[u8]) -> Result<Written, Error> {
    call("sftp.write", Some(host), &params(path, body))
}

fn params(path: &str, body: &[u8]) -> Value {
    json!({
        "path": path,
        "data": BASE64.encode(body),
        "create": true,
        "truncate": true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replaces_the_whole_file() {
        assert_eq!(
            params("/srv/motd", b"hi"),
            json!({ "path": "/srv/motd", "data": "aGk=", "create": true, "truncate": true })
        );
    }
}
