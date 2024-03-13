use rand::{distributions::Alphanumeric, Rng};

pub fn generate_random_url() -> String {
    let random_path: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    format!("pathlinker://{}", random_path)
}