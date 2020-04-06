use termion::event::Key;

use crate::app::App;
use crate::handlers::Handler;

pub struct InputHandler {
    data: String,

}

impl Handler for InputHandler {
    fn handle(key: Key, app: &mut App) {
        match key {
            Key::Left => {
                app.input_position -= 1;
                if app.input_position < 0 {
                    app.input_position = 0;
                }
            }
            Key::Right => {
                app.input_position += 1;
                if app.input_position > app.input_string.chars().count() {
                    app.input_position = app.input_string.chars().count();
                }
            }
            Key::Down => {
                // app.items.next();
            }
            Key::Up => {
                // app.items.previous();
            }
            Key::Backspace => {
                if app.input_string.chars().count() > 0 {
                    let _last_c = app.input_string.remove(app.input_position - 1);
                    app.input_position -= 1;
                }
            }
            Key::Char(x) => {
                app.input_string.insert(app.input_position, x);
                app.input_position += 1;
            }
            _ => {}
        }
    }
}