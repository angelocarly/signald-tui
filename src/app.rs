use std::{thread, time};
use std::error::Error;
use std::io;
use std::io::{stdin, Stdout};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use signald_rust::signald::Signald;
use signald_rust::signaldresponse::ResponseType;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tokio::prelude::*;
use tokio::task;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::Color;
use tui::Terminal;
use tui::widgets::{Block, Borders, List, Paragraph, Text, Widget};

use crate::common::SOCKET_PATH;
use crate::event::event::{Event, Events};
use std::sync::mpsc::Sender;
use crate::network::IoEvent;

pub struct Point {
    pub x: u16,
    pub y: u16,
}

pub struct App {
    pub username: String,
    pub contacts: Vec<String>,

    pub io_tx: Sender<IoEvent>,

    pub input_string: String,
    pub input_position: usize,
    pub draw_cursor: bool,
    pub cursor_pos: Point,
}

impl App {
    pub fn new(io_tx: Sender<IoEvent>, username: String) -> Self {
        Self {
            username,
            input_string: String::new(),
            input_position: 0,
            draw_cursor: false,
            cursor_pos: Point { x: 0, y: 0 },
            contacts: Vec::new(),
            io_tx,
        }
    }

    pub fn run() {}
}