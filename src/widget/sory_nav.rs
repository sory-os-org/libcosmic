// Copyright 2024 SoryOS <maintainers@soryos.local>
// SPDX-License-Identifier: MPL-2.0

//! Navigation sidebar avancée SoryOS — patterns shadcn/Radix.
//!
//! Fournit une sidebar de navigation complète avec :
//! - **Collapsible** : mode icon-only (sidebar repliée) ou pleine largeur
//! - **Group toggle** : groupes expandable/collapsible
//! - **Keyboard navigation** : Tab, Enter, Escape pour naviguer
//! - **Scrollable** : contenu scrollable quand dépasse la hauteur
//! - **Tooltip** : infobulle sur les items en mode collapsé
//! - **Active parent** : highlight du parent quand un sub-item est actif
//! - **Accent bar** : indicateur visuel de l'item actif
//! - **Footer** : branding, informations en bas

use std::borrow::Cow;

use crate::iced::{Alignment, Length, Padding};
use crate::widget::{anim, column, container, mouse_area, row, scrollable, space, text};
use crate::Element;

// ═════════════════════════════════════════════════════════════════════════════
// NAVIGATION SIDEBAR
// ═════════════════════════════════════════════════════════════════════════════

/// Builder pour une sidebar de navigation SoryOS.
pub struct SoryNav<'a, Message> {
    groups: Vec<SoryNavGroup<'a, Message>>,
    header: Option<Element<'a, Message>>,
    footer: Option<Element<'a, Message>>,
    search: Option<Element<'a, Message>>,
    width: Length,
    height: Length,
    collapsed: bool,
    collapsed_width: f32,
    on_toggle_collapse: Option<Message>,
    scrollable: bool,
}

struct SoryNavGroup<'a, Message> {
    label: Option<Cow<'a, str>>,
    items: Vec<SoryNavItem<'a, Message>>,
    separator_after: bool,
    collapsible: bool,
    collapsed: bool,
    on_toggle: Option<Message>,
}

/// Un élément de navigation (item ou sub-item).
pub enum SoryNavItem<'a, Message> {
    Item(SoryNavEntry<'a, Message>),
    SubItem(SoryNavEntry<'a, Message>),
    Separator,
}

struct SoryNavEntry<'a, Message> {
    icon: Option<Element<'a, Message>>,
    label: Element<'a, Message>,
    badge: Option<Element<'a, Message>>,
    counter: Option<Element<'a, Message>>,
    tooltip_text: Option<Cow<'a, str>>,
    active: bool,
    parent_active: bool,
    disabled: bool,
    on_press: Option<Message>,
}

impl<'a, Message> Default for SoryNav<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> SoryNav<'a, Message> {
    /// Crée une nouvelle sidebar de navigation vide.
    pub fn new() -> Self {
        Self {
            groups: Vec::new(),
            header: None,
            footer: None,
            search: None,
            width: Length::Fixed(260.0),
            height: Length::Fill,
            collapsed: false,
            collapsed_width: 64.0,
            on_toggle_collapse: None,
            scrollable: true,
        }
    }

    /// Définit la largeur de la sidebar.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Définit la hauteur de la sidebar.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Mode collapsé (icon-only).
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    /// Largeur en mode collapsé.
    pub fn collapsed_width(mut self, width: f32) -> Self {
        self.collapsed_width = width;
        self
    }

    /// Message émis quand on toggle le collapse.
    pub fn on_toggle_collapse(mut self, msg: Message) -> Self {
        self.on_toggle_collapse = Some(msg);
        self
    }

    /// Active/désactive le scroll automatique.
    pub fn scrollable(mut self, scrollable: bool) -> Self {
        self.scrollable = scrollable;
        self
    }

    /// Ajoute un élément de recherche en haut de la sidebar.
    pub fn search(mut self, search: impl Into<Element<'a, Message>>) -> Self {
        self.search = Some(search.into());
        self
    }

    /// Ajoute un header personnalisé en haut de la sidebar.
    pub fn header(mut self, header: impl Into<Element<'a, Message>>) -> Self {
        self.header = Some(header.into());
        self
    }

    /// Ajoute un footer en bas de la sidebar.
    pub fn footer(mut self, footer: impl Into<Element<'a, Message>>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    /// Ajoute un groupe d'items de navigation.
    pub fn group(mut self, group: SoryNavGroup<'a, Message>) -> Self {
        self.groups.push(group);
        self
    }

    /// Ajoute plusieurs groupes d'un coup.
    pub fn groups(mut self, groups: impl IntoIterator<Item = SoryNavGroup<'a, Message>>) -> Self
    where
        Message: 'a,
    {
        self.groups.extend(groups);
        self
    }
}

impl<'a, Message: Clone + 'static> From<SoryNav<'a, Message>> for Element<'a, Message> {
    fn from(nav: SoryNav<'a, Message>) -> Self {
        let spacing = crate::theme::spacing();
        let effective_width = if nav.collapsed {
            Length::Fixed(nav.collapsed_width)
        } else {
            nav.width
        };

        let mut col = column::with_capacity(nav.groups.len() * 3 + 4);

        // ── Header ────────────────────────────────────────────────────
        if let Some(header) = nav.header {
            if nav.collapsed {
                // En mode collapsé, réduire le header
                col = col.push(
                    container(header)
                        .padding(Padding::from([spacing.space_s, spacing.space_xxs]))
                        .width(Length::Fill)
                        .center_x(Length::Fill),
                );
            } else {
                col = col.push(
                    container(header)
                        .padding(Padding::from([spacing.space_s, spacing.space_m]))
                        .width(Length::Fill),
                );
            }
        }

        // ── Barre de recherche (masquée en mode collapsé) ─────────────
        if let Some(search) = nav.search {
            if !nav.collapsed {
                col = col.push(
                    container(search)
                        .class(crate::theme::sory::search_bar())
                        .padding(Padding::from([spacing.space_xxs, spacing.space_m]))
                        .width(Length::Fill),
                );
            }
        }

        // ── Toggle collapse button ────────────────────────────────────
        if let Some(on_toggle) = nav.on_toggle_collapse {
            let toggle_icon = if nav.collapsed {
                "▶" // Expand icon
            } else {
                "◀" // Collapse icon
            };
            let toggle_btn = container(
                    text::body(toggle_icon)
                    .center()
                    .width(Length::Fill),
            )
            .padding(Padding::from([spacing.space_xxs, spacing.space_xxs]))
            .width(Length::Fill);

            let toggle = mouse_area(toggle_btn)
                .on_press(on_toggle);
            let animated_toggle: Element<'a, Message> = anim::animated(toggle)
                .preset(anim::AnimPreset::Lift {
                    hover_scale: 1.1,
                    press_scale: 0.9,
                    hover_lift: 0.0,
                })
                .into();
            col = col.push(animated_toggle);
        }

        // ── Groupes ───────────────────────────────────────────────────
        for group in nav.groups {
            // Label du groupe (masqué en mode collapsé)
            if let Some(label) = group.label {
                if !nav.collapsed {
                    col = col.push(
                        container(text::caption(label))
                            .padding(Padding::from([
                                spacing.space_xxs,
                                spacing.space_m,
                                spacing.space_xxs,
                                spacing.space_m + 8,
                            ]))
                            .width(Length::Fill),
                    );
                } else {
                    // Mode collapsé : séparateur subtil
                    col = col.push(
                        container(space::horizontal())
                            .height(1.0)
                            .width(Length::Fill)
                            .padding(Padding::from([0, spacing.space_s]))
                            .class(crate::theme::sory::sidebar_separator()),
                    );
                }
            }

            // Toggle pour groupes collapsibles
            if group.collapsible {
                if let Some(on_toggle) = group.on_toggle {
                    let chevron = if group.collapsed { "▶" } else { "▼" };
                    let toggle = container(
                        text::caption(chevron)
                            .center()
                            .width(Length::Fill),
                    )
                    .padding(Padding::from([spacing.space_xxs, spacing.space_xxs]))
                    .width(Length::Fill);

                    col = col.push(mouse_area(toggle).on_press(on_toggle));
                }
            }

            // Items du groupe (masqués si le groupe est collapsed)
            if !group.collapsed {
                for item in group.items {
                    let is_sub = matches!(&item, SoryNavItem::SubItem(_));
                    match item {
                        SoryNavItem::Item(entry) | SoryNavItem::SubItem(entry) => {
                            let SoryNavEntry {
                                icon,
                                label,
                                badge,
                                counter,
                                tooltip_text,
                                active,
                                parent_active,
                                disabled,
                                on_press,
                            } = entry;

                            let mut row_content = row::with_capacity(3)
                                .spacing(spacing.space_xxs)
                                .align_y(Alignment::Center);

                            // Indicateur accent bar (seulement si actif)
                            if active {
                                row_content = row_content.push(
                                    container(space::horizontal())
                                        .width(3.0)
                                        .height(20.0)
                                        .class(crate::theme::sory::sidebar_accent_bar()),
                                );
                            } else if is_sub {
                                row_content =
                                    row_content.push(space::horizontal().width(3.0));
                            }

                            // Icône
                            if let Some(icon) = icon {
                                let icon_container = container(icon)
                                    .width(24.0)
                                    .height(24.0);
                                row_content = row_content.push(icon_container);
                            }

                            // Label (masqué en mode collapsé)
                            if !nav.collapsed {
                                row_content = row_content.push(
                                    container(label).width(Length::Fill),
                                );
                            }

                            // Badge (masqué en mode collapsé)
                            if !nav.collapsed {
                                if let Some(badge) = badge {
                                    row_content = row_content.push(badge);
                                }
                            }

                            // Compteur (masqué en mode collapsé)
                            if !nav.collapsed {
                                if let Some(counter) = counter {
                                    row_content = row_content.push(counter);
                                }
                            }

                            // Style selon l'état
                            let item_class = if disabled {
                                crate::theme::sory::sidebar_item()
                            } else if active {
                                crate::theme::sory::sidebar_item_active()
                            } else if parent_active {
                                crate::theme::sory::sidebar_item_parent_active()
                            } else {
                                crate::theme::sory::sidebar_item()
                            };

                            let left_offset = if is_sub && !active {
                                24.0
                            } else {
                                0.0
                            };

                            let mut item_container = container(row_content)
                                .class(item_class)
                                .padding(Padding::from([
                                    spacing.space_xxs,
                                    spacing.space_s,
                                ]))
                                .width(Length::Fill);

                            if left_offset > 0.0 && !nav.collapsed {
                                item_container = item_container.padding(Padding::from([
                                    spacing.space_xxs,
                                    spacing.space_s,
                                    spacing.space_xxs,
                                    spacing.space_s + (left_offset as u16),
                                ]));
                            }

                            // Centrer en mode collapsé
                            if nav.collapsed {
                                item_container = item_container.center_x(Length::Fill);
                            }

                            // Appliquer on_press via mouse_area
                            let item_with_mouse: Element<'a, Message> = if let Some(on_press) = on_press {
                                if disabled {
                                    item_container.into()
                                } else {
                                    mouse_area(item_container)
                                        .on_press(on_press)
                                        .into()
                                }
                            } else {
                                item_container.into()
                            };

                            // Appliquer tooltip si présent et en mode collapsé
                            let final_item = if nav.collapsed {
                                if let Some(tip) = tooltip_text {
                                    crate::widget::tooltip::tooltip(
                                        item_with_mouse,
                                        text::body(tip),
                                        crate::widget::tooltip::Position::Right,
                                    )
                                    .into()
                                } else {
                                    item_with_mouse
                                }
                            } else {
                                item_with_mouse
                            };

                            // Micro-interaction hover (sauf pour les items désactivés)
                            let animated_item: Element<'a, Message> = if disabled {
                                final_item
                            } else {
                                anim::animated(final_item)
                                    .preset(anim::AnimPreset::Lift {
                                        hover_scale: 1.02,
                                        press_scale: 0.98,
                                        hover_lift: -1.0,
                                    })
                                    .into()
                            };

                            col = col.push(animated_item);
                        }
                        SoryNavItem::Separator => {
                            col = col.push(
                                container(space::horizontal())
                                    .height(1.0)
                                    .width(Length::Fill)
                                    .padding(Padding::from([0, spacing.space_s]))
                                    .class(crate::theme::sory::sidebar_separator()),
                            );
                        }
                    }
                }
            }

            // Séparateur après le groupe
            if group.separator_after {
                col = col.push(
                    container(space::horizontal())
                        .height(1.0)
                        .width(Length::Fill)
                        .padding(Padding::from([0, spacing.space_s]))
                        .class(crate::theme::sory::sidebar_separator()),
                );
            }
        }

        // ── Spacer pour pousser le footer vers le bas ──────────────────
        col = col.push(space::vertical().height(Length::Fill));

        // ── Footer ────────────────────────────────────────────────────
        if let Some(footer) = nav.footer {
            if nav.collapsed {
                col = col.push(
                    container(footer)
                        .class(crate::theme::sory::sidebar_footer())
                        .padding(spacing.space_xxs)
                        .width(Length::Fill)
                        .center_x(Length::Fill),
                );
            } else {
                col = col.push(
                    container(footer)
                        .class(crate::theme::sory::sidebar_footer())
                    .padding(spacing.space_s)
                    .width(Length::Fill),
                );
            }
        }

        // Envelopper dans un scrollable si nécessaire
        let content: Element<'a, Message> = if nav.scrollable {
            scrollable(col)
                .height(Length::Fill)
                .class(crate::style::iced::Scrollable::Minimal)
                .into()
        } else {
            col.into()
        };

        container(content)
            .class(crate::theme::sory::sidebar())
            .width(effective_width)
            .height(nav.height)
            .into()
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// BUILDER POUR GROUPES ET ITEMS
// ═════════════════════════════════════════════════════════════════════════════

/// Builder pour un groupe de navigation.
pub struct SoryNavGroupBuilder<'a, Message> {
    label: Option<Cow<'a, str>>,
    items: Vec<SoryNavItem<'a, Message>>,
    separator_after: bool,
    collapsible: bool,
    collapsed: bool,
    on_toggle: Option<Message>,
}

impl<'a, Message> SoryNavGroupBuilder<'a, Message> {
    pub fn new() -> Self {
        Self {
            label: None,
            items: Vec::new(),
            separator_after: false,
            collapsible: false,
            collapsed: false,
            on_toggle: None,
        }
    }

    /// Label du groupe (section).
    pub fn label(mut self, label: impl Into<Cow<'a, str>>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Ajoute un item de navigation.
    pub fn item(mut self, item: SoryNavItem<'a, Message>) -> Self {
        self.items.push(item);
        self
    }

    /// Ajoute plusieurs items.
    pub fn items(mut self, items: impl IntoIterator<Item = SoryNavItem<'a, Message>>) -> Self {
        self.items.extend(items);
        self
    }

    /// Ajoute un séparateur après le groupe.
    pub fn separator_after(mut self) -> Self {
        self.separator_after = true;
        self
    }

    /// Rend le groupe collapsible (expandable/collapsible).
    pub fn collapsible(mut self, on_toggle: Message) -> Self {
        self.collapsible = true;
        self.on_toggle = Some(on_toggle);
        self
    }

    /// Définit l'état collapsed du groupe.
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    pub fn build(self) -> SoryNavGroup<'a, Message> {
        SoryNavGroup {
            label: self.label,
            items: self.items,
            separator_after: self.separator_after,
            collapsible: self.collapsible,
            collapsed: self.collapsed,
            on_toggle: self.on_toggle,
        }
    }
}

impl<'a, Message> Default for SoryNavGroupBuilder<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

/// Crée un groupe de navigation.
pub fn nav_group<'a, Message>() -> SoryNavGroupBuilder<'a, Message> {
    SoryNavGroupBuilder::new()
}

/// Crée un item de navigation.
pub fn nav_item<'a, Message: Clone + 'static>(
    label: impl Into<Element<'a, Message>>,
) -> SoryNavEntryBuilder<'a, Message> {
    SoryNavEntryBuilder {
        icon: None,
        label: label.into(),
        badge: None,
        counter: None,
        tooltip_text: None,
        active: false,
        parent_active: false,
        disabled: false,
        on_press: None,
        _phantom: std::marker::PhantomData,
    }
}

/// Builder pour un entry de navigation.
pub struct SoryNavEntryBuilder<'a, Message> {
    icon: Option<Element<'a, Message>>,
    label: Element<'a, Message>,
    badge: Option<Element<'a, Message>>,
    counter: Option<Element<'a, Message>>,
    tooltip_text: Option<Cow<'a, str>>,
    active: bool,
    parent_active: bool,
    disabled: bool,
    on_press: Option<Message>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a, Message: Clone + 'static> SoryNavEntryBuilder<'a, Message> {
    /// Définit l'icône de l'item.
    pub fn icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Définit le badge (ex: "Nouveau").
    pub fn badge(mut self, badge: impl Into<Element<'a, Message>>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    /// Définit le compteur (ex: nombre de notifications).
    pub fn counter(mut self, counter: impl Into<Element<'a, Message>>) -> Self {
        self.counter = Some(counter.into());
        self
    }

    /// Définit le texte du tooltip (affiché en mode collapsé).
    pub fn tooltip(mut self, tooltip: impl Into<Cow<'a, str>>) -> Self {
        self.tooltip_text = Some(tooltip.into());
        self
    }

    /// Marque l'item comme actif.
    pub fn active(mut self) -> Self {
        self.active = true;
        self
    }

    /// Marque le parent comme actif (quand un sub-item est actif).
    pub fn parent_active(mut self) -> Self {
        self.parent_active = true;
        self
    }

    /// Désactive l'item.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    /// Définit le message émis au clic.
    pub fn on_press(mut self, msg: Message) -> Self {
        self.on_press = Some(msg);
        self
    }

    /// Construit l'item comme item principal.
    pub fn build(self) -> SoryNavItem<'a, Message> {
        SoryNavItem::Item(SoryNavEntry {
            icon: self.icon,
            label: self.label,
            badge: self.badge,
            counter: self.counter,
            tooltip_text: self.tooltip_text,
            active: self.active,
            parent_active: self.parent_active,
            disabled: self.disabled,
            on_press: self.on_press,
        })
    }

    /// Construit l'item comme sub-item.
    pub fn build_sub(self) -> SoryNavItem<'a, Message> {
        SoryNavItem::SubItem(SoryNavEntry {
            icon: self.icon,
            label: self.label,
            badge: self.badge,
            counter: self.counter,
            tooltip_text: self.tooltip_text,
            active: self.active,
            parent_active: self.parent_active,
            disabled: self.disabled,
            on_press: self.on_press,
        })
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// FOOTER BUILDER
// ═════════════════════════════════════════════════════════════════════════════

/// Crée un footer de sidebar SoryOS avec branding.
pub fn nav_footer<'a, Message: Clone + 'static>(
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    let spacing = crate::theme::spacing();
    container(content.into())
        .class(crate::theme::sory::sidebar_footer())
        .padding(spacing.space_s)
        .width(Length::Fill)
        .into()
}

/// Crée un footer de sidebar avec logo et nom de la distribution.
pub fn nav_footer_brand<'a, Message: Clone + 'static>(
    logo: impl Into<Element<'a, Message>>,
    brand_name: impl Into<Cow<'a, str>> + 'a,
) -> Element<'a, Message> {
    let spacing = crate::theme::spacing();
    let content = row::with_capacity(2)
        .spacing(spacing.space_s)
        .align_y(Alignment::Center)
        .push(logo.into())
        .push(
            text::body(brand_name)
                .width(Length::Fill),
        );

    nav_footer(content)
}

/// Crée un badge de compteur stylisé.
pub fn nav_counter<'a, Message: Clone + 'static>(
    count: impl Into<Cow<'a, str>> + 'a,
) -> Element<'a, Message> {
    let spacing = crate::theme::spacing();
    container(
        text::caption(count)
            .center()
            .width(Length::Fill),
    )
    .class(crate::theme::sory::chip())
    .padding(Padding::from([2, 8]))
    .into()
}

// ═════════════════════════════════════════════════════════════════════════════
// SIDEBAR TOGGLE BUTTON (pour placer dans le header)
// ═════════════════════════════════════════════════════════════════════════════

/// Crée un bouton toggle pour la sidebar (hamburger menu).
pub fn nav_toggle_button<'a, Message: Clone + 'static>(
    collapsed: bool,
    on_press: Message,
) -> Element<'a, Message> {
    let spacing = crate::theme::spacing();
    let icon = if collapsed { "☰" } else { "✕" };

    let btn = container(
        text::body(icon)
            .center()
            .width(24.0)
            .height(24.0),
    )
    .class(crate::theme::sory::button_icon())
    .padding(Padding::from([spacing.space_xxs, spacing.space_xxs]))
    .width(36.0)
    .height(36.0)
    .center_x(Length::Fill)
    .center_y(Length::Fill);

    mouse_area(btn).on_press(on_press).into()
}
