use serde::Serialize;

use crate::error::Error;
use crate::json;
use crate::view::Patch;
use crate::wit::synt::extension::ui;

pub fn patch_view(view: &str, patch: &Patch) -> Result<(), Error> {
    Ok(ui::patch_view(view, &patch.to_json())?)
}

pub fn refresh_tree(tree: &str, item: Option<&str>) -> Result<(), Error> {
    Ok(ui::refresh_tree(tree, item)?)
}

pub fn post(panel: &str, message: &(impl Serialize + ?Sized)) -> Result<(), Error> {
    Ok(ui::post(panel, &json::encode(message)?)?)
}
