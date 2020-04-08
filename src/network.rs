use std::sync::Arc;

use futures::SinkExt;
use signald_rust::signald::Signald;
use signald_rust::signaldresponse::{Account, ResponseType, SignaldResponse};
use tokio::sync::Mutex;

use crate::app::{App, Message, Contact};
use bus::BusReader;
use std::time::{UNIX_EPOCH, SystemTime};

pub enum IoEvent {
    Subscribe,
    GetContactList,
    SendMessage(SendMessageData),
    LoadAccount,
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
    pub fn new(app: Arc<Mutex<App>>) -> Self {
        let mut signald = Signald::connect();
        let bus_rx = signald.get_rx();
        Self {
            username: "".to_string(),
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
            IoEvent::LoadAccount => {
                self.load_accounts().await;
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
                            if let Some(conv) = mutapp.get_conversation(sent.destination) {
                                conv.messages.push(tui_message);
                            }
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
                        if let Some(conv) = mutapp.get_conversation(source) {
                            conv.messages.push(tui_message);
                        }
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
        if let Ok(_) = self.signald.subscribe(self.username.clone()).await {}
    }

    async fn get_contact_list(&mut self) {
        if let Ok(res) = self.signald.list_contacts(self.username.clone()).await {
            match res.data {
                ResponseType::ContactList(a) => {
                    let mut contacts: Vec<Contact> = Vec::new();

                    for account in a.unwrap().iter() {

                        let contact = Contact {
                            name: account.name.clone(),
                            number: account.number.clone(),
                            color: account.color.clone(),
                        };

                        contacts.push(contact);
                    }
                    let mut app = self.app.lock().await;
                    app.update_contacts(contacts);
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
        let start = SystemTime::now();
        let datetime = start.duration_since(UNIX_EPOCH).expect("Time looped over");
        let mesg = Message {
            message: data.message,
            receiver: data.recipient.clone(),
            sender: app.username.clone(),
            timestamp: datetime.as_millis() as i64,
        };
        if let Some(conv) = app.get_conversation(data.recipient) {
            conv.messages.push(mesg);
        }
    }

    async fn load_accounts(&mut self) {

        if let Ok(res) = self.signald.list_accounts().await {

            match res.data {
                ResponseType::AccountList(a) => {
                    let accounts = a.unwrap().accounts;

                    if accounts.len() == 0 {
                        panic!("No signald account found! exiting");
                    }
                    let username = accounts.get(0).unwrap().username.clone();
                    self.username = username.clone();

                    let mut app = self.app.lock().await;
                    app.username = username;
                    app.loaded = true;
                }
                _ => {}
            }
        }
    }
}
