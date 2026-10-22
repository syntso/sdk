use crate::binding::Binding;
use crate::wit::synt::extension::notifications;

#[derive(Debug, Clone, PartialEq)]
pub struct Button {
    title: String,
    binding: Binding,
}

impl Button {
    pub fn new(title: impl Into<String>, binding: Binding) -> Self {
        Self {
            title: title.into(),
            binding,
        }
    }
}

impl From<Button> for notifications::Button {
    fn from(button: Button) -> Self {
        Self {
            title: button.title,
            binding: button.binding.into(),
        }
    }
}
