use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::error::Error;
use crate::json;
use crate::wit::synt::extension::host;

pub fn call<T: DeserializeOwned>(
    action: &str,
    target: Option<&str>,
    params: &(impl Serialize + ?Sized),
) -> Result<T, Error> {
    let params = json::encode(params)?;
    let answer = host::call(action, target, &params)?;

    json::decode(&answer)
}
