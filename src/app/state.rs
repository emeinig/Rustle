use tui::style::Color;

pub struct AppState {
    pub input: String,
    pub solution: String,
    pub guesses: Vec<String>,
    pub square_colors: Vec<Vec<Color>>,
    pub attempt: u8,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            input: String::new(),
            // TODO: Remove hardcoded word
            solution: String::from("rusty"),
            attempt: 0,
            square_colors: Vec::new(),
            guesses: Vec::new(),
        }
    }
}

impl AppState {
    pub fn new() -> AppState {
        AppState::default()
    }

    pub fn check_status(&mut self) {
        let mut colors = Vec::new();

        if let Some(last_guess) = self.guesses.last() {
            // We'll be a bit lazy here and hardcode the word length (5) here.
            for i in 0..5 {
                let guess_letter = last_guess.chars().nth(i);

                if guess_letter == self.solution.chars().nth(i) {
                    colors.push(Color::Green)
                } else if self.solution.contains(guess_letter.unwrap()) {
                    colors.push(Color::Yellow)
                } else {
                    colors.push(Color::Reset)
                }
            }
        }

        self.square_colors.push(colors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_status_works() {
        let mut test_state = AppState {
            input: String::from(""),
            solution: String::from("rebus"),
            attempt: 0,
            square_colors: vec![vec![
                Color::Yellow,
                Color::Reset,
                Color::Yellow,
                Color::Reset,
                Color::Yellow,
            ]],
            guesses: vec![String::from("route")],
        };

        test_state.check_status();
        let expected_result = vec![
            Color::Green,
            Color::Reset,
            Color::Yellow,
            Color::Reset,
            Color::Yellow,
        ];

        assert_eq!(test_state.square_colors.last().unwrap(), &expected_result);
    }
}
