use super::{Access, Handle};
use crate::wit::synt::extension::files;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Picked {
    pub handle: Handle,
    pub name: String,
    pub access: Access,
}

impl Picked {
    pub fn chosen() -> Vec<Self> {
        files::chosen().into_iter().map(Self::from).collect()
    }

    pub fn writable(&self) -> bool {
        self.access == Access::ReadWrite
    }
}

impl From<files::Picked> for Picked {
    fn from(picked: files::Picked) -> Self {
        Self {
            handle: Handle::new(picked.handle),
            name: picked.name,
            access: picked.access.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn knows_whether_it_may_write() {
        let picked = Picked::from(files::Picked {
            handle: "f-1".to_owned(),
            name: "plan.txt".to_owned(),
            access: files::Access::ReadWrite,
        });

        assert!(picked.writable());
        assert_eq!(picked.handle.as_str(), "f-1");
        assert_eq!(picked.name, "plan.txt");
    }
}
