//! SplitPane — conteneur avec deux panneaux séparés par un ratio.
//!
//! Permet de diviser l'espace horizontalement ou verticalement
//! entre deux enfants avec un ratio configurable.
//!
//! # Exemple
//! ```ignore
//! use cosmic::layout::SplitPane;
//!
//! let split = SplitPane::horizontal(sidebar, content)
//!     .ratio(0.25)  // 25% sidebar, 75% content
//!     .min_first(200.0)
//!     .max_first(400.0);
//! ```

use crate::iced::{Length, Padding};
use crate::widget::{column, container, row, space};
use crate::Element;

/// Axe du split.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

/// Conteneur split pane avec deux panneaux et un ratio.
pub struct SplitPane<'a, Message> {
    first: Element<'a, Message>,
    second: Element<'a, Message>,
    axis: Axis,
    ratio: f32,
    min_first: Option<f32>,
    max_first: Option<f32>,
    min_second: Option<f32>,
    max_second: Option<f32>,
    spacing: f32,
    divider_width: f32,
}

impl<'a, Message: 'static> SplitPane<'a, Message> {
    /// Crée un split horizontal (premier à gauche, second à droite).
    pub fn horizontal(
        first: impl Into<Element<'a, Message>>,
        second: impl Into<Element<'a, Message>>,
    ) -> Self {
        Self {
            first: first.into(),
            second: second.into(),
            axis: Axis::Horizontal,
            ratio: 0.3,
            min_first: None,
            max_first: None,
            min_second: None,
            max_second: None,
            spacing: 0.0,
            divider_width: 1.0,
        }
    }

    /// Crée un split vertical (premier en haut, second en bas).
    pub fn vertical(
        first: impl Into<Element<'a, Message>>,
        second: impl Into<Element<'a, Message>>,
    ) -> Self {
        Self {
            first: first.into(),
            second: second.into(),
            axis: Axis::Vertical,
            ratio: 0.3,
            min_first: None,
            max_first: None,
            min_second: None,
            max_second: None,
            spacing: 0.0,
            divider_width: 1.0,
        }
    }

    /// Définit le ratio (0.0 à 1.0) de l'espace pour le premier panneau.
    pub fn ratio(mut self, ratio: f32) -> Self {
        self.ratio = ratio.clamp(0.0, 1.0);
        self
    }

    /// Définit la taille minimale du premier panneau.
    pub fn min_first(mut self, min: f32) -> Self {
        self.min_first = Some(min);
        self
    }

    /// Définit la taille maximale du premier panneau.
    pub fn max_first(mut self, max: f32) -> Self {
        self.max_first = Some(max);
        self
    }

    /// Définit la taille minimale du second panneau.
    pub fn min_second(mut self, min: f32) -> Self {
        self.min_second = Some(min);
        self
    }

    /// Définit la taille maximale du second panneau.
    pub fn max_second(mut self, max: f32) -> Self {
        self.max_second = Some(max);
        self
    }

    /// Définit l'espacement entre les panneaux.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Définit la largeur du séparateur.
    pub fn divider_width(mut self, width: f32) -> Self {
        self.divider_width = width;
        self
    }
}

impl<'a, Message: 'static> From<SplitPane<'a, Message>> for Element<'a, Message> {
    fn from(split: SplitPane<'a, Message>) -> Self {
        match split.axis {
            Axis::Horizontal => {
                let first = container(split.first)
                    .width(Length::Fill)
                    .height(Length::Fill);

                let second = container(split.second)
                    .width(Length::Fill)
                    .height(Length::Fill);

                let divider = container(space::horizontal())
                    .width(split.divider_width)
                    .height(Length::Fill);

                row::with_capacity(3)
                    .push(first)
                    .push(divider)
                    .push(second)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            }
            Axis::Vertical => {
                let first = container(split.first)
                    .width(Length::Fill)
                    .height(Length::Fill);

                let second = container(split.second)
                    .width(Length::Fill)
                    .height(Length::Fill);

                let divider = container(space::vertical())
                    .width(Length::Fill)
                    .height(split.divider_width);

                column::with_capacity(3)
                    .push(first)
                    .push(divider)
                    .push(second)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            }
        }
    }
}
