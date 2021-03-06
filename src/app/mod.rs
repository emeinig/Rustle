use self::actions::Actions;
use self::state::AppState;
use crate::app::actions::Action;
use crate::inputs::key::Key;
use crate::io::IoEvent;

pub mod actions;
pub mod state;
pub mod ui;
pub mod words;

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

/// The main application, containing the state
pub struct App {
    /// We could dispatch an IO event
    io_tx: tokio::sync::mpsc::Sender<IoEvent>,
    /// Contextual actions
    actions: Actions,
    /// State
    is_loading: bool,
    state: AppState,
}

impl App {
    pub fn new(io_tx: tokio::sync::mpsc::Sender<IoEvent>) -> Self {
        let actions = vec![Action::Quit].into();
        let is_loading = false;
        let state = AppState::default();

        Self {
            io_tx,
            actions,
            is_loading,
            state,
        }
    }

    /// Handle a user action
    pub async fn do_action(&mut self, key: Key) -> AppReturn {
        if let Some(action) = self.actions.find(key) {
            match action {
                Action::Quit => AppReturn::Exit,
                Action::Edit => {
                    if let Key::Char(character) = key {
                        self.state.input.push(character);
                    }

                    AppReturn::Continue
                }
                Action::Submit => {
                    let word = self.state.input.drain(..).collect::<String>();

                    // Because we're only allowing letters (and not any
                    // graphemes), we can get away with just using the len method
                    // to count bytes rather than characters
                    if word.len() == 5 && words::check_validity(&word) {
                        self.state.guesses.push(word);

                        AppState::check_word_status(&mut self.state);
                        AppState::increment_attempt(&mut self.state);
                    }

                    AppReturn::Continue
                }
                Action::Backspace => {
                    self.state.input.pop();

                    AppReturn::Continue
                }
            }
        } else {
            AppReturn::Continue
        }
    }

    /// We could update the app or dispatch event on tick
    pub async fn update_on_tick(&mut self) -> AppReturn {
        // Check if we won or lost
        if let Some(last_guess) = self.state.guesses.last() {
            if last_guess == &self.state.solution {
                AppState::game_won(&mut self.state)
            } else if self.state.attempt >= 6 {
                AppState::game_lost(&mut self.state)
            }
        }

        AppReturn::Continue
    }

    /// Send a network event to the IO thread
    pub async fn dispatch(&mut self, action: IoEvent) {
        // `is_loading` will be set to false again after the async action has finished in io/handler.rs
        self.is_loading = true;
        if let Err(_e) = self.io_tx.send(action).await {
            self.is_loading = false;
        };
    }

    pub fn actions(&self) -> &Actions {
        &self.actions
    }
    pub fn state(&self) -> &AppState {
        &self.state
    }

    pub fn is_loading(&self) -> bool {
        self.is_loading
    }

    pub fn initialized(&mut self) {
        // Update contextual actions
        self.actions = vec![
            Action::Quit,
            Action::Backspace,
            Action::Submit,
            Action::Edit,
        ]
        .into();

        self.state = AppState::new()
    }
}
