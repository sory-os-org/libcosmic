//! Cache de styles CSS — évite de reparsher et rematcher à chaque frame.

use std::collections::HashMap;
use std::sync::Mutex;

use super::selector::MatchContext;
use super::theme::CssStyleSet;

/// Entrée de cache pour un contexte de widget donné.
#[derive(Debug, Clone)]
struct CacheEntry {
    styles: CssStyleSet,
    generation: u64,
}

/// Cache global des styles CSS résolus.
pub struct StyleCache {
    entries: HashMap<(String, String, String), CacheEntry>,
    generation: u64,
    max_entries: usize,
}

impl StyleCache {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            generation: 0,
            max_entries: 1000,
        }
    }

    /// Incrémente la génération (appelé après rechargement CSS).
    pub fn invalidate(&mut self) {
        self.generation += 1;
    }

    /// Vide complètement le cache.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Récupère les styles depuis le cache ou les calcule.
    pub fn get_or_compute(&mut self, context: &MatchContext) -> CssStyleSet {
        let key = self.make_key(context);

        // Check cache
        if let Some(entry) = self.entries.get(&key) {
            if entry.generation == self.generation {
                return entry.styles.clone();
            }
        }

        // Compute and cache
        let styles = CssStyleSet::from_context(context);

        // Evict oldest if at capacity
        if self.entries.len() >= self.max_entries {
            // Simple eviction: remove first entry
            if let Some(key) = self.entries.keys().next().cloned() {
                self.entries.remove(&key);
            }
        }

        self.entries.insert(key, CacheEntry {
            styles: styles.clone(),
            generation: self.generation,
        });

        styles
    }

    fn make_key(&self, context: &MatchContext) -> (String, String, String) {
        let type_key = context.widget_type.to_lowercase();
        let mut classes = context.classes.clone();
        classes.sort();
        let class_key = classes.join(".");
        let pseudo_key = context.pseudo.join(":");
        (type_key, class_key, pseudo_key)
    }
}

impl Default for StyleCache {
    fn default() -> Self { Self::new() }
}

/// Cache global.
static GLOBAL_CACHE: std::sync::LazyLock<Mutex<StyleCache>> =
    std::sync::LazyLock::new(|| Mutex::new(StyleCache::new()));

/// Accède au cache global.
pub fn global_cache() -> std::sync::MutexGuard<'static, StyleCache> {
    GLOBAL_CACHE.lock().unwrap()
}

/// Invalide le cache global.
pub fn invalidate_cache() {
    global_cache().invalidate();
}

/// Récupère les styles CSS pour un contexte via le cache global.
pub fn get_css_styles(context: &MatchContext) -> CssStyleSet {
    global_cache().get_or_compute(context)
}
