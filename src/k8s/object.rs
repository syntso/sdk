use serde::Serialize;
use serde_json::Value;

use crate::error::Error;
use crate::host::call;

#[derive(Serialize)]
struct Params<'a> {
    kind: &'a str,
    name: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<&'a str>,
}

pub fn get(cluster: &str, kind: &str, name: &str, namespace: Option<&str>) -> Result<Value, Error> {
    let params = Params {
        kind,
        name,
        namespace,
    };

    call("k8s.get", Some(cluster), &params)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn names_the_object_k8s_get_reads() {
        let params = Params {
            kind: "pods",
            name: "api-0",
            namespace: Some("web"),
        };
        let cluster_wide = Params {
            kind: "nodes",
            name: "n1",
            namespace: None,
        };

        assert_eq!(
            serde_json::to_value(params).unwrap(),
            json!({ "kind": "pods", "name": "api-0", "namespace": "web" })
        );
        assert_eq!(
            serde_json::to_value(cluster_wide).unwrap(),
            json!({ "kind": "nodes", "name": "n1" })
        );
    }
}
