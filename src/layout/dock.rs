//! Dock layout — conteneur avec régions (North, South, East, West, Center).
//!
//! Pattern classique des IDE : Toolbar en haut, StatusBar en bas,
//! Sidebar à gauche, Panel à droite, Editor au centre.
//!
//! Chaque région est optionnelle et peut avoir une taille configurée.
//!
//! # Exemple
//! ```ignore
//! use cosmic::layout::{Dock, DockRegion};
//!
//! let dock = Dock::new(editor)
//!     .north(toolbar)
//!     .west(sidebar)
//!     .south(statusbar);
//! ```

use crate::iced::Length;
use crate::widget::{column, container, row};
use crate::Element;

/// Région du dock.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DockRegion {
    North,
    South,
    East,
    West,
    Center,
}

/// Conteneur dock (5 régions maximum).
pub struct Dock<'a, Message> {
    north: Option<Element<'a, Message>>,
    south: Option<Element<'a, Message>>,
    east: Option<Element<'a, Message>>,
    west: Option<Element<'a, Message>>,
    center: Element<'a, Message>,
    north_height: Length,
    south_height: Length,
    east_width: Length,
    west_width: Length,
    spacing: f32,
}

impl<'a, Message: 'static> Dock<'a, Message> {
    pub fn new(center: impl Into<Element<'a, Message>>) -> Self {
        Self {
            north: None,
            south: None,
            east: None,
            west: None,
            center: center.into(),
            north_height: Length::Shrink,
            south_height: Length::Shrink,
            east_width: Length::Shrink,
            west_width: Length::Shrink,
            spacing: 0.0,
        }
    }

    /// Ajoute une région nord (toolbar, header).
    pub fn north(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.north = Some(child.into());
        self
    }

    /// Ajoute une région sud (status bar, footer).
    pub fn south(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.south = Some(child.into());
        self
    }

    /// Ajoute une région est (panel droit).
    pub fn east(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.east = Some(child.into());
        self
    }

    /// Ajoute une région ouest (sidebar gauche).
    pub fn west(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.west = Some(child.into());
        self
    }

    /// Définit la hauteur de la région nord.
    pub fn north_height(mut self, height: impl Into<Length>) -> Self {
        self.north_height = height.into();
        self
    }

    /// Définit la hauteur de la région sud.
    pub fn south_height(mut self, height: impl Into<Length>) -> Self {
        self.south_height = height.into();
        self
    }

    /// Définit la largeur de la région est.
    pub fn east_width(mut self, width: impl Into<Length>) -> Self {
        self.east_width = width.into();
        self
    }

    /// Définit la largeur de la région ouest.
    pub fn west_width(mut self, width: impl Into<Length>) -> Self {
        self.west_width = width.into();
        self
    }

    /// Définit l'espacement entre les régions.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}

impl<'a, Message: 'static> From<Dock<'a, Message>> for Element<'a, Message> {
    fn from(dock: Dock<'a, Message>) -> Self {
        // Build the center row: west + center + east
        let mut center_row = row::with_capacity(3);
        if let Some(west) = dock.west {
            center_row = center_row.push(
                container(west)
                    .width(dock.west_width)
                    .height(Length::Fill),
            );
        }
        center_row = center_row.push(
            container(dock.center)
                .width(Length::Fill)
                .height(Length::Fill),
        );
        if let Some(east) = dock.east {
            center_row = center_row.push(
                container(east)
                    .width(dock.east_width)
                    .height(Length::Fill),
            );
        }

        // Build the main column: north + center_row + south
        let mut main_col = column::with_capacity(3);
        if let Some(north) = dock.north {
            main_col = main_col.push(
                container(north)
                    .width(Length::Fill)
                    .height(dock.north_height),
            );
        }
        main_col = main_col.push(center_row.height(Length::Fill));
        if let Some(south) = dock.south {
            main_col = main_col.push(
                container(south)
                    .width(Length::Fill)
                    .height(dock.south_height),
            );
        }

        container(main_col)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
