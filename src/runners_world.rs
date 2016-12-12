
pub struct FormBuilder {
    gender: String,
    temperature: String,
    conditions: String,
    wind: String,
    time_of_day: String,
    intensity: String,
    feel: String,
}

pub fn form_builder() -> FormBuilder {
    FormBuilder {
        gender: "male".to_owned(),
        temperature: "10".to_owned(),
        conditions: "10".to_owned(),
        wind: "10".to_owned(),
        time_of_day: "10".to_owned(),
        intensity: "10".to_owned(),
        feel: "10".to_owned(),
    }
}
