use serde::Serialize;
use serde_json::Value;

use super::Rows;
use crate::error::Error;
use crate::host::call;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Query {
    #[serde(skip_serializing_if = "Option::is_none")]
    sql: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    params: Vec<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u64>,
}

impl Query {
    pub fn sql(sql: impl Into<String>) -> Self {
        Self::of(Some(sql.into()), None)
    }

    pub fn command<W: Into<String>>(words: impl IntoIterator<Item = W>) -> Self {
        Self::of(None, Some(words.into_iter().map(Into::into).collect()))
    }

    #[must_use]
    pub fn param(mut self, value: impl Into<Value>) -> Self {
        self.params.push(value.into());
        self
    }

    #[must_use]
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    #[must_use]
    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn run(&self, database: &str) -> Result<Rows, Error> {
        call("db.query", Some(database), self)
    }

    fn of(sql: Option<String>, command: Option<Vec<String>>) -> Self {
        Self {
            sql,
            command,
            params: Vec::new(),
            limit: None,
            offset: None,
        }
    }
}

pub fn query(database: &str, sql: &str) -> Result<Rows, Error> {
    Query::sql(sql).run(database)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn sends_a_statement_with_params() {
        let query = Query::sql("SELECT * FROM orders WHERE id = $1")
            .param(7)
            .limit(10)
            .offset(20);

        assert_eq!(
            serde_json::to_value(query).unwrap(),
            json!({ "sql": "SELECT * FROM orders WHERE id = $1", "params": [7], "limit": 10, "offset": 20 })
        );
    }

    #[test]
    fn sends_a_redis_command() {
        assert_eq!(
            serde_json::to_value(Query::command(["GET", "session:1"])).unwrap(),
            json!({ "command": ["GET", "session:1"] })
        );
    }
}
