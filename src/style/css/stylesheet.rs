//! Chargement et gestion des fichiers CSS.
//!
//! Supporte le chargement depuis des fichiers, le rechargement à chaud,
//! et la mise en cache des feuilles de style parsées.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use super::parser::{parse_stylesheet, ParsedStylesheet, Rule, Declaration};
use super::properties::{PropertyValue, TypedValue, resolve_property};
use super::selector::{MatchContext, find_best_match, Selector};

/// Erreur de chargement CSS.
#[derive(Debug)]
pub enum CssError {
    Io(std::io::Error),
    Parse(String),
    NotFound(String),
}

impl std::fmt::Display for CssError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CssError::Io(e) => write!(f, "IO error: {}", e),
            CssError::Parse(s) => write!(f, "Parse error: {}", s),
            CssError::NotFound(s) => write!(f, "File not found: {}", s),
        }
    }
}

impl From<std::io::Error> for CssError {
    fn from(e: std::io::Error) -> Self { CssError::Io(e) }
}

/// Une feuille de style chargée, avec suivi de fichier pour le hot-reload.
#[derive(Debug, Clone)]
pub struct CssFile {
    pub path: PathBuf,
    pub source: String,
    pub stylesheet: ParsedStylesheet,
    pub last_modified: std::time::SystemTime,
}

impl CssFile {
    pub fn load(path: impl Into<PathBuf>) -> Result<Self, CssError> {
        let path: PathBuf = path.into();
        let source = std::fs::read_to_string(&path)?;
        let last_modified = std::fs::metadata(&path)
            .and_then(|m| m.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH);

        let stylesheet = parse_stylesheet(&source);

        Ok(Self {
            path,
            source,
            stylesheet,
            last_modified,
        })
    }

    pub fn reload(&mut self) -> Result<bool, CssError> {
        let modified = std::fs::metadata(&self.path)
            .and_then(|m| m.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH);

        if modified > self.last_modified {
            self.source = std::fs::read_to_string(&self.path)?;
            self.stylesheet = parse_stylesheet(&self.source);
            self.last_modified = modified;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

/// Gestionnaire global de feuilles de style.
pub struct StyleManager {
    files: HashMap<String, CssFile>,
    watch_enabled: bool,
}

impl StyleManager {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            watch_enabled: true,
        }
    }

    pub fn load(&mut self, name: &str, path: impl Into<PathBuf>) -> Result<(), CssError> {
        let css = CssFile::load(path)?;
        self.files.insert(name.to_string(), css);
        Ok(())
    }

    pub fn load_string(&mut self, name: &str, source: &str) {
        let stylesheet = parse_stylesheet(source);
        let file = CssFile {
            path: PathBuf::new(),
            source: source.to_string(),
            stylesheet,
            last_modified: std::time::SystemTime::UNIX_EPOCH,
        };
        self.files.insert(name.to_string(), file);
    }

    pub fn remove(&mut self, name: &str) {
        self.files.remove(name);
    }

    pub fn reload_all(&mut self) -> Vec<String> {
        let mut changed = Vec::new();
        for (name, file) in &mut self.files {
            if file.reload().unwrap_or(false) {
                changed.push(name.clone());
            }
        }
        changed
    }

    pub fn get_styles_for(&self, context: &MatchContext) -> Vec<(String, TypedValue)> {
        let mut results: Vec<(String, TypedValue, u32, bool)> = Vec::new();

        for file in self.files.values() {
            for rule in &file.stylesheet.rules {
                if let Some((_, _sel)) = find_best_match(&rule.selectors, context) {
                    for decl in &rule.declarations {
                        if let Some(typed) = resolve_property(&decl.property, &decl.value) {
                            let spec = rule.selectors.iter()
                                .map(|s| super::selector::specificity(s))
                                .max().unwrap_or(0);
                            results.push((decl.property.clone(), typed, spec, decl.important));
                        }
                    }
                }
            }
        }

        // Sort by specificity descending
        results.sort_by(|a, b| b.2.cmp(&a.2).then_with(|| b.3.cmp(&a.3)));

        // Deduplicate by property name (keep highest specificity)
        let mut seen = std::collections::HashSet::new();
        let mut final_results = Vec::new();
        for (prop, val, _, _) in results {
            if seen.insert(prop.clone()) {
                final_results.push((prop, val));
            }
        }

        final_results
    }

    pub fn set_watch(&mut self, enabled: bool) {
        self.watch_enabled = enabled;
    }
}

impl Default for StyleManager {
    fn default() -> Self { Self::new() }
}

/// StyleManager global accessible depuis toute l'application.
static GLOBAL_MANAGER: std::sync::LazyLock<Mutex<StyleManager>> =
    std::sync::LazyLock::new(|| Mutex::new(StyleManager::new()));

/// Accède au gestionnaire global.
pub fn global_style_manager() -> std::sync::MutexGuard<'static, StyleManager> {
    GLOBAL_MANAGER.lock().unwrap()
}

/// Charge un fichier CSS dans le gestionnaire global.
pub fn load_css_file(name: &str, path: impl AsRef<Path>) -> Result<(), CssError> {
    global_style_manager().load(name, path.as_ref().to_path_buf())
}

/// Charge une chaîne CSS dans le gestionnaire global.
pub fn load_css_string(name: &str, source: &str) {
    global_style_manager().load_string(name, source);
}

/// Recharge tous les fichiers CSS modifiés.
pub fn reload_css_files() -> Vec<String> {
    global_style_manager().reload_all()
}
