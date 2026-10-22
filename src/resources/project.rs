use crate::wit::synt::extension::resources;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Project {
    pub id: String,
    pub name: String,
}

impl Project {
    pub fn all() -> Vec<Self> {
        resources::projects().into_iter().map(Self::from).collect()
    }
}

impl From<resources::Project> for Project {
    fn from(project: resources::Project) -> Self {
        Self {
            id: project.id,
            name: project.name,
        }
    }
}
