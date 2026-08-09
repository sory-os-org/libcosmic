//! Moteur de styles CSS natif pour libcosmic.
//!
//! Ce module implémente un parseur, un moteur de sélecteurs, un chargeur
//! de fichiers et un cache pour un système de styles CSS entièrement natif.
//!
//! Aucun navigateur, WebView, HTML ou JavaScript n'est utilisé.
//! Le CSS est uniquement un langage déclaratif de styles pour les widgets.
//!
//! # Architecture
//!
//! ```text
//! Fichier CSS  →  Tokenizer  →  Parser  →  Stylesheet (Rules)
//!                                              ↓
//! Widget .class("name")  →  Selector Engine  →  Match
//!                                              ↓
//!                                      Properties → TypedValues
//!                                              ↓
//!                                      CssStyleSet → StyleBuilder
//!                                              ↓
//!                                      Container::custom(|_| style)
//! ```
//!
//! # Utilisation rapide
//!
//! ```ignore
//! use cosmic::style::css;
//!
//! // Charger les thèmes SoryOS
//! css::load_soryos_theme(true); // dark = true
//!
//! // Ou charger un fichier CSS personnalisé
//! css::load_css_file("theme", "assets/styles/main.css").unwrap();
//!
//! // Appliquer des styles à un widget
//! let styled = container(content).class(css::apply_css("Card", &["elevated"]));
//! ```

pub mod cache;
pub mod parser;
pub mod properties;
pub mod selector;
pub mod stylesheet;
pub mod theme;
pub mod tokenizer;

pub use cache::{get_css_styles, invalidate_cache};
pub use selector::{MatchContext, match_selector, Selector};
pub use stylesheet::{load_css_file, load_css_string, reload_css_files, StyleManager, CssFile};
pub use theme::CssStyleSet;

/// Raccourci pour récupérer les styles CSS pour un type de widget + classes.
pub fn get_styles_for(widget_type: &str, classes: &[&str]) -> CssStyleSet {
    let mut ctx = MatchContext::new(widget_type);
    for class in classes {
        ctx = ctx.class(*class);
    }
    get_css_styles(&ctx)
}

/// Applique les styles CSS à un container.
pub fn apply_css<'a>(
    widget_type: &str,
    classes: &[&str],
) -> impl Fn(&crate::Theme) -> iced_widget::container::Style + 'a {
    let styles = get_styles_for(widget_type, classes);

    move |_theme: &crate::Theme| {
        let mut s = iced_widget::container::Style::default();

        if let Some(bg) = styles.background {
            s.background = Some(crate::iced::Background::Color(bg));
        }
        if let Some(c) = styles.text_color {
            s.text_color = Some(c);
        }
        if let Some(r) = styles.border_radius {
            s.border.radius = crate::iced::border::Radius::from(r);
        }
        if let Some(w) = styles.border_width {
            s.border.width = w;
        }
        if let Some(c) = styles.border_color {
            s.border.color = c;
        }

        s
    }
}

/// Charge les thèmes CSS SoryOS (dark ou light).
///
/// Les feuilles de style sont chargées en mémoire et disponibles
/// via `get_styles_for()` ou `apply_css()`.
pub fn load_soryos_theme(dark: bool) {
    if dark {
        load_css_string("soryos-dark", include_str!("soryos-dark.css"));
    } else {
        load_css_string("soryos-light", include_str!("soryos-light.css"));
    }
    load_css_string("soryos-layout", include_str!("soryos-layout.css"));
}

/// Crée un style closure pour un widget type avec des classes CSS données.
///
/// Utilisation:
/// ```ignore
/// use cosmic::style::css;
///
/// let btn_style = css::widget_style("Button", &["primary"]);
/// let card_style = css::widget_style("Card", &["elevated"]);
///
/// let button = button::custom("Click me").class(btn_style);
/// let card = container(content).class(card_style);
/// ```
pub fn widget_style<'a>(
    widget_type: &'a str,
    classes: &'a [&str],
) -> impl Fn(&crate::Theme) -> iced_widget::container::Style + 'a {
    apply_css(widget_type, classes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_basic() {
        let tokens = tokenizer::tokenize("Button { color: red; }");
        assert!(!tokens.is_empty());
    }

    #[test]
    fn test_parse_basic() {
        let css = r#"
            Button {
                background: #3b82f6;
                color: white;
                border-radius: 12px;
                padding: 10px 16px;
            }
        "#;
        let sheet = parser::parse_stylesheet(css);
        assert_eq!(sheet.rules.len(), 1);
        assert_eq!(sheet.rules[0].declarations.len(), 4);
    }

    #[test]
    fn test_parse_multiple_selectors() {
        let css = r#"
            Button.primary {
                background: #3b82f6;
            }
            Card {
                background: #18181b;
                border-radius: 16px;
            }
        "#;
        let sheet = parser::parse_stylesheet(css);
        assert_eq!(sheet.rules.len(), 2);
    }

    #[test]
    fn test_selector_matching() {
        let ctx = MatchContext::new("Button").class("primary");
        let type_sel = Selector::Type("Button".into());
        let class_sel = Selector::Class("primary".into());
        let and_sel = Selector::And(Box::new(type_sel.clone()), Box::new(class_sel.clone()));

        assert_eq!(match_selector(&type_sel, &ctx), selector::MatchResult::Match);
        assert_eq!(match_selector(&class_sel, &ctx), selector::MatchResult::Match);
        assert_eq!(match_selector(&and_sel, &ctx), selector::MatchResult::Match);
        let wrong_sel = Selector::Type("Card".into());
        assert_eq!(match_selector(&wrong_sel, &ctx), selector::MatchResult::NoMatch);
    }

    #[test]
    fn test_color_parsing() {
        if let Some(TypedValue::Color(c)) = properties::parse_hex_color("#ff0000") {
            assert!((c.r - 1.0).abs() < 0.01);
            assert!(c.g.abs() < 0.01);
        } else {
            panic!("Expected Color");
        }
        assert!(properties::parse_hex_color("#fff").is_some());
    }

    #[test]
    fn test_stylesheet_load_string() {
        load_css_string("test", r#"
            Button { background: #ff0000; color: white; }
        "#);
        let styles = get_styles_for("Button", &[]);
        assert!(styles.background.is_some());
        assert!(styles.text_color.is_some());
        stylesheet::global_style_manager().remove("test");
        invalidate_cache();
    }

    #[test]
    fn test_specificity() {
        let id_sel = Selector::Id("main".into());
        let class_sel = Selector::Class("primary".into());
        let type_sel = Selector::Type("Button".into());
        assert!(selector::specificity(&id_sel) > selector::specificity(&class_sel));
        assert!(selector::specificity(&class_sel) > selector::specificity(&type_sel));
    }
}