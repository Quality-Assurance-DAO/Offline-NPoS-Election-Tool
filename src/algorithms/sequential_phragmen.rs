//! Sequential Phragmen algorithm implementation using sp-npos-elections
//! 
//! Note: This implementation integrates with Substrate's sp-npos-elections crate.
//! The exact API may need adjustment based on the version of sp-npos-elections used.

use crate::algorithms::trait_def::ElectionAlgorithm;
use crate::error::ElectionError;
use crate::models::election_config::ElectionConfiguration;
use crate::models::election_data::ElectionData;
use crate::models::election_result::{ElectionResult, SelectedValidator, StakeAllocation, ExecutionMetadata};

/// Sequential Phragmen algorithm implementation
pub struct SequentialPhragmen;

impl ElectionAlgorithm for SequentialPhragmen {
    fn execute(
        &self,
        data: &ElectionData,
        config: &ElectionConfiguration,
    ) -> Result<ElectionResult, ElectionError> {
        // Convert our data models to sp-npos-elections format
        // Note: The exact API of sp-npos-elections may vary by version
        // This structure provides the integration point and may need adjustment
        
        let candidate_count = data.candidates.len();
        let voter_count = data.nominators.len();

        if candidate_count == 0 || voter_count == 0 {
            return Err(ElectionError::ValidationError {
                message: "Cannot run election with zero candidates or voters".to_string(),
                field: None,
            });
        }

        // Build candidate list with indices and stakes
        let candidates: Vec<(usize, u128)> = data
            .candidates
            .iter()
            .enumerate()
            .map(|(idx, c)| (idx, c.stake))
            .collect();

        // Build voter list: (voter_index, stake, [(candidate_index, weight)])
        let mut voters: Vec<(usize, u128, Vec<(usize, u128)>)> = Vec::new();
        let candidate_id_to_index: std::collections::HashMap<&String, usize> = data
            .candidates
            .iter()
            .enumerate()
            .map(|(idx, c)| (&c.account_id, idx))
            .collect();

        for (voter_idx, nominator) in data.nominators.iter().enumerate() {
            let mut targets = Vec::new();
            for target_id in &nominator.targets {
                if let Some(&candidate_idx) = candidate_id_to_index.get(target_id) {
                    // Use equal weight distribution for now
                    // In practice, this might be proportional to stake
                    targets.push((candidate_idx, nominator.stake));
                }
            }
            if !targets.is_empty() {
                voters.push((voter_idx, nominator.stake, targets));
            }
        }

        // Run sequential phragmen algorithm using sp-npos-elections
        // Note: The exact function signature may vary - this is a placeholder structure
        let solution = sp_npos_elections::seq_phragmen(
            config.active_set_size as usize,
            candidates,
            voters,
        )
        .map_err(|e| ElectionError::AlgorithmError {
            message: format!("Sequential phragmen algorithm failed: {:?}", e),
            algorithm: crate::types::AlgorithmType::SequentialPhragmen,
        })?;

        // Convert results back to our format
        let mut selected_validators = Vec::new();
        let mut stake_distribution = Vec::new();
        let mut total_stake = 0u128;

        for (rank, (candidate_idx, _)) in solution.winners.iter().enumerate() {
            let candidate = &data.candidates[*candidate_idx];
            let mut total_backing = 0u128;
            let mut nominator_count = 0u32;

            // Calculate stake distribution from assignments
            for assignment in &solution.assignment {
                let (voter_idx, assignments) = assignment;
                for (c_idx, portion) in assignments {
                    if *c_idx == *candidate_idx {
                        let nominator = &data.nominators[*voter_idx];
                        let amount = (nominator.stake as f64 * portion) as u128;
                        total_backing += amount;
                        nominator_count += 1;

                        stake_distribution.push(StakeAllocation {
                            nominator_id: nominator.account_id.clone(),
                            validator_id: candidate.account_id.clone(),
                            amount,
                            proportion: *portion,
                        });
                    }
                }
            }

            total_stake += total_backing;

            selected_validators.push(SelectedValidator {
                account_id: candidate.account_id.clone(),
                total_backing_stake: total_backing,
                nominator_count,
                rank: Some(rank as u32 + 1),
            });
        }

        // Calculate total stake from all nominators
        let total_nominator_stake: u128 = data.nominators.iter().map(|n| n.stake).sum();

        Ok(ElectionResult {
            selected_validators,
            stake_distribution,
            total_stake: total_nominator_stake.max(total_stake),
            algorithm_used: crate::types::AlgorithmType::SequentialPhragmen,
            execution_metadata: ExecutionMetadata {
                block_number: config.block_number,
                execution_timestamp: Some(chrono::Utc::now().to_rfc3339()),
                data_source: None,
            },
        })
    }

    fn name(&self) -> &'static str {
        "sequential-phragmen"
    }
}
