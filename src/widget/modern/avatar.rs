//! Avatar moderne SoryOS — icône utilisateur ou image.

use crate::iced::{Length, Padding};
use crate::widget::{container, text};
use crate::Element;
use std::borrow::Cow;

/// Taille d'avatar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvatarSize {
    XS,
    SM,
    MD,
    LG,
    XL,
}

impl Default for AvatarSize {
    fn default() -> Self { Self::MD }
}

/// Avatar moderne avec icône ou image.
pub struct ModernAvatar<'a, Message> {
    label: Cow<'a, str>,
    size: AvatarSize,
    icon: Option<Element<'a, Message>>,
    online: bool,
    color: Option<crate::iced::Color>,
}

impl<'a, Message: 'static> ModernAvatar<'a, Message> {
    pub fn new(label: impl Into<Cow<'a, str>>) -> Self {
        Self {
            label: label.into(),
            size: AvatarSize::MD,
            icon: None,
            online: false,
            color: None,
        }
    }

    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    pub fn icon(mut self, icon: Element<'a, Message>) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn online(mut self, online: bool) -> Self {
        self.online = online;
        self
    }

    pub fn color(mut self, color: crate::iced::Color) -> Self {
        self.color = Some(color);
        self
    }
}

impl<'a, Message: 'static> From<ModernAvatar<'a, Message>> for Element<'a, Message> {
    fn from(avatar: ModernAvatar<'a, Message>) -> Self {
        let (w, h) = match avatar.size {
            AvatarSize::XS => (24.0, 24.0),
            AvatarSize::SM => (32.0, 32.0),
            AvatarSize::MD => (40.0, 40.0),
            AvatarSize::LG => (56.0, 56.0),
            AvatarSize::XL => (80.0, 80.0),
        };

        let initial = avatar.label.chars().next().unwrap_or('?').to_uppercase().to_string();

        let avatar_content = if let Some(icon) = avatar.icon {
            container(icon)
                .width(w)
                .height(h)
        } else {
            container(
                text::body(initial).center(),
            )
            .width(w)
            .height(h)
        };

        avatar_content
            .class(crate::theme::Container::Card)
            .into()
    }
}
