use std::{thread, time};
use std::borrow::BorrowMut;
use std::error::Error;
use std::io;
use std::io::stdin;
use std::io::Write;
use std::sync::Arc;
use std::sync::mpsc::{channel, Receiver};
use std::thread::sleep;
use std::time::{Duration, Instant};

use crossterm::cursor::MoveTo;
use crossterm::ExecutableCommand;
use futures::executor::block_on;
use futures::SinkExt;
use signald_rust::signald::Signald;
use signald_rust::signaldresponse::ResponseType;
use termion::cursor::{Goto, Hide, Show};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tokio::prelude::*;
use tokio::runtime::Handle;
use tokio::sync::Mutex;
use tokio::task;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::Color;
use tui::Terminal;
use tui::widgets::{Block, Borders, List, Paragraph, Text, Widget};

use crate::app::App;
use crate::common::SOCKET_PATH;
use crate::event::event::{Event, Events};
use crate::handlers::Handler;
use crate::handlers::inputhandler::InputHandler;
use crate::network::{IoEvent, Network};
use crate::ui::draw_basic_view;

pub mod common;
pub mod network;
pub mod event;
pub mod app;
pub mod handlers;
pub mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // Terminal setup
    let mut stdout = io::stdout().into_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear();

    // Io setup
    let (tx, rx) = std::sync::mpsc::channel::<IoEvent>();
    let mut app = Arc::new(Mutex::new(App::new(tx, "my_number".to_string())));

    // Network setup
    let mut appclone = Arc::clone(&app);
    std::thread::spawn(move || {
        let mut network = Network::new(&appclone);
        handle_network(rx, network);
    });

    let events: Events = Events::new();
    loop {

        // Receive signald messages
        // match mesgiter.try_recv() {
        //     Ok(x) => {
        //         match x.data {
        //             ResponseType::ContactList(contactData) => {
        //             },
        //             ResponseType::Message(message) => {
        //                 let message = message.unwrap();
        //                 if message.sync_message.is_some() {
        //                     let sync = message.sync_message.unwrap();
        //                     if sync.sent.is_some() {
        //                         let mes = sync.sent.unwrap().message.message;
        //                         messages.push(format!("SENT: {}",mes));
        //                     }
        //                 }
        //                 if message.data_message.is_some() {
        //                     let mesg = message.data_message.unwrap().message;
        //                     messages.push(format!("RECEIVED: {}", mesg));
        //                 }
        //             }
        //             _ => {}
        //         }
        //     },
        //     Err(e) => {}
        // }
        let mut mutapp = app.lock().await;

        // Render the UI
        terminal.draw(|mut f| {
            draw_basic_view(&mut f, &mut mutapp);
        })?;

        // Update cursor information
        terminal.backend_mut().execute(MoveTo(
            mutapp.cursor_pos.x as u16, mutapp.cursor_pos.y as u16,
        ));
        if mutapp.draw_cursor {
            terminal.show_cursor();
        } else {
            terminal.hide_cursor();
        }

        // Handle user input
        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Char('y') => {
                    mutapp.io_tx.send(IoEvent::GetContactList);
                }
                Key::Char(x) => {
                    InputHandler::handle(input, &mut mutapp);
                }
                _ => {
                    InputHandler::handle(input, &mut mutapp);
                }
            },
            Event::Tick => {}
        }
    }

    Ok(())
}

#[tokio::main]
pub async fn handle_network(rx: Receiver<IoEvent>, mut network: Network) {
    let mut eventcount = 0;
    while let Ok(event) = rx.recv() {
        println!("RECEIVED IOEVENT {}", eventcount);
        eventcount += 1;
        network.handle_events(event).await;
    }
}
