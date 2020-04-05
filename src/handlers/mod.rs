use termion::event::Key;

use crate::app::App;

pub mod inputhandler;

pub trait Handler {
    fn handle(key: Key, app: &mut App);
}