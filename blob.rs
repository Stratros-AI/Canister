//! # Blob Upload Handling
//! This module handles three main tasks:
//! - Managing the storage map with a BTreeMap.
//! - Handling expiration with a time heap.
//! - Generating signatures.

use candid::{CandidType, Deserialize};
use ic_cdk::print;
use serde::Serialize;

use crate::{BLOBS, DACONFIG};

/// Wrapper struct for blob data
pub struct BlobData(pub Vec<u8>);

/// Struct representing a chunk of a blob, used for segmented uploads
#[derive(Deserialize, Serialize, CandidType, Debug, Clone)]
pub struct BlobChunk {
    /// Index of the segmented upload.
    pub index: usize,

    /// SHA256 digest of the blob in hex format.
    pub digest: [u8; 32],

    /// Timestamp in nanoseconds since the epoch.
    pub timestamp: u128,

    /// Total size of the blob in bytes.
    pub total: usize,

    /// A piece of the blob data.
    pub data: Vec<u8>,
}

/// Struct representing a complete blob stored in the system
#[derive(CandidType, Deserialize, Serialize, Clone, Default)]
pub struct Blob {
    /// The data of the blob.
    pub data: Vec<u8>,

    /// The index of the next segment, if applicable.
    pub next: Option<usize>,
}