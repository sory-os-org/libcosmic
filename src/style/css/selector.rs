//! Sélecteurs CSS — types et système de matching.
//!
//! Supporte les sélecteurs de type (Button, Card), de classe (.primary, .large),
//! d'ID (#main), universel (*), pseudo-classes (:hover), et combinateurs
//! descendants (A B).

use std::fmt;

/// Un sélecteur CSS.
#[derive(Debug, Clone, PartialEq)]
pub enum Selector {
    /// Sélecteur de type : `Button`, `Card`, `Sidebar`
    Type(String),
    /// Sélecteur de classe : `.primary`, `.large`
    Class(String),
    /// Sélecteur d'ID : `#main`, `#header`
    Id(String),
    /// Sélecteur universel : `*`
    Universal,
    /// Pseudo-classe : `:hover`, `:active`
    PseudoClass(String),
    /// Combinateur descendant : `A B`
    Descendant(Box<Selector>, Box<Selector>),
    /// Combinateur enfant : `A > B`
    Child(Box<Selector>, Box<Selector>),
    /// ET logique : `A.B` (type + classe)
    And(Box<Selector>, Box<Selector>),
}

impl fmt::Display for Selector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Selector::Type(name) => write!(f, "{}", name),
            Selector::Class(name) => write!(f, ".{}", name),
            Selector::Id(name) => write!(f, "#{}", name),
            Selector::Universal => write!(f, "*"),
            Selector::PseudoClass(name) => write!(f, ":{}", name),
            Selector::Descendant(a, b) => write!(f, "{} {}", a, b),
            Selector::Child(a, b) => write!(f, "{} > {}", a, b),
            Selector::And(a, b) => write!(f, "{}{}", a, b),
        }
    }
}

/// Contexte de matching pour un sélecteur.
///
/// Représente les attributs d'un widget auxquels on applique les sélecteurs.
#[derive(Debug, Clone, Default)]
pub struct MatchContext {
    /// Type du widget : `"Button"`, `"Card"`, `"Sidebar"`
    pub widget_type: String,
    /// Classes CSS du widget : `["primary", "large"]`
    pub classes: Vec<String>,
    /// ID du widget (optionnel)
    pub id: Option<String>,
    /// Pseudo-classes actives : `["hover"]`, `["active"]`
    pub pseudo: Vec<String>,
}

impl MatchContext {
    pub fn new(widget_type: impl Into<String>) -> Self {
        Self {
            widget_type: widget_type.into(),
            classes: Vec::new(),
            id: None,
            pseudo: Vec::new(),
        }
    }

    pub fn class(mut self, name: impl Into<String>) -> Self {
        self.classes.push(name.into());
        self
    }

    pub fn id(mut self, name: impl Into<String>) -> Self {
        self.id = Some(name.into());
        self
    }

    pub fn hover(mut self) -> Self {
        self.pseudo.push("hover".to_string());
        self
    }

    pub fn active(mut self) -> Self {
        self.pseudo.push("active".to_string());
        self
    }

    pub fn focus(mut self) -> Self {
        self.pseudo.push("focus".to_string());
        self
    }
}

/// Résultat d'un matching de sélecteur.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchResult {
    Match,
    NoMatch,
}

/// Vérifie si un sélecteur correspond à un contexte donné.
pub fn match_selector(selector: &Selector, context: &MatchContext) -> MatchResult {
    match selector {
        Selector::Universal => MatchResult::Match,
        Selector::Type(name) => {
            if name.eq_ignore_ascii_case(&context.widget_type) {
                MatchResult::Match
            } else {
                MatchResult::NoMatch
            }
        }
        Selector::Class(name) => {
            if context.classes.iter().any(|c| c == name) {
                MatchResult::Match
            } else {
                MatchResult::NoMatch
            }
        }
        Selector::Id(name) => {
            if context.id.as_deref() == Some(name) {
                MatchResult::Match
            } else {
                MatchResult::NoMatch
            }
        }
        Selector::PseudoClass(name) => {
            if context.pseudo.iter().any(|p| p == name) {
                MatchResult::Match
            } else {
                MatchResult::NoMatch
            }
        }
        Selector::And(a, b) => {
            match match_selector(a, context) {
                MatchResult::NoMatch => MatchResult::NoMatch,
                _ => match_selector(b, context),
            }
        }
        Selector::Descendant(..) | Selector::Child(..) => {
            // For simplicity in v1, check if the rightmost matches
            // and the leftmost might match a parent context
            // In a full implementation, this would need parent context
            // For now, we try to match the innermost (rightmost) selector
            match_rightmost(selector, context)
        }
    }
}

fn match_rightmost(selector: &Selector, context: &MatchContext) -> MatchResult {
    match selector {
        Selector::Descendant(_, right) | Selector::Child(_, right) => {
            match_selector(right, context)
        }
        other => match_selector(other, context),
    }
}

/// Trouve la meilleure règle pour un contexte donné dans une liste de sélecteurs.
/// Retourne l'index de la meilleure correspondance dans la liste.
pub fn find_best_match<'a>(
    selectors: &'a [Selector],
    context: &MatchContext,
) -> Option<(usize, &'a Selector)> {
    let mut best: Option<(usize, &Selector)> = None;
    let mut best_specificity = 0u32;

    for (i, sel) in selectors.iter().enumerate() {
        if match_selector(sel, context) == MatchResult::Match {
            let spec = specificity(sel);
            if spec > best_specificity {
                best_specificity = spec;
                best = Some((i, sel));
            }
        }
    }

    best
}

/// Calcule la spécificité d'un sélecteur CSS.
/// IDs = 100, Classes/pseudo = 10, Types = 1
pub fn specificity(selector: &Selector) -> u32 {
    match selector {
        Selector::Id(_) => 100,
        Selector::Class(_) | Selector::PseudoClass(_) => 10,
        Selector::Type(_) => 1,
        Selector::Universal => 0,
        Selector::And(a, b) => specificity(a) + specificity(b),
        Selector::Descendant(a, b) | Selector::Child(a, b) => specificity(a) + specificity(b),
    }
}
