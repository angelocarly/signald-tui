use serde::Serialize;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

/// A Signald response
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct SignaldResponse {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "id")]
    pub _id: Option<String>,
    #[serde(rename = "data")]
    pub _data: Value,
}

pub trait ResponseData {}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct VersionData {
    #[serde(rename = "name")]
    pub _name: String,
    #[serde(rename = "version")]
    pub _version: String,
    #[serde(rename = "branch")]
    pub _branch: String,
    #[serde(rename = "commit")]
    pub _commit: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct MessageData {
    #[serde(rename = "username")]
    pub _username: Option<String>,
    #[serde(rename = "uuid")]
    pub _uuid: Option<String>,
    #[serde(rename = "source")]
    pub _source: Option<String>,
    #[serde(rename = "sourceDevice")]
    pub _source_device: Option<i32>,
    #[serde(rename = "type")]
    pub _type: i32,
    #[serde(rename = "timestamp")]
    pub _timestamp: i64,
    #[serde(rename = "timestampISO")]
    pub _timestamp_iso: String,
    #[serde(rename = "serverTimestamp")]
    pub _server_timestamp: i64,
    #[serde(rename = "hasContent")]
    pub _has_content: bool,
    #[serde(rename = "isReceipt")]
    pub _is_receipt: bool,
    #[serde(rename = "isUnidentifiedSender")]
    pub _is_unidentified_sender: bool,
    #[serde(rename = "syncMessage")]
    pub _sync_message: Value,
    #[serde(rename = "dataMessage")]
    pub _data_message: Option<Message>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct SyncMessage {
    #[serde(rename = "sent")]
    pub _sent: Option<SentMessage>,
    #[serde(rename = "contacts")]
    pub _contacts: Option<Contacts>,
    #[serde(rename = "contactsComplete")]
    pub _contacts_complete: bool,
    #[serde(rename = "readMessages")]
    pub _read_messages: Vec<ReadMessage>,
    #[serde(rename = "stickerPackOperations")]
    pub _sticker_pack_operations: Option<Vec<String>>,
    #[serde(rename = "unidentifiedStatus")]
    pub _unidentified_status: Option<Vec<String>>,
    #[serde(rename = "isRecipientUpdate")]
    pub _is_recipient_update: bool,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Message {
   #[serde(rename = "timestamp")]
   pub _timestamp: i64,
   #[serde(rename = "message")]
   pub _message: String,
   #[serde(rename = "expiresInSeconds")]
   pub _expires_in_seconds: i32,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct SentMessage {
    #[serde(rename = "destination")]
    pub _destination: String,
    #[serde(rename = "timestamp")]
    pub _timestamp: i64,
    #[serde(rename = "expirationStartTimestamp")]
    pub _expiration_start_timestamp: i64,
    #[serde(rename = "unidentifiedStatus")]
    pub _unidentified_status: HashMap<String, i64>,
    #[serde(rename = "expirationStartTimestamp")]
    pub _is_recipient_update: bool,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ReadMessage {
    #[serde(rename = "sender")]
    pub _sender: String,
    #[serde(rename = "timestamp")]
    pub _timestamp: i64,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Contacts {
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Receipt {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "timestamps")]
    pub _timestamps: Vec<String>,
    #[serde(rename = "when")]
    pub _when: i32,
}
