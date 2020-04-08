use std::error::Error;
use std::io::{Write, stdout};
use std::sync::Arc;
use std::{time::Duration, sync::mpsc::{Sender, Receiver}};

use crossterm::cursor::MoveTo;
use crossterm::ExecutableCommand;
use tokio::sync::Mutex;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::app::{App, View};
use crate::event::event::{Event, Events};
use crate::handlers::Handler;
use crate::handlers::inputhandler::InputHandler;
use crate::network::{IoEvent, Network};
use crate::ui::draw_basic_view;
use event::key::Key;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use crate::handlers::contacthandler::ContactHandler;

pub mod common;
pub mod network;
pub mod event;
pub mod app;
pub mod handlers;
pub mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Io setup
    let (tx, rx) = std::sync::mpsc::channel::<IoEvent>();
    let txclone = tx.clone();
    let app = Arc::new(Mutex::new(App::new(tx)));

    // Network setup
    let appclone = Arc::clone(&app);
    std::thread::spawn(move || {
    let network = Network::new(appclone);
        handle_network_io(txclone, rx, network);
    });

    // Initial network setup
    {
        let mut mutapp = app.lock().await;
        mutapp.io_tx.send(IoEvent::LoadAccount)?;
        mutapp.io_tx.send(IoEvent::Subscribe)?;
        mutapp.io_tx.send(IoEvent::GetContactList)?;
    }

    terminal.clear()?;

    let events: Events = Events::new(250);
    loop {

        let mut app = app.lock().await;

        // Render the UI
        terminal.draw(|mut f| {
            draw_basic_view(&mut f, &mut app);
        })?;

        // Update cursor information
        terminal.backend_mut().execute(MoveTo(
           app.cursor_pos.x as u16, app.cursor_pos.y as u16,
        ))?;
        if app.draw_cursor {
            terminal.show_cursor()?;
        } else {
            terminal.hide_cursor()?;
        }

        // Handle user input
        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    disable_raw_mode()?;
                    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                    terminal.show_cursor()?;
                    break;
                }
                _ => {
                    match app.focused_view {
                        View::Contacts => {
                            ContactHandler::handle(input, &mut app);
                        }
                        View::Chat => {
                            InputHandler::handle(input, &mut app);
                        }
                    }
                }
            },
            Event::Tick => {},
        }
    }

    Ok(())
}

#[tokio::main]
pub async fn handle_network_io(tx: Sender<IoEvent>, rx: Receiver<IoEvent>, mut network: Network) {
    std::thread::spawn(move || {
        let tx = tx.clone();
        let duration = Duration::from_micros(250);
        loop {
            tx.send(IoEvent::Tick).unwrap();
            std::thread::sleep(duration);
        }
    });
    while let Ok(event) = rx.recv() {
        network.handle_event(event).await;
    }
}