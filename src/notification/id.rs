use crate::wit::synt::extension::notifications;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NotificationId(u64);

impl NotificationId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn get(self) -> u64 {
        self.0
    }

    pub fn dismiss(self) {
        notifications::dismiss(self.0);
    }
}
