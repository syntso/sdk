use serde_json::{Map, Value, json};

use super::{Input, Style};
use crate::binding::Binding;

#[derive(Debug, Clone, PartialEq)]
pub struct Node(Map<String, Value>);

pub fn stack() -> Node {
    Node::of("stack")
        .set("direction", "column")
        .set("children", json!([]))
}

pub fn row() -> Node {
    stack().set("direction", "row")
}

pub fn text(text: impl Into<String>) -> Node {
    Node::of("text").set("text", text.into())
}

pub fn title(title: impl Into<String>) -> Node {
    text(title).style(Style::Title)
}

pub fn log<L: Into<String>>(lines: impl IntoIterator<Item = L>) -> Node {
    let lines: Vec<String> = lines.into_iter().map(Into::into).collect();

    Node::of("log").set("lines", lines)
}

pub fn list() -> Node {
    Node::of("list").set("items", json!([]))
}

pub fn table<C: Into<String>>(columns: impl IntoIterator<Item = C>) -> Node {
    let columns: Vec<String> = columns.into_iter().map(Into::into).collect();

    Node::of("table")
        .set("columns", columns)
        .set("rows", json!([]))
}

pub fn keyvalue() -> Node {
    Node::of("keyvalue").set("items", json!([]))
}

pub fn field(name: impl Into<String>, label: impl Into<String>, input: Input) -> Node {
    Node::of("field")
        .set("name", name.into())
        .set("label", label.into())
        .set("input", input.name())
}

pub fn form(submit: impl Into<String>, binding: &Binding) -> Node {
    let mut submit_fields = binding.fields();
    submit_fields.insert("title".to_owned(), Value::String(submit.into()));

    Node::of("form")
        .set("children", json!([]))
        .set("submit", Value::Object(submit_fields))
}

pub fn button(title: impl Into<String>, binding: &Binding) -> Node {
    let mut node = Node::of("button").set("title", title.into());
    node.0.extend(binding.fields());

    node
}

pub fn line_chart() -> Node {
    chart("line")
}

pub fn bar_chart() -> Node {
    chart("bar")
}

fn chart(kind: &str) -> Node {
    Node::of("chart").set("kind", kind).set("series", json!([]))
}

impl Node {
    #[must_use]
    pub fn child(self, child: Node) -> Self {
        self.push("children", Value::Object(child.0))
    }

    #[must_use]
    pub fn children(self, children: impl IntoIterator<Item = Node>) -> Self {
        children.into_iter().fold(self, Self::child)
    }

    #[must_use]
    pub fn style(self, style: Style) -> Self {
        self.set("style", style.name())
    }

    #[must_use]
    pub fn item(self, title: impl Into<String>) -> Self {
        self.push("items", json!({ "title": title.into() }))
    }

    #[must_use]
    pub fn link(self, title: impl Into<String>, binding: &Binding) -> Self {
        let mut item = binding.fields();
        item.insert("title".to_owned(), Value::String(title.into()));

        self.push("items", Value::Object(item))
    }

    #[must_use]
    pub fn pair(self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.push("items", json!({ "key": key.into(), "value": value.into() }))
    }

    #[must_use]
    pub fn row(self, cells: impl IntoIterator<Item = Value>) -> Self {
        let cells: Vec<Value> = cells.into_iter().collect();

        self.push("rows", Value::Array(cells))
    }

    #[must_use]
    pub fn options<O: Into<String>>(self, options: impl IntoIterator<Item = O>) -> Self {
        let options: Vec<String> = options.into_iter().map(Into::into).collect();

        self.set("options", options)
    }

    #[must_use]
    pub fn series(
        self,
        name: impl Into<String>,
        points: impl IntoIterator<Item = (f64, f64)>,
    ) -> Self {
        let points: Vec<[f64; 2]> = points.into_iter().map(|(x, y)| [x, y]).collect();

        self.push("series", json!({ "name": name.into(), "points": points }))
    }

    pub fn into_value(self) -> Value {
        Value::Object(self.0)
    }

    pub fn to_json(&self) -> String {
        Value::Object(self.0.clone()).to_string()
    }

    fn of(kind: &str) -> Self {
        let mut fields = Map::new();
        fields.insert("type".to_owned(), Value::String(kind.to_owned()));

        Self(fields)
    }

    fn set(mut self, field: &str, value: impl Into<Value>) -> Self {
        self.0.insert(field.to_owned(), value.into());
        self
    }

    fn push(mut self, field: &str, value: Value) -> Self {
        match self.0.get_mut(field) {
            Some(Value::Array(items)) => items.push(value),
            _ => {
                self.0.insert(field.to_owned(), Value::Array(vec![value]));
            }
        }

        self
    }
}

impl From<Node> for Value {
    fn from(node: Node) -> Self {
        node.into_value()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn builds_the_view_tree_the_core_checks() {
        let view = stack()
            .child(title("web-1"))
            .child(keyvalue().pair("up", "3 days").pair("load", 0.42))
            .child(line_chart().series("load", [(0.0, 0.4), (1.0, 0.5)]))
            .child(button(
                "Check",
                &Binding::action("every.ping").target("web-1"),
            ));

        assert_eq!(
            view.into_value(),
            json!({
                "type": "stack",
                "direction": "column",
                "children": [
                    { "type": "text", "text": "web-1", "style": "title" },
                    { "type": "keyvalue", "items": [
                        { "key": "up", "value": "3 days" },
                        { "key": "load", "value": 0.42 }
                    ]},
                    { "type": "chart", "kind": "line", "series": [
                        { "name": "load", "points": [[0.0, 0.4], [1.0, 0.5]] }
                    ]},
                    { "type": "button", "title": "Check", "action": "every.ping", "target": "web-1" }
                ]
            })
        );
    }

    #[test]
    fn builds_forms_tables_lists_and_logs() {
        let form = form("Save", &Binding::action("every.save"))
            .child(field("every", "Every", Input::Number))
            .child(field("unit", "Unit", Input::Select).options(["s", "m"]));
        let table = table(["pod", "ready"]).row([json!("api"), json!(true)]);
        let list = list().item("plain").link(
            "open",
            &Binding::action("every.open").params(json!({ "id": 1 })),
        );

        let form = form.into_value();
        assert_eq!(
            form["submit"],
            json!({ "title": "Save", "action": "every.save" })
        );
        assert_eq!(form["children"][0]["input"], "number");
        assert_eq!(form["children"][1]["options"], json!(["s", "m"]));
        assert_eq!(table.into_value()["rows"], json!([["api", true]]));
        assert_eq!(
            list.into_value()["items"][1],
            json!({ "title": "open", "action": "every.open", "params": { "id": 1 } })
        );
        assert_eq!(
            serde_json::from_str::<Value>(&log(["a", "b"]).to_json()).unwrap(),
            json!({ "type": "log", "lines": ["a", "b"] })
        );
    }

    #[test]
    fn rows_bar_charts_and_styles() {
        let view = row()
            .children([
                text("ok").style(Style::Success),
                bar_chart().series("cpu", [(1.0, 2.0)]),
            ])
            .into_value();

        assert_eq!(view["direction"], "row");
        assert_eq!(view["children"][0]["style"], "success");
        assert_eq!(view["children"][1]["kind"], "bar");
    }
}
