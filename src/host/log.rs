use crate::level::Level;
use crate::wit::synt::extension::host;

pub fn log(level: Level, message: &str) {
    host::log(level.into(), message);
}
