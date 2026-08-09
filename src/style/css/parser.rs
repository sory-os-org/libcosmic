//! Parseur CSS — construit un arbre syntaxique (Rules + Selectors + Declarations).

use super::selector::Selector;
use super::tokenizer::{Token, tokenize};
use super::properties::PropertyValue;

/// Une règle CSS complète : selecteurs + déclarations.
#[derive(Debug, Clone)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

/// Une déclaration CSS : propriété + valeur.
#[derive(Debug, Clone)]
pub struct Declaration {
    pub property: String,
    pub value: PropertyValue,
    pub important: bool,
}

/// Feuille de style parsée.
#[derive(Debug, Clone)]
pub struct ParsedStylesheet {
    pub rules: Vec<Rule>,
}

/// Parse une chaîne CSS en une feuille de style structurée.
pub fn parse_stylesheet(input: &str) -> ParsedStylesheet {
    let tokens = tokenize(input);
    let mut parser = Parser::new(tokens);
    parser.parse()
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<&Token> {
        let t = self.tokens.get(self.pos);
        self.pos += 1;
        t
    }

    fn expect(&mut self, expected: &Token) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn parse(&mut self) -> ParsedStylesheet {
        let mut rules = Vec::new();
        loop {
            match self.peek() {
                None | Some(Token::Eof) => break,
                _ => {
                    if let Some(rule) = self.parse_rule() {
                        rules.push(rule);
                    } else {
                        self.advance();
                    }
                }
            }
        }
        ParsedStylesheet { rules }
    }

    fn parse_rule(&mut self) -> Option<Rule> {
        // Skip at-rules for now
        if matches!(self.peek(), Some(Token::AtKeyword(_))) {
            self.skip_at_rule();
            return None;
        }

        let selectors = self.parse_selector_list()?;
        if !self.expect(&Token::LeftBrace) {
            return None;
        }

        let declarations = self.parse_declarations();
        self.expect(&Token::RightBrace);
        if selectors.is_empty() {
            return None;
        }
        Some(Rule {
            selectors,
            declarations,
        })
    }

    fn skip_at_rule(&mut self) {
        // Skip until semicolon or block
        while let Some(tok) = self.advance() {
            match tok {
                Token::LeftBrace => {
                    // Skip block
                    let mut depth = 1;
                    while depth > 0 {
                        match self.advance() {
                            Some(Token::LeftBrace) => depth += 1,
                            Some(Token::RightBrace) => depth -= 1,
                            None => break,
                            _ => {}
                        }
                    }
                    return;
                }
                Token::Semicolon => return,
                _ => {}
            }
        }
    }

    fn parse_selector_list(&mut self) -> Option<Vec<Selector>> {
        let mut selectors = Vec::new();
        loop {
            let sel = self.parse_selector()?;
            selectors.push(sel);
            if self.expect(&Token::Comma) {
                continue;
            }
            break;
        }
        Some(selectors)
    }

    fn parse_selector(&mut self) -> Option<Selector> {
        let mut simple = self.parse_simple_selector()?;

        loop {
            match self.peek().cloned() {
                Some(Token::Dot) => {
                    self.advance();
                    let ident = self.expect_ident()?;
                    let class_sel = Selector::Class(ident);
                    // Combine type + class (e.g. Button.primary -> And(Type("Button"), Class("primary")))
                    simple = Selector::And(Box::new(simple), Box::new(class_sel));
                }
                Some(Token::Hash(_)) => {
                    if let Some(Token::Hash(name)) = self.advance().cloned() {
                        let id_sel = Selector::Id(name);
                        simple = Selector::And(Box::new(simple), Box::new(id_sel));
                    }
                }
                Some(Token::Colon) => {
                    self.advance();
                    let pseudo = self.expect_ident()?;
                    let pseudo_sel = Selector::PseudoClass(pseudo);
                    simple = Selector::And(Box::new(simple), Box::new(pseudo_sel));
                }
                Some(Token::LeftBrace) | Some(Token::Comma) => break,
                Some(_) => break,
                None => break,
            }
        }
        Some(simple)
    }

    fn parse_simple_selector(&mut self) -> Option<Selector> {
        match self.peek() {
            Some(Token::Ident(name)) => {
                let name = name.clone();
                self.advance();
                Some(Selector::Type(name))
            }
            Some(Token::Dot) => {
                self.advance();
                let name = self.expect_ident()?;
                Some(Selector::Class(name))
            }
            Some(Token::Hash(_)) => {
                if let Token::Hash(name) = self.advance().unwrap().clone() {
                    Some(Selector::Id(name))
                } else {
                    None
                }
            }
            Some(Token::Delim('*')) => {
                self.advance();
                Some(Selector::Universal)
            }
            Some(Token::LeftBrace) | Some(Token::Comma) => None,
            _ => {
                // Try treating as ident anyway
                let name = self.expect_ident()?;
                Some(Selector::Type(name))
            }
        }
    }

    fn expect_ident(&mut self) -> Option<String> {
        match self.peek() {
            Some(Token::Ident(name)) => {
                let name = name.clone();
                self.advance();
                Some(name)
            }
            _ => None,
        }
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        let mut decls = Vec::new();
        loop {
            match self.peek() {
                Some(Token::RightBrace) | None => break,
                Some(Token::Semicolon) => {
                    self.advance();
                }
                Some(Token::Ident(_)) => {
                    if let Some(decl) = self.parse_declaration() {
                        decls.push(decl);
                    }
                    self.expect(&Token::Semicolon);
                }
                _ => {
                    self.advance();
                }
            }
        }
        decls
    }

    fn parse_declaration(&mut self) -> Option<Declaration> {
        let property = self.expect_ident()?;
        if !self.expect(&Token::Colon) {
            return None;
        }

        let value = self.parse_value();
        let important = if matches!(self.peek(), Some(Token::Delim('!'))) {
            self.advance();
            self.expect_ident();
            true
        } else {
            false
        };

        Some(Declaration {
            property,
            value,
            important,
        })
    }

    fn parse_value(&mut self) -> PropertyValue {
        let mut parts = Vec::new();
        loop {
            match self.peek() {
                Some(Token::Semicolon) | Some(Token::RightBrace) | None => break,
                Some(Token::Delim('!')) => break,
                Some(Token::Ident(s)) => {
                    parts.push(PropertyValue::Keyword(s.clone()));
                    self.advance();
                }
                Some(Token::Number(n)) => {
                    let n = *n;
                    self.advance();
                    parts.push(PropertyValue::Number(n));
                }
                Some(Token::Dimension(n, unit)) => {
                    let n = *n;
                    let unit = unit.clone();
                    self.advance();
                    parts.push(PropertyValue::Dimension(n, unit));
                }
                Some(Token::Percentage(p)) => {
                    let p = *p;
                    self.advance();
                    parts.push(PropertyValue::Percentage(p));
                }
                Some(Token::Hash(name)) => {
                    let name = name.clone();
                    self.advance();
                    parts.push(PropertyValue::Color(name));
                }
                Some(Token::String(s)) => {
                    let s = s.clone();
                    self.advance();
                    parts.push(PropertyValue::String(s));
                }
                Some(Token::Dot) => {
                    // Could be part of a number like 0.5, or part of a selector
                    // For values, treat as part of number
                    self.advance();
                    if let Some(Token::Number(n)) = self.peek() {
                        // Already consumed as dot, next is fractional part
                        // This handles cases where the tokenizer already got the number right
                    }
                    parts.push(PropertyValue::Keyword(".".to_string()));
                }
                Some(Token::Comma) => {
                    self.advance();
                    parts.push(PropertyValue::Comma);
                }
                Some(Token::Whitespace) => {
                    self.advance();
                    // Skip whitespace between value parts
                }
                _ => {
                    self.advance();
                }
            }
        }
        PropertyValue::List(parts)
    }
}
