use serde::Deserialize;
use serde_json::Value;

use crate::error::Error;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase", deny_unknown_fields)]
pub enum Subject {
    Resource { address: String, kind: String },
    Session { session: u64 },
    File { path: String },
    Journal { seq: u64 },
    Document { project: String, path: String },
    Tree { tree: String, item: String },
}

impl Subject {
    pub(crate) fn read(value: &Value) -> Result<Self, Error> {
        Self::deserialize(value).map_err(|error| Error::invalid_params(format!("subject: {error}")))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn reads_every_kind_of_subject() {
        let cases = [
            (
                json!({ "type": "session", "session": 7 }),
                Subject::Session { session: 7 },
            ),
            (
                json!({ "type": "file", "path": "/etc/hosts" }),
                Subject::File {
                    path: "/etc/hosts".to_owned(),
                },
            ),
            (
                json!({ "type": "journal", "seq": 42 }),
                Subject::Journal { seq: 42 },
            ),
            (
                json!({ "type": "document", "project": "orders", "path": "plan.note" }),
                Subject::Document {
                    project: "orders".to_owned(),
                    path: "plan.note".to_owned(),
                },
            ),
            (
                json!({ "type": "tree", "tree": "hosts", "item": "web-1" }),
                Subject::Tree {
                    tree: "hosts".to_owned(),
                    item: "web-1".to_owned(),
                },
            ),
        ];

        for (value, subject) in cases {
            assert_eq!(Subject::read(&value).unwrap(), subject);
        }
    }

    #[test]
    fn refuses_an_unknown_subject() {
        let error = Subject::read(&json!({ "type": "planet" })).unwrap_err();

        assert_eq!(error.code(), "invalid_params");
    }
}
