use serde_json::{Value, json};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Patch(Vec<Value>);

impl Patch {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn replace(mut self, path: &str, value: impl Into<Value>) -> Self {
        self.0
            .push(json!({ "op": "replace", "path": path, "value": value.into() }));
        self
    }

    #[must_use]
    pub fn add(mut self, path: &str, value: impl Into<Value>) -> Self {
        self.0
            .push(json!({ "op": "add", "path": path, "value": value.into() }));
        self
    }

    #[must_use]
    pub fn remove(mut self, path: &str) -> Self {
        self.0.push(json!({ "op": "remove", "path": path }));
        self
    }

    pub fn to_json(&self) -> String {
        Value::Array(self.0.clone()).to_string()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::Patch;
    use crate::view::text;

    #[test]
    fn writes_json_patch_operations() {
        let patch = Patch::new()
            .replace("/children/1/items/1/value", 0.7)
            .add("/children/-", text("new"))
            .remove("/children/0");

        let operations: Value = serde_json::from_str(&patch.to_json()).unwrap();

        assert_eq!(
            operations,
            json!([
                { "op": "replace", "path": "/children/1/items/1/value", "value": 0.7 },
                { "op": "add", "path": "/children/-", "value": { "type": "text", "text": "new" } },
                { "op": "remove", "path": "/children/0" }
            ])
        );
    }
}
