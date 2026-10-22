use serde_json::{Map, Value};

use crate::wit::synt::extension::types;

#[derive(Debug, Clone, PartialEq)]
pub struct Binding {
    action: String,
    target: Option<String>,
    params: Value,
}

impl Binding {
    pub fn action(action: impl Into<String>) -> Self {
        Self {
            action: action.into(),
            target: None,
            params: Value::Object(Map::new()),
        }
    }

    #[must_use]
    pub fn target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    #[must_use]
    pub fn params(mut self, params: Value) -> Self {
        self.params = params;
        self
    }

    pub fn action_id(&self) -> &str {
        &self.action
    }

    pub(crate) fn fields(&self) -> Map<String, Value> {
        let mut fields = Map::new();
        fields.insert("action".to_owned(), Value::String(self.action.clone()));

        if let Some(target) = &self.target {
            fields.insert("target".to_owned(), Value::String(target.clone()));
        }

        if self
            .params
            .as_object()
            .is_some_and(|params| !params.is_empty())
        {
            fields.insert("params".to_owned(), self.params.clone());
        }

        fields
    }
}

impl From<Binding> for types::Binding {
    fn from(binding: Binding) -> Self {
        Self {
            action: binding.action,
            target: binding.target,
            params: binding.params.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn binds_an_action_with_target_and_params() {
        let binding = Binding::action("docker.restart")
            .target("@orders/web-1")
            .params(json!({ "force": true }));

        let wire = types::Binding::from(binding.clone());

        assert_eq!(binding.action_id(), "docker.restart");
        assert_eq!(wire.action, "docker.restart");
        assert_eq!(wire.target.as_deref(), Some("@orders/web-1"));
        assert_eq!(wire.params, r#"{"force":true}"#);
    }

    #[test]
    fn leaves_out_what_is_not_set_in_views() {
        let fields = Binding::action("docker.refresh").fields();

        assert_eq!(Value::Object(fields), json!({ "action": "docker.refresh" }));
        assert_eq!(types::Binding::from(Binding::action("a")).params, "{}");
    }
}
