#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Input {
    Text,
    Number,
    Toggle,
    Select,
    Textarea,
}

impl Input {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Number => "number",
            Self::Toggle => "toggle",
            Self::Select => "select",
            Self::Textarea => "textarea",
        }
    }
}
