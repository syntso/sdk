use crate::wit::synt::extension::types;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Actor {
    Person,
    Core,
    Agent(String),
    Runbook(String),
    Extension(String),
}

impl Actor {
    pub fn is_person(&self) -> bool {
        matches!(self, Self::Person)
    }

    pub fn is_agent(&self) -> bool {
        matches!(self, Self::Agent(_))
    }
}

impl From<types::Actor> for Actor {
    fn from(actor: types::Actor) -> Self {
        match actor {
            types::Actor::Person => Self::Person,
            types::Actor::Core => Self::Core,
            types::Actor::Agent(name) => Self::Agent(name),
            types::Actor::Runbook(name) => Self::Runbook(name),
            types::Actor::Extension(id) => Self::Extension(id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_every_actor() {
        assert_eq!(Actor::from(types::Actor::Person), Actor::Person);
        assert_eq!(Actor::from(types::Actor::Core), Actor::Core);
        assert_eq!(
            Actor::from(types::Actor::Runbook("deploy".to_owned())),
            Actor::Runbook("deploy".to_owned())
        );
        assert_eq!(
            Actor::from(types::Actor::Extension("docker".to_owned())),
            Actor::Extension("docker".to_owned())
        );
        assert!(Actor::Person.is_person());
        assert!(Actor::Agent("helper".to_owned()).is_agent());
        assert!(!Actor::Core.is_person());
    }
}
