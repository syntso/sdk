use std::time::Duration;

use crate::error::Error;
use crate::seconds::seconds;
use crate::wit::synt::extension::timers;

pub fn set(name: &str, every: Duration) -> Result<(), Error> {
    Ok(timers::set(name, seconds(every))?)
}

pub fn clear(name: &str) {
    timers::clear(name);
}
