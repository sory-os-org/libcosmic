// Copyright 2024 SoryOS <maintainers@soryos.local>
// SPDX-License-Identifier: MPL-2.0

//! Cartes avancées SoryOS — patterns shadcn Card.
//!
//! Fournit des composants carte de haute qualité visuelle :
//! - **Structure** : header, body, footer
//! - **États** : normal, hover, selected, disabled
//! - **Effets glow** : dynamiques selon l'état
//! - **Carte cliquable** : avec mouse_area
//! - **Carte sélectionnable** : sélection multiple
//! - **Card group** : groupe de cartes homogènes
//! - **Compact variant** : pour les listes denses
//! - **Info card** : icône + titre + description
//! - **Stat card** : valeur + label + trend
//! - **Animations** : micro-interactions hover/press (optionnel)

use std::borrow::Cow;

use crate::iced::{Alignment, Length, Padding};

use crate::widget::{column, container, mouse_area, row, space, text};
use crate::widget::anim::{self, AnimPreset};
use crate::Element;

// ═════════════════════════════════════════════════════════════════════════════
// ÉTAT DE LA CARTE
// ═════════════════════════════════════════════════════════════════════════════

/// État visuel d'une carte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardState {
    /// État normal.
    Normal,
    /// État survolé.
    Hovered,
    /// État sélectionné (glow bleu actif).
    Selected,
    /// État désactivé.
    Disabled,
}

impl Default for CardState {
    fn default() -> Self {
        Self::Normal
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// CARTE PRINCIPALE
// ═════════════════════════════════════════════════════════════════════════════

/// Builder pour une carte SoryOS.
pub struct SoryCard<'a, Message> {
    header: Option<Element<'a, Message>>,
    body: Element<'a, Message>,
    footer: Option<Element<'a, Message>>,
    state: CardState,
    on_press: Option<Message>,
    on_hover_enter: Option<Message>,
    on_hover_exit: Option<Message>,
    width: Length,
    height: Length,
    max_width: Option<f32>,
    max_height: Option<f32>,
    padding: Padding,
    compact: bool,
    animated: bool,
}

impl<'a, Message: Clone + 'static> SoryCard<'a, Message> {
    /// Crée une nouvelle carte avec un contenu body.
    pub fn new(body: impl Into<Element<'a, Message>>) -> Self {
        Self {
            header: None,
            body: body.into(),
            footer: None,
            state: CardState::Normal,
            on_press: None,
            on_hover_enter: None,
            on_hover_exit: None,
            width: Length::Fill,
            height: Length::Shrink,
            max_width: None,
            max_height: None,
            padding: Padding::from([16, 16]),
            compact: false,
            animated: true,
        }
    }

    /// Crée une carte compacte (padding réduit).
    pub fn compact(body: impl Into<Element<'a, Message>>) -> Self {
        Self {
            header: None,
            body: body.into(),
            footer: None,
            state: CardState::Normal,
            on_press: None,
            on_hover_enter: None,
            on_hover_exit: None,
            width: Length::Fill,
            height: Length::Shrink,
            max_width: None,
            max_height: None,
            padding: Padding::from([10, 12]),
            compact: true,
            animated: true,
        }
    }

    /// Définit le header de la carte.
    pub fn header(mut self, header: impl Into<Element<'a, Message>>) -> Self {
        self.header = Some(header.into());
        self
    }

    /// Définit le footer de la carte.
    pub fn footer(mut self, footer: impl Into<Element<'a, Message>>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    /// Définit l'état de la carte.
    pub fn state(mut self, state: CardState) -> Self {
        self.state = state;
        self
    }

    /// Rend la carte cliquable.
    pub fn on_press(mut self, msg: Message) -> Self {
        self.on_press = Some(msg);
        self
    }

    /// Message au survol entrant.
    pub fn on_hover_enter(mut self, msg: Message) -> Self {
        self.on_hover_enter = Some(msg);
        self
    }

    /// Message au survol sortant.
    pub fn on_hover_exit(mut self, msg: Message) -> Self {
        self.on_hover_exit = Some(msg);
        self
    }

    /// Définit la largeur.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Définit la hauteur.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Définit la largeur maximale.
    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = Some(max_width);
        self
    }

    /// Définit la hauteur maximale.
    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = Some(max_height);
        self
    }

    /// Définit le padding interne.
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Mode compact (padding réduit).
    pub fn compact_mode(mut self, compact: bool) -> Self {
        self.compact = compact;
        if compact {
            self.padding = Padding::from([10, 12]);
        }
        self
    }

    /// Active ou désactive les micro-interactions animées (hover scale, press scale).
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }
}

impl<'a, Message: Clone + 'static> From<SoryCard<'a, Message>> for Element<'a, Message> {
    fn from(card: SoryCard<'a, Message>) -> Self {
        let spacing = crate::theme::spacing();

        // ── Style selon l'état ────────────────────────────────────────
        let card_class = match card.state {
            CardState::Normal => crate::theme::sory::folder_card(),
            CardState::Hovered => crate::theme::sory::folder_card_hover(),
            CardState::Selected => crate::theme::sory::folder_card_selected(),
            CardState::Disabled => crate::theme::sory::tile(),
        };

        // ── Construction du contenu ───────────────────────────────────
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

        // ── Assemblage ────────────────────────────────────────────────
        let mut card_container = container(col)
            .class(card_class)
            .padding(card.padding)
            .width(card.width)
            .height(card.height);

        if let Some(max_w) = card.max_width {
            card_container = card_container.max_width(max_w);
        }
        if let Some(max_h) = card.max_height {
            card_container = card_container.max_height(max_h);
        }

        // ── Cliquable + hover ────────────────────────────────────────
        let mut mouse = mouse_area(card_container);

        if let Some(on_press) = card.on_press {
            mouse = mouse.on_press(on_press);
        }

        // Note: hover enter/exit supporté via message mapping côté app
        // Ex: mouse.on_enter(Msg::CardHoverEnter(id)).on_exit(Msg::CardHoverExit(id))

        let base: Element<'_, Message> = mouse.into();

        // ── Animations micro-interactions (optionnel) ────────────────
        if card.animated {
            anim::animated(base)
                .preset(AnimPreset::Lift {
                    hover_scale: 1.02,
                    press_scale: 0.98,
                    hover_lift: -2.0,
                })
                .into()
        } else {
            base
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// CARTE AVEC OVERLAY D'ACTION
// ═════════════════════════════════════════════════════════════════════════════

/// Builder pour une carte avec un overlay d'action au survol.
pub struct SoryCardOverlay<'a, Message> {
    card: SoryCard<'a, Message>,
    overlay: Element<'a, Message>,
}

impl<'a, Message: Clone + 'static> SoryCardOverlay<'a, Message> {
    /// Crée une carte avec overlay.
    pub fn new(
        body: impl Into<Element<'a, Message>>,
        overlay: impl Into<Element<'a, Message>>,
    ) -> Self {
        Self {
            card: SoryCard::new(body),
            overlay: overlay.into(),
        }
    }

    /// Passe au builder du card interne.
    pub fn card(self) -> SoryCard<'a, Message> {
        self.card
    }
}

impl<'a, Message: Clone + 'static> From<SoryCardOverlay<'a, Message>> for Element<'a, Message> {
    fn from(overlay_card: SoryCardOverlay<'a, Message>) -> Self {
        let spacing = crate::theme::spacing();

        // Layer avec le contenu et l'overlay
        let stack = crate::iced::widget::Stack::with_children(vec![
            Element::from(overlay_card.card),
            container(overlay_card.overlay)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(spacing.space_s)
                .align_x(Alignment::End)
                .align_y(Alignment::Start)
                .into(),
        ]);

        stack.into()
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// CARD GROUP — Groupe de cartes homogènes
// ═════════════════════════════════════════════════════════════════════════════

/// Crée un groupe de cartes avec espacement uniforme.
pub fn card_group<'a, Message: Clone + 'static>(
    cards: Vec<Element<'a, Message>>,
    columns: usize,
    spacing_val: f32,
) -> Element<'a, Message> {
    if cards.is_empty() {
        return sory_empty_card_group();
    }

    let mut rows = column::with_capacity(cards.len() / columns + 1)
        .spacing(spacing_val);

    let mut iter = cards.into_iter().peekable();
    while iter.peek().is_some() {
        let mut row = row::with_capacity(columns)
            .spacing(spacing_val);
        for _ in 0..columns {
            if let Some(card) = iter.next() {
                row = row.push(container(card).width(Length::Fill));
            } else {
                row = row.push(space::horizontal().width(Length::Fill));
            }
        }
        rows = rows.push(row);
    }

    rows.into()
}

/// État vide pour un groupe de cartes.
fn sory_empty_card_group<'a, Message: Clone + 'static>() -> Element<'a, Message> {
    let spacing = crate::theme::spacing();
    container(
        text::body("Aucune carte")
            .center()
            .width(Length::Fill),
    )
    .padding(spacing.space_xl)
    .width(Length::Fill)
    .into()
}

// ═════════════════════════════════════════════════════════════════════════════
// CARTE STATIQUE (pas d'interaction)
// ═════════════════════════════════════════════════════════════════════════════

/// Crée une carte statique sans interaction (pas de glow dynamique).
pub fn static_card<'a, Message: Clone + 'static>(
    body: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    let spacing = crate::theme::spacing();

    container(body.into())
        .class(crate::theme::sory::tile())
        .padding(spacing.space_m)
        .width(Length::Fill)
        .into()
}

// ═════════════════════════════════════════════════════════════════════════════
// CARTE INFORMATIVE (avec icône + titre + description)
// ═════════════════════════════════════════════════════════════════════════════

/// Crée une carte informative avec icône, titre et description.
pub fn info_card<'a, Message: Clone + 'static>(
    icon: impl Into<Element<'a, Message>>,
    title: impl Into<Cow<'a, str>> + 'a,
    description: impl Into<Cow<'a, str>> + 'a,
) -> Element<'a, Message> {
    let spacing = crate::theme::spacing();

    let content = row::with_capacity(2)
        .spacing(spacing.space_m)
        .align_y(Alignment::Start)
        .push(
            container(icon.into())
                .class(crate::theme::sory::tile_icon_blue())
                .padding(12),
        )
        .push(
            column::with_capacity(2)
                .spacing(spacing.space_xxs)
                .push(text::body(title).width(Length::Fill))
                .push(
                    text::caption(description)
                        .width(Length::Fill),
                )
                .width(Length::Fill),
        );

    SoryCard::new(content)
        .into()
}

// ═════════════════════════════════════════════════════════════════════════════
// STAT CARTE (valeur + label + trend)
// ═════════════════════════════════════════════════════════════════════════════

/// Direction du trend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trend {
    /// En hausse.
    Up,
    /// En baisse.
    Down,
    /// Stable.
    Stable,
}

/// Crée une carte statistique avec valeur, label et trend.
pub fn stat_card<'a, Message: Clone + 'static>(
    value: impl Into<Cow<'a, str>> + 'a,
    label: impl Into<Cow<'a, str>> + 'a,
    trend: Trend,
) -> Element<'a, Message> {
    let spacing = crate::theme::spacing();

    let trend_char = match trend {
        Trend::Up => "↑",
        Trend::Down => "↓",
        Trend::Stable => "→",
    };

    let content = column::with_capacity(2)
        .spacing(spacing.space_xxs)
        .push(
            row::with_capacity(2)
                .spacing(spacing.space_s)
                .align_y(Alignment::Center)
                .push(
                    text::title2(value)
                )
                .push(
                    text::body(trend_char),
                ),
        )
        .push(
            text::caption(label),
        );

    SoryCard::new(content)
        .padding(Padding::from([12, 16]))
        .into()
}

// ═════════════════════════════════════════════════════════════════════════════
// SELECTABLE CARD (sélection multiple)
// ═════════════════════════════════════════════════════════════════════════════

/// Crée une carte sélectionnable avec coche.
pub fn selectable_card<'a, Message: Clone + 'static>(
    body: impl Into<Element<'a, Message>>,
    selected: bool,
    on_toggle: Message,
) -> Element<'a, Message> {
    let spacing = crate::theme::spacing();

    let card_class = if selected {
        crate::theme::sory::card_selectable_active()
    } else {
        crate::theme::sory::card_selectable()
    };

    let check = if selected {
        text::body("✓")
    } else {
        text::body("○")
    };

    let content = row::with_capacity(2)
        .spacing(spacing.space_s)
        .align_y(Alignment::Center)
        .push(check)
        .push(container(body.into()).width(Length::Fill));

    let card_container = container(content)
        .class(card_class)
        .padding(Padding::from([10, 14]))
        .width(Length::Fill);

    mouse_area(card_container)
        .on_press(on_toggle)
        .into()
}
