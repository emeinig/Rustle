use tui::style::Color;

pub struct AppState {
    pub input: String,
    pub solution: String,
    pub guesses: Vec<String>,
    pub square_colors: Vec<Vec<Color>>,
    pub attempt: u8,
    pub game_status: GameStatus,
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
            game_status: GameStatus::InProgress,
        }
    }
}

impl AppState {
    pub fn new() -> AppState {
        AppState::default()
    }

    pub fn game_lost(&mut self) {
        self.game_status = GameStatus::Lose
    }

    pub fn game_won(&mut self) {
        self.game_status = GameStatus::Win
    }

    pub fn increment_attempt(&mut self) {
        self.attempt += 1
    }

    pub fn check_word_status(&mut self) {
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

#[derive(PartialEq)]
pub enum GameStatus {
    InProgress,
    Win,
    Lose,
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

        test_state.check_word_status();
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
