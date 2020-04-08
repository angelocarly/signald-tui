
use crate::{event::key::Key, app::App};

pub mod contacthandler;
pub mod inputhandler;

pub trait Handler {
    fn handle(key: Key, app: &mut App);
}