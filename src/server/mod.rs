pub mod message;
pub mod messages;

use std::os::unix::net::{UnixListener, UnixStream};

use crate::client::{self, Client};

pub struct Server {
    socket: UnixListener,
    clients: Clients,
}

pub struct Clients {
    clients: Vec<Client>,
}

impl Clients {
    pub fn new() -> Self {
        Self {
            clients: Vec::new(),
        }
    }

    pub fn handle_incoming_client(&mut self, client: UnixStream) -> Result<(), client::DeserializeError> {
        println!("There is a new client in town!");
        let client = Client::new(client)?;
        // we should read the socket's name, then find it in the list of clients
        // if such a client exists, attach it to the socket
        // otherwise, create a new client

        self.clients.push(client);

        Ok(())
    }
}

impl Server {
    pub fn new() -> Result<Self, std::io::Error> {
        let socket = UnixListener::bind("/tmp/rtcv.sock")?;

        socket.set_nonblocking(true)?;

        Ok(Self {
            socket,
            clients: Clients::new(),
        })
    }

    pub fn run(mut self) {
        for client in self.socket.incoming() {
            if let Ok(client) = client {
                match self.clients.handle_incoming_client(client) {
                    Ok(_) => {},
                    Err(e) => println!("Error: {:?}", e),
                }
            }
        }
    }
}
