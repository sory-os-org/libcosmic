//! Fil d'Ariane (breadcrumbs) moderne SoryOS.

use crate::iced::{Alignment, Length, Padding};
use crate::widget::{container, mouse_area, row, text};
use crate::Element;
use std::borrow::Cow;

/// Un élément de breadcrumb.
pub struct BreadcrumbItem<'a, Message> {
    label: Cow<'a, str>,
    on_press: Option<Message>,
    icon: Option<Element<'a, Message>>,
}

/// Breadcrumbs modernes.
pub struct Breadcrumbs<'a, Message> {
    items: Vec<BreadcrumbItem<'a, Message>>,
    separator: String,
}

impl<'a, Message: Clone + 'static> Breadcrumbs<'a, Message> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            separator: "›".to_string(),
        }
    }

    pub fn separator(mut self, sep: impl Into<String>) -> Self {
        self.separator = sep.into();
        self
    }

    pub fn push_item(
        mut self,
        label: impl Into<Cow<'a, str>>,
        on_press: Option<Message>,
    ) -> Self {
        self.items.push(BreadcrumbItem {
            label: label.into(),
            on_press,
            icon: None,
        });
        self
    }

    pub fn push_item_with_icon(
        mut self,
        label: impl Into<Cow<'a, str>>,
        icon: Element<'a, Message>,
        on_press: Option<Message>,
    ) -> Self {
        self.items.push(BreadcrumbItem {
            label: label.into(),
            on_press,
            icon: Some(icon),
        });
        self
    }

    pub fn push_home(mut self, on_press: Message) -> Self {
        self.items.push(BreadcrumbItem {
            label: Cow::Borrowed("Accueil"),
            on_press: Some(on_press),
            icon: Some(crate::widget::text::body("🏠").into()),
        });
        self
    }
}

impl<'a, Message: Clone + 'static> From<Breadcrumbs<'a, Message>> for Element<'a, Message> {
    fn from(breadcrumbs: Breadcrumbs<'a, Message>) -> Self {
        let spacing = crate::theme::spacing();
        let mut row = row::with_capacity(breadcrumbs.items.len() * 2)
            .spacing(spacing.space_xxs)
            .align_y(Alignment::Center);

        let num_items = breadcrumbs.items.len();

        for (i, item) in breadcrumbs.items.into_iter().enumerate() {
            if i > 0 {
                row = row.push(
                    text::caption(breadcrumbs.separator.clone())
                        .width(Length::Shrink),
                );
            }

            let label_str = item.label.into_owned();
            let mut content = row::with_capacity(2)
                .spacing(spacing.space_xxs)
                .align_y(Alignment::Center);

            if let Some(icon) = item.icon {
                content = content.push(icon);
            }
            content = content.push(text::caption(label_str));

            if let Some(on_press) = item.on_press {
                row = row.push(
                    mouse_area(content)
                        .on_press(on_press),
                );
            } else {
                row = row.push(content);
            }
        }

        row.into()
    }
}
