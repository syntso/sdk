use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Rows {
    pub kind: String,
    pub effect: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Value>>,
    pub more: bool,
    pub next_offset: Option<u64>,
    pub affected: Option<u64>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn reads_what_db_query_returns() {
        let rows: Rows = serde_json::from_value(json!({
            "kind": "select", "effect": "read", "columns": ["id"], "rows": [[1], [2]],
            "offset": 0, "more": true, "next_offset": 2, "affected": null, "bytes": 4,
            "values_cut": false, "duration_ms": 3
        }))
        .unwrap();

        assert_eq!(rows.rows.len(), 2);
        assert_eq!(rows.next_offset, Some(2));
        assert!(rows.more);
    }
}
