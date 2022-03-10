use std::collections::HashMap;
use std::fmt::{self, Display};
use std::slice::Iter;

use crate::inputs::key::Key;

/// We define all available action
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Action {
    Quit,
    Edit,
    Backspace,
    Submit,
}

impl Action {
    /// All available actions
    pub fn iterator() -> Iter<'static, Action> {
        static ACTIONS: [Action; 4] = [
            Action::Quit,
            Action::Edit,
            Action::Backspace,
            Action::Submit,
        ];
        ACTIONS.iter()
    }

    /// List of key associated to action
    pub fn keys(&self) -> &[Key] {
        match self {
            Action::Quit => &[Key::Ctrl('c'), Key::Esc],
            Action::Submit => &[Key::Enter],
            Action::Backspace => &[Key::Backspace, Key::Delete],
            Action::Edit => &[
                Key::Char('a'),
                Key::Char('b'),
                Key::Char('c'),
                Key::Char('d'),
                Key::Char('e'),
                Key::Char('f'),
                Key::Char('g'),
                Key::Char('h'),
                Key::Char('i'),
                Key::Char('j'),
                Key::Char('k'),
                Key::Char('l'),
                Key::Char('m'),
                Key::Char('n'),
                Key::Char('o'),
                Key::Char('p'),
                Key::Char('q'),
                Key::Char('r'),
                Key::Char('s'),
                Key::Char('t'),
                Key::Char('u'),
                Key::Char('v'),
                Key::Char('w'),
                Key::Char('x'),
                Key::Char('y'),
                Key::Char('z'),
            ],
        }
    }
}

/// Could display a user friendly short description of action
impl Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Action::Quit => "Quit",
            Action::Submit => "Submit Guess",
            Action::Backspace => "Delete",
            Action::Edit => "",
        };
        write!(f, "{}", str)
    }
}

/// The application should have some contextual actions.
#[derive(Default, Debug, Clone)]
pub struct Actions(Vec<Action>);

impl Actions {
    /// Given a key, find the corresponding action
    pub fn find(&self, key: Key) -> Option<&Action> {
        Action::iterator().find(|action| action.keys().contains(&key))
    }

    /// Get contextual actions.
    /// (just for building a help view)
    pub fn actions(&self) -> &[Action] {
        self.0.as_slice()
    }
}

impl From<Vec<Action>> for Actions {
    /// Build contextual action
    ///
    /// # Panics
    ///
    /// If two actions have same key
    fn from(actions: Vec<Action>) -> Self {
        // Check key unicity
        let mut map: HashMap<Key, Vec<Action>> = HashMap::new();
        for action in actions.iter() {
            for key in action.keys().iter() {
                match map.get_mut(key) {
                    Some(vec) => vec.push(*action),
                    None => {
                        map.insert(*key, vec![*action]);
                    }
                }
            }
        }
        let errors = map
            .iter()
            .filter(|(_, actions)| actions.len() > 1) // at least two actions share same shortcut
            .map(|(key, actions)| {
                let actions = actions
                    .iter()
                    .map(Action::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("Conflict key {} with actions {}", key, actions)
            })
            .collect::<Vec<_>>();
        if !errors.is_empty() {
            panic!("{}", errors.join("; "))
        }

        // Ok, we can create contextual actions
        Self(actions)
    }
}
