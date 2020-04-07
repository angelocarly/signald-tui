/*
 * MIT License
 * 
 * Copyright (c) 2019 Alexander Keliris
 * 
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 * 
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use crossterm::event;
use std::{sync::mpsc, thread, time::Duration};
use super::key::Key;

#[derive(Debug, Clone, Copy)]
/// Configuration for event handling.
pub struct EventConfig {
  /// The key that is used to exit the application.
  pub exit_key: Key,
  /// The tick rate at which the application will sent an tick event.
  pub tick_rate: Duration,
}

impl Default for EventConfig {
  fn default() -> EventConfig {
    EventConfig {
      exit_key: Key::Ctrl('c'),
      tick_rate: Duration::from_millis(250),
    }
  }
}

/// An occurred event.
pub enum Event<I> {
  /// An input event occurred.
  Input(I),
  /// An tick event occurred.
  Tick,
}

/// A small event handler that wrap crossterm input and tick event. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
  rx: mpsc::Receiver<Event<Key>>,
  // Need to be kept around to prevent disposing the sender side.
  _tx: mpsc::Sender<Event<Key>>,
}

impl Events {
  /// Constructs an new instance of `Events` with the default config.
  pub fn new(tick_rate: u64) -> Events {
    Events::with_config(EventConfig {
      tick_rate: Duration::from_millis(tick_rate),
      ..Default::default()
    })
  }

  /// Constructs an new instance of `Events` from given config.
  pub fn with_config(config: EventConfig) -> Events {
    let (tx, rx) = mpsc::channel();

    let event_tx = tx.clone();
    thread::spawn(move || {
      loop {
        // poll for tick rate duration, if no event, sent tick event.
        if event::poll(config.tick_rate).unwrap() {
          if let event::Event::Key(key) = event::read().unwrap() {
            let key = Key::from(key);


            event_tx.send(Event::Input(key)).unwrap();
          }
        }

        event_tx.send(Event::Tick).unwrap();
      }
    });

    Events { rx, _tx: tx }
  }

  /// Attempts to read an event.
  /// This function will block the current thread.
  pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
    self.rx.recv()
  }
}