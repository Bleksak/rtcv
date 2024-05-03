use crate::{
    client_data::ClientData,
    server::{
        message::{self, Message},
        messages::Name,
    },
};
use std::{
    fmt::Debug, io::{Read, Write}, os::unix::net::UnixStream
};

#[derive(Debug)]
pub enum DeserializeError
{
    Io(std::io::Error),
    Deserialize(message::DeserializeError),
}

pub enum SerializeError
{
    Io(std::io::Error),
    Serialize(message::SerializeError),
}

pub struct Client {
    socket: UnixStream,
    client_data: ClientData,
}

impl Client {
    pub fn new(mut socket: UnixStream) -> Result<Self, DeserializeError> {
        socket
            .set_nonblocking(true)
            .map_err(|e| DeserializeError::Io(e))?;

        let client_name = Self::read_raw::<Name>(&mut socket)?.0;

        println!("Read client name: {:?}", client_name);

        Ok(Self {
            socket,
            client_data: ClientData::new(client_name),
        })
    }

    pub fn read_raw<T>(sock: &mut UnixStream) -> Result<T, DeserializeError>
    where
        T: Message,
    {
        let mut buf = [0 as u8; std::mem::size_of::<u64>()];

        println!("Reading message...");

        // TODO: handle io::ErrorKind::WouldBlock
        sock.read_exact(&mut buf)
            .map_err(|e| DeserializeError::Io(e))?;

        let length = u64::from_le_bytes(buf) as usize;

        let mut buf = vec![0 as u8; length + std::mem::size_of::<u64>()];

        sock.read_exact(&mut buf)
            .map_err(|e| DeserializeError::Io(e))?;

        println!("Read {} bytes", buf.len());
        message::deserialize(length, &buf).map_err(|e| DeserializeError::Deserialize(e))
    }

    pub fn write_raw<T>(sock: &mut UnixStream, message: T) -> Result<(), SerializeError>
    where
        T: Message,
    {
        let message_type = message.message_type() as u16;
        let mut buf = message::serialize(message).map_err(|e| SerializeError::Serialize(e))?;
        let length = buf.len() as u64;

        buf.splice(0..0, message_type.to_le_bytes().into_iter());
        buf.splice(0..0, length.to_le_bytes().into_iter());

        sock.write_all(&buf).map_err(|e| SerializeError::Io(e))?;

        Ok(())
    }

    pub fn read<T>(&mut self) -> Result<T, DeserializeError>
    where
        T: Message,
    {
        Self::read_raw(&mut self.socket)
    }

    pub fn write<T>(&mut self, message: T) -> Result<(), SerializeError>
    where
        T: Message,
    {
        Self::write_raw(&mut self.socket, message)
    }
}
