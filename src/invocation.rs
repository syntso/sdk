mod actor;
mod subject;

use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::error::Error;
use crate::json;
use crate::wit::synt::extension::types;

pub use actor::Actor;
pub use subject::Subject;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invocation {
    action: String,
    target: Option<String>,
    params: String,
    actor: Actor,
    project: Option<String>,
}

impl Invocation {
    pub fn new(action: impl Into<String>, params: impl Into<String>, actor: Actor) -> Self {
        Self {
            action: action.into(),
            target: None,
            params: params.into(),
            actor,
            project: None,
        }
    }

    #[must_use]
    pub fn on(mut self, target: impl Into<String>, project: Option<String>) -> Self {
        self.target = Some(target.into());
        self.project = project;
        self
    }

    pub fn action(&self) -> &str {
        &self.action
    }

    pub fn target(&self) -> Option<&str> {
        self.target.as_deref()
    }

    pub fn require_target(&self) -> Result<&str, Error> {
        self.target()
            .ok_or_else(|| Error::invalid_params(format!("{} needs a target", self.action)))
    }

    pub fn actor(&self) -> &Actor {
        &self.actor
    }

    pub fn project(&self) -> Option<&str> {
        self.project.as_deref()
    }

    pub fn params<T: DeserializeOwned>(&self) -> Result<T, Error> {
        json::object(&self.params)
    }

    pub fn params_value(&self) -> Result<Value, Error> {
        json::value(&self.params)
    }

    pub fn subject(&self) -> Result<Option<Subject>, Error> {
        let params = self.params_value()?;

        match params.get("subject") {
            None | Some(Value::Null) => Ok(None),
            Some(subject) => Subject::read(subject).map(Some),
        }
    }
}

impl From<types::Invocation> for Invocation {
    fn from(invocation: types::Invocation) -> Self {
        Self {
            action: invocation.action,
            target: invocation.target,
            params: invocation.params,
            actor: invocation.actor.into(),
            project: invocation.project,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Containers {
        #[serde(default)]
        all: bool,
    }

    fn wire(params: &str) -> types::Invocation {
        types::Invocation {
            action: "containers".to_owned(),
            target: Some("@orders/api-1".to_owned()),
            params: params.to_owned(),
            actor: types::Actor::Agent("claude-code".to_owned()),
            project: Some("orders".to_owned()),
        }
    }

    #[test]
    fn reads_what_the_core_passes() {
        let invocation = Invocation::from(wire(r#"{"all":true}"#));

        assert_eq!(invocation.action(), "containers");
        assert_eq!(invocation.target(), Some("@orders/api-1"));
        assert_eq!(invocation.require_target().unwrap(), "@orders/api-1");
        assert_eq!(invocation.project(), Some("orders"));
        assert_eq!(invocation.actor(), &Actor::Agent("claude-code".to_owned()));
        assert_eq!(
            invocation.params::<Containers>().unwrap(),
            Containers { all: true }
        );
    }

    #[test]
    fn missing_params_read_as_defaults() {
        let invocation = Invocation::new("containers", "", Actor::Person);

        assert_eq!(
            invocation.params::<Containers>().unwrap(),
            Containers { all: false }
        );
        assert_eq!(invocation.subject().unwrap(), None);
        assert_eq!(
            invocation.require_target().unwrap_err().code(),
            "invalid_params"
        );
    }

    #[test]
    fn reads_the_subject_a_menu_passes() {
        let invocation = Invocation::from(wire(
            r#"{"subject":{"type":"resource","address":"@orders/api-1","kind":"host"}}"#,
        ));

        assert_eq!(
            invocation.subject().unwrap(),
            Some(Subject::Resource {
                address: "@orders/api-1".to_owned(),
                kind: "host".to_owned(),
            })
        );
    }
}
