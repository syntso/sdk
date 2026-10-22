mod button;
mod id;
mod tone;

use crate::binding::Binding;
use crate::error::Error;
use crate::wit::synt::extension::notifications;

pub use button::Button;
pub use id::NotificationId;
pub use tone::Tone;

#[derive(Debug, Clone, PartialEq)]
pub struct Notification {
    tone: Tone,
    title: String,
    body: Option<String>,
    buttons: Vec<Button>,
}

impl Notification {
    pub fn new(tone: Tone, title: impl Into<String>) -> Self {
        Self {
            tone,
            title: title.into(),
            body: None,
            buttons: Vec::new(),
        }
    }

    pub fn info(title: impl Into<String>) -> Self {
        Self::new(Tone::Info, title)
    }

    pub fn success(title: impl Into<String>) -> Self {
        Self::new(Tone::Success, title)
    }

    pub fn warning(title: impl Into<String>) -> Self {
        Self::new(Tone::Warning, title)
    }

    pub fn error(title: impl Into<String>) -> Self {
        Self::new(Tone::Error, title)
    }

    #[must_use]
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    #[must_use]
    pub fn button(mut self, title: impl Into<String>, binding: Binding) -> Self {
        self.buttons.push(Button::new(title, binding));
        self
    }

    pub fn show(self) -> Result<NotificationId, Error> {
        let id = notifications::notify(&self.into())?;

        Ok(NotificationId::new(id))
    }
}

impl From<Notification> for notifications::Notification {
    fn from(notification: Notification) -> Self {
        Self {
            tone: notification.tone.into(),
            title: notification.title,
            body: notification.body,
            buttons: notification.buttons.into_iter().map(Into::into).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn carries_tone_body_and_buttons() {
        let notification = Notification::warning("3 containers unhealthy")
            .body("web-1, web-2, worker")
            .button(
                "Restart",
                Binding::action("docker.restart").target("@orders/web-1"),
            );

        let wire = notifications::Notification::from(notification);

        assert_eq!(wire.tone, notifications::Tone::Warning);
        assert_eq!(wire.title, "3 containers unhealthy");
        assert_eq!(wire.body.as_deref(), Some("web-1, web-2, worker"));
        assert_eq!(wire.buttons[0].title, "Restart");
        assert_eq!(wire.buttons[0].binding.action, "docker.restart");
    }

    #[test]
    fn every_tone_has_a_shortcut() {
        let tones = [
            Notification::info("a"),
            Notification::success("b"),
            Notification::warning("c"),
            Notification::error("d"),
        ]
        .map(|notification| notifications::Notification::from(notification).tone);

        assert_eq!(
            tones,
            [
                notifications::Tone::Info,
                notifications::Tone::Success,
                notifications::Tone::Warning,
                notifications::Tone::Error,
            ]
        );
    }
}
