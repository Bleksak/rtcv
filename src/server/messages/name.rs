use std::{error::Error, fmt::Display};

use crate::{
    serialize::{Deserialize, Serialize},
    server::message::{DeserializeError, Message, MessageType, SerializeError},
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Name(pub String);

impl Error for DeserializeError {}

impl Display for DeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Serialize for Name {
    fn serialize(&self) -> Result<Vec<u8>, SerializeError> {
        Ok(self.0.as_bytes().to_vec())
    }
}

impl Deserialize for Name {
    fn deserialize(data: &[u8]) -> Result<Self, DeserializeError> {
        let name = String::from_utf8(data.to_vec())
            .map_err(|e| DeserializeError::CouldNotDeserialize(e.to_string()))?;

        if name.trim().is_empty() {
            return Err(DeserializeError::CouldNotDeserialize(
                "Name is empty".into(),
            ));
        }

        Ok(Self(name))
    }
}

impl Message for Name {
    fn message_type(&self) -> MessageType {
        MessageType::Name
    }
}
