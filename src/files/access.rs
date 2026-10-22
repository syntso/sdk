use crate::wit::synt::extension::files;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Access {
    Read,
    ReadWrite,
}

impl From<files::Access> for Access {
    fn from(access: files::Access) -> Self {
        match access {
            files::Access::Read => Self::Read,
            files::Access::ReadWrite => Self::ReadWrite,
        }
    }
}

impl From<Access> for files::Access {
    fn from(access: Access) -> Self {
        match access {
            Access::Read => Self::Read,
            Access::ReadWrite => Self::ReadWrite,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_both_ways() {
        for access in [Access::Read, Access::ReadWrite] {
            assert_eq!(Access::from(files::Access::from(access)), access);
        }
    }
}
