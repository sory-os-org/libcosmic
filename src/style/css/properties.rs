//! Valeurs de propriétés CSS — types fortement typés pour l'application aux widgets.

use crate::design::style_builder::Style;

/// Valeur d'une propriété CSS.
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    /// Un mot-clé : `auto`, `none`, `block`
    Keyword(String),
    /// Un nombre : `16`, `0.5`
    Number(f64),
    /// Une dimension avec unité : `16px`, `2rem`, `50%`
    Dimension(f64, String),
    /// Un pourcentage : `50%`
    Percentage(f64),
    /// Une couleur hexadécimale : `#3b82f6`, `#fff`
    Color(String),
    /// Une chaîne de caractères : `"Helvetica"`
    String(String),
    /// Une virgule (séparateur dans une liste)
    Comma,
    /// Une liste de valeurs
    List(Vec<PropertyValue>),
}

/// Propriété CSS résolue typée.
#[derive(Debug, Clone, PartialEq)]
pub enum TypedValue {
    Color(crate::iced::Color),
    Length(f32),
    Percentage(f32),
    String(String),
    Keyword(String),
    BorderRadius(f32),
    Shadow {
        x: f32,
        y: f32,
        blur: f32,
        color: crate::iced::Color,
    },
}

/// Une propriété CSS résolue (nom + valeur typée).
#[derive(Debug, Clone)]
pub struct ResolvedProperty {
    pub name: String,
    pub value: TypedValue,
}

/// Résout une PropertyValue en TypedValue en fonction du nom de propriété.
pub fn resolve_property(name: &str, value: &PropertyValue) -> Option<TypedValue> {
    match name {
        "background" | "background-color" => resolve_color(value),
        "color" => resolve_color(value),
        "border-radius" => resolve_border_radius(value),
        "opacity" => resolve_number(value, 0.0, 1.0),
        "width" | "height" | "min-width" | "min-height" | "max-width" | "max-height" => {
            resolve_length(value)
        }
        "padding" | "margin" | "gap" | "spacing" => resolve_length(value),
        "font-size" => resolve_length(value),
        "font-weight" => resolve_number(value, 100.0, 900.0),
        "shadow" | "box-shadow" => resolve_shadow(value),
        "icon-size" => resolve_length(value),
        _ => resolve_generic(value),
    }
}

fn resolve_color(value: &PropertyValue) -> Option<TypedValue> {
    match value {
        PropertyValue::Color(hex) => parse_hex_color(hex),
        PropertyValue::Keyword(name) => parse_named_color(name),
        PropertyValue::List(parts) => {
            // Handle rgb/rgba function: rgb(255, 0, 0) or rgb(255 0 0)
            if parts.len() >= 2 {
                if let PropertyValue::Keyword(func) = &parts[0] {
                    if func == "rgb" || func == "rgba" {
                        return parse_rgb_function(parts);
                    }
                }
            }
            // Try each part
            for part in parts {
                if let Some(color) = resolve_color(part) {
                    return Some(color);
                }
            }
            None
        }
        _ => None,
    }
}

fn resolve_border_radius(value: &PropertyValue) -> Option<TypedValue> {
    match value {
        PropertyValue::Dimension(n, _) => Some(TypedValue::BorderRadius(*n as f32)),
        PropertyValue::Number(n) => Some(TypedValue::BorderRadius(*n as f32)),
        PropertyValue::Keyword(name) if name == "none" => Some(TypedValue::BorderRadius(0.0)),
        PropertyValue::Keyword(name) if name == "pill" || name == "full" => {
            Some(TypedValue::BorderRadius(999.0))
        }
        PropertyValue::List(parts) => {
            for part in parts {
                if let Some(val) = resolve_border_radius(part) {
                    return Some(val);
                }
            }
            None
        }
        _ => None,
    }
}

fn resolve_number(value: &PropertyValue, min: f64, max: f64) -> Option<TypedValue> {
    match value {
        PropertyValue::Number(n) => Some(TypedValue::Length(n.clamp(min, max) as f32)),
        PropertyValue::Keyword(name) => name.parse::<f64>().ok().map(|n| {
            TypedValue::Length(n.clamp(min, max) as f32)
        }),
        _ => None,
    }
}

fn resolve_length(value: &PropertyValue) -> Option<TypedValue> {
    match value {
        PropertyValue::Dimension(n, _) => Some(TypedValue::Length(*n as f32)),
        PropertyValue::Number(n) => Some(TypedValue::Length(*n as f32)),
        PropertyValue::Percentage(p) => Some(TypedValue::Percentage(*p as f32)),
        PropertyValue::Keyword(name) if name == "auto" => Some(TypedValue::Keyword("auto".into())),
        PropertyValue::Keyword(name) if name == "fill" || name == "full" => {
            Some(TypedValue::Keyword("fill".into()))
        }
        PropertyValue::Keyword(name) if name == "shrink" => {
            Some(TypedValue::Keyword("shrink".into()))
        }
        _ => None,
    }
}

fn resolve_shadow(value: &PropertyValue) -> Option<TypedValue> {
    match value {
        PropertyValue::Keyword(name) if name == "none" => {
            Some(TypedValue::Shadow {
                x: 0.0, y: 0.0, blur: 0.0,
                color: crate::iced::Color::TRANSPARENT,
            })
        }
        PropertyValue::List(parts) => {
            // shadow format: x y blur color
            let mut x = 0.0_f32;
            let mut y = 0.0_f32;
            let mut blur = 0.0_f32;
            let mut color = crate::iced::Color::from_rgba(0.0, 0.0, 0.0, 0.25);

            for part in parts {
                match part {
                    PropertyValue::Dimension(n, _) => {
                        if x == 0.0 { x = *n as f32; }
                        else if y == 0.0 { y = *n as f32; }
                        else { blur = *n as f32; }
                    }
                    PropertyValue::Number(n) => {
                        if x == 0.0 && *n != 0.0 { x = *n as f32; }
                        else if y == 0.0 { y = *n as f32; }
                        else { blur = *n as f32; }
                    }
                    PropertyValue::Color(hex) => {
                        if let Some(TypedValue::Color(c)) = parse_hex_color(hex) {
                            color = c;
                        }
                    }
                    _ => {}
                }
            }

            Some(TypedValue::Shadow { x, y, blur, color })
        }
        _ => None,
    }
}

fn resolve_generic(value: &PropertyValue) -> Option<TypedValue> {
    match value {
        PropertyValue::Keyword(k) => Some(TypedValue::Keyword(k.clone())),
        PropertyValue::String(s) => Some(TypedValue::String(s.clone())),
        PropertyValue::Number(n) => Some(TypedValue::Length(*n as f32)),
        PropertyValue::Dimension(n, _) => Some(TypedValue::Length(*n as f32)),
        _ => None,
    }
}

fn parse_hex_color(hex: &str) -> Option<TypedValue> {
    let hex = hex.trim_start_matches('#');
    let (r, g, b, a) = match hex.len() {
        3 => {
            let r = u8::from_str_radix(&hex[0..1], 16).ok()? * 17;
            let g = u8::from_str_radix(&hex[1..2], 16).ok()? * 17;
            let b = u8::from_str_radix(&hex[2..3], 16).ok()? * 17;
            (r, g, b, 255)
        }
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            (r, g, b, 255)
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            (r, g, b, a)
        }
        _ => return None,
    };
    Some(TypedValue::Color(crate::iced::Color::from_rgba8(r, g, b, a as f32 / 255.0)))
}

fn parse_named_color(name: &str) -> Option<TypedValue> {
    match name.to_lowercase().as_str() {
        "transparent" => Some(TypedValue::Color(crate::iced::Color::TRANSPARENT)),
        "white" => Some(TypedValue::Color(crate::iced::Color::WHITE)),
        "black" => Some(TypedValue::Color(crate::iced::Color::BLACK)),
        "red" => Some(TypedValue::Color(crate::iced::Color::from_rgb8(255, 0, 0))),
        "green" => Some(TypedValue::Color(crate::iced::Color::from_rgb8(0, 128, 0))),
        "blue" => Some(TypedValue::Color(crate::iced::Color::from_rgb8(0, 0, 255))),
        _ => None,
    }
}

fn parse_rgb_function(parts: &[PropertyValue]) -> Option<TypedValue> {
    let numbers: Vec<f64> = parts.iter().filter_map(|p| {
        match p {
            PropertyValue::Number(n) => Some(*n),
            _ => None,
        }
    }).collect();

    match numbers.len() {
        3 => Some(TypedValue::Color(crate::iced::Color::from_rgb8(
            numbers[0] as u8, numbers[1] as u8, numbers[2] as u8,
        ))),
        4 => Some(TypedValue::Color(crate::iced::Color::from_rgba(
            numbers[0] as f32 / 255.0,
            numbers[1] as f32 / 255.0,
            numbers[2] as f32 / 255.0,
            numbers[3] as f32 / 255.0,
        ))),
        _ => None,
    }
}
