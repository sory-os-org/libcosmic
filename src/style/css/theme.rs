//! Pont entre le moteur CSS et le système de thème libcosmic.
//!
//! Convertit les propriétés CSS résolues en styles applicables aux widgets
//! via `Container::custom()` et `StyleBuilder`.

use std::collections::HashMap;

use crate::design::style_builder::{StyleBuilder, Style};
use crate::iced::Color;

use super::properties::TypedValue;
use super::selector::MatchContext;
use super::stylesheet::global_style_manager;

/// Ensemble de propriétés CSS résolues pour un widget.
#[derive(Debug, Clone, Default)]
pub struct CssStyleSet {
    pub background: Option<Color>,
    pub text_color: Option<Color>,
    pub border_radius: Option<f32>,
    pub border_width: Option<f32>,
    pub border_color: Option<Color>,
    pub opacity: Option<f32>,
    pub width: Option<CssLength>,
    pub height: Option<CssLength>,
    pub min_width: Option<CssLength>,
    pub min_height: Option<CssLength>,
    pub max_width: Option<CssLength>,
    pub max_height: Option<CssLength>,
    pub padding: Option<CssBoxEdges>,
    pub margin: Option<CssBoxEdges>,
    pub font_size: Option<f32>,
    pub font_weight: Option<f32>,
    pub shadow: Option<CssShadow>,
    pub icon_size: Option<f32>,
    pub gap: Option<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CssLength {
    Fixed(f32),
    Fill,
    Shrink,
    Auto,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CssBoxEdges {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CssShadow {
    pub x: f32,
    pub y: f32,
    pub blur: f32,
    pub color: Color,
}

impl CssStyleSet {
    pub fn new() -> Self {
        Self::default()
    }

    /// Récupère les styles CSS pour un contexte de widget donné.
    pub fn from_context(context: &MatchContext) -> Self {
        let manager = global_style_manager();
        let props = manager.get_styles_for(context);
        drop(manager);

        let mut style = Self::new();
        for (name, value) in props {
            style.apply_property(&name, &value);
        }
        style
    }

    /// Applique une propriété résolue au style set.
    pub fn apply_property(&mut self, name: &str, value: &TypedValue) {
        match name {
            "background" | "background-color" => {
                if let TypedValue::Color(c) = value {
                    self.background = Some(*c);
                }
            }
            "color" => {
                if let TypedValue::Color(c) = value {
                    self.text_color = Some(*c);
                }
            }
            "opacity" => {
                if let TypedValue::Length(n) = value {
                    self.opacity = Some((*n).clamp(0.0, 1.0));
                }
            }
            "border-radius" => {
                if let TypedValue::BorderRadius(r) = value {
                    self.border_radius = Some(*r);
                }
            }
            "border-width" | "border" => {
                if let TypedValue::Length(n) = value {
                    self.border_width = Some(*n);
                }
            }
            "border-color" => {
                if let TypedValue::Color(c) = value {
                    self.border_color = Some(*c);
                }
            }
            "width" => self.width = css_length(value),
            "height" => self.height = css_length(value),
            "min-width" => self.min_width = css_length(value),
            "min-height" => self.min_height = css_length(value),
            "max-width" => self.max_width = css_length(value),
            "max-height" => self.max_height = css_length(value),
            "padding" => {
                if let TypedValue::Length(n) = value {
                    let v = *n;
                    self.padding = Some(CssBoxEdges { top: v, right: v, bottom: v, left: v });
                }
            }
            "margin" => {
                if let TypedValue::Length(n) = value {
                    let v = *n;
                    self.margin = Some(CssBoxEdges { top: v, right: v, bottom: v, left: v });
                }
            }
            "font-size" => {
                if let TypedValue::Length(n) = value {
                    self.font_size = Some(*n);
                }
            }
            "font-weight" => {
                if let TypedValue::Length(n) = value {
                    self.font_weight = Some(*n);
                }
            }
            "shadow" | "box-shadow" => {
                if let TypedValue::Shadow { x, y, blur, color } = value {
                    self.shadow = Some(CssShadow { x: *x, y: *y, blur: *blur, color: *color });
                }
            }
            "icon-size" => {
                if let TypedValue::Length(n) = value {
                    self.icon_size = Some(*n);
                }
            }
            "gap" | "spacing" => {
                if let TypedValue::Length(n) = value {
                    self.gap = Some(*n);
                }
            }
            _ => {}
        }
    }

    /// Convertit en un StyleBuilder pour application.
    pub fn to_style_builder(&self) -> StyleBuilder {
        let mut builder = crate::design::style_builder::style();
        if let Some(bg) = self.background {
            builder = builder.bg(bg);
        }
        if let Some(c) = self.text_color {
            builder = builder.text(c);
        }
        if let Some(r) = self.border_radius {
            builder = builder.rounded(r);
        }
        if let Some(o) = self.opacity {
            builder = builder.opacity(o);
        }
        // Padding
        if let Some(p) = self.padding {
            let pad = crate::iced::Padding::from([p.top, p.right, p.bottom, p.left]);
            builder = builder.padding(pad);
        }
        builder
    }
}

fn css_length(value: &TypedValue) -> Option<CssLength> {
    match value {
        TypedValue::Length(n) => Some(CssLength::Fixed(*n)),
        TypedValue::Keyword(k) if k == "fill" || k == "full" => Some(CssLength::Fill),
        TypedValue::Keyword(k) if k == "shrink" => Some(CssLength::Shrink),
        TypedValue::Keyword(k) if k == "auto" => Some(CssLength::Auto),
        _ => None,
    }
}
