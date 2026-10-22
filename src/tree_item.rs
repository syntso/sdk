use crate::binding::Binding;
use crate::wit::synt::extension::types;

#[derive(Debug, Clone, PartialEq)]
pub struct TreeItem {
    id: String,
    title: String,
    subtitle: Option<String>,
    icon: Option<String>,
    expandable: bool,
    open: Option<Binding>,
    subject: Option<String>,
}

impl TreeItem {
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            subtitle: None,
            icon: None,
            expandable: false,
            open: None,
            subject: None,
        }
    }

    #[must_use]
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    #[must_use]
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    #[must_use]
    pub fn expandable(mut self) -> Self {
        self.expandable = true;
        self
    }

    #[must_use]
    pub fn open(mut self, binding: Binding) -> Self {
        self.open = Some(binding);
        self
    }

    #[must_use]
    pub fn subject(mut self, address: impl Into<String>) -> Self {
        self.subject = Some(address.into());
        self
    }
}

impl From<TreeItem> for types::TreeItem {
    fn from(item: TreeItem) -> Self {
        Self {
            id: item.id,
            title: item.title,
            subtitle: item.subtitle,
            icon: item.icon,
            expandable: item.expandable,
            open: item.open.map(Into::into),
            subject: item.subject,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_an_item_the_sidebar_draws() {
        let item = TreeItem::new("api-1", "api-1")
            .subtitle("3 containers")
            .icon("server")
            .expandable()
            .open(Binding::action("docker.containers").target("@orders/api-1"))
            .subject("@orders/api-1");

        let wire = types::TreeItem::from(item);

        assert_eq!(wire.id, "api-1");
        assert!(wire.expandable);
        assert_eq!(wire.icon.as_deref(), Some("server"));
        assert_eq!(wire.subtitle.as_deref(), Some("3 containers"));
        assert_eq!(wire.open.unwrap().action, "docker.containers");
        assert_eq!(wire.subject.as_deref(), Some("@orders/api-1"));
    }

    #[test]
    fn a_leaf_has_nothing_but_id_and_title() {
        let wire = types::TreeItem::from(TreeItem::new("web-1", "web-1"));

        assert!(!wire.expandable);
        assert!(wire.open.is_none() && wire.subject.is_none() && wire.icon.is_none());
    }
}
