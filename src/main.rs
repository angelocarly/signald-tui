use std::error::Error;
use std::io;
use std::sync::Arc;
use std::{time::Duration, sync::mpsc::{Sender, Receiver}};

use crossterm::cursor::MoveTo;
use crossterm::ExecutableCommand;
use termion::event::Key;
use termion::raw::IntoRawMode;
use tokio::sync::Mutex;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::app::App;
use crate::event::event::{Event, Events};
use crate::handlers::Handler;
use crate::handlers::inputhandler::InputHandler;
use crate::network::{IoEvent, Network};
use crate::ui::draw_basic_view;
use signald_rust::signaldresponse::SignaldResponse;
use network::SendMessageData;

pub mod common;
pub mod network;
pub mod event;
pub mod app;
pub mod handlers;
pub mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let number = "my_number".to_string();

    // Terminal setup
    let stdout = io::stdout().into_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Io setup
    let (tx, rx) = std::sync::mpsc::channel::<IoEvent>();
    let txclone = tx.clone();
    let app = Arc::new(Mutex::new(App::new(tx, number.clone())));

    // Network setup
    let appclone = Arc::clone(&app);
    std::thread::spawn(move || {
    let network = Network::new(number.clone(), &appclone);
        handle_network_io(txclone, rx, network);
    });

    // Initial network setup
    {
        let mutapp = app.lock().await;
        mutapp.io_tx.send(IoEvent::Subscribe)?;
        mutapp.io_tx.send(IoEvent::GetContactList)?;
    }

    let events: Events = Events::new();
    loop {


        // Render the UI
        {
            let mut mutapp = app.lock().await;
            terminal.draw(|mut f| {
                draw_basic_view(&mut f, &mut mutapp);
            })?;
        }

        // Update cursor information
        {
            let mut mutapp = app.lock().await;
            terminal.backend_mut().execute(MoveTo(
                mutapp.cursor_pos.x as u16, mutapp.cursor_pos.y as u16,
            ))?;
            if mutapp.draw_cursor {
                terminal.show_cursor()?;
            } else {
                terminal.hide_cursor()?;
            }
        }

        // Handle user input
        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Char('\n') => {
                    let mut mutapp = app.lock().await;
                    mutapp.io_tx.send(IoEvent::SendMessage(SendMessageData {
                        recipient: number.clone(),
                        message: mutapp.input_string.clone()
                    }))?;
                    mutapp.input_string.clear();
                    mutapp.input_position = 0;
                }
                Key::Char(_x) => {
                    let mut mutapp = app.lock().await;
                    InputHandler::handle(input, &mut mutapp);
                }
                _ => {
                    let mut mutapp = app.lock().await;
                    InputHandler::handle(input, &mut mutapp);
                }
            },
            Event::Tick => {}
        }
    }

    Ok(())
}

#[tokio::main]
pub async fn handle_network_io(tx: Sender<IoEvent>, rx: Receiver<IoEvent>, mut network: Network) {
    std::thread::spawn(move || {
        let tx = tx.clone();
        loop {
            tx.send(IoEvent::Tick).unwrap();
            std::thread::sleep(Duration::from_micros(250));
        }
    });
    while let Ok(event) = rx.recv() {
        network.handle_event(event).await;
    }
}