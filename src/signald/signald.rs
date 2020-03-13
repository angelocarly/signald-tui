#![feature(deadline_api)]
use crate::signald::signaldrequest::SignaldRequestBuilder;
use crate::signald::signaldrequest::SignaldRequest;
use crate::signald::signaldsocket::{SignaldSocket};
use tokio::time::*;
use std::time::Duration;
use std::sync::mpsc;
use bus::Bus;
use serde_json::Value;
use std::sync::mpsc::RecvTimeoutError::Timeout;
use std::sync::mpsc::RecvTimeoutError;

/// Responsible for all the communication to the signald socket
pub struct Signald {
    // The signald socket
    socket: SignaldSocket,
    // A request builder which is reused to limit memory allocation
    request_builder: SignaldRequestBuilder,
    // A count of all the sent messages on this socket
    message_count: u32,
}
impl Signald {

    /// Connect the Signald socket
    pub fn connect(socket_path: String) -> Signald {
        Signald {
            socket: SignaldSocket::connect(socket_path, 100),
            request_builder: SignaldRequestBuilder::new(),
            message_count: 0,
        }
    }
    /// Send a signald request on the socket
    pub fn send_request(&mut self, request: &SignaldRequest) {
        self.socket.send_request(&request);
        self.message_count += 1;
    }
    /// Enable receiving user events such as received messages
    pub fn subscribe(&mut self, username: String) {
        self.request_builder.flush();
        self.request_builder.set_type("subscribe".to_string());
        self.request_builder.set_username(username);
        let request = self.request_builder.build();

        self.send_request(&request);
    }
    /// Disable receiving user events such as received messages
    pub fn unsubscribe(&mut self, username: String) {
        self.request_builder.flush();
        self.request_builder.set_type("unsubscribe".to_string());
        self.request_builder.set_username(username);
        let request = self.request_builder.build();

        self.send_request(&request);
    }
    /// Link an existing signal account
    pub fn link(&mut self) {
        self.request_builder.flush();
        self.request_builder.set_type("link".to_string());
        let request = self.request_builder.build();

        self.send_request(&request);
    }
    /// Get the current signald version
    pub fn version(&mut self) {
        self.request_builder.flush();
        self.request_builder.set_type("version".to_string());
        let request = self.request_builder.build();

        self.send_request(&request);
    }
    /// Query all the user's contacts
    pub async fn list_contacts(&mut self, username: String) -> Result<String, RecvTimeoutError> {
        self.request_builder.flush();
        self.request_builder.set_type("list_contacts".to_string());
        self.request_builder.set_username(username);
        self.request_builder.set_id(self.message_count.to_string());
        let request = self.request_builder.build();

        let id = self.message_count.to_string();

        self.send_request(&request);

        return self.wait_for_request(id).await;
    }
    /// Send a contact sync request to the other devices on this account
    pub fn sync_contacts(&mut self, username: String) {
        self.request_builder.flush();
        self.request_builder.set_type("sync_contacts".to_string());
        self.request_builder.set_username(username);
        self.request_builder.set_id(self.message_count.to_string());
        let request = self.request_builder.build();

        self.send_request(&request);
    }

    /// Get a response from the bus with a matching id
    /// Returns a RecvTimeoutError if the message took more than 3 seconds to return
    async fn wait_for_request(&mut self, id: String) -> Result<String, RecvTimeoutError> {
        // The max possible time to receive a message
        let end = Instant::now() + Duration::from_millis(3000);
        let mut rx = self.socket.get_rx();

        let result = rx.iter()
            // Stop the receiver once the time is over, this keeps updating thanks to the update messages in systemdsocket
            .take_while(|x| Instant::now() < end )
            .find(|y| {
                // The systemdsocket sends an 'update' message each second, don't parse this
                if y == "update" { return false; }

                let v: Value = serde_json::from_str(y).expect("Couldn't parse message");
                match v["id"].as_str() {
                    Some(s) => {
                        return s == id;
                    },
                    None => {}
                }
                false
            });

        // When no results are found within the time limit, an error is returned
        match result {
            Some(x) => {
                Ok(x)
            },
            None => {
                Err(Timeout)
            }
        }

    }
}

