#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
    Title,
    Body,
    Muted,
    Mono,
    Error,
    Success,
}

impl Style {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Title => "title",
            Self::Body => "body",
            Self::Muted => "muted",
            Self::Mono => "mono",
            Self::Error => "error",
            Self::Success => "success",
        }
    }
}
