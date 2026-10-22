use super::Choice;
use crate::error::Error;
use crate::wit::synt::extension::dialogs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pick {
    title: String,
    choices: Vec<Choice>,
    many: bool,
}

impl Pick {
    pub fn one(title: impl Into<String>) -> Self {
        Self::new(title, false)
    }

    pub fn many(title: impl Into<String>) -> Self {
        Self::new(title, true)
    }

    #[must_use]
    pub fn choice(mut self, choice: Choice) -> Self {
        self.choices.push(choice);
        self
    }

    #[must_use]
    pub fn option(self, id: impl Into<String>, title: impl Into<String>) -> Self {
        self.choice(Choice::new(id, title))
    }

    pub fn ask(self) -> Result<Vec<String>, Error> {
        Ok(dialogs::pick(&self.into())?)
    }

    fn new(title: impl Into<String>, many: bool) -> Self {
        Self {
            title: title.into(),
            choices: Vec::new(),
            many,
        }
    }
}

impl From<Pick> for dialogs::Selection {
    fn from(pick: Pick) -> Self {
        Self {
            title: pick.title,
            choices: pick.choices.into_iter().map(Into::into).collect(),
            many: pick.many,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_one_or_many_of_the_choices() {
        let one = dialogs::Selection::from(
            Pick::one("Container")
                .option("web-1", "web-1")
                .choice(Choice::new("worker", "worker").detail("stopped")),
        );
        let many = dialogs::Selection::from(Pick::many("Hosts").option("a", "A"));

        assert!(!one.many);
        assert_eq!(one.choices.len(), 2);
        assert_eq!(one.choices[1].detail.as_deref(), Some("stopped"));
        assert!(many.many);
        assert_eq!(many.title, "Hosts");
    }
}
