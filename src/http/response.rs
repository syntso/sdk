use serde::de::DeserializeOwned;

use crate::error::Error;
use crate::wit::synt::extension::http;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    status: u16,
    headers: Vec<http::Header>,
    body: Vec<u8>,
}

impl Response {
    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }

    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|header| header.name.eq_ignore_ascii_case(name))
            .map(|header| header.value.as_str())
    }

    pub fn body(&self) -> &[u8] {
        &self.body
    }

    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).into_owned()
    }

    pub fn json<T: DeserializeOwned>(&self) -> Result<T, Error> {
        Ok(serde_json::from_slice(&self.body)?)
    }
}

impl From<http::Response> for Response {
    fn from(response: http::Response) -> Self {
        Self {
            status: response.status,
            headers: response.headers,
            body: response.body,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;

    #[test]
    fn reads_status_headers_and_body() {
        let response = Response::from(http::Response {
            status: 200,
            headers: vec![http::Header {
                name: "content-type".to_owned(),
                value: "application/json".to_owned(),
            }],
            body: br#"{"ok":true}"#.to_vec(),
        });

        assert!(response.is_success());
        assert_eq!(response.status(), 200);
        assert_eq!(response.header("Content-Type"), Some("application/json"));
        assert_eq!(response.json::<Value>().unwrap()["ok"], true);
        assert_eq!(response.text(), r#"{"ok":true}"#);
        assert_eq!(response.body().len(), 11);
    }
}
