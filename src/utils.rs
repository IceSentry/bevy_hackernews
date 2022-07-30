use rand::{distributions::Alphanumeric, Rng};

pub fn num_as_f32(number: &Option<serde_json::Number>) -> f32 {
    number
        .as_ref()
        .unwrap_or(&serde_json::Number::from_f64(0.0).unwrap())
        .as_f64()
        .unwrap() as f32
}

pub fn _random_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(rand::thread_rng().gen_range(16..=64))
        .map(char::from)
        .collect()
}
