use std::time::Duration;

use serde::Serialize;

use super::Response;
use crate::error::Error;
use crate::seconds::millis;
use crate::wit::synt::extension::http;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    method: String,
    url: String,
    headers: Vec<http::Header>,
    secrets: Vec<http::SecretHeader>,
    body: Option<Vec<u8>>,
    timeout: Option<Duration>,
}

impl Request {
    pub fn new(method: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            method: method.into(),
            url: url.into(),
            headers: Vec::new(),
            secrets: Vec::new(),
            body: None,
            timeout: None,
        }
    }

    pub fn get(url: impl Into<String>) -> Self {
        Self::new("GET", url)
    }

    pub fn post(url: impl Into<String>) -> Self {
        Self::new("POST", url)
    }

    pub fn put(url: impl Into<String>) -> Self {
        Self::new("PUT", url)
    }

    pub fn delete(url: impl Into<String>) -> Self {
        Self::new("DELETE", url)
    }

    #[must_use]
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push(http::Header {
            name: name.into(),
            value: value.into(),
        });
        self
    }

    #[must_use]
    pub fn secret_header(self, name: impl Into<String>, secret: impl Into<String>) -> Self {
        self.secret_header_prefixed(name, secret, "")
    }

    #[must_use]
    pub fn bearer(self, secret: impl Into<String>) -> Self {
        self.secret_header_prefixed("Authorization", secret, "Bearer ")
    }

    #[must_use]
    pub fn secret_header_prefixed(
        mut self,
        name: impl Into<String>,
        secret: impl Into<String>,
        prefix: impl Into<String>,
    ) -> Self {
        self.secrets.push(http::SecretHeader {
            name: name.into(),
            secret: secret.into(),
            prefix: prefix.into(),
        });
        self
    }

    #[must_use]
    pub fn body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn json(self, body: &(impl Serialize + ?Sized)) -> Result<Self, Error> {
        let bytes = serde_json::to_vec(body)?;

        Ok(self.header("Content-Type", "application/json").body(bytes))
    }

    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn send(self) -> Result<Response, Error> {
        let response = http::send(&self.into())?;

        Ok(Response::from(response))
    }
}

impl From<Request> for http::Request {
    fn from(request: Request) -> Self {
        Self {
            method: request.method,
            url: request.url,
            headers: request.headers,
            secrets: request.secrets,
            body: request.body,
            timeout_ms: request.timeout.map(millis),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn carries_headers_secrets_and_body_to_the_interface() {
        let request = Request::post("https://grafana.example.com/api/ds/query")
            .header("Accept", "application/json")
            .bearer("grafana/token")
            .secret_header("X-Api-Key", "grafana/key")
            .body("{}")
            .timeout(Duration::from_secs(5));

        let wire = http::Request::from(request);

        assert_eq!(wire.method, "POST");
        assert_eq!(wire.headers[0].name, "Accept");
        assert_eq!(wire.secrets[0].name, "Authorization");
        assert_eq!(wire.secrets[0].secret, "grafana/token");
        assert_eq!(wire.secrets[0].prefix, "Bearer ");
        assert_eq!(wire.secrets[1].prefix, "");
        assert_eq!(wire.body.as_deref(), Some(&b"{}"[..]));
        assert_eq!(wire.timeout_ms, Some(5_000));
    }

    #[test]
    fn sends_json_with_its_content_type() {
        let wire = http::Request::from(
            Request::put("https://api.example.com/items/1")
                .json(&json!({ "name": "web" }))
                .unwrap(),
        );

        assert_eq!(wire.method, "PUT");
        assert_eq!(wire.headers[0].value, "application/json");
        assert_eq!(wire.body.as_deref(), Some(&br#"{"name":"web"}"#[..]));
        assert_eq!(wire.timeout_ms, None);
    }
}
