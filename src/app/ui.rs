use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Clear, Paragraph};
use tui::Frame;

use super::state::GameStatus;
use crate::app::App;

pub fn draw<B>(frame: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let size = frame.size();
    check_size(&size);

    // Guess Area & Keyboard
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(19),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(size);

    // Title
    let title = draw_title();
    frame.render_widget(title, chunks[0]);

    draw_squares(frame, app, chunks[1]);

    let input = Paragraph::new(app.state.input.as_ref())
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Input"));
    frame.render_widget(input, chunks[2]);

    draw_keyboard(frame, app, chunks[3]);

    // We want the popup to go over the input and keyboard
    let popup_area = chunks[2].union(chunks[3]);

    match app.state.game_status {
        GameStatus::Win => {
            let paragraph = create_paragraph(format!(
                "You have won! It took {} attempts.\nPress ESC or CTRL+C to exit",
                app.state.attempt
            ));
            frame.render_widget(Clear, popup_area); //this clears out the background
            frame.render_widget(paragraph, popup_area);
        }
        GameStatus::Lose => {
            let paragraph = create_paragraph(format!(
                "You lost. The correct word was \"{}\".\nPress ESC or CTRL+C to exit",
                app.state.solution
            ));
            frame.render_widget(Clear, popup_area); //this clears out the background
            frame.render_widget(paragraph, popup_area);
        }
        _ => {}
    }
}

fn draw_keyboard<B>(frame: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    // row chunks are 3 rows with a length ("height") of 3 lines
    let row_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                // Padding for the bottom
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(area);

    // Top row ("qwerty") has 10 keys, so we create 10 columns with a length of 3 and add in some
    // padding
    let top_row_letters = ["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P"];
    let top_row_horizontal_padding = (area.width - 3 * 10) / 2;
    let top_row_col_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(top_row_horizontal_padding),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(top_row_horizontal_padding),
            ]
            .as_ref(),
        )
        .split(row_chunks[0]);

    // Home row ("asdfg") has 9 keys, so we create 9 columns with a length of 3 and add in some
    // padding
    let home_row_letters = ["A", "S", "D", "F", "G", "H", "J", "K", "L"];
    let home_row_horizontal_padding = (area.width - 3 * 9) / 2;
    let home_row_col_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(home_row_horizontal_padding),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(home_row_horizontal_padding),
            ]
            .as_ref(),
        )
        .split(row_chunks[1]);

    // Bottom row has 7 keys, so we create 7 columns with a length of 3 and add in some padding as
    // per usual.
    let bottom_row_letters = ["Z", "X", "C", "V", "B", "N", "M"];
    let bottom_row_horizontal_padding = (area.width - 3 * 7) / 2;
    let bottom_row_col_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(bottom_row_horizontal_padding),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(bottom_row_horizontal_padding),
            ]
            .as_ref(),
        )
        .split(row_chunks[2]);

    for i in 1..11 {
        let key_color = app
            .state
            .keyboard_colors
            .get(
                &top_row_letters[i - 1]
                    .chars()
                    .next()
                    .unwrap()
                    .to_ascii_lowercase(),
            )
            .unwrap();

        let keys = Paragraph::new(top_row_letters[i - 1])
            .style(Style::default().bg(*key_color))
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        frame.render_widget(keys, top_row_col_chunks[i]);
    }

    for i in 1..10 {
        let key_color = app
            .state
            .keyboard_colors
            .get(
                &home_row_letters[i - 1]
                    .chars()
                    .next()
                    .unwrap()
                    .to_ascii_lowercase(),
            )
            .unwrap();

        let keys = Paragraph::new(home_row_letters[i - 1])
            .style(Style::default().bg(*key_color))
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        frame.render_widget(keys, home_row_col_chunks[i]);
    }

    for i in 1..8 {
        let key_color = app
            .state
            .keyboard_colors
            .get(
                &bottom_row_letters[i - 1]
                    .chars()
                    .next()
                    .unwrap()
                    .to_ascii_lowercase(),
            )
            .unwrap();

        let keys = Paragraph::new(bottom_row_letters[i - 1])
            .style(Style::default().bg(*key_color))
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        frame.render_widget(keys, bottom_row_col_chunks[i]);
    }
}

fn draw_squares<B>(frame: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let horizontal_padding = (area.width - 15) / 2;
    let guesses = &app.state.guesses;
    let square_colors = &app.state.square_colors;

    // row chunks are 6 rows with a length ("height") of 3 lines
    let row_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                // Padding for the bottom
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(area);

    // We iterate through all of the constraints except the last
    for i in 0..6 {
        // column chunks are 5 rows with a length ("width") of 3 lines
        let col_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(horizontal_padding),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(horizontal_padding),
                ]
                .as_ref(),
            )
            .split(row_chunks[i]);

        // We want a word broken into its individual letters for each row
        let mut letters = if let Some(guess) = guesses.get(i) {
            guess.chars()
        } else {
            // For the rest of the vector that isn't present, we just give a
            // blank Char with 5 spaces
            "     ".chars()
        };

        let colors = if let Some(color_vec) = square_colors.get(i) {
            color_vec.clone()
        } else {
            vec![
                Color::Reset,
                Color::Reset,
                Color::Reset,
                Color::Reset,
                Color::Reset,
            ]
        };

        for n in 1..6 {
            let colored_square = if let Some(letter) = letters.next() {
                Paragraph::new(letter.to_string())
                    .style(Style::default().bg(colors[n - 1]))
                    .block(Block::default().borders(Borders::ALL))
                    .alignment(Alignment::Center)
            } else {
                Paragraph::new("")
                    .style(Style::default().bg(Color::Reset))
                    .block(Block::default().borders(Borders::ALL))
                    .alignment(Alignment::Center)
            };

            frame.render_widget(colored_square, col_chunks[n]);
        }
    }
}

fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("Rustle")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
}

fn check_size(rect: &Rect) {
    if rect.width < 52 {
        panic!("Require width >= 52, (got {})", rect.width);
    }
    if rect.height < 28 {
        panic!("Require height >= 28, (got {})", rect.height);
    }
}

fn create_paragraph(text: String) -> Paragraph<'static> {
    Paragraph::new(text)
        .style(Style::default().bg(Color::White).fg(Color::Black))
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center)
}
