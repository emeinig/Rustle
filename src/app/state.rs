use crate::app::words;
use std::collections::HashMap;
use tui::style::Color;

pub struct AppState {
    pub input: String,
    pub solution: String,
    pub guesses: Vec<String>,
    pub square_colors: Vec<Vec<Color>>,
    pub keyboard_colors: HashMap<char, Color>,
    pub attempt: u8,
    pub game_status: GameStatus,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            input: String::new(),
            solution: words::random_word(),
            attempt: 0,
            square_colors: Vec::new(),
            guesses: Vec::new(),
            game_status: GameStatus::InProgress,
            keyboard_colors: HashMap::from([
                ('a', Color::Reset),
                ('b', Color::Reset),
                ('c', Color::Reset),
                ('d', Color::Reset),
                ('e', Color::Reset),
                ('f', Color::Reset),
                ('g', Color::Reset),
                ('h', Color::Reset),
                ('i', Color::Reset),
                ('j', Color::Reset),
                ('k', Color::Reset),
                ('l', Color::Reset),
                ('m', Color::Reset),
                ('n', Color::Reset),
                ('o', Color::Reset),
                ('p', Color::Reset),
                ('q', Color::Reset),
                ('r', Color::Reset),
                ('s', Color::Reset),
                ('t', Color::Reset),
                ('u', Color::Reset),
                ('v', Color::Reset),
                ('w', Color::Reset),
                ('x', Color::Reset),
                ('y', Color::Reset),
                ('z', Color::Reset),
            ]),
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
                    if let Some(val) = self.keyboard_colors.get_mut(&guess_letter.unwrap()) {
                        *val = Color::Green;
                    }

                    colors.push(Color::Green)
                } else if self.solution.contains(guess_letter.unwrap()) {
                    if let Some(val) = self.keyboard_colors.get_mut(&guess_letter.unwrap()) {
                        *val = Color::Yellow;
                    }

                    colors.push(Color::Yellow)
                } else {
                    if let Some(val) = self.keyboard_colors.get_mut(&guess_letter.unwrap()) {
                        *val = Color::Black;
                    }

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
        solution: String::from("rebus"),
        guesses: vec![String::from("route")],
        square_colors: vec![vec![
            Color::Yellow,
            Color::Reset,
            Color::Reset,
            Color::Yellow,
            Color::Yellow,
        ]],
            ..AppState::default()
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
