use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::{
    AuthProof, PersonaSignalError, Reply, Request,
    version::{HandshakeReply, HandshakeRequest},
};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    pub auth: Option<AuthProof>,
    pub body: FrameBody,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum FrameBody {
    HandshakeRequest(HandshakeRequest),
    HandshakeReply(HandshakeReply),
    Request(Request),
    Reply(Reply),
}

impl Frame {
    pub fn new(body: FrameBody) -> Self {
        Self { auth: None, body }
    }

    pub fn with_auth(mut self, auth: AuthProof) -> Self {
        self.auth = Some(auth);
        self
    }

    pub fn encode(&self) -> Result<Vec<u8>, PersonaSignalError> {
        rkyv::to_bytes::<rkyv::rancor::Error>(self)
            .map(|bytes| bytes.to_vec())
            .map_err(|_| PersonaSignalError::ArchiveValidation)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, PersonaSignalError> {
        let archived = rkyv::access::<ArchivedFrame, rkyv::rancor::Error>(bytes)
            .map_err(|_| PersonaSignalError::ArchiveValidation)?;

        rkyv::deserialize::<Frame, rkyv::rancor::Error>(archived)
            .map_err(|_| PersonaSignalError::ArchiveDeserialize)
    }

    pub fn encode_length_prefixed(&self) -> Result<Vec<u8>, PersonaSignalError> {
        let archive = self.encode()?;
        let len = u32::try_from(archive.len()).map_err(|_| PersonaSignalError::LengthMismatch {
            expected: u32::MAX as usize,
            found: archive.len(),
        })?;
        let mut framed = Vec::with_capacity(4 + archive.len());
        framed.extend_from_slice(&len.to_be_bytes());
        framed.extend_from_slice(&archive);
        Ok(framed)
    }

    pub fn decode_length_prefixed(bytes: &[u8]) -> Result<Self, PersonaSignalError> {
        if bytes.len() < 4 {
            return Err(PersonaSignalError::ShortLengthPrefix);
        }

        let expected = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
        let payload = &bytes[4..];

        if payload.len() != expected {
            return Err(PersonaSignalError::LengthMismatch {
                expected,
                found: payload.len(),
            });
        }

        Self::decode(payload)
    }
}
