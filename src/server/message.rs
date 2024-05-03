use std::fmt::Debug;

use crate::serialize::{Deserialize, Serialize};

pub trait Message: Serialize + Deserialize + Debug {
    fn message_type(&self) -> MessageType;
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum MessageType {
    OK = 0,
    ERR = 1,
    Ping = 2,
    Pong = 3,
    Name = 4,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum SerializeError
{
    CouldNotSerialize,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum DeserializeError
{
    LengthMismatch,
    InvalidType,
    ShouldNotHappen,
    CouldNotDeserialize(String),
}

pub fn serialize(message: impl Message) -> Result<Vec<u8>, SerializeError>
{
    let mut data = message
        .serialize()
        .map_err(|e| SerializeError::CouldNotSerialize)?;

    let len = data.len() as u64;

    data.splice(0..0, len.to_le_bytes().into_iter());
    Ok(data)
}

pub fn deserialize<T>(length: usize, data: &[u8]) -> Result<T, DeserializeError>
where
    T: Deserialize + Debug,
{
    // TODO: we need to do this outside of this function, this function has no way of knowing the
    // <T> from the message_type variable!!
    let message_type = u16::from_le_bytes(
        data.get(0..2)
            .ok_or(DeserializeError::InvalidType)?
            .try_into()
            .map_err(|_| DeserializeError::ShouldNotHappen)?,
    );

    if data.len() < length + std::mem::size_of::<u16>() {
        return Err(DeserializeError::LengthMismatch);
    }

    let data = &data[std::mem::size_of::<u16>()..length + std::mem::size_of::<u16>()];

    T::deserialize(data)
}
