//! Moteur de style moderne pour SoryOS.
//!
//! Système de style inspiré de Tailwind CSS et des Design Tokens.
//! Permet de définir l'apparence des composants de manière déclarative
//! et composable, sans jamais toucher au rendu bas niveau.
//!
//! Utilisation :
//! ```ignore
//! use cosmic::style::*;
//!
//! let style = style()
//!     .bg(colors.background)
//!     .text(colors.text)
//!     .rounded(radii.card)
//!     .shadow(shadows.md)
//!     .padding(spacing.lg);
//! ```

use iced_core::{Background, Color, Padding, Vector};
use iced_core::border::Radius;
use iced_core::Shadow;

use crate::design::SemanticColor;

/// Style complet d'un composant — toutes les propriétés visuelles.
#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    pub background: Option<Background>,
    pub text_color: Option<Color>,
    pub border_color: Option<Color>,
    pub border_width: f32,
    pub border_radius: Radius,
    pub shadow: Option<Shadow>,
    pub padding: Padding,
    pub opacity: f32,
    pub scale: f32,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            background: None,
            text_color: None,
            border_color: None,
            border_width: 0.0,
            border_radius: Radius::default(),
            shadow: None,
            padding: Padding::ZERO,
            opacity: 1.0,
            scale: 1.0,
        }
    }
}

/// Builder pour construire un style de manière fluide.
#[derive(Debug, Clone, PartialEq)]
pub struct StyleBuilder {
    pub style: Style,
}

pub fn style() -> StyleBuilder {
    StyleBuilder { style: Style::default() }
}

impl StyleBuilder {
    pub fn bg(mut self, color: impl Into<Background>) -> Self {
        self.style.background = Some(color.into());
        self
    }

    pub fn text(mut self, color: Color) -> Self {
        self.style.text_color = Some(color);
        self
    }

    pub fn border(mut self, color: Color) -> Self {
        self.style.border_color = Some(color);
        self.style.border_width = 1.0;
        self
    }

    pub fn border_width(mut self, width: f32) -> Self {
        self.style.border_width = width;
        self
    }

    pub fn rounded(mut self, radius: impl Into<Radius>) -> Self {
        self.style.border_radius = radius.into();
        self
    }

    pub fn shadow(mut self, shadow: Shadow) -> Self {
        self.style.shadow = Some(shadow);
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.style.padding = padding.into();
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.style.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.style.scale = scale;
        self
    }

    pub fn build(self) -> Style {
        self.style
    }
}

impl From<StyleBuilder> for Style {
    fn from(builder: StyleBuilder) -> Self {
        builder.style
    }
}

/// Presets de styles pour les composants communs.
pub mod presets {
    use super::*;
    use crate::design::SemanticColor;

    pub fn card(colors: &SemanticColorResolver) -> Style {
        style()
            .bg(colors.surface)
            .rounded(12.0)
            .shadow(Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            })
            .padding([16, 16])
            .build()
    }

    pub fn card_hovered(colors: &SemanticColorResolver) -> Style {
        style()
            .bg(colors.elevated)
            .rounded(12.0)
            .shadow(Shadow {
                color: Color::from_rgba(0x4a as f32 / 255.0, 0x8a as f32 / 255.0, 0xff as f32 / 255.0, 0.2),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 12.0,
            })
            .padding([16, 16])
            .build()
    }

    pub fn button_primary(colors: &SemanticColorResolver) -> Style {
        style()
            .bg(colors.primary)
            .text(colors.text_on_accent)
            .rounded(10.0)
            .padding([10, 20])
            .build()
    }

    pub fn button_ghost(colors: &SemanticColorResolver) -> Style {
        style()
            .text(colors.text)
            .rounded(10.0)
            .padding([10, 20])
            .build()
    }

    pub fn dialog(colors: &SemanticColorResolver) -> Style {
        style()
            .bg(colors.modal)
            .rounded(16.0)
            .shadow(Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.4),
                offset: Vector::new(0.0, 10.0),
                blur_radius: 25.0,
            })
            .padding([24, 24])
            .build()
    }
}

/// Résout les couleurs sémantiques vers des couleurs concrètes selon le mode.
pub struct SemanticColorResolver {
    pub primary: Color,
    pub primary_bright: Color,
    pub primary_light: Color,
    pub primary_dark: Color,
    pub background: Color,
    pub surface: Color,
    pub elevated: Color,
    pub modal: Color,
    pub text: Color,
    pub text_secondary: Color,
    pub text_on_accent: Color,
    pub border: Color,
    pub border_active: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
}

impl SemanticColorResolver {
    pub fn dark() -> Self {
        use crate::design::color;
        Self {
            primary: color::DARK.primary.base,
            primary_bright: color::DARK.primary.bright,
            primary_light: color::DARK.primary.light,
            primary_dark: color::DARK.primary.dark,
            background: color::DARK.neutral.step_0,
            surface: color::DARK.neutral.step_100,
            elevated: color::DARK.neutral.step_200,
            modal: color::DARK.neutral.step_300,
            text: color::DARK.neutral.step_950,
            text_secondary: color::DARK.neutral.step_500,
            text_on_accent: color::DARK.neutral.step_0,
            border: color::DARK.neutral.step_300,
            border_active: color::DARK.primary.base,
            success: color::DARK.status.success,
            warning: color::DARK.status.warning,
            error: color::DARK.status.error,
            info: color::DARK.status.info,
        }
    }

    pub fn light() -> Self {
        use crate::design::color;
        Self {
            primary: color::LIGHT.primary.base,
            primary_bright: color::LIGHT.primary.bright,
            primary_light: color::LIGHT.primary.light,
            primary_dark: color::LIGHT.primary.dark,
            background: color::LIGHT.neutral.step_0,
            surface: color::LIGHT.neutral.step_100,
            elevated: color::LIGHT.neutral.step_200,
            modal: color::LIGHT.neutral.step_300,
            text: color::LIGHT.neutral.step_950,
            text_secondary: color::LIGHT.neutral.step_500,
            text_on_accent: color::LIGHT.neutral.step_0,
            border: color::LIGHT.neutral.step_300,
            border_active: color::LIGHT.primary.base,
            success: color::LIGHT.status.success,
            warning: color::LIGHT.status.warning,
            error: color::LIGHT.status.error,
            info: color::LIGHT.status.info,
        }
    }
}
