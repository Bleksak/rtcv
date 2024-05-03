use crate::server::message::{DeserializeError, SerializeError};

pub trait Serialize {
    fn serialize(&self) -> Result<Vec<u8>, SerializeError>;
}

pub trait Deserialize {
    fn deserialize(data: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized;
}
