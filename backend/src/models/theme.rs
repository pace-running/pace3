
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Queryable, Serialize, Deserialize, Clone, Debug)]
pub struct ThemeSetting {
    pub event_key: String,
    pub event_value: String,
}
