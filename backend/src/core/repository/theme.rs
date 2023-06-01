#[cfg(test)]
use mockall::automock;

use crate::models::theme::{Theme, ThemeSetting};

#[cfg_attr(test, automock)]
pub trait ThemeRepository {
    fn get_theme_value(&self, theme_key: &str) -> anyhow::Result<Option<String>>; // talisman-ignore-line
    fn get_theme_entries(&self) -> anyhow::Result<Vec<ThemeSetting>>;
    fn update_theme(&self, theme: Theme) -> anyhow::Result<()>;
}
