//! Election engine for executing elections
//!
//! The [`ElectionEngine`] is the main entry point for running election simulations.
//! It handles algorithm selection, parameter overrides, validation, and result generation.

use crate::algorithms::trait_def::ElectionAlgorithm;
use crate::algorithms::sequential_phragmen::SequentialPhragmen;
use crate::diagnostics::explainer::DiagnosticsGenerator;
use crate::error::ElectionError;
use crate::models::election_config::ElectionConfiguration;
use crate::models::election_data::ElectionData;
use crate::models::election_result::ElectionResult;
use crate::types::AlgorithmType;

/// Election engine for executing elections with various algorithms
///
/// The engine coordinates election execution by:
/// 1. Validating election data and configuration
/// 2. Applying parameter overrides if specified
/// 3. Selecting and executing the appropriate algorithm
/// 4. Validating results
/// 5. Optionally generating diagnostics
///
/// # Example
///
/// ```no_run
/// use offline_election::{ElectionEngine, ElectionConfiguration, ElectionData, AlgorithmType};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let engine = ElectionEngine::new();
/// let config = ElectionConfiguration::new()
///     .algorithm(AlgorithmType::SequentialPhragmen)
///     .active_set_size(100)
///     .build()?;
/// let data = ElectionData::from_rpc("https://rpc.polkadot.io", Some(10000000)).await?;
///
/// let result = engine.execute(&config, &data)?;
/// println!("Selected {} validators", result.validator_count());
/// # Ok(())
/// # }
/// ```
///
/// # Thread Safety
///
/// `ElectionEngine` is `Send + Sync` and can be safely shared across threads.
pub struct ElectionEngine;

impl ElectionEngine {
    /// Create a new election engine
    ///
    /// The engine is stateless and can be reused for multiple elections.
    pub fn new() -> Self {
        Self
    }

    /// Execute an election with the given configuration and data
    ///
    /// This is the main method for running elections. It validates inputs,
    /// applies overrides, executes the algorithm, and returns results.
    ///
    /// # Arguments
    ///
    /// * `config` - Election configuration specifying algorithm and parameters
    /// * `data` - Election data containing candidates and nominators
    ///
    /// # Returns
    ///
    /// Returns `Ok(ElectionResult)` on success, or `Err(ElectionError)` if validation
    /// fails, the algorithm errors, or other issues occur.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - Election data validation fails (no candidates, duplicate IDs, invalid edges)
    /// - Configuration validation fails (invalid active set size)
    /// - Algorithm execution fails
    /// - Result validation fails
    pub fn execute(
        &self,
        config: &ElectionConfiguration,
        data: &ElectionData,
    ) -> Result<ElectionResult, ElectionError> {
        self.execute_with_diagnostics(config, data, false)
    }

    /// Execute an election with optional diagnostics generation
    ///
    /// Similar to [`execute`](Self::execute), but allows requesting diagnostics
    /// to be generated and included in the result.
    ///
    /// # Arguments
    ///
    /// * `config` - Election configuration
    /// * `data` - Election data
    /// * `generate_diagnostics` - If `true`, generate detailed diagnostics explaining results
    ///
    /// # Returns
    ///
    /// Returns `Ok(ElectionResult)` with optional diagnostics if requested.
    pub fn execute_with_diagnostics(
        &self,
        config: &ElectionConfiguration,
        data: &ElectionData,
        generate_diagnostics: bool,
    ) -> Result<ElectionResult, ElectionError> {
        // Validate election data
        data.validate()?;

        // Auto-adjust active set size if there are fewer candidates available
        let candidate_count = data.candidates().len();
        let effective_active_set_size = if config.active_set_size as usize > candidate_count {
            eprintln!(
                "Warning: Requested {} validators but only {} candidates available. Using {} instead.",
                config.active_set_size,
                candidate_count,
                candidate_count
            );
            candidate_count as u32
        } else {
            config.active_set_size
        };

        // Create a modified config with the adjusted active set size
        let mut adjusted_config = config.clone();
        adjusted_config.active_set_size = effective_active_set_size;

        // Select algorithm based on configuration
        let algorithm: Box<dyn ElectionAlgorithm> = match config.algorithm {
            AlgorithmType::SequentialPhragmen => Box::new(SequentialPhragmen),
            AlgorithmType::ParallelPhragmen => Box::new(crate::algorithms::parallel_phragmen::ParallelPhragmen),
            AlgorithmType::MultiPhase => Box::new(crate::algorithms::multi_phase::MultiPhase),
        };

        // Apply overrides if present
        let mut modified_data = data.clone();
        if let Some(ref overrides) = config.overrides {
            self.apply_overrides(&mut modified_data, overrides)?;
        }

        // Execute algorithm with adjusted config
        let result = algorithm.execute(&modified_data, &adjusted_config)?;

        // Validate result against adjusted config
        self.validate_result(&result, &adjusted_config)?;

        // Generate diagnostics if requested
        let result = if generate_diagnostics {
            let diagnostics_gen = DiagnosticsGenerator::new();
            match diagnostics_gen.generate(&result, &modified_data) {
                Ok(diagnostics) => result.with_diagnostics(diagnostics),
                Err(e) => {
                    // Log error but don't fail the election
                    eprintln!("Warning: Failed to generate diagnostics: {}", e);
                    result
                }
            }
        } else {
            result
        };

        Ok(result)
    }

    /// Apply parameter overrides to election data
    fn apply_overrides(
        &self,
        data: &mut ElectionData,
        overrides: &crate::models::election_overrides::ElectionOverrides,
    ) -> Result<(), ElectionError> {
        // Apply candidate stake overrides
        for (account_id, stake) in &overrides.candidate_stakes {
            if let Some(candidate) = data.candidates.iter_mut().find(|c| c.account_id == *account_id) {
                candidate.stake = *stake;
            }
        }

        // Apply nominator stake overrides
        for (account_id, stake) in &overrides.nominator_stakes {
            if let Some(nominator) = data.nominators.iter_mut().find(|n| n.account_id == *account_id) {
                nominator.stake = *stake;
            }
        }

        // Apply voting edge modifications
        for edge_mod in &overrides.voting_edges {
            match edge_mod.action {
                crate::models::election_overrides::EdgeAction::Add => {
                    if let Some(nominator) = data.nominators.iter_mut().find(|n| n.account_id == edge_mod.nominator_id) {
                        nominator.add_target(edge_mod.candidate_id.clone());
                    }
                }
                crate::models::election_overrides::EdgeAction::Remove => {
                    if let Some(nominator) = data.nominators.iter_mut().find(|n| n.account_id == edge_mod.nominator_id) {
                        nominator.remove_target(&edge_mod.candidate_id);
                    }
                }
                crate::models::election_overrides::EdgeAction::Modify => {
                    // Modify is similar to remove + add
                    if let Some(nominator) = data.nominators.iter_mut().find(|n| n.account_id == edge_mod.nominator_id) {
                        nominator.remove_target(&edge_mod.candidate_id);
                        nominator.add_target(edge_mod.candidate_id.clone());
                    }
                }
            }
        }

        Ok(())
    }

    /// Validate election result
    fn validate_result(
        &self,
        result: &ElectionResult,
        config: &ElectionConfiguration,
    ) -> Result<(), ElectionError> {
        // Check that number of selected validators matches active set size
        if result.selected_validators.len() != config.active_set_size as usize {
            return Err(ElectionError::ValidationError {
                message: format!(
                    "Result has {} validators but expected {}",
                    result.selected_validators.len(),
                    config.active_set_size
                ),
                field: Some("selected_validators".to_string()),
            });
        }

        // Check that total stake matches
        let total_allocated: u128 = result.stake_distribution.iter().map(|a| a.amount).sum();
        if total_allocated != result.total_stake {
            return Err(ElectionError::ValidationError {
                message: format!(
                    "Stake distribution total {} doesn't match total stake {}",
                    total_allocated, result.total_stake
                ),
                field: Some("stake_distribution".to_string()),
            });
        }

        Ok(())
    }
}

impl Default for ElectionEngine {
    fn default() -> Self {
        Self::new()
    }
}


