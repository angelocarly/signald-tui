use std::{thread, time};
use crate::signald::signaldrequest::SignaldRequestBuilder;
use signald::signald::Signald;
use tokio::prelude::*;
use tokio::task;
use std::time::Instant;
use std::sync::{Arc, Mutex};

pub mod common;
pub mod signald;
pub mod event;

use std::io;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};
use std::io::stdin;
use termion::input::{TermRead};
use termion::event::Key;
use crate::event::event::{Events, Event};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear();

    let events: Events = Events::new();
    loop {
        terminal.draw(|mut f| {
            let size = f.size();
            Block::default()
                .title("YOOOOOO")
                .borders(Borders::ALL)
                .render(&mut f, size);
        })?;
        //println!("test");
        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Left => {
                    // app.items.unselect();
                }
                Key::Down => {
                    // app.items.next();
                }
                Key::Up => {
                    // app.items.previous();
                }
                _ => {}
            },
            Event::Tick => {
            }
        }
    }

    Ok(())
}
