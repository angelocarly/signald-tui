use serde::Serialize;

/// A Signald request
/// Contains of all the possible fields necessary by signald
#[derive(Serialize, Default, Clone)]
pub struct SignaldRequest {
    #[serde(rename = "type")]
    pub typ: String,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "messageBody", skip_serializing_if = "Option::is_none")]
    pub message_body: Option<String>,
    #[serde(rename = "recipientNumber", skip_serializing_if = "Option::is_none")]
    pub recipient_number: Option<String>,
    #[serde(rename = "recipientGroupId",skip_serializing_if = "Option::is_none")]
    pub recipient_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "deviceName", skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_filenames: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}
impl SignaldRequest {
    /// Parse a request to json
    pub fn to_json_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub struct SignaldRequestBuilder {
    request: SignaldRequest
}
impl SignaldRequestBuilder {
    pub fn new() -> SignaldRequestBuilder {
        SignaldRequestBuilder {
            request: Default::default(),
        }
    }

    pub fn set_type(&mut self, typ: String) {
        self.request.typ = typ;
    }

    pub fn set_username(&mut self, username: String) {
        self.request.username = username;
    }

    pub fn set_recipient_number(&mut self, recipient_number: String) {
        self.request.recipient_number = Some(recipient_number);
    }

    pub fn set_message_body(&mut self, message_body: String) {
        self.request.message_body = Some(message_body);
    }

    pub fn set_id(&mut self, id: String) {
        self.request.id = Some(id);
    }

    /// Resets the internal request object, useful for creating a new request
    pub fn flush(&mut self) {
        self.request = Default::default();
    }

    /// Create a request
    /// Returns a clone of the internal request
    pub fn build(&self) -> SignaldRequest {
        self.request.clone()
    }
}
