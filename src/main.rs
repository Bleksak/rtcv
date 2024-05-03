use std::error::Error;

use server::Server;

mod server;
mod client;
mod client_data;
mod serialize;

fn main() -> Result<(), Box<dyn Error>> {
    let server = Server::new()?;
    server.run();
    Ok(())
}
