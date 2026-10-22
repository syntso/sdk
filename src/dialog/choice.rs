use crate::wit::synt::extension::dialogs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Choice {
    id: String,
    title: String,
    detail: Option<String>,
}

impl Choice {
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            detail: None,
        }
    }

    #[must_use]
    pub fn detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }
}

impl From<Choice> for dialogs::Choice {
    fn from(choice: Choice) -> Self {
        Self {
            id: choice.id,
            title: choice.title,
            detail: choice.detail,
        }
    }
}
