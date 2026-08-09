//! Responsive — layout adaptatif selon la taille disponible.
//!
//! Permet de changer la disposition en fonction de la largeur
//! ou de la hauteur de la fenêtre.
//!
//! # Exemple
//! ```ignore
//! use cosmic::layout::{responsive_layout, Breakpoint};
//!
//! let layout = responsive_layout(|width| {
//!     if width < 600.0 {
//!         // Mobile: colonne unique
//!         column![sidebar, content].into()
//!     } else if width < 1200.0 {
//!         // Tablette: sidebar réduite
//!         row![sidebar_narrow, content].into()
//!     } else {
//!         // Desktop: sidebar complète + détail
//!         row![sidebar, content, details].into()
//!     }
//! });
//! ```

use crate::Element;

/// Point de rupture pour le layout adaptatif.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Breakpoint {
    /// Mobile (< 600px)
    Mobile,
    /// Tablette (600px - 1200px)
    Tablet,
    /// Desktop (> 1200px)
    Desktop,
}

impl Breakpoint {
    /// Détermine le breakpoint à partir de la largeur.
    pub fn from_width(width: f32) -> Self {
        if width < 600.0 {
            Self::Mobile
        } else if width < 1200.0 {
            Self::Tablet
        } else {
            Self::Desktop
        }
    }
}

/// Crée un layout adaptatif qui change en fonction de la largeur disponible.
///
/// Le builder reçoit la largeur en pixels et retourne un Element.
///
/// # Exemple
/// ```ignore
/// let layout = responsive_layout(|width| {
///     if width < 600.0 {
///         column![sidebar, content].into()
///     } else {
///         row![sidebar, content].into()
///     }
/// });
/// ```
pub fn responsive_layout<'a, Message, F>(builder: F) -> crate::Element<'a, Message>
where
    Message: 'a,
    F: Fn(f32) -> Element<'a, Message> + 'a,
{
    crate::iced::widget::responsive(move |size| builder(size.width)).into()
}
