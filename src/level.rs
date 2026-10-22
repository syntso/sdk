use crate::wit::synt::extension::types;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl From<Level> for types::Level {
    fn from(level: Level) -> Self {
        match level {
            Level::Trace => Self::Trace,
            Level::Debug => Self::Debug,
            Level::Info => Self::Info,
            Level::Warn => Self::Warn,
            Level::Error => Self::Error,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_every_level() {
        assert_eq!(types::Level::from(Level::Trace), types::Level::Trace);
        assert_eq!(types::Level::from(Level::Debug), types::Level::Debug);
        assert_eq!(types::Level::from(Level::Info), types::Level::Info);
        assert_eq!(types::Level::from(Level::Warn), types::Level::Warn);
        assert_eq!(types::Level::from(Level::Error), types::Level::Error);
    }
}
