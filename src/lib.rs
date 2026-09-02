//! Ontology-neutral causal persistence substrate.
//!
//! The package currently contains the Tecton E1 record-framing candidate.
//! That candidate is not institutionally accepted by package import or by its
//! own implementation evidence.

#![forbid(unsafe_code)]

pub mod frame;

pub use frame::{
    FRAME_HEADER_LEN, FRAME_MAGIC, FRAME_VERSION, FrameError, RecordFrame, decode_frame,
    encode_frame,
};

/// The currently materialized extraction candidate.
pub const ACTIVE_EXTRACTION_CANDIDATE: &str = "E1_RECORD_FRAMING_CANDIDATE";

/// Constitutional invariant: importing this package never grants authority.
pub const PACKAGE_IMPORT_GRANTS_AUTHORITY: bool = false;

/// Tecton cannot accept its own E1 candidate.
pub const E1_ACCEPTED_BY_TECTON: bool = false;
