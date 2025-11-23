//! Input data loading from various sources (RPC, JSON files, synthetic data)
//!
//! This module provides loaders for election data from different sources:
//!
//! - [`RpcLoader`] - Load data from Substrate RPC endpoints
//! - [`JsonLoader`] - Load data from JSON files
//! - [`SyntheticDataBuilder`] - Create synthetic election data programmatically
//!
//! # Examples
//!
//! ## Loading from RPC
//!
//! ```no_run
//! use offline_election::input::RpcLoader;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let loader = RpcLoader::new("https://rpc.polkadot.io")?;
//! let data = loader.load_at_block(10000000).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Loading from JSON
//!
//! ```no_run
//! use offline_election::input::JsonLoader;
//! use std::path::PathBuf;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let loader = JsonLoader::new();
//! let data = loader.load_from_file(PathBuf::from("election_data.json"))?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Creating Synthetic Data
//!
//! ```no_run
//! use offline_election::input::SyntheticDataBuilder;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut builder = SyntheticDataBuilder::new();
//! builder
//!     .add_candidate("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(), 1000000)?
//!     .add_nominator("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(), 500000, vec!["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string()])?;
//! let data = builder.build()?;
//! # Ok(())
//! # }
//! ```

pub mod rpc;
pub mod json;
pub mod synthetic;

/// RPC loader for fetching election data from Substrate nodes
///
/// Connects to a Substrate RPC endpoint and fetches validator candidates,
/// nominators, and stake information at a specific block.
pub use rpc::RpcLoader;

/// JSON loader for reading election data from files
///
/// Loads election data from JSON files that match the `ElectionData` schema.
pub use json::JsonLoader;

/// Builder for creating synthetic election data
///
/// Allows creating election data with arbitrary account IDs that don't
/// need to exist on-chain. Useful for testing and "what-if" scenarios.
pub use synthetic::SyntheticDataBuilder;


