use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::{Block, BorderType, Borders, Clear, Paragraph};
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

    let keyboard_area = draw_keyboard_area();
    frame.render_widget(keyboard_area, chunks[3]);

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

fn draw_keyboard_area<'a>() -> Block<'a> {
    Block::default()
        .title(vec![Span::styled(
            "Keyboard Area",
            Style::default().fg(Color::Yellow),
        )])
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .border_type(BorderType::Plain)
}

fn create_paragraph(text: String) -> Paragraph<'static> {
    Paragraph::new(text)
        .style(Style::default().bg(Color::White).fg(Color::Black))
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_squares_works() {
        let guess = String::from("bar");
        let colors = vec![Color::Red, Color::Green, Color::Blue];
        let expected_result = ListItem::new(Spans::from(vec![
            Span::styled("b", Style::default().bg(Color::Red)),
            Span::styled("a", Style::default().bg(Color::Green)),
            Span::styled("r", Style::default().bg(Color::Blue)),
        ]));

        let result = color_squares(&guess, &colors);

        assert_eq!(result, expected_result)
    }
}
