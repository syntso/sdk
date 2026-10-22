use data_encoding::BASE64;
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::host::call;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chunk {
    pub path: String,
    pub offset: u64,
    pub data: Vec<u8>,
    pub eof: bool,
}

#[derive(Serialize)]
struct Params<'a> {
    path: &'a str,
    offset: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    length: Option<u64>,
}

#[derive(Deserialize)]
struct Answer {
    path: String,
    offset: u64,
    eof: bool,
    data: String,
}

pub fn read(host: &str, path: &str) -> Result<Chunk, Error> {
    read_at(host, path, 0, None)
}

pub fn read_at(host: &str, path: &str, offset: u64, length: Option<u64>) -> Result<Chunk, Error> {
    let params = Params {
        path,
        offset,
        length,
    };
    let answer: Answer = call("sftp.read", Some(host), &params)?;

    Chunk::try_from(answer)
}

impl TryFrom<Answer> for Chunk {
    type Error = Error;

    fn try_from(answer: Answer) -> Result<Self, Error> {
        let data = BASE64
            .decode(answer.data.as_bytes())
            .map_err(|error| Error::new("invalid_output", format!("sftp.read data: {error}")))?;

        Ok(Self {
            path: answer.path,
            offset: answer.offset,
            data,
            eof: answer.eof,
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn decodes_what_sftp_read_returns() {
        let answer: Answer = serde_json::from_value(json!({
            "path": "/etc/hostname", "offset": 0, "bytes": 6, "eof": true, "data": "d2ViLTEK"
        }))
        .unwrap();

        let chunk = Chunk::try_from(answer).unwrap();

        assert_eq!(chunk.data, b"web-1\n");
        assert!(chunk.eof);
    }

    #[test]
    fn asks_for_a_length_only_when_given() {
        let params = Params {
            path: "/a",
            offset: 4,
            length: None,
        };

        assert_eq!(
            serde_json::to_value(params).unwrap(),
            json!({ "path": "/a", "offset": 4 })
        );
    }
}
