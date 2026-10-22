use crate::error::Error;
use crate::files::{Access, Picked};
use crate::wit::synt::extension::dialogs;

pub fn choose_file(title: &str, access: Access) -> Result<Option<Picked>, Error> {
    let picked = dialogs::choose_file(title, access.into())?;

    Ok(picked.map(Picked::from))
}
