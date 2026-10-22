use serde::de::DeserializeOwned;

use crate::error::Error;
use crate::json;
use crate::wit::synt::extension::types;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    topic: String,
    payload: String,
}

impl Event {
    pub fn new(topic: impl Into<String>, payload: impl Into<String>) -> Self {
        Self {
            topic: topic.into(),
            payload: payload.into(),
        }
    }

    pub fn topic(&self) -> &str {
        &self.topic
    }

    pub fn payload<T: DeserializeOwned>(&self) -> Result<T, Error> {
        json::decode(&self.payload)
    }

    pub fn payload_text(&self) -> &str {
        &self.payload
    }
}

impl From<types::Event> for Event {
    fn from(event: types::Event) -> Self {
        Self::new(event.topic, event.payload)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;

    #[test]
    fn reads_topic_and_payload() {
        let event = Event::from(types::Event {
            topic: "resource".to_owned(),
            payload: r#"{"address":"@orders/web-1"}"#.to_owned(),
        });

        let payload: Value = event.payload().unwrap();

        assert_eq!(event.topic(), "resource");
        assert_eq!(payload["address"], "@orders/web-1");
        assert!(event.payload_text().contains("web-1"));
    }
}
