use crate::error::Error;
use crate::wit::synt::extension::dialogs;

pub fn ask_secret(secret: &str, title: &str) -> Result<bool, Error> {
    Ok(dialogs::ask_secret(secret, title)?)
}
