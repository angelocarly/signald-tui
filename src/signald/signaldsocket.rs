use crate::signald::signaldrequest::SignaldRequest;
use std::sync::{mpsc, Arc, Mutex};
use std::os::unix::net::UnixStream;
use std::thread;
use std::io::{Write, BufReader, BufRead};
use std::sync::mpsc::{Receiver, TryIter};
use bus::{Bus, BusReader};
use std::time::Duration;

pub struct SignaldSocket {
    socket_path: String,
    socket: UnixStream,
    bus: Arc<Mutex<Bus<String>>>,
}
impl SignaldSocket {
    pub fn connect(socket_path:String, bus_size: usize) -> SignaldSocket {

        // Connect the socket
        let socket = match UnixStream::connect(socket_path.to_string()){
            Ok(stream) => {
                println!("Connected to socket");
                stream
            }
            Err(err) => {
                panic!("Failed to connect socket");
            }
        };
        let socket_clone = socket.try_clone().unwrap();

        // Create a bus
        let bus = Arc::new(Mutex::new(Bus::new(bus_size)));

        // Broadcast on the bus in a new thread
        let bus_tx = bus.clone();
        thread::spawn(move || {
            let reader = BufReader::new(socket);
            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        //tx.send(l);
                        bus_tx.lock().unwrap().broadcast(l);
                    },
                    Err(_) => {

                    }
                }
            }
        });

        // An update message every second to make sure that the receivers can verify the time they're waiting
        // When there are no messages on the bus the receivers would otherwise be stuck waiting
        // This is a hacky implementation and should be changed once recv_deadline can be implemented
        let bus_tx_seconds = bus.clone();
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(1));
                bus_tx_seconds.lock().unwrap().broadcast("update".to_string());
            }
        });


        Self {
            socket_path: socket_path,
            socket: socket_clone,
            bus: bus,
        }
    }

    pub fn send_request(&mut self, request: &SignaldRequest) {
        let formatted_request = request.to_json_string() + "\n";
        match self.socket.write_all(formatted_request.as_bytes()) {
            Err(_) => panic!("Failed to send message"),
            Ok(_) => {
                //println!("mesg sent {}", formatted_request);
            }
        }
    }

    pub fn get_rx(&mut self) -> BusReader<String> {
        self.bus.lock().unwrap().add_rx()
    }
}
