/// Lookup and interpolation helpers for translations.
pub struct TextResolver;

impl TextResolver {
    /// Resolves a key using the active catalog, then the fallback catalog.
    pub fn resolve(active: Option<&str>, fallback: Option<&str>, key: &str) -> String {
        active.or(fallback).unwrap_or(key).to_owned()
    }

    /// Replaces `{name}` placeholders in `template`.
    pub fn interpolate(template: &str, arguments: &[(&str, &str)]) -> String {
        let mut result = template.to_owned();

        for (name, value) in arguments {
            result = result.replace(&format!("{{{name}}}"), value);
        }

        result
    }
}
