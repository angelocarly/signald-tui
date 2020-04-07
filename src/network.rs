use std::sync::Arc;

use futures::SinkExt;
use signald_rust::signald::Signald;
use signald_rust::signaldresponse::{ResponseType, SignaldResponse};
use tokio::sync::Mutex;

use crate::app::{App, Message};
use bus::BusReader;

pub enum IoEvent {
    Subscribe,
    GetContactList,
    SendMessage(SendMessageData),
    Tick,
}

pub struct SendMessageData {
    pub recipient: String,
    pub message: String,
}

pub struct Network {
    username: String,
    app: Arc<Mutex<App>>,
    pub signald: Signald,
    bus_rx: BusReader<SignaldResponse>,
}

impl Network {
    pub fn new(username: String, app: Arc<Mutex<App>>) -> Self {
        let mut signald = Signald::connect();
        let bus_rx = signald.get_rx();
        Self {
            username,
            app,
            signald,
            bus_rx,
        }
    }

    pub async fn handle_event(&mut self, io_event: IoEvent) {
        match io_event {
            IoEvent::GetContactList => {
                self.get_contact_list().await;
            }
            IoEvent::Subscribe => {
                self.subscribe().await;
            }
            IoEvent::SendMessage(d) => {
                self.send_message(d).await;
            }
            IoEvent::Tick => {
                self.handle_responses().await;
            }
        }
    }

    pub async fn handle_responses(&mut self) {
        while let Ok(res) = self.bus_rx.try_recv() {
            match res.data {
                ResponseType::BusUpdate => {}
                ResponseType::Message(message) => {
                    let message = message.unwrap();
                    // Received sync message
                    if message.sync_message.is_some() {
                        let sync = message.sync_message.unwrap();
                        if sync.sent.is_some() {
                            let sent = sync.sent.unwrap();
                            let mesg = sent.message.message;

                            let tui_message = Message {
                                message: mesg,
                                sender: self.username.clone(),
                                receiver: sent.destination.clone(),
                                timestamp: sent.timestamp,
                            };

                            let mut mutapp = self.app.lock().await;
                            mutapp
                                .get_conversation(sent.destination)
                                .messages
                                .push(tui_message);
                        }
                    }
                    // Received data message
                    if message.data_message.is_some() {
                        let mesg = message.data_message.unwrap();

                        let source = message.source.unwrap();
                        let tui_message = Message {
                            message: mesg.message,
                            sender: source.clone(),
                            receiver: self.username.clone(),
                            timestamp: mesg.timestamp,
                        };

                        let mut mutapp = self.app.lock().await;
                        mutapp.get_conversation(source).messages.push(tui_message);
                    }
                }
                ResponseType::Version(_) => {}
                ResponseType::ContactList(_) => {}
                ResponseType::LinkingUri(_) => {}
                ResponseType::LinkingError(_) => {}
                ResponseType::Subscribed => {}
                ResponseType::Unsubscribed => {}
                ResponseType::Unknown(_, _) => {}
                _ => {}
            }
        }
    }

    async fn subscribe(&mut self) {
        if let Ok(_) = self.signald.subscribe(self.username.clone()).await {
            
        }
    }

    async fn get_contact_list(&mut self) {
        if let Ok(res) = self.signald.list_contacts(self.username.clone()).await {
            match res.data {
                ResponseType::ContactList(a) => {
                    let mut app = self.app.lock().await;
                    app.contacts.clear();
                    for contact in a.unwrap().iter() {
                        if !contact.name.is_empty() {
                            app.contacts.push(contact.name.clone());
                        }
                    }
                }
                _ => {}
            }
        }
    }

    async fn send_message(&mut self, data: SendMessageData) {
        self.signald
            .send(
                self.username.clone(),
                data.recipient.clone(),
                Some(data.message.clone()),
            )
            .await;

        let mut app = self.app.lock().await;
        let mesg = Message {
            message: data.message,
            receiver: data.recipient.clone(),
            sender: app.username.clone(),
            timestamp: 0,
        };
        app.get_conversation(data.recipient).messages.push(mesg);
    }
}
