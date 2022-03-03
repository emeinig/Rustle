use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, List, ListItem, Paragraph};
use tui::Frame;

use super::state::AppState;
use crate::app::App;

pub fn draw<B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let size = rect.size();
    check_size(&size);

    // Vertical layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
        .split(size);

    // Title
    let title = draw_title();
    rect.render_widget(title, chunks[0]);

    // Guess Area & Keyboard
    let body_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    let guess_area = draw_guess_area(app.state());
    rect.render_widget(guess_area, body_chunks[0]);

    let keyboard_area = draw_keyboard_area();
    rect.render_widget(keyboard_area, body_chunks[1]);
}

fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("Rustle")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
}

fn check_size(rect: &Rect) {
    if rect.width < 52 {
        panic!("Require width >= 52, (got {})", rect.width);
    }
    if rect.height < 28 {
        panic!("Require height >= 28, (got {})", rect.height);
    }
}

fn color_squares<'a>(guess: &'a String, colors: &'a Vec<Color>) -> ListItem<'a> {
    let styled_spans = guess.chars()
        .zip(colors.iter())
        .map(|(letter, color)|
            Span::styled(letter.to_string(), Style::default().bg(*color))
            )
        .collect::<Vec<Span>>();

    ListItem::new(Spans::from(styled_spans))
}

fn draw_guess_area<'a>(state: &'a AppState) -> List<'a> {
    let guesses = state.guesses
        .iter()
        .zip(state.square_colors.iter())
        .map(|(guess, square_colors)|{
            color_squares(&guess, &square_colors)
        })
    .collect::<Vec<ListItem>>();

    List::new(guesses)
        .style(Style::default().fg(Color::White))
        // .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_squares_works() {
        let guess = String::from("bar");
        let colors = vec![Color::Red, Color::Green, Color::Blue];
        let expected_result = ListItem::new(
            Spans::from(
            vec![
            Span::styled("b", Style::default().bg(Color::Red)),
            Span::styled("a", Style::default().bg(Color::Green)),
            Span::styled("r", Style::default().bg(Color::Blue)),
            ]));

        let result = color_squares(&guess, &colors);

        assert_eq!(result, expected_result)
    }
}
