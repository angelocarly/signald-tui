use std::sync::Arc;

use signald_rust::signald::Signald;
use tokio::sync::Mutex;

use crate::app::App;
use signald_rust::signaldresponse::ResponseType;
use futures::SinkExt;

pub enum IoEvent {
    Subscribe,
    GetContactList,
}

pub struct Network<'a> {
    app: &'a Arc<Mutex<App>>,
    signald: Signald,
}

impl<'a> Network<'a> {
    pub fn new(app: &'a Arc<Mutex<App>>) -> Self {
        Self {
            app,
            signald: Signald::connect(),
        }
    }

    pub async fn handle_events(&mut self, io_event: IoEvent) {
        match io_event {
            IoEvent::GetContactList => {
                self.get_contact_list().await;
            }
            IoEvent::Subscribe => {
                self.subscribe().await;
            }
        }
    }

    async fn subscribe(&mut self) {
        let mut app = self.app.lock().await;

        self.signald.subscribe(app.username.clone()).await;
    }

    async fn get_contact_list(&mut self) {
        let mut app = self.app.lock().await;

        let mut res = self.signald.list_contacts(app.username.clone()).await.unwrap();
        match res.data {
            ResponseType::ContactList(a) => {
                app.contacts.flush();
                for contact in a.unwrap().iter() {
                    app.contacts.push(contact.name.clone());
                }
            }
            _ => {

            }
        }
        println!()
    }
}