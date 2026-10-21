const START_TAB: &str = "start_tab";
const START_WIDGET: &str = "start_widget";
const START_VARIANT: &str = "start_variant";

#[derive(Debug, Default, Clone, PartialEq)]
pub struct InitialView {
    pub start_tab: Option<String>,
    pub start_widget: Option<String>,
    pub start_variant: Option<String>,
}

impl InitialView {
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

