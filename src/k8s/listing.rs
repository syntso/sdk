use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::Error;
use crate::host::call;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Listing {
    pub kind: String,
    pub items: Vec<Value>,

    #[serde(default, rename = "continue")]
    pub continue_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct List {
    kind: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<String>,

    #[serde(skip_serializing_if = "std::ops::Not::not")]
    all_namespaces: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    selector: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,

    #[serde(rename = "continue", skip_serializing_if = "Option::is_none")]
    continue_token: Option<String>,
}

impl List {
    pub fn of(kind: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            namespace: None,
            all_namespaces: false,
            selector: None,
            limit: None,
            continue_token: None,
        }
    }

    #[must_use]
    pub fn namespace(mut self, namespace: impl Into<String>) -> Self {
        self.namespace = Some(namespace.into());
        self.all_namespaces = false;
        self
    }

    #[must_use]
    pub fn all_namespaces(mut self) -> Self {
        self.namespace = None;
        self.all_namespaces = true;
        self
    }

    #[must_use]
    pub fn selector(mut self, selector: impl Into<String>) -> Self {
        self.selector = Some(selector.into());
        self
    }

    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    #[must_use]
    pub fn after(mut self, listing: &Listing) -> Self {
        self.continue_token.clone_from(&listing.continue_token);
        self
    }

    pub fn run(&self, cluster: &str) -> Result<Listing, Error> {
        call("k8s.list", Some(cluster), self)
    }
}

pub fn list(cluster: &str, kind: &str) -> Result<Listing, Error> {
    List::of(kind).run(cluster)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn sends_the_query_k8s_list_takes() {
        let query = List::of("pods")
            .namespace("web")
            .selector("app=api")
            .limit(50);
        let everywhere = List::of("pods").namespace("web").all_namespaces();

        assert_eq!(
            serde_json::to_value(query).unwrap(),
            json!({ "kind": "pods", "namespace": "web", "selector": "app=api", "limit": 50 })
        );
        assert_eq!(
            serde_json::to_value(everywhere).unwrap(),
            json!({ "kind": "pods", "all_namespaces": true })
        );
    }

    #[test]
    fn pages_with_the_continue_token() {
        let listing: Listing =
            serde_json::from_value(json!({ "kind": "pods", "items": [{}], "continue": "abc" }))
                .unwrap();

        let next = serde_json::to_value(List::of("pods").after(&listing)).unwrap();

        assert_eq!(next["continue"], "abc");
        assert_eq!(listing.items.len(), 1);
    }
}
