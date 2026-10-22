use std::marker::PhantomData;

use crate::event::Event;
use crate::extension::Extension;
use crate::invocation::Invocation;
use crate::json;
use crate::wit::Guest;
use crate::wit::synt::extension::types;

pub struct Adapter<E>(PhantomData<E>);

impl<E: Extension> Guest for Adapter<E> {
    fn run(invocation: types::Invocation) -> Result<String, types::Error> {
        let answer = E::run(&Invocation::from(invocation))?;

        Ok(answer.to_string())
    }

    fn start() -> Result<(), types::Error> {
        Ok(E::start()?)
    }

    fn render(
        view: String,
        target: Option<String>,
        params: String,
    ) -> Result<String, types::Error> {
        let params = json::value(&params)?;
        let tree = E::render(&view, target.as_deref(), &params)?;

        Ok(tree.to_json())
    }

    fn children(
        tree: String,
        parent: Option<String>,
    ) -> Result<Vec<types::TreeItem>, types::Error> {
        let items = E::children(&tree, parent.as_deref())?;

        Ok(items.into_iter().map(Into::into).collect())
    }

    fn panel_message(panel: String, message: String) -> Result<String, types::Error> {
        let message = json::value(&message)?;
        let answer = E::panel_message(&panel, &message)?;

        Ok(answer.to_string())
    }

    fn asset(path: String) -> Option<Vec<u8>> {
        E::asset(&path)
    }

    fn on_event(event: types::Event) -> Result<(), types::Error> {
        Ok(E::on_event(&Event::from(event))?)
    }

    fn on_timer(name: String) -> Result<(), types::Error> {
        Ok(E::on_timer(&name)?)
    }

    fn merge(
        kind: String,
        base: Option<Vec<u8>>,
        ours: Vec<u8>,
        theirs: Vec<u8>,
    ) -> Result<Vec<u8>, types::Error> {
        E::merge(&kind, base.as_deref(), &ours, &theirs).map_err(types::Error::from)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::*;
    use crate::binding::Binding;
    use crate::error::Error;
    use crate::tree_item::TreeItem;
    use crate::view::{Node, text};

    struct Echo;

    impl Extension for Echo {
        fn run(invocation: &Invocation) -> Result<Value, Error> {
            match invocation.action() {
                "say" => Ok(json!({
                    "target": invocation.target(),
                    "params": invocation.params_value()?,
                    "person": invocation.actor().is_person(),
                })),
                other => Err(Error::unknown_action(other)),
            }
        }

        fn render(view: &str, target: Option<&str>, params: &Value) -> Result<Node, Error> {
            Ok(text(format!(
                "{view} of {} with {params}",
                target.unwrap_or("nothing")
            )))
        }

        fn children(tree: &str, parent: Option<&str>) -> Result<Vec<TreeItem>, Error> {
            let parent = parent.unwrap_or("root");

            Ok(vec![
                TreeItem::new(format!("{parent}/a"), tree)
                    .expandable()
                    .open(Binding::action("echo.say")),
            ])
        }

        fn panel_message(_panel: &str, message: &Value) -> Result<Value, Error> {
            Ok(json!({ "echo": message }))
        }

        fn asset(path: &str) -> Option<Vec<u8>> {
            (path == "index.html").then(|| b"<p>hi</p>".to_vec())
        }

        fn on_event(event: &Event) -> Result<(), Error> {
            match event.topic() {
                "journal" => Ok(()),
                other => Err(Error::new("unexpected", other)),
            }
        }

        fn merge(
            _kind: &str,
            _base: Option<&[u8]>,
            ours: &[u8],
            theirs: &[u8],
        ) -> Result<Vec<u8>, Error> {
            Ok([ours, theirs].concat())
        }
    }

    type Guest = Adapter<Echo>;

    fn invocation(action: &str) -> types::Invocation {
        types::Invocation {
            action: action.to_owned(),
            target: Some("web-1".to_owned()),
            params: r#"{"n":1}"#.to_owned(),
            actor: types::Actor::Person,
            project: None,
        }
    }

    #[test]
    fn runs_through_the_interface() {
        let output = <Guest as crate::wit::Guest>::run(invocation("say")).unwrap();

        assert_eq!(
            serde_json::from_str::<Value>(&output).unwrap(),
            json!({ "target": "web-1", "params": { "n": 1 }, "person": true })
        );
    }

    #[test]
    fn passes_errors_through_the_interface() {
        let error = <Guest as crate::wit::Guest>::run(invocation("shout")).unwrap_err();

        assert_eq!(error.code, "unknown_action");
        assert_eq!(error.message, "shout");
    }

    #[test]
    fn renders_lists_children_and_answers_panels() {
        let rendered = <Guest as crate::wit::Guest>::render(
            "load".to_owned(),
            Some("web-1".to_owned()),
            r#"{"a":1}"#.to_owned(),
        )
        .unwrap();
        let items = <Guest as crate::wit::Guest>::children("hosts".to_owned(), None).unwrap();
        let answer =
            <Guest as crate::wit::Guest>::panel_message("p".to_owned(), "[1]".to_owned()).unwrap();

        assert!(rendered.contains(r#"load of web-1 with {\"a\":1}"#));
        assert_eq!(items[0].id, "root/a");
        assert!(items[0].expandable);
        assert_eq!(answer, r#"{"echo":[1]}"#);
    }

    #[test]
    fn refuses_a_panel_message_that_is_not_json() {
        let error = <Guest as crate::wit::Guest>::panel_message("p".to_owned(), "{".to_owned())
            .unwrap_err();

        assert_eq!(error.code, "invalid_json");
    }

    #[test]
    fn serves_assets_hears_events_and_merges() {
        let event = types::Event {
            topic: "session".to_owned(),
            payload: "{}".to_owned(),
        };

        assert_eq!(
            <Guest as crate::wit::Guest>::asset("index.html".to_owned()),
            Some(b"<p>hi</p>".to_vec())
        );
        assert_eq!(
            <Guest as crate::wit::Guest>::asset("other".to_owned()),
            None
        );
        assert_eq!(
            <Guest as crate::wit::Guest>::on_event(event)
                .unwrap_err()
                .code,
            "unexpected"
        );
        assert_eq!(
            <Guest as crate::wit::Guest>::merge(
                "note".to_owned(),
                None,
                b"a".to_vec(),
                b"b".to_vec()
            )
            .unwrap(),
            b"ab"
        );
    }

    struct Quiet;

    impl Extension for Quiet {
        fn run(_invocation: &Invocation) -> Result<Value, Error> {
            Ok(Value::Null)
        }
    }

    #[test]
    fn defaults_start_quietly_and_refuse_the_rest() {
        type Quietly = Adapter<Quiet>;

        assert!(<Quietly as crate::wit::Guest>::start().is_ok());
        assert!(<Quietly as crate::wit::Guest>::on_timer("tick".to_owned()).is_ok());
        assert_eq!(
            <Quietly as crate::wit::Guest>::render("v".to_owned(), None, String::new())
                .unwrap_err()
                .code,
            "unknown_view"
        );
        assert_eq!(
            <Quietly as crate::wit::Guest>::children("t".to_owned(), None)
                .unwrap_err()
                .code,
            "unknown_tree"
        );
        assert_eq!(
            <Quietly as crate::wit::Guest>::panel_message("p".to_owned(), "{}".to_owned())
                .unwrap_err()
                .code,
            "unknown_panel"
        );
        assert_eq!(
            <Quietly as crate::wit::Guest>::merge("k".to_owned(), None, vec![], vec![])
                .unwrap_err()
                .code,
            "no_merge"
        );
        assert_eq!(
            <Quietly as crate::wit::Guest>::run(invocation("x")).unwrap(),
            "null"
        );
    }
}
