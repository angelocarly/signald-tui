use std::{collections::HashMap, sync::mpsc::Sender};

use crate::network::IoEvent;

pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Clone)]
pub struct Conversation {
    pub contact: String,
    pub messages: Vec<Message>,
}
impl Conversation {
    pub fn new(contact: String) -> Self {
        Self {
            contact,
            messages: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct Message {
    pub sender: String,
    pub receiver: String,
    pub timestamp: i64,
    pub message: String,
}

pub struct App {
    pub username: String,
    pub contacts: Vec<String>,
    pub conversations: HashMap<String, Conversation>,
    pub selected_conversation: String,

    pub io_tx: Sender<IoEvent>,

    pub input_string: String,
    pub input_position: usize,
    pub draw_cursor: bool,
    pub cursor_pos: Point,
}

impl App {
    pub fn new(io_tx: Sender<IoEvent>, username: String) -> Self {
        let self_conversation = Conversation {
            contact: username.clone(),
            messages: Vec::new(),
        };
        let mut conversations = HashMap::new();
        conversations.insert(username.clone(), self_conversation);

        Self {
            username: username.clone(),
            input_string: String::new(),
            input_position: 0,
            draw_cursor: false,
            cursor_pos: Point { x: 0, y: 0 },
            contacts: Vec::new(),
            conversations,
            selected_conversation: username.clone(),
            io_tx,
        }
    }

    pub fn get_current_conversation(&mut self) -> &mut Conversation {
        self.conversations.get_mut(&self.selected_conversation).unwrap()
    }

    pub fn get_conversation(&mut self, contact: String) -> &mut Conversation {
        let conv = self.conversations.get(&contact);
        if conv.is_none() {
            let new_conv = Conversation::new(contact.clone());
            self.conversations.insert(contact.clone(), new_conv);
        }
        self.conversations.get_mut(&contact).unwrap()
    }
}