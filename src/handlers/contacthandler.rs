use crate::app::App;
use crate::{event::key::Key, handlers::Handler, network::{SendMessageData, IoEvent}};

pub struct ContactHandler {
    data: String,
}

impl Handler for ContactHandler {
    fn handle(key: Key, app: &mut App) {
        if !app.loaded {
            return;
        }
        match key {
            Key::Char('j') => {
                // let index = app.
                // let curindex = app.contacts.
                app.select_conversation(app.selected_contact_index + 1);
            }
            Key::Char('k') => {
                app.select_conversation(app.selected_contact_index - 1);
            }
            _ => {}
        }
    }
}
