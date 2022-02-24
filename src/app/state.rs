pub struct AppState {
    pub input: String,
    pub guesses: Vec<String>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            input: String::new(),
            guesses: vec![String::from("Hello World!")]
        }
    }
}

impl AppState {
    pub fn new() -> AppState {
        AppState::default()
    }
}
