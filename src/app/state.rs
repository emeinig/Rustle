pub struct AppState {
    pub input: String,
    pub guesses: Vec<String>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            input: String::new(),
            guesses: Vec::new()
        }
    }
}

impl AppState {
    pub fn new() -> AppState {
        AppState::default()
    }
}
