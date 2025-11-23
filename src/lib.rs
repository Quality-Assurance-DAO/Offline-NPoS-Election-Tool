//! Offline NPoS Election Tool
//!
//! A Rust library for running offline NPoS (Nominated Proof of Stake) election simulations
//! that exactly mirror the election logic of Substrate chains.
//!
//! # Quick Start
//!
//! ```no_run
//! use offline_election::*;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Load election data from RPC
//! let data = ElectionData::from_rpc(
//!     "https://rpc.polkadot.io",
//!     Some(10000000)
//! ).await?;
//!
//! // Configure election
//! let config = ElectionConfiguration::new()
//!     .algorithm(AlgorithmType::SequentialPhragmen)
//!     .active_set_size(100)
//!     .build()?;
//!
//! // Execute election
//! let engine = ElectionEngine::new();
//! let result = engine.execute(&config, &data)?;
//!
//! // Access results
//! println!("Selected {} validators", result.validator_count());
//! # Ok(())
//! # }
//! ```
//!
//! # Modules
//!
//! - [`engine`] - Election execution engine
//! - [`models`] - Data models for elections, results, and configuration
//! - [`input`] - Data loading from RPC, JSON files, or synthetic generation
//! - [`algorithms`] - Election algorithm implementations
//! - [`diagnostics`] - Result analysis and explanations
//! - [`error`] - Error types

pub mod algorithms;
pub mod api;
pub mod cli;
pub mod diagnostics;
pub mod engine;
pub mod error;
pub mod input;
pub mod models;
pub mod types;

// Re-export commonly used types

/// Election execution engine
///
/// The main entry point for running election simulations. Create an instance
/// and call [`execute`](ElectionEngine::execute) with a configuration and data.
pub use engine::ElectionEngine;

/// Error type for election operations
///
/// All operations return `Result<T, ElectionError>` to handle validation errors,
/// RPC failures, algorithm errors, and other issues.
pub use error::ElectionError;

/// Election configuration
///
/// Builder pattern for configuring elections. Specify algorithm, active set size,
/// parameter overrides, and other settings.
///
/// # Example
///
/// ```no_run
/// use offline_election::{ElectionConfiguration, AlgorithmType};
///
/// let config = ElectionConfiguration::new()
///     .algorithm(AlgorithmType::SequentialPhragmen)
///     .active_set_size(100)
///     .build()?;
/// # Ok::<(), offline_election::ElectionError>(())
/// ```
pub use models::election_config::ElectionConfiguration;

/// Election data containing candidates and nominators
///
/// Contains all validator candidates, nominators, and their voting preferences.
/// Can be loaded from RPC, JSON files, or created synthetically.
///
/// # Example
///
/// ```no_run
/// use offline_election::ElectionData;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Load from RPC
/// let data = ElectionData::from_rpc(
///     "https://rpc.polkadot.io",
///     Some(10000000)
/// ).await?;
/// # Ok(())
/// # }
/// ```
pub use models::election_data::ElectionData;

/// Election result containing selected validators and stake distribution
///
/// The outcome of an election execution, including which validators were selected,
/// how stake is distributed, and optional diagnostics.
pub use models::election_result::ElectionResult;

/// Parameter overrides for modifying election data
///
/// Allows overriding candidate stakes, nominator stakes, and voting edges
/// without modifying the original data source.
pub use models::election_overrides::ElectionOverrides;

/// Nominator model
///
/// Represents an account that stakes tokens and votes for validator candidates.
pub use models::nominator::Nominator;

/// Validator candidate model
///
/// Represents a potential validator in the election with associated stake.
pub use models::validator::ValidatorCandidate;

/// Election algorithm type
///
/// Supported algorithms: Sequential Phragmen, Parallel Phragmen, and Multi-phase.
pub use types::AlgorithmType;

/// Data source type
///
/// Specifies where election data comes from: RPC endpoint, JSON file, or synthetic.
pub use types::DataSource;

