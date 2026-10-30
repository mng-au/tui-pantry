const START_TAB: &str = "start_tab";
const START_WIDGET: &str = "start_widget";
const START_VARIANT: &str = "start_variant";

/// Initial harness view requested via `pantry.toml` `[config]` keys.
///
/// Every field is optional. `None` means "use the built-in default", so
/// [`InitialView::default`] reproduces the harness's stock opening view
/// (Widgets tab, first variant, top of the sidebar).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct InitialView {
    /// Top-bar tab to open (`Widgets`, `Panes`, `Views`, `Styles`).
    pub start_tab: Option<String>,
    /// Widget group (`Ingredient::group()`) whose first variant is selected.
    pub start_widget: Option<String>,
    /// Specific variant (`Ingredient::name()`) within `start_widget`.
    pub start_variant: Option<String>,
}

impl InitialView {
    /// Parse the optional `start_tab`, `start_widget`, and `start_variant`
    /// keys from `[config]` in `pantry.toml`.
    ///
    /// A key is captured only when present *and* a string; missing keys,
    /// non-string values, and a missing `[config]` table all yield `None`.
    pub fn from_toml(table: &toml::Table) -> Self {
        let config = table.get("config").and_then(|v| v.as_table());

        let string = |key: &str| {
            config
                .and_then(|c| c.get(key))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        };

        Self {
            start_tab: string(START_TAB),
            start_widget: string(START_WIDGET),
            start_variant: string(START_VARIANT),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table(src: &str) -> toml::Table {
        src.parse().unwrap()
    }

    #[test]
    fn all_keys_present() {
        let t = table(
            r#"
[config]
start_tab = "Views"
start_widget = "dashboard"
start_variant = "summary"
"#,
        );

        let iv = InitialView::from_toml(&t);
        assert_eq!(iv.start_tab.as_deref(), Some("Views"));
        assert_eq!(iv.start_widget.as_deref(), Some("dashboard"));
        assert_eq!(iv.start_variant.as_deref(), Some("summary"));
    }

    #[test]
    fn missing_keys_are_none() {
        let t = table("[config]\ntheme = \"dark\"\n");
        let iv = InitialView::from_toml(&t);
        assert_eq!(iv.start_tab, None);
        assert_eq!(iv.start_widget, None);
        assert_eq!(iv.start_variant, None);
    }

    #[test]
    fn no_config_table_is_none() {
        let t = table(r##"[colors.brand]
x = "#000""##);
        let iv = InitialView::from_toml(&t);
        assert_eq!(iv, InitialView::default());
    }

    #[test]
    fn non_string_values_are_ignored() {
        let t = table(
            r#"
[config]
start_tab = 5
start_widget = true
"#,
        );
        let iv = InitialView::from_toml(&t);
        assert_eq!(iv.start_tab, None);
        assert_eq!(iv.start_widget, None);
        assert_eq!(iv.start_variant, None);
    }
}
