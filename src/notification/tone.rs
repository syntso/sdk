use crate::wit::synt::extension::notifications;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    Info,
    Success,
    Warning,
    Error,
}

impl From<Tone> for notifications::Tone {
    fn from(tone: Tone) -> Self {
        match tone {
            Tone::Info => Self::Info,
            Tone::Success => Self::Success,
            Tone::Warning => Self::Warning,
            Tone::Error => Self::Error,
        }
    }
}
