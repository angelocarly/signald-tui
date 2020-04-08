use std::{collections::HashMap, sync::mpsc::Sender};

use crate::network::IoEvent;
use std::ops::Deref;

pub enum View {
    Contacts,
    Chat
}

pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Clone)]
pub struct Conversation {
    pub contact: Contact,
    pub messages: Vec<Message>,
}
impl Conversation {
    pub fn new(contact: Contact) -> Self {
        Self {
            contact,
            messages: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct Contact {
    pub number: String,
    pub name: Option<String>,
    pub color: Option<String>,
}

#[derive(Clone)]
pub struct Message {
    pub sender: String,
    pub receiver: String,
    pub timestamp: i64,
    pub message: String,
}

pub struct App {
    pub loaded: bool,

    pub username: String,
    pub contacts: Vec<Contact>,
    pub conversations: HashMap<String, Conversation>,

    pub io_tx: Sender<IoEvent>,

    // Input
    pub input_string: String,
    pub input_position: usize,
    pub draw_cursor: bool,
    pub cursor_pos: Point,

    // Contact list
    pub selected_contact_index: usize,

    // View
    pub focused_view: View,
}

impl App {
    pub fn new(io_tx: Sender<IoEvent>) -> Self {
        Self {
            loaded: false,
            username: "".to_string(),
            input_string: String::new(),
            input_position: 0,
            draw_cursor: false,
            cursor_pos: Point { x: 0, y: 0 },
            contacts: Vec::new(),
            conversations: HashMap::new(),
            selected_contact_index: 0,
            io_tx,
            focused_view: View::Contacts,
        }
    }

    pub fn get_current_conversation(&mut self) -> Option<&mut Conversation> {
        let contact = self.get_selected_contact();
        if contact.is_none() { return None; }

        let contact_num = contact.unwrap().number.clone();
        self.get_conversation(contact_num)
    }

    pub fn get_conversation(&mut self, contact: String) -> Option<&mut Conversation> {
        self.conversations.get_mut(&contact)
    }

    pub fn get_selected_contact(&mut self) -> Option<Contact> {
        let con = self.contacts.get(self.selected_contact_index);
        if con.is_none() { return None; }
        return Some((*con.unwrap()).clone());
    }

    pub fn update_contacts(&mut self, mut contacts: Vec<Contact>) {
        contacts.iter_mut().for_each(|c| {
            // If the contact is yourself, set the name
            if c.number == self.username {
                if c.name.is_none() || c.name.clone().unwrap().is_empty() {
                    c.name = Some("Me".to_string());
                }
            }

            // Add a new conversation if it doesn't exist yet
            if !self.conversations.contains_key(c.number.clone().as_str()) {
                self.add_conversation((*c).clone());
            }
        });

        self.contacts = contacts;
    }

    pub fn add_conversation(&mut self, contact: Contact) {
        if !self.conversations.contains_key(contact.number.clone().as_str()) {
            let conv = Conversation {
                contact: contact.clone(),
                messages: Vec::new(),
            };

            self.conversations.insert(contact.number.clone(), conv);
        }
    }

    pub fn select_conversation(&mut self, contact_index: usize) {
        if self.loaded {
            if contact_index >= 0 && contact_index < self.conversations.len() {
                self.selected_contact_index = contact_index;
            }
        }
    }
}