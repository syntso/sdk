use crate::error::Error;
use crate::wit::synt::extension::dialogs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Confirm {
    title: String,
    message: String,
    label: String,
    destructive: bool,
}

impl Confirm {
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            label: "Confirm".to_owned(),
            destructive: false,
        }
    }

    #[must_use]
    pub fn confirm_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    #[must_use]
    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }

    pub fn ask(self) -> Result<bool, Error> {
        Ok(dialogs::confirm(&self.into())?)
    }
}

impl From<Confirm> for dialogs::Confirmation {
    fn from(confirm: Confirm) -> Self {
        Self {
            title: confirm.title,
            message: confirm.message,
            confirm: confirm.label,
            destructive: confirm.destructive,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asks_with_a_label_and_a_warning() {
        let wire = dialogs::Confirmation::from(
            Confirm::new("Remove web-1?", "The container is stopped first.")
                .confirm_label("Remove")
                .destructive(),
        );

        assert_eq!(wire.title, "Remove web-1?");
        assert_eq!(wire.confirm, "Remove");
        assert!(wire.destructive);
    }

    #[test]
    fn confirms_plainly_by_default() {
        let wire = dialogs::Confirmation::from(Confirm::new("Go?", "Really."));

        assert_eq!(wire.confirm, "Confirm");
        assert!(!wire.destructive);
    }
}
