use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::error::Error;

pub(crate) fn encode(value: &(impl Serialize + ?Sized)) -> Result<String, Error> {
    Ok(serde_json::to_string(value)?)
}

pub(crate) fn decode<T: DeserializeOwned>(text: &str) -> Result<T, Error> {
    Ok(serde_json::from_str(or_null(text))?)
}

pub(crate) fn value(text: &str) -> Result<Value, Error> {
    decode(text)
}

pub(crate) fn object<T: DeserializeOwned>(text: &str) -> Result<T, Error> {
    let found = value(text)?;
    let given = if found.is_null() {
        Value::Object(serde_json::Map::new())
    } else {
        found
    };

    T::deserialize(given).map_err(|error| Error::invalid_params(error.to_string()))
}

fn or_null(text: &str) -> &str {
    if text.trim().is_empty() { "null" } else { text }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Debug, Default, Deserialize, PartialEq)]
    struct Options {
        #[serde(default)]
        all: bool,
    }

    #[test]
    fn empty_params_read_as_an_empty_object() {
        assert_eq!(object::<Options>("").unwrap(), Options::default());
        assert_eq!(object::<Options>("null").unwrap(), Options::default());
        assert_eq!(
            object::<Options>(r#"{"all":true}"#).unwrap(),
            Options { all: true }
        );
    }

    #[test]
    fn params_of_the_wrong_shape_are_invalid_params() {
        let error = object::<Options>(r#"{"all":"yes"}"#).unwrap_err();

        assert_eq!(error.code(), "invalid_params");
    }

    #[test]
    fn empty_text_is_null() {
        assert_eq!(value(" ").unwrap(), Value::Null);
        assert_eq!(encode(&vec![1, 2]).unwrap(), "[1,2]");
    }
}
