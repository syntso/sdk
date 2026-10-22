use serde::de::DeserializeOwned;

use crate::error::Error;
use crate::json;
use crate::wit::synt::extension::host;

pub fn settings<T: DeserializeOwned>() -> Result<T, Error> {
    json::object(&host::settings())
}
