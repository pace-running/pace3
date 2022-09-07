use serde::Serialize;

#[derive(Serialize)]
pub struct Event {
    pub name: String,
    pub description: String,
    pub date: String,
}

pub fn current_event() -> Event {
    Event {
        name: "Lauf gegen Rechts".to_string(),
        description:
            "Die Anmeldung startet im Februar 2023. Weitere Infos findet Ihr hier in kürze"
                .to_string(),
        date: "29.5.2023".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_event_returns_valid_data() {
        assert!(current_event().name.len() > 0);
        assert!(current_event().description.len() > 0);
        assert!(current_event().date.len() > 0);
    }
}
