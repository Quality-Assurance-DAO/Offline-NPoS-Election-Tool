//! Error types for the Offline NPoS Election Tool
//!
//! All operations return `Result<T, ElectionError>` to provide detailed error information.

use crate::types::AlgorithmType;
use std::path::PathBuf;
use thiserror::Error;

/// Main error type for election operations
///
/// All API functions return `Result<T, ElectionError>`. Errors provide detailed
/// messages and context to help diagnose issues.
///
/// # Error Handling
///
/// ```no_run
/// use offline_election::{ElectionEngine, ElectionConfiguration, ElectionData, ElectionError};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let engine = ElectionEngine::new();
/// # let config = ElectionConfiguration::default();
/// # let data = ElectionData::default();
/// match engine.execute(&config, &data) {
///     Ok(result) => println!("Success: {} validators selected", result.validator_count()),
///     Err(ElectionError::ValidationError { message, field }) => {
///         eprintln!("Validation failed: {} (field: {:?})", message, field);
///     }
///     Err(ElectionError::RpcError { message, url }) => {
///         eprintln!("RPC error: {} (URL: {})", message, url);
///     }
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Error)]
pub enum ElectionError {
    /// Validation error with optional field name
    ///
    /// Occurs when input data or configuration fails validation checks.
    /// The `field` parameter indicates which field caused the error, if applicable.
    #[error("Validation error: {message}")]
    ValidationError {
        /// Human-readable error message explaining what failed validation
        message: String,
        /// Optional field name that caused the validation error
        field: Option<String>,
    },

    /// RPC connection or query error
    ///
    /// Occurs when connecting to or querying a Substrate RPC endpoint fails.
    /// This includes network errors, timeout errors, and invalid responses.
    #[error("RPC error: {message} (URL: {url})")]
    RpcError {
        /// Error message describing the RPC failure
        message: String,
        /// RPC endpoint URL that failed
        url: String,
    },

    /// Algorithm execution error
    ///
    /// Occurs when an election algorithm fails to execute or converge.
    /// This can happen with invalid input data or algorithm-specific issues.
    #[error("Algorithm error: {message} (algorithm: {algorithm:?})")]
    AlgorithmError {
        /// Error message describing the algorithm failure
        message: String,
        /// Algorithm that failed
        algorithm: AlgorithmType,
    },

    /// Insufficient candidates for requested active set size
    ///
    /// Occurs when the requested active set size exceeds the number of
    /// available candidates in the election data.
    #[error("Insufficient candidates: requested {requested}, available {available}")]
    InsufficientCandidates {
        /// Number of validators requested
        requested: u32,
        /// Number of candidates available
        available: u32,
    },

    /// Invalid data structure
    ///
    /// Occurs when data structures are malformed or contain invalid values
    /// that don't match expected formats.
    #[error("Invalid data: {message}")]
    InvalidData {
        /// Error message describing the invalid data
        message: String,
    },

    /// File I/O error
    ///
    /// Occurs when reading or writing files fails, such as when loading
    /// JSON files or writing output files.
    #[error("File error: {message} (path: {path:?})")]
    FileError {
        /// Error message describing the file operation failure
        message: String,
        /// Path to the file that caused the error
        path: PathBuf,
    },
}


