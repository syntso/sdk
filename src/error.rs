use crate::wit::synt::extension::types;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{code}: {message}")]
pub struct Error {
    code: String,
    message: String,
}

impl Error {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }

    pub fn invalid_params(message: impl Into<String>) -> Self {
        Self::new("invalid_params", message)
    }

    pub fn unknown_action(action: &str) -> Self {
        Self::new("unknown_action", action)
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<types::Error> for Error {
    fn from(error: types::Error) -> Self {
        Self::new(error.code, error.message)
    }
}

impl From<Error> for types::Error {
    fn from(error: Error) -> Self {
        Self {
            code: error.code,
            message: error.message,
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::new("invalid_json", error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shows_code_and_message() {
        let error = Error::new("not_found", "no such host");

        assert_eq!(error.to_string(), "not_found: no such host");
    }

    #[test]
    fn round_trips_through_the_interface() {
        let error = Error::invalid_params("port is missing");

        let wire = types::Error::from(error.clone());
        let back = Error::from(wire);

        assert_eq!(back, error);
        assert_eq!(back.code(), "invalid_params");
    }

    #[test]
    fn bad_json_becomes_an_error() {
        let broken = serde_json::from_str::<serde_json::Value>("{").unwrap_err();

        assert_eq!(Error::from(broken).code(), "invalid_json");
    }
}
