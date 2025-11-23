//! Edge case test: Algorithm convergence validation

use offline_election::engine::ElectionEngine;
use offline_election::models::election_config::ElectionConfiguration;
use offline_election::models::election_data::ElectionData;
use offline_election::models::{Nominator, ValidatorCandidate};
use offline_election::types::AlgorithmType;
use crate::common::assertions::assert_election_result_valid;

#[test]
fn test_algorithm_convergence_small_dataset() {
    let engine = ElectionEngine::new();
    let mut election_data = ElectionData::new();
    
    // Create a small but valid dataset
    let candidate1 = ValidatorCandidate::new(
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
        1_000_000_000,
    );
    let candidate2 = ValidatorCandidate::new(
        "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(),
        2_000_000_000,
    );
    let candidate3 = ValidatorCandidate::new(
        "5FLSigC9HGRKVhB9F7BqHjXJxZJxZJxZJxZJxZJxZJxZJxZJxZ".to_string(),
        3_000_000_000,
    );
    
    election_data.add_candidate(candidate1).unwrap();
    election_data.add_candidate(candidate2).unwrap();
    election_data.add_candidate(candidate3).unwrap();
    
    let nominator = Nominator {
        account_id: "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY".to_string(),
        stake: 10_000_000_000,
        targets: vec![
            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
            "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(),
            "5FLSigC9HGRKVhB9F7BqHjXJxZJxZJxZJxZJxZJxZJxZJxZJxZ".to_string(),
        ],
        metadata: None,
    };
    
    election_data.add_nominator(nominator).unwrap();
    
    let config = ElectionConfiguration {
        active_set_size: 3,
        algorithm: AlgorithmType::SequentialPhragmen,
        overrides: None,
        block_number: None,
    };
    
    // Algorithm should converge and produce valid results
    let result = engine.execute(&config, &election_data)
        .expect("Algorithm should converge and produce results");
    
    // Validate result structure
    assert_election_result_valid(&result);
    
    // Verify algorithm converged (result has expected number of validators)
    assert_eq!(result.selected_validators.len(), 3, "Should select all 3 candidates");
    assert!(result.total_stake > 0, "Total stake should be positive");
    
    // Verify stake distribution is valid
    let total_allocated: u128 = result.stake_distribution.iter()
        .map(|alloc| alloc.amount)
        .sum();
    
    // Total allocated should not exceed total stake
    assert!(
        total_allocated <= result.total_stake,
        "Total allocated stake {} should not exceed total stake {}",
        total_allocated,
        result.total_stake
    );
    
    println!("✓ Algorithm converged successfully with valid results");
}

#[test]
fn test_algorithm_convergence_large_dataset() {
    use crate::common::data_generator::generate_synthetic_election_data;
    
    let engine = ElectionEngine::new();
    let election_data = generate_synthetic_election_data(100, 1_000);
    
    let config = ElectionConfiguration {
        active_set_size: 50,
        algorithm: AlgorithmType::SequentialPhragmen,
        overrides: None,
        block_number: None,
    };
    
    // Algorithm should converge even with larger datasets
    let result = engine.execute(&config, &election_data)
        .expect("Algorithm should converge with large dataset");
    
    // Validate result structure
    assert_election_result_valid(&result);
    
    // Verify algorithm converged
    assert_eq!(result.selected_validators.len(), 50);
    assert!(result.total_stake > 0);
    
    println!("✓ Algorithm converged successfully with large dataset");
}

