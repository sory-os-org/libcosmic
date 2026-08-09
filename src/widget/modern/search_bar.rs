//! Barre de recherche moderne SoryOS.

use crate::iced::{Length, Padding};
use crate::widget::{container, row, text, text_input};
use crate::Element;
use std::borrow::Cow;

/// Barre de recherche moderne.
pub struct ModernSearchBar<'a, Message> {
    value: Cow<'a, str>,
    on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
    placeholder: Cow<'a, str>,
    width: Length,
}

impl<'a, Message: 'static> ModernSearchBar<'a, Message> {
    pub fn new() -> Self {
        Self {
            value: Cow::Borrowed(""),
            on_input: None,
            placeholder: Cow::Borrowed("Rechercher..."),
            width: Length::Fill,
        }
    }

    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.value = value.into();
        self
    }

    pub fn on_input(mut self, f: impl Fn(String) -> Message + 'a) -> Self {
        self.on_input = Some(Box::new(f));
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<Cow<'a, str>>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

impl<'a, Message: Clone + 'static> From<ModernSearchBar<'a, Message>> for Element<'a, Message> {
    fn from(sb: ModernSearchBar<'a, Message>) -> Self {
        let mut input = text_input::search_input(sb.placeholder, sb.value)
            .width(sb.width);

        if let Some(on_input) = sb.on_input {
            input = input.on_input(on_input);
        }

        container(input)
            .padding(Padding::from([4, 8]))
            .class(crate::theme::Container::Card)
            .into()
    }
}
