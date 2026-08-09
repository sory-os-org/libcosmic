// Copyright 2024 SoryOS <maintainers@soryos.local>
// SPDX-License-Identifier: MPL-2.0

//! Navigation par breadcrumb SoryOS.
//!
//! Fournit un chemin de navigation horizontal avec séparateurs,
//! segments cliquables et icônes.

use std::borrow::Cow;

use crate::iced::{Alignment, Length, Padding};
use crate::widget::{container, mouse_area, row, text};
use crate::Element;

// ═════════════════════════════════════════════════════════════════════════════
// BREADCRUMB
// ═════════════════════════════════════════════════════════════════════════════

/// Builder pour une navigation breadcrumb SoryOS.
pub struct SoryBreadcrumb<'a, Message> {
    segments: Vec<BreadcrumbSegment<'a, Message>>,
    separator: BreadcrumbSeparator,
}

enum BreadcrumbSegment<'a, Message> {
    /// Segment cliquable ( lien).
    Link {
        label: Cow<'a, str>,
        on_press: Message,
    },
    /// Segment actuel (non cliquable).
    Current(Cow<'a, str>),
    /// Segment avec icône personnalisée.
    IconLink {
        icon: Element<'a, Message>,
        label: Option<Cow<'a, str>>,
        on_press: Message,
    },
}

/// Style de séparateur entre les segments.
pub enum BreadcrumbSeparator {
    /// Séparateur chevron (›).
    Chevron,
    /// Séparateur slash (/).
    Slash,
    /// Séparateur point (•).
    Dot,
    /// Séparateur flèche (→).
    Arrow,
    /// Séparateur personnalisé.
    Custom(Cow<'static, str>),
}

impl<'a, Message> Default for SoryBreadcrumb<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> SoryBreadcrumb<'a, Message> {
    /// Crée un nouveau breadcrumb vide.
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
            separator: BreadcrumbSeparator::Chevron,
        }
    }

    /// Définit le style de séparateur.
    pub fn separator(mut self, sep: BreadcrumbSeparator) -> Self {
        self.separator = sep;
        self
    }

    /// Ajoute un segment cliquable.
    pub fn link(mut self, label: impl Into<Cow<'a, str>>, on_press: Message) -> Self {
        self.segments.push(BreadcrumbSegment::Link {
            label: label.into(),
            on_press,
        });
        self
    }

    /// Ajoute un segment avec icône.
    pub fn icon_link(
        mut self,
        icon: impl Into<Element<'a, Message>>,
        label: Option<impl Into<Cow<'a, str>>>,
        on_press: Message,
    ) -> Self {
        self.segments.push(BreadcrumbSegment::IconLink {
            icon: icon.into(),
            label: label.map(Into::into),
            on_press,
        });
        self
    }

    /// Ajoute le segment actuel (non cliquable).
    pub fn current(mut self, label: impl Into<Cow<'a, str>>) -> Self {
        self.segments.push(BreadcrumbSegment::Current(label.into()));
        self
    }

    /// Ajoute plusieurs segments d'un coup.
    pub fn segments(
        mut self,
        segs: impl IntoIterator<Item = BreadcrumbSegment<'a, Message>>,
    ) -> Self {
        self.segments.extend(segs);
        self
    }
}

impl<'a, Message: Clone + 'static> From<SoryBreadcrumb<'a, Message>> for Element<'a, Message> {
    fn from(breadcrumb: SoryBreadcrumb<'a, Message>) -> Self {
        let spacing = crate::theme::spacing();

        let sep_text: Cow<'a, str> = match breadcrumb.separator {
            BreadcrumbSeparator::Chevron => Cow::Borrowed("›"),
            BreadcrumbSeparator::Slash => Cow::Borrowed("/"),
            BreadcrumbSeparator::Dot => Cow::Borrowed("•"),
            BreadcrumbSeparator::Arrow => Cow::Borrowed("→"),
            BreadcrumbSeparator::Custom(s) => s,
        };

        let mut row = row::with_capacity(breadcrumb.segments.len() * 2)
            .spacing(spacing.space_xxs)
            .align_y(Alignment::Center);

        let total = breadcrumb.segments.len();
        for (i, segment) in breadcrumb.segments.into_iter().enumerate() {
            let is_last = i + 1 == total;

            match segment {
                BreadcrumbSegment::Link { label, on_press } => {
                    let label_widget = text::body(label);
                    let clickable = mouse_area(
                        container(label_widget).padding(Padding::from([2, 6])),
                    )
                    .on_press(on_press);

                    row = row.push(clickable);
                }
                BreadcrumbSegment::IconLink { icon, label, on_press } => {
                    let mut seg_row = row::with_capacity(2)
                        .spacing(spacing.space_xxs)
                        .align_y(Alignment::Center)
                        .push(icon);

                    if let Some(label) = label {
                        seg_row = seg_row.push(text::body(label));
                    }

                    let clickable = mouse_area(
                        container(seg_row).padding(Padding::from([2, 6])),
                    )
                    .on_press(on_press);

                    row = row.push(clickable);
                }
                BreadcrumbSegment::Current(label) => {
                    row = row.push(
                        container(
                            text::body(label),
                        )
                        .padding(Padding::from([2, 6])),
                    );
                }
            }

            // Séparateur après chaque segment sauf le dernier
            if !is_last {
                row = row.push(
                    text::body(sep_text.clone())
                        .center()
                        .width(Length::Shrink),
                );
            }
        }

        container(row)
            .class(crate::theme::sory::breadcrumb())
            .into()
    }
}

/// Crée un breadcrumb rapide avec des chemins de chaînes.
pub fn simple_breadcrumb<'a, Message: Clone + 'static>(
    segments: &[(&'a str, Option<Message>)],
) -> Element<'a, Message> {
    let mut breadcrumb = SoryBreadcrumb::new();

    for (i, (label, on_press)) in segments.iter().enumerate() {
        let is_last = i + 1 == segments.len();
        if is_last {
            breadcrumb = breadcrumb.current(*label);
        } else if let Some(msg) = on_press.clone() {
            breadcrumb = breadcrumb.link(*label, msg);
        } else {
            breadcrumb = breadcrumb.current(*label);
        }
    }

    breadcrumb.into()
}
