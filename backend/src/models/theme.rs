
use std::collections::HashMap;
use std::vec::Vec;

use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Queryable, Serialize, Deserialize, Clone, Debug)]
pub struct ThemeSetting {
    pub event_key: String,
    pub event_value: String,
}

impl From<Vec<ThemeSetting>> for HashMap<String,String> {
    fn from(values: Vec<ThemeSetting>) -> Self {
        let mut hashmap = HashMap::<String,String>::new();
        for setting in values.into_iter() {
            hashmap.insert(setting.event_key, setting.event_value);
        }
        hashmap
    }
}
