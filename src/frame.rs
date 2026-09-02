//! E1 ontology-neutral record framing.
//!
//! This module defines only the byte-level shape of one record frame. It does
//! not validate relationships between frames and does not interpret payload
//! bytes.

use core::fmt;

/// Fixed discriminator for the SNE causal record-frame family.
pub const FRAME_MAGIC: [u8; 8] = *b"SNECFRM\0";

/// Canonical E1 frame protocol version.
pub const FRAME_VERSION: u16 = 1;

/// Fixed bytes before the opaque payload.
pub const FRAME_HEADER_LEN: usize = 8 + 2 + 8 + 8;

/// One decoded E1 record frame.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordFrame {
    ordinal: u64,
    payload: Vec<u8>,
}

impl RecordFrame {
    /// Construct a frame from a structural ordinal and opaque payload bytes.
    pub fn new(ordinal: u64, payload: impl Into<Vec<u8>>) -> Self {
        Self {
            ordinal,
            payload: payload.into(),
        }
    }

    /// Structural ordinal carried by this frame.
    ///
    /// E1 assigns no continuity, ordering, or cross-frame validity policy to
    /// this value.
    pub const fn ordinal(&self) -> u64 {
        self.ordinal
    }

    /// Opaque payload bytes, preserved without text or schema interpretation.
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    /// Consume the frame and return its opaque payload bytes.
    pub fn into_payload(self) -> Vec<u8> {
        self.payload
    }

    /// Encode this frame using the canonical E1 byte layout.
    pub fn encode(&self) -> Result<Vec<u8>, FrameError> {
        encode_frame(self.ordinal, &self.payload)
    }
}

/// Strict E1 framing failures.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FrameError {
    PayloadTooLarge,
    HeaderTruncated { minimum: usize, actual: usize },
    FormatMismatch,
    UnsupportedVersion(u16),
    LengthOverflow,
    LengthMismatch { expected: usize, actual: usize },
}

impl fmt::Display for FrameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PayloadTooLarge => write!(formatter, "payload length does not fit u64"),
            Self::HeaderTruncated { minimum, actual } => write!(
                formatter,
                "frame shorter than fixed header: need {minimum} bytes, got {actual}"
            ),
            Self::FormatMismatch => write!(formatter, "record-frame format discriminator mismatch"),
            Self::UnsupportedVersion(version) => {
                write!(formatter, "unsupported record-frame version: {version}")
            }
            Self::LengthOverflow => write!(formatter, "record-frame length arithmetic overflow"),
            Self::LengthMismatch { expected, actual } => write!(
                formatter,
                "record-frame length mismatch: expected {expected} bytes, got {actual}"
            ),
        }
    }
}

impl std::error::Error for FrameError {}

/// Canonically encode one ontology-neutral record frame.
///
/// Layout:
///
/// ```text
/// 0..8    format discriminator
/// 8..10   version, u16 big-endian
/// 10..18  structural ordinal, u64 big-endian
/// 18..26  payload length, u64 big-endian
/// 26..    opaque payload bytes
/// ```
pub fn encode_frame(ordinal: u64, payload: &[u8]) -> Result<Vec<u8>, FrameError> {
    let payload_len = u64::try_from(payload.len()).map_err(|_| FrameError::PayloadTooLarge)?;
    let capacity = FRAME_HEADER_LEN
        .checked_add(payload.len())
        .ok_or(FrameError::LengthOverflow)?;

    let mut bytes = Vec::with_capacity(capacity);
    bytes.extend_from_slice(&FRAME_MAGIC);
    bytes.extend_from_slice(&FRAME_VERSION.to_be_bytes());
    bytes.extend_from_slice(&ordinal.to_be_bytes());
    bytes.extend_from_slice(&payload_len.to_be_bytes());
    bytes.extend_from_slice(payload);
    Ok(bytes)
}

/// Strictly decode exactly one canonical E1 record frame.
///
/// E1 validates only frame-local structure. It deliberately does not enforce
/// ordinal continuity or any other relationship with neighboring frames.
pub fn decode_frame(bytes: &[u8]) -> Result<RecordFrame, FrameError> {
    if bytes.len() < FRAME_HEADER_LEN {
        return Err(FrameError::HeaderTruncated {
            minimum: FRAME_HEADER_LEN,
            actual: bytes.len(),
        });
    }

    if bytes[..8] != FRAME_MAGIC {
        return Err(FrameError::FormatMismatch);
    }

    let version = u16::from_be_bytes(bytes[8..10].try_into().expect("fixed version field"));
    if version != FRAME_VERSION {
        return Err(FrameError::UnsupportedVersion(version));
    }

    let ordinal = u64::from_be_bytes(bytes[10..18].try_into().expect("fixed ordinal field"));
    let payload_len = u64::from_be_bytes(bytes[18..26].try_into().expect("fixed length field"));
    let payload_len = usize::try_from(payload_len).map_err(|_| FrameError::LengthOverflow)?;
    let expected = FRAME_HEADER_LEN
        .checked_add(payload_len)
        .ok_or(FrameError::LengthOverflow)?;

    if bytes.len() != expected {
        return Err(FrameError::LengthMismatch {
            expected,
            actual: bytes.len(),
        });
    }

    Ok(RecordFrame::new(
        ordinal,
        bytes[FRAME_HEADER_LEN..].to_vec(),
    ))
}
