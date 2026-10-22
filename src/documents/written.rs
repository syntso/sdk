use super::DocumentInfo;
use crate::wit::synt::extension::documents;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Written {
    pub info: DocumentInfo,
    pub merged: bool,
}

impl From<documents::Written> for Written {
    fn from(written: documents::Written) -> Self {
        Self {
            info: written.info.into(),
            merged: written.merged,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn carries_the_info_and_whether_it_merged() {
        let written = Written::from(documents::Written {
            info: documents::DocumentInfo {
                project: "orders".to_owned(),
                path: "plan.note".to_owned(),
                kind: "ext.every.note".to_owned(),
                revision: "r2".to_owned(),
                size: 4,
            },
            merged: true,
        });

        assert!(written.merged);
        assert_eq!(written.info.kind, "ext.every.note");
        assert_eq!(written.info.size, 4);
    }
}
