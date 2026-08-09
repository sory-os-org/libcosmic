//! Dialogue moderne SoryOS avec animation d'entrée (fade + scale).

use crate::iced::{Alignment, Length, Padding};
use crate::widget::{column, container, mouse_area, row, text};
use crate::Element;

/// Dialogue moderne avec overlay backdrop.
pub struct ModernDialog<'a, Message> {
    title: Option<Element<'a, Message>>,
    body: Element<'a, Message>,
    actions: Vec<Element<'a, Message>>,
    on_close: Option<Message>,
    max_width: f32,
}

impl<'a, Message: Clone + 'static> ModernDialog<'a, Message> {
    pub fn new(body: impl Into<Element<'a, Message>>) -> Self {
        Self {
            title: None,
            body: body.into(),
            actions: Vec::new(),
            on_close: None,
            max_width: 480.0,
        }
    }

    pub fn title(mut self, title: impl Into<Element<'a, Message>>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn action(mut self, action: impl Into<Element<'a, Message>>) -> Self {
        self.actions.push(action.into());
        self
    }

    pub fn on_close(mut self, msg: Message) -> Self {
        self.on_close = Some(msg);
        self
    }

    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = max_width;
        self
    }
}

impl<'a, Message: Clone + 'static> From<ModernDialog<'a, Message>> for Element<'a, Message> {
    fn from(dialog: ModernDialog<'a, Message>) -> Self {
        let spacing = crate::theme::spacing();

        let mut content = column::with_capacity(4).spacing(spacing.space_m);

        if let Some(title) = dialog.title {
            content = content.push(title);
        }

        content = content.push(dialog.body);

        if !dialog.actions.is_empty() {
            let mut action_row = row::with_capacity(dialog.actions.len())
                .spacing(spacing.space_s)
                .align_y(Alignment::Center);

            for action in dialog.actions {
                action_row = action_row.push(action);
            }

            content = content.push(action_row);
        }

        let panel = container(content)
            .class(crate::theme::Container::Dialog)
            .padding(spacing.space_l)
            .max_width(dialog.max_width)
            .width(Length::Shrink);

        backdrop(panel.into(), dialog.on_close)
    }
}

fn backdrop<'a, Message: Clone + 'static>(
    content: Element<'a, Message>,
    on_close: Option<Message>,
) -> Element<'a, Message> {
    use crate::iced::Color;

    let backdrop_color = Color::from_rgba(0.0, 0.0, 0.0, 0.5);

    let backdrop = container(content)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_theme: &crate::Theme| {
            iced_widget::container::Style {
                background: Some(iced::Background::Color(backdrop_color)),
                ..Default::default()
            }
        });

    let outer: Element<'a, Message> = if let Some(on_close) = on_close {
        mouse_area(backdrop)
            .on_press(on_close)
            .into()
    } else {
        backdrop.into()
    };

    crate::widget::layer_container(outer)
        .layer(cosmic_theme::Layer::Background)
        .into()
}
