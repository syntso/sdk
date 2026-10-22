use crate::error::Error;
use crate::wit::synt::extension::dialogs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prompt {
    title: String,
    message: Option<String>,
    placeholder: Option<String>,
    initial: Option<String>,
}

impl Prompt {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: None,
            placeholder: None,
            initial: None,
        }
    }

    #[must_use]
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    #[must_use]
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    #[must_use]
    pub fn initial(mut self, initial: impl Into<String>) -> Self {
        self.initial = Some(initial.into());
        self
    }

    pub fn ask(self) -> Result<Option<String>, Error> {
        Ok(dialogs::input(&self.into())?)
    }
}

impl From<Prompt> for dialogs::Prompt {
    fn from(prompt: Prompt) -> Self {
        Self {
            title: prompt.title,
            message: prompt.message,
            placeholder: prompt.placeholder,
            initial: prompt.initial,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn carries_message_placeholder_and_initial_text() {
        let wire = dialogs::Prompt::from(
            Prompt::new("Tag")
                .message("Which image tag?")
                .placeholder("latest")
                .initial("1.4.2"),
        );

        assert_eq!(wire.title, "Tag");
        assert_eq!(wire.message.as_deref(), Some("Which image tag?"));
        assert_eq!(wire.placeholder.as_deref(), Some("latest"));
        assert_eq!(wire.initial.as_deref(), Some("1.4.2"));
    }
}
