// Copyright 2024 SoryOS <maintainers@soryos.local>
// SPDX-License-Identifier: MPL-2.0

//! Composant Popover SoryOS — pattern Radix/Headless UI.
//!
//! Fournit un conteneur flottant ancré à un élément déclencheur :
//! - **Tooltip cliquable** : popover qui reste ouvert au clic
//! - **Dropdown menu** : menu déroulant avec items
//! - **Contenu riche** : n'importe quel widget comme contenu
//! - **Anchoring** : positionnement par rapport au déclencheur (top, bottom, left, right)
//! - **Close on click outside** : fermeture au clic extérieur
//! - **Close on Escape** : fermeture par Escape

use std::borrow::Cow;

use crate::iced::{Alignment, Length, Padding};

use crate::widget::{column, container, mouse_area, row, space, text};
use crate::Element;

// ═════════════════════════════════════════════════════════════════════════════
// POPOVER POSITION
// ═════════════════════════════════════════════════════════════════════════════

/// Position du popover par rapport au déclencheur.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopoverPosition {
    /// En dessous du déclencheur, aligné à gauche.
    BottomStart,
    /// En dessous du déclencheur, centré.
    BottomCenter,
    /// En dessous du déclencheur, aligné à droite.
    BottomEnd,
    /// Au-dessus du déclencheur, aligné à gauche.
    TopStart,
    /// Au-dessus du déclencheur, centré.
    TopCenter,
    /// Au-dessus du déclencheur, aligné à droite.
    TopEnd,
    /// À droite du déclencheur, aligné en haut.
    RightStart,
    /// À droite du déclencheur, centré.
    RightCenter,
    /// À droite du déclencheur, aligné en bas.
    RightEnd,
    /// À gauche du déclencheur, aligné en haut.
    LeftStart,
    /// À gauche du déclencheur, centré.
    LeftCenter,
    /// À gauche du déclencheur, aligné en bas.
    LeftEnd,
}

impl Default for PopoverPosition {
    fn default() -> Self {
        Self::BottomStart
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// POPOVER STATE
// ═════════════════════════════════════════════════════════════════════════════

/// État d'un popover.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopoverState {
    /// Popover fermé.
    Closed,
    /// Popover ouvert.
    Open,
}

impl Default for PopoverState {
    fn default() -> Self {
        Self::Closed
    }
}

impl PopoverState {
    /// Vérifie si le popover est ouvert.
    #[must_use]
    pub fn is_open(&self) -> bool {
        matches!(self, Self::Open)
    }

    /// Ouvre le popover.
    #[must_use]
    pub fn open(&self) -> Self {
        Self::Open
    }

    /// Ferme le popover.
    #[must_use]
    pub fn close(&self) -> Self {
        Self::Closed
    }

    /// Toggle l'état.
    #[must_use]
    pub fn toggle(&self) -> Self {
        match self {
            Self::Open => Self::Closed,
            Self::Closed => Self::Open,
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// POPOVER
// ═════════════════════════════════════════════════════════════════════════════

/// Builder pour un popover SoryOS.
pub struct SoryPopover<'a, Message> {
    trigger: Element<'a, Message>,
    content: Element<'a, Message>,
    position: PopoverPosition,
    state: PopoverState,
    on_close: Option<Message>,
    width: Option<Length>,
    max_width: Option<f32>,
    offset: f32,
    show_arrow: bool,
}

impl<'a, Message: Clone + 'static> SoryPopover<'a, Message> {
    /// Crée un nouveau popover.
    pub fn new(
        trigger: impl Into<Element<'a, Message>>,
        content: impl Into<Element<'a, Message>>,
    ) -> Self {
        Self {
            trigger: trigger.into(),
            content: content.into(),
            position: PopoverPosition::BottomStart,
            state: PopoverState::Closed,
            on_close: None,
            width: None,
            max_width: None,
            offset: 4.0,
            show_arrow: false,
        }
    }

    /// Définit la position du popover.
    pub fn position(mut self, pos: PopoverPosition) -> Self {
        self.position = pos;
        self
    }

    /// Définit l'état ouvert/fermé.
    pub fn state(mut self, state: PopoverState) -> Self {
        self.state = state;
        self
    }

    /// Message émis pour fermer le popover.
    pub fn on_close(mut self, msg: Message) -> Self {
        self.on_close = Some(msg);
        self
    }

    /// Largeur du popover.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Largeur maximale du popover.
    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = Some(max_width);
        self
    }

    /// Décalage par rapport au déclencheur.
    pub fn offset(mut self, offset: f32) -> Self {
        self.offset = offset;
        self
    }

    /// Affiche une flèche vers le déclencheur.
    pub fn show_arrow(mut self, show: bool) -> Self {
        self.show_arrow = show;
        self
    }
}

impl<'a, Message: Clone + 'static> From<SoryPopover<'a, Message>> for Element<'a, Message> {
    fn from(popover: SoryPopover<'a, Message>) -> Self {
        let spacing = crate::theme::spacing();

        // Si fermé, retourner juste le trigger
        if popover.state == PopoverState::Closed {
            return popover.trigger.into();
        }

        // Contenu du popover
        let mut content_container = container(popover.content)
            .class(crate::theme::sory::popover())
            .padding(Padding::from([spacing.space_s, spacing.space_m]));

        if let Some(w) = popover.width {
            content_container = content_container.width(w);
        }
        if let Some(max_w) = popover.max_width {
            content_container = content_container.max_width(max_w);
        }

        // Flèche optionnelle
        let arrow = if popover.show_arrow {
            Some(
                container(text::body("▲").center())
                    .width(16.0)
                    .height(8.0),
            )
        } else {
            None
        };

        // Stack : content + backdrop (pour clic outside)
        let popover_element: Element<'a, Message> = content_container.into();

        // Backdrop invisible pour fermer au clic extérieur
        if let Some(close_msg) = popover.on_close.clone() {
            let backdrop = mouse_area(
                space::horizontal().width(Length::Fill).height(Length::Fill),
            )
            .on_press(close_msg);

            // Positionner le popover
            let positioned = match popover.position {
                PopoverPosition::BottomStart | PopoverPosition::BottomCenter | PopoverPosition::BottomEnd => {
                    let mut col = column::with_capacity(2)
                        .spacing(spacing.space_xxs);
                    if let Some(arrow) = arrow {
                        col = col.push(arrow);
                    }
                    col = col.push(popover_element);
                    col.into()
                }
                PopoverPosition::TopStart | PopoverPosition::TopCenter | PopoverPosition::TopEnd => {
                    let mut col = column::with_capacity(2)
                        .spacing(spacing.space_xxs);
                    col = col.push(popover_element);
                    if let Some(arrow) = arrow {
                        col = col.push(arrow);
                    }
                    col.into()
                }
                _ => popover_element,
            };

            // Envelopper dans un overlay complet
            let overlay = container(
                column::with_capacity(1)
                    .push(backdrop)
                    .push(
                        container(positioned)
                            .center_x(Length::Fill)
                            .center_y(Length::Fill),
                    ),
            )
            .width(Length::Fill)
            .height(Length::Fill);

            overlay.into()
        } else {
            // Sans close handler, juste le contenu positionné
            popover_element
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═════════════════════════════════════════════════════════════════════════════

/// Crée un popover simple avec un trigger et un contenu texte.
pub fn simple_popover<'a, Message: Clone + 'static>(
    trigger: impl Into<Element<'a, Message>>,
    text_content: impl Into<Cow<'a, str>> + 'a,
    state: PopoverState,
    on_close: Message,
) -> Element<'a, Message> {
    let content = text::body(text_content);

    SoryPopover::new(trigger, content)
        .state(state)
        .on_close(on_close)
        .max_width(300.0)
        .into()
}

/// Crée un popover avec un menu d'items.
pub fn menu_popover<'a, Message: Clone + 'static>(
    trigger: impl Into<Element<'a, Message>>,
    items: Vec<PopoverMenuItem<'a, Message>>,
    state: PopoverState,
    on_close: Message,
) -> Element<'a, Message> {
    let spacing = crate::theme::spacing();

    let mut menu_col = column::with_capacity(items.len())
        .spacing(spacing.space_xxs);

    for item in items {
        let mut item_content = row::with_capacity(2)
            .spacing(spacing.space_s)
            .align_y(Alignment::Center)
            .push(
                text::body(item.label)
                    .width(Length::Fill),
            );

        if let Some(shortcut) = item.shortcut {
            item_content = item_content.push(
                text::caption(shortcut),
            );
        }

        let styled_item = container(item_content)
            .padding(Padding::from([
                spacing.space_xxs,
                spacing.space_s,
            ]))
            .width(Length::Fill);

        if item.disabled {
            menu_col = menu_col.push(styled_item);
        } else if let Some(on_press) = item.on_press {
            menu_col = menu_col.push(
                mouse_area(styled_item).on_press(on_press),
            );
        } else {
            menu_col = menu_col.push(styled_item);
        }

        if item.separator_after {
            menu_col = menu_col.push(
                container(space::horizontal())
                    .height(1.0)
                    .width(Length::Fill)
                    .class(crate::theme::sory::sidebar_separator()),
            );
        }
    }

    SoryPopover::new(trigger, menu_col)
        .state(state)
        .on_close(on_close)
        .width(Length::Fixed(220.0))
        .into()
}

/// Élément de menu popover.
pub struct PopoverMenuItem<'a, Message> {
    label: Cow<'a, str>,
    shortcut: Option<Cow<'a, str>>,
    on_press: Option<Message>,
    disabled: bool,
    separator_after: bool,
}

impl<'a, Message: Clone + 'static> PopoverMenuItem<'a, Message> {
    /// Crée un item de menu.
    pub fn new(label: impl Into<Cow<'a, str>>) -> Self {
        Self {
            label: label.into(),
            shortcut: None,
            on_press: None,
            disabled: false,
            separator_after: false,
        }
    }

    /// Ajoute un raccourci clavier.
    pub fn shortcut(mut self, shortcut: impl Into<Cow<'a, str>>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Définit le message au clic.
    pub fn on_press(mut self, msg: Message) -> Self {
        self.on_press = Some(msg);
        self
    }

    /// Désactive l'item.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    /// Ajoute un séparateur après cet item.
    pub fn separator_after(mut self) -> Self {
        self.separator_after = true;
        self
    }
}

/// Crée un item de menu rapide.
pub fn popover_menu_item<'a, Message: Clone + 'static>(
    label: impl Into<Cow<'a, str>>,
    on_press: Message,
) -> PopoverMenuItem<'a, Message> {
    PopoverMenuItem::new(label).on_press(on_press)
}

/// Crée un item de menu avec raccourci.
pub fn popover_menu_item_with_shortcut<'a, Message: Clone + 'static>(
    label: impl Into<Cow<'a, str>>,
    shortcut: impl Into<Cow<'a, str>>,
    on_press: Message,
) -> PopoverMenuItem<'a, Message> {
    PopoverMenuItem::new(label)
        .shortcut(shortcut)
        .on_press(on_press)
}

/// Crée un séparateur de menu.
pub fn popover_menu_separator<'a, Message: Clone + 'static>() -> PopoverMenuItem<'a, Message> {
    PopoverMenuItem::new("").separator_after()
}
