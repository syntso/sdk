use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::error::Error;
use crate::wit::synt::extension::storage;

pub struct Storage;

impl Storage {
    pub fn get(key: &str) -> Option<Vec<u8>> {
        storage::get(key)
    }

    pub fn set(key: &str, value: &[u8]) -> Result<(), Error> {
        Ok(storage::set(key, value)?)
    }

    pub fn remove(key: &str) -> Result<(), Error> {
        Ok(storage::remove(key)?)
    }

    pub fn keys(prefix: &str) -> Result<Vec<String>, Error> {
        Ok(storage::keys(prefix)?)
    }

    pub fn get_json<T: DeserializeOwned>(key: &str) -> Result<Option<T>, Error> {
        Self::get(key)
            .map(|bytes| serde_json::from_slice(&bytes))
            .transpose()
            .map_err(Error::from)
    }

    pub fn set_json(key: &str, value: &(impl Serialize + ?Sized)) -> Result<(), Error> {
        Self::set(key, &serde_json::to_vec(value)?)
    }
}
