use serde_json::Value;

use crate::error::Error;
use crate::event::Event;
use crate::invocation::Invocation;
use crate::tree_item::TreeItem;
use crate::view::Node;

pub trait Extension {
    fn run(invocation: &Invocation) -> Result<Value, Error>;

    fn start() -> Result<(), Error> {
        Ok(())
    }

    fn render(view: &str, _target: Option<&str>, _params: &Value) -> Result<Node, Error> {
        Err(Error::new("unknown_view", view))
    }

    fn children(tree: &str, _parent: Option<&str>) -> Result<Vec<TreeItem>, Error> {
        Err(Error::new("unknown_tree", tree))
    }

    fn panel_message(panel: &str, _message: &Value) -> Result<Value, Error> {
        Err(Error::new("unknown_panel", panel))
    }

    fn asset(_path: &str) -> Option<Vec<u8>> {
        None
    }

    fn on_event(_event: &Event) -> Result<(), Error> {
        Ok(())
    }

    fn on_timer(_name: &str) -> Result<(), Error> {
        Ok(())
    }

    fn merge(
        kind: &str,
        _base: Option<&[u8]>,
        _ours: &[u8],
        _theirs: &[u8],
    ) -> Result<Vec<u8>, Error> {
        Err(Error::new(
            "no_merge",
            format!("this extension cannot merge {kind} documents"),
        ))
    }
}
