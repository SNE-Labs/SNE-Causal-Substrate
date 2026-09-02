//! Ontology-neutral causal persistence substrate.
//!
//! This package is an authorized scaffold. No persistence mechanism has been
//! extracted yet. Importing it grants no append, admission, economic, truth,
//! or store authority.

#![forbid(unsafe_code)]

/// The first extraction step authorized by R10.
pub const NEXT_EXTRACTION_STEP: &str = "E1_RECORD_FRAMING";

/// Constitutional invariant: importing this package never grants authority.
pub const PACKAGE_IMPORT_GRANTS_AUTHORITY: bool = false;
