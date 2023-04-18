#[cfg(test)]
use mockall::automock;

use crate::models::theme::Theme;

#[cfg_attr(test, automock)]
pub trait ThemeRepository {
    fn update_theme(&self, theme: Theme) -> anyhow::Result<()>;
}
