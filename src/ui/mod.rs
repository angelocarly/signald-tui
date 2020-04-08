use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, List, Paragraph, Text, Widget};

use crate::app::{App, Conversation, Point};
use chrono::{NaiveDateTime, Local, TimeZone};
use tui::style::{Style, Color};

pub fn draw_basic_view<B>(f: &mut Frame<B>, app: &mut App)
    where B: Backend,
{
    let size = f.size();

    let panels = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Max(20),
            Constraint::Length(9)
        ].as_ref()).split(size);

    let sidebar = panels[0];

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(4)
        ].as_ref())
        .split(panels[1]);

    // Contacts
    if let Some(contact) = app.get_selected_contact() {
        List::new(app.contacts.iter().map(|i| {
            if i.number.clone() == contact.number.clone() {
                return Text::styled(i.name.clone().unwrap(), Style::default().fg(Color::Blue));
            }
            return Text::styled(i.name.clone().unwrap(), Style::default());
        }))
            .block(Block::default()
                .borders(Borders::ALL)
                .title("List")
            )
            .render(f, sidebar);
    } else {
        Paragraph::new([Text::raw("No conversation selected")].iter())
            .block(Block::default()
                .borders(Borders::ALL)
            )
            .render(f, sidebar);
    }

// Chat
    if let Some(conv) = app.get_current_conversation() {
        List::new(conv.messages.iter_mut()
            .map(|i| {
                let date = Local.timestamp(i.timestamp / 1000, 0);
                Text::raw(
                    format!("{}: {}", date, i.message.clone())
                )
            })
        )
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Chat")
            )
            .render(f, chunks[0]);
    } else {
        Paragraph::new([Text::raw("No conversation selected")].iter())
            .block(Block::default()
                .borders(Borders::ALL)
            )
            .render(f, chunks[0]);
    }

    // Input
    Paragraph::new([Text::raw(&app.input_string)].iter())
        .block(Block::default()
            .borders(Borders::ALL)
        )
        .render(f, chunks[1]);

    app.draw_cursor = true;
    app.cursor_pos = Point {
        x: chunks[1].x + app.input_position as u16 + 1,
        y: chunks[1].y + 1,
    }
}