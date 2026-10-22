use serde::Deserialize;
use serde_json::json;

use super::Entry;
use crate::error::Error;
use crate::host::call;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Listing {
    pub path: String,
    pub entries: Vec<Entry>,
    pub truncated: bool,
}

pub fn list(host: &str, path: &str) -> Result<Listing, Error> {
    call("sftp.list", Some(host), &json!({ "path": path }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_what_sftp_list_returns() {
        let listing: Listing = serde_json::from_value(json!({
            "path": "/srv",
            "entries": [
                { "name": "app", "kind": "dir", "size": null, "mode": "755", "modified": 1, "uid": 0, "gid": 0 },
                { "name": "app.log", "kind": "file", "size": 120, "mode": "644", "modified": 2, "uid": 0, "gid": 0 }
            ],
            "truncated": false
        }))
        .unwrap();

        assert!(listing.entries[0].is_dir());
        assert_eq!(listing.entries[1].size, Some(120));
    }
}
