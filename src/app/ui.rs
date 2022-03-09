use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph};
use tui::Frame;

use super::state::AppState;
use super::state::GameStatus;
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

    // let keyboard_area = draw_keyboard_area();
    // rect.render_widget(keyboard_area, body_chunks[1]);
    draw_squares(rect, app, body_chunks[1]);

    let popup_area = centered_rect(60, 20, size);
    match app.state.game_status {
        GameStatus::Win => {
            let paragraph = create_paragraph(format!("You have won! It took {} attempts.\nPress ESC or CTRL+C to exit", app.state.attempt));
            rect.render_widget(Clear, popup_area); //this clears out the background
            rect.render_widget(paragraph, popup_area);
        },
        GameStatus::Lose => {
            let paragraph = create_paragraph(format!("You lost. The correct word was \"{}\".\nPress ESC or CTRL+C to exit", app.state.solution));
            rect.render_widget(Clear, popup_area); //this clears out the background
            rect.render_widget(paragraph, popup_area);
        },
        _ => {}
    }
}

fn draw_squares<B>(frame: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    // let paragraph = Paragraph::new("G")
    //     .style(Style::default().bg(Color::Reset))
    //     .block(Block::default().borders(Borders::ALL))
    //     .alignment(Alignment::Center);
    let horizontal_padding = (area.width - 15)/2;

    // row chunks are 6 rows with a length ("height") of 3 lines
    let row_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            // Padding for the bottom
            Constraint::Min(1),
        ].as_ref())
        .split(area);

    // We iterate through all of the constraints except the last
    for i in 0..6 {
        // column chunks are 5 rows with a length ("width") of 3 lines
        let col_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(horizontal_padding),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(horizontal_padding),
            ].as_ref())
            .split(row_chunks[i]);

        // frame.render_widget(Block::default(), col_chunks[0]);
        for n in 1..6 {
            frame.render_widget(Paragraph::new(format!("{}", n)).block(Block::default().borders(Borders::ALL)), col_chunks[n]);
        }
        // frame.render_widget(Block::default(), col_chunks[6]);
    }
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

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

fn create_paragraph(text: String) -> Paragraph<'static> {
    Paragraph::new(text)
        .style(Style::default().bg(Color::White).fg(Color::Black))
        .block(Block::default().title("Popup").borders(Borders::ALL))
        .alignment(Alignment::Center)
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
