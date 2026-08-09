//! Dropdown moderne SoryOS — Select/Menu déroulant.

use crate::iced::{Alignment, Length, Padding};
use crate::widget::{column, container, mouse_area, row, scrollable, text};
use crate::Element;
use std::borrow::Cow;

/// Un élément de dropdown.
pub struct DropdownItem<'a, Message> {
    label: Cow<'a, str>,
    icon: Option<Element<'a, Message>>,
    on_select: Option<Message>,
    disabled: bool,
}

/// Dropdown moderne avec items, icônes, séparateurs.
pub struct ModernDropdown<'a, Message> {
    label: Cow<'a, str>,
    items: Vec<DropdownItem<'a, Message>>,
    selected: Option<usize>,
    is_open: bool,
    on_toggle: Option<Message>,
    width: Length,
}

impl<'a, Message: Clone + 'static> ModernDropdown<'a, Message> {
    pub fn new(label: impl Into<Cow<'a, str>>) -> Self {
        Self {
            label: label.into(),
            items: Vec::new(),
            selected: None,
            is_open: false,
            on_toggle: None,
            width: Length::Fill,
        }
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn selected(mut self, index: usize) -> Self {
        self.selected = Some(index);
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.is_open = open;
        self
    }

    pub fn on_toggle(mut self, msg: Message) -> Self {
        self.on_toggle = Some(msg);
        self
    }

    pub fn push_item(
        mut self,
        label: impl Into<Cow<'a, str>>,
        on_select: Message,
    ) -> Self {
        self.items.push(DropdownItem {
            label: label.into(),
            icon: None,
            on_select: Some(on_select),
            disabled: false,
        });
        self
    }

    pub fn push_item_with_icon(
        mut self,
        label: impl Into<Cow<'a, str>>,
        icon: Element<'a, Message>,
        on_select: Message,
    ) -> Self {
        self.items.push(DropdownItem {
            label: label.into(),
            icon: Some(icon),
            on_select: Some(on_select),
            disabled: false,
        });
        self
    }

    pub fn push_disabled_item(mut self, label: impl Into<Cow<'a, str>>) -> Self {
        self.items.push(DropdownItem {
            label: label.into(),
            icon: None,
            on_select: None,
            disabled: true,
        });
        self
    }

    pub fn push_separator(mut self) -> Self {
        self.items.push(DropdownItem {
            label: Cow::Borrowed(""),
            icon: None,
            on_select: None,
            disabled: true,
        });
        self
    }

    pub fn push_danger_item(
        mut self,
        label: impl Into<Cow<'a, str>>,
        on_select: Message,
    ) -> Self {
        self.items.push(DropdownItem {
            label: label.into(),
            icon: None,
            on_select: Some(on_select),
            disabled: false,
        });
        self
    }
}

impl<'a, Message: Clone + 'static> From<ModernDropdown<'a, Message>> for Element<'a, Message> {
    fn from(dropdown: ModernDropdown<'a, Message>) -> Self {
        let spacing = crate::theme::spacing();

        // Compute selected label before consuming
        let selected_label = dropdown.selected
            .and_then(|i| dropdown.items.get(i))
            .map(|item| item.label.clone().into_owned())
            .unwrap_or_else(|| dropdown.label.clone().into_owned());

        // Trigger button
        let trigger = container(
            row::with_capacity(2)
                .spacing(spacing.space_s)
                .align_y(Alignment::Center)
                .push(text::body(selected_label))
                .push(text::caption("▾")),
        )
        .padding(Padding::from([spacing.space_s, spacing.space_m]))
        .width(dropdown.width)
        .class(crate::theme::Container::Card);

        let trigger_element: Element<'_, Message> = if let Some(on_toggle) = dropdown.on_toggle.clone() {
            mouse_area(trigger).on_press(on_toggle).into()
        } else {
            trigger.into()
        };

        if !dropdown.is_open {
            return trigger_element;
        }

        // Dropdown menu
        let mut menu_items = column::with_capacity(dropdown.items.len())
            .spacing(0);

        for item in dropdown.items {
            if item.label.is_empty() && item.on_select.is_none() {
                // Separator
                menu_items = menu_items.push(
                    container(
                        crate::widget::space::horizontal()
                    )
                    .width(Length::Fill)
                    .height(1.0)
                );
            } else {
                let label_str = item.label.into_owned();
                let mut item_content = row::with_capacity(2)
                    .spacing(spacing.space_s)
                    .align_y(Alignment::Center);

                if let Some(icon) = item.icon {
                    item_content = item_content.push(icon);
                }
                item_content = item_content.push(text::body(label_str));

                let item_container = container(item_content)
                    .padding(Padding::from([spacing.space_s, spacing.space_m]))
                    .width(Length::Fill);

                if item.disabled {
                    menu_items = menu_items.push(item_container);
                } else if let Some(on_select) = item.on_select {
                    menu_items = menu_items.push(
                        mouse_area(item_container).on_press(on_select)
                    );
                }
            }
        }

        let menu = container(
            scrollable(menu_items)
                .height(Length::FillPortion(3))
        )
        .padding(spacing.space_xxs)
        .width(dropdown.width)
        .max_height(300.0)
        .class(crate::theme::Container::Card);

        column::with_capacity(2)
            .push(trigger_element)
            .push(menu)
            .into()
    }
}
