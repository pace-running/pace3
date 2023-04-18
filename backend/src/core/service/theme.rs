use crate::{core::repository::ThemeRepository, models::theme::Theme};

use anyhow::Ok;
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait ThemeService {
    fn update_theme(&self, theme: Theme) -> anyhow::Result<()>;
}

pub struct DefaultThemeService<TR: ThemeRepository> {
    theme_repository: TR,
}

impl<TR: ThemeRepository> DefaultThemeService<TR> {
    pub fn new(theme_repository: TR) -> Self {
        Self { theme_repository }
    }
}

impl<TR: ThemeRepository> ThemeService for DefaultThemeService<TR> {
    fn update_theme(&self, theme: Theme) -> anyhow::Result<()> {
        self.theme_repository
            .update_theme(theme)
            .and_then(|_| Ok(()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::repository::MockThemeRepository;
    use mockall::predicate::*;

    #[test]
    fn update_theme_must_call_repository() {
        let mut theme_repository = MockThemeRepository::new();
        let theme = Theme::new(
            "test title".to_string(),
            "test description".to_string(),
            "registration is closed!".to_string(),
            "true".to_string(),
            "false".to_string(),
        )
        .unwrap();

        theme_repository
            .expect_update_theme()
            .with(eq(theme.clone()))
            .times(1)
            .returning(|_| Ok(()));

        let theme_service = DefaultThemeService::new(theme_repository);

        let result = theme_service.update_theme(theme);

        assert!(result.is_ok());
    }
}
