use serde::Serialize;

#[derive(Serialize)]
pub struct Event {
    name: String,
    description: String,
    date: String,
}

pub fn current_event() -> Event {
    let race = Event {
        name: "Lauf gegen Rechts".to_string(),
        description: "Die Anmeldung startet im Februar 2023. Weitere Infos findet Ihr hier in kürze".to_string(),
        date: "29.5.2023".to_string()
    };
    return race;
}
