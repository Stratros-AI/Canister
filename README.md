# Stratros.AI Storage Canister and Signature Canister

## Overview

This repository provides implementations for two core canisters in the Stratros.AI system:
1.	Storage Canister: A key-value (K-V) storage and retrieval system designed to handle blob data efficiently.
2.	Signature Canister: A canister responsible for generating threshold signatures and maintaining confirmations through a Merkle tree-based structure.

These canisters work together to ensure decentralized, scalable, and verifiable data storage and confirmation processes.

## Storage Canister

The Storage Canister is designed to manage blob data in a decentralized manner. It supports segmented uploads for efficient use of storage and retrieval operations.

### Features

	•	Decentralized K-V storage system for blob data.
	•	Support for segmented uploads to optimize storage.
	•	Configurable parameters for storage thresholds and query response sizes.

### Canister Types

```rust
/// Storage Canister
// Canister Blob Data Structure
struct Blob {
  data: Vec<u8>, // The blob data
  next: Option<usize>, // Index of the next slice, if applicable
}

/// Structure for uploading blob slices
/// This is used to manage large blobs by splitting them into smaller chunks.
struct BlobChunk {
  index: usize,        // Index of the current chunk
  digest: [u8; 32],    // SHA256 digest of the blob in hex format
  timestamp: u128,     // Timestamp in nanoseconds since epoch
  total: usize,        // Total size of the blob in bytes
  data: Vec<u8>,       // Chunk data (a part of the blob)
}

/// Storage Canister Configuration
struct Config {
  signature_canister: Principal,       // Principal allowed to upload to the canister
  owner: Principal,                    // Owner responsible for confirmations and configurations
  query_response_size: usize,          // Query response size for blob retrieval
  canister_storage_threshold: u32,     // Number of blobs to store (e.g., over a 7-day period)
}

enum Result {
  Ok(),
  Err(String),
}
```

### Canister Services

The Storage Canister provides the following services:

```
/// Retrieve the first slice of a blob by its digest
fn get_blob(digest: [u8; 32]) -> Blob {}

/// Retrieve a specific slice of a blob by its digest and slice index
fn get_blob_with_index(digest: [u8; 32], index: usize) -> Blob {}

/// Save a blob slice to the canister
fn save_blob(chunk: BlobChunk) -> Result<(), String> {}
```

## Signature Canister

The Signature Canister manages confirmations and generates threshold signatures. It uses a Merkle tree to organize and verify blob digests.

### Features

	•	Supports batch processing of blob digests.
	•	Implements Merkle trees for efficient verification and proof generation.
	•	Configurable confirmation retention period and batch sizes.

### Workflow

	1.	Batching: Blob digests are grouped into batches (e.g., 12 digests per batch, configurable).
	2.	Signature Generation: When a batch is complete, the Merkle root is signed.
	3.	Proof Generation: When requested, the canister generates a Merkle proof for a specific digest within a batch.
	4.	Confirmation Retention: Each batch confirmation is stored for a specified duration (e.g., one week, configurable).

### Canister Types

```rust
/// Enum representing the status of a confirmation
enum ConfirmationStatus {
  Pending,             // Confirmation exists but is not yet signed
  Confirmed(Confirmation), // Confirmation is signed and valid
  Invalid,             // The requested digest is invalid or retired
}

/// A confirmation containing proof, signature, and Merkle root
struct Confirmation {
  pub root: [u8; 32],   // Merkle root hash
  pub proof: Proof,     // Merkle proof for the digest
  pub signature: String, // Hex-encoded signature of the Merkle root
}

/// Merkle proof for a specific digest
struct Proof {
  pub proof_bytes: Vec<u8>, // Merkle proof in byte form
  pub leaf_index: usize,    // Index of the digest in the Merkle tree
  pub leaf_digest: [u8; 32], // The digest being proven
}

/// Structure representing a batch of confirmations
struct BatchConfirmation {
  pub signature: Option<String>, // Optional signature of the Merkle root
  pub root: [u8; 32],            // Merkle root hash
  pub nodes: Vec<[u8; 32]>,      // List of blob digests in the batch
}

/// Configuration for the Signature Canister
struct Config {
  pub confirmation_batch_size: usize,    // Number of digests in a batch
  pub confirmation_live_time: u32,      // Duration for which confirmations are retained (in seconds)
  pub da_canisters: HashSet<Principal>, // Data availability (storage) canisters
  pub owner: Principal,                 // Principal authorized to update configuration
} 
```

### Canister Services

The Signature Canister provides the following services:
```
/// Retrieve a confirmation by its digest
fn get_confirmation(digest: [u8; 32]) -> ConfirmationStatus {}

/// Retrieve the canister's public key
fn public_key() -> Vec<u8> {}

/// Insert a new blob digest into the confirmation system (restricted to storage canisters)
fn insert_digest(digest: [u8; 32]) {}

/// Update the configuration of the Signature Canister
fn update_config(config: Config) {}
```

## How to Use

	1.	Deploy the Canisters:
		Set up and deploy the Storage Canister and Signature Canister on the Internet Computer.
	2.	Configure the Canisters:
		Configure the Config for each canister, ensuring compatibility between the storage and signature systems.
	3.	Upload and Confirm Blobs:
		Use the Storage Canister to upload blobs in chunks.
		Use the Signature Canister to manage and verify blob confirmations.
	4.	Integrate with Your Application:
		Use the provided services to integrate storage and confirmation features into your decentralized application.

## Contributions

Contributions are welcome! Please follow the guidelines below:
•	Fork the repository and create a new branch for your feature or bug fix.
•	Write clear and concise commit messages.
•	Ensure all changes pass tests and are well-documented.

## License

This project is licensed under the MIT License. See the LICENSE file for details.