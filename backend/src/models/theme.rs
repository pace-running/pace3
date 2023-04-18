use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Queryable, Serialize, Deserialize, Clone, Debug)]
pub struct ThemeSetting {
    pub event_key: String,
    pub event_value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Theme {
    event_title: String,
    event_description: String,
    closed_registration_message: String,
    is_registration_open: bool,
    tshirts_enabled: bool,
}

impl Theme {
    pub fn new(
        event_title: String,
        event_description: String,
        closed_registration_message: String,
        is_registration_open: bool,
        tshirts_enabled: bool,
    ) -> anyhow::Result<Self> {
        if event_title.len() < 3 {
            return Err(anyhow::Error::msg(
                "Event title should contain at least three characters!",
            ));
        } else if event_title.len() > 50 {
            return Err(anyhow::Error::msg(
                "Event title should contain at most 50 characters!",
            ));
        } else if event_description.len() < 3 {
            return Err(anyhow::Error::msg(
                "Event description should contain at least three characters!",
            ));
        } else if event_description.len() > 2000 {
            return Err(anyhow::Error::msg(
                "Event description should contain at most 2000 characters!",
            ));
        } else if closed_registration_message.len() > 500 {
            return Err(anyhow::Error::msg(
                "Closed registration message should contain at most 500 characters!",
            ));
        }

        Ok(Self {
            event_title,
            event_description,
            closed_registration_message,
            is_registration_open,
            tshirts_enabled,
        })
    }

    pub fn event_title(&self) -> &str {
        &self.event_title
    }

    pub fn event_description(&self) -> &str {
        &self.event_description
    }

    pub fn closed_registration_message(&self) -> &str {
        &self.closed_registration_message
    }

    pub fn is_registration_open(&self) -> &bool {
        &self.is_registration_open
    }

    pub fn tshirts_enabled(&self) -> &bool {
        &self.tshirts_enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_return_ok_if_data_is_valid_with_minimum_length() {
        let event_title = "t".repeat(3);
        let event_description = "d".repeat(3);
        let closed_registration_message = "".to_string();
        let is_registration_open = true;
        let tshirts_enabled = false;
        let result = Theme::new(
            event_title,
            event_description,
            closed_registration_message,
            is_registration_open,
            tshirts_enabled,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn new_should_return_ok_if_data_is_valid_with_maximum_length() {
        let event_title = "t".repeat(50);
        let event_description = "d".repeat(2000);
        let closed_registration_message = "m".repeat(500);
        let is_registration_open = true;
        let tshirts_enabled = false;
        let result = Theme::new(
            event_title,
            event_description,
            closed_registration_message,
            is_registration_open,
            tshirts_enabled,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn new_should_return_err_if_title_is_too_short() {
        let event_title = "ti".to_string();
        let event_description = "descr".to_string();
        let closed_registration_message = "msg".to_string();
        let is_registration_open = true;
        let tshirts_enabled = false;
        let result = Theme::new(
            event_title,
            event_description,
            closed_registration_message,
            is_registration_open,
            tshirts_enabled,
        );
        assert!(result.is_err());
    }

    #[test]
    fn new_should_return_err_if_title_is_too_long() {
        let event_title = "X".repeat(51);
        let event_description = "descr".to_string();
        let closed_registration_message = "msg".to_string();
        let is_registration_open = true;
        let tshirts_enabled = false;
        let result = Theme::new(
            event_title,
            event_description,
            closed_registration_message,
            is_registration_open,
            tshirts_enabled,
        );
        assert!(result.is_err());
    }

    #[test]
    fn new_should_return_err_if_description_is_too_short() {
        let event_title = "title".to_string();
        let event_description = "d".to_string();
        let closed_registration_message = "msg".to_string();
        let is_registration_open = true;
        let tshirts_enabled = false;
        let result = Theme::new(
            event_title,
            event_description,
            closed_registration_message,
            is_registration_open,
            tshirts_enabled,
        );
        assert!(result.is_err());
    }

    #[test]
    fn new_should_return_err_if_description_is_too_long() {
        let event_title = "title".to_string();
        let event_description = "d".repeat(2001);
        let closed_registration_message = "msg".to_string();
        let is_registration_open = true;
        let tshirts_enabled = false;
        let result = Theme::new(
            event_title,
            event_description,
            closed_registration_message,
            is_registration_open,
            tshirts_enabled,
        );
        assert!(result.is_err());
    }

    #[test]
    fn new_should_return_err_if_closed_registration_message_is_too_long() {
        let event_title = "title".to_string();
        let event_description = "ddd".to_string();
        let closed_registration_message = "m".repeat(501);
        let is_registration_open = true;
        let tshirts_enabled = false;
        let result = Theme::new(
            event_title,
            event_description,
            closed_registration_message,
            is_registration_open,
            tshirts_enabled,
        );
        assert!(result.is_err());
    }
}
