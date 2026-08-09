//! Carte moderne SoryOS avec animations hover/press et états visuels.

use crate::iced::{Alignment, Length, Padding};
use crate::widget::anim::{self, AnimPreset};
use crate::widget::{column, container, mouse_area, row, text};
use crate::Element;

/// Carte moderne avec animation hover/press intégrée.
pub struct ModernCard<'a, Message> {
    header: Option<Element<'a, Message>>,
    body: Element<'a, Message>,
    footer: Option<Element<'a, Message>>,
    on_press: Option<Message>,
    width: Length,
    height: Length,
    padding: Padding,
    selected: bool,
}

impl<'a, Message: Clone + 'static> ModernCard<'a, Message> {
    pub fn new(body: impl Into<Element<'a, Message>>) -> Self {
        Self {
            header: None,
            body: body.into(),
            footer: None,
            on_press: None,
            width: Length::Fill,
            height: Length::Shrink,
            padding: Padding::from([16, 16]),
            selected: false,
        }
    }

    pub fn header(mut self, header: impl Into<Element<'a, Message>>) -> Self {
        self.header = Some(header.into());
        self
    }

    pub fn footer(mut self, footer: impl Into<Element<'a, Message>>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    pub fn on_press(mut self, msg: Message) -> Self {
        self.on_press = Some(msg);
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl<'a, Message: Clone + 'static> From<ModernCard<'a, Message>> for Element<'a, Message> {
    fn from(card: ModernCard<'a, Message>) -> Self {
        let spacing = crate::theme::spacing();

        let mut col = column::with_capacity(3).spacing(spacing.space_s);

        if let Some(header) = card.header {
            col = col.push(header);
        }

        col = col.push(container(card.body).width(Length::Fill));

        if let Some(footer) = card.footer {
            col = col.push(
                container(footer)
                    .width(Length::Fill)
                    .padding(Padding::from([spacing.space_xxs, 0, 0, 0])),
            );
        }

        let card_class = if card.selected {
            crate::theme::sory::folder_card_selected()
        } else {
            crate::theme::sory::folder_card()
        };

        let container = container(col)
            .class(card_class)
            .padding(card.padding)
            .width(card.width)
            .height(card.height);

        let base: Element<'_, Message> = if let Some(on_press) = card.on_press {
            mouse_area(container).on_press(on_press).into()
        } else {
            container.into()
        };

        anim::animated(base)
            .preset(AnimPreset::Lift {
                hover_scale: 1.02,
                press_scale: 0.98,
                hover_lift: -2.0,
            })
            .into()
    }
}
