use std::time::Duration;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::error::Error;
use crate::seconds::seconds;
use crate::wit::synt::extension::cache;

pub struct Cache;

impl Cache {
    pub fn get(key: &str) -> Option<Vec<u8>> {
        cache::get(key)
    }

    pub fn set(key: &str, value: &[u8], ttl: Duration) -> Result<(), Error> {
        Ok(cache::put(key, value, seconds(ttl))?)
    }

    pub fn remove(key: &str) {
        cache::remove(key);
    }

    pub fn get_json<T: DeserializeOwned>(key: &str) -> Result<Option<T>, Error> {
        Self::get(key)
            .map(|bytes| serde_json::from_slice(&bytes))
            .transpose()
            .map_err(Error::from)
    }

    pub fn set_json(
        key: &str,
        value: &(impl Serialize + ?Sized),
        ttl: Duration,
    ) -> Result<(), Error> {
        Self::set(key, &serde_json::to_vec(value)?, ttl)
    }
}
