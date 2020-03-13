use std::{thread, time};
use crate::signald::signaldrequest::SignaldRequestBuilder;
use common::SOCKET_PATH;
use signald::signald::Signald;
use tokio::prelude::*;
use tokio::task;

pub mod common;
pub mod signald;

/**
 * Main is currently used for debugging purposes
 * Will be removed once I figure out how to create a proper Crate
 * The library itself only consists of the signal/ folder
 */
#[tokio::main]
async fn main() {
    let mut signald = Signald::connect(SOCKET_PATH.to_string());

    let mut messagebuilder = SignaldRequestBuilder::new();
    messagebuilder.set_type("send".to_string());
    messagebuilder.set_username("+32472271852".to_string());
    messagebuilder.set_recipient_number("+32472271852".to_string());
    messagebuilder.set_message_body("Heeey jarne".to_string());
    let req = messagebuilder.build();

    signald.subscribe("+32472271852".to_string());

    // signald.subscribe("+32472271852".to_string());
    //signald.send_request(&req);
    
    // signald.link();
    // signald.version();
    // signald.list_contacts("+32472271852".to_string());
    // signald.list_contacts("+32472271852".to_string());

    match signald.list_contacts("+32472271852".to_string()).await {
        Ok(r) => {
            println!("{:?}", r.as_str());
        },
        Err(e) => {
            println!("Request timed out") ;
        }
    }
    // let contacts: String = signald.list_contacts("+32472271852".to_string()).await;
    // let contacts1 = signald.list_contacts("+32472271852".to_string()).await.unwrap();
    // println!("Received 1: {}\n", contacts);
    //println!("Received 2: {}", contacts1);
//    signald.read_requests();

}

