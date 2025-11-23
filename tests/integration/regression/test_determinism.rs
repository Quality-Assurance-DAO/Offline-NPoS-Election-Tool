//! Regression test: Deterministic results across multiple runs

use offline_election::engine::ElectionEngine;
use offline_election::models::election_config::ElectionConfiguration;
use offline_election::types::AlgorithmType;
use crate::common::data_generator::generate_synthetic_election_data;
use crate::common::assertions::compare_results_exact_match;

#[test]
fn test_deterministic_results_multiple_runs() {
    const RUNS: usize = 10;
    const CANDIDATE_COUNT: usize = 50;
    const NOMINATOR_COUNT: usize = 500;
    
    println!("Testing deterministic results across {} runs", RUNS);
    
    let election_data = generate_synthetic_election_data(CANDIDATE_COUNT, NOMINATOR_COUNT);
    let engine = ElectionEngine::new();
    let config = ElectionConfiguration {
        active_set_size: 25,
        algorithm: AlgorithmType::SequentialPhragmen,
        overrides: None,
        block_number: None,
    };
    
    // Run election multiple times
    let mut results = Vec::new();
    for run in 0..RUNS {
        let result = engine.execute(&config, &election_data)
            .expect(&format!("Election run {} should succeed", run));
        results.push(result);
    }
    
    // Compare all results - they should be identical
    let first_result = &results[0];
    for (run, result) in results.iter().enumerate().skip(1) {
        compare_results_exact_match(result, first_result)
            .expect(&format!("Run {} should produce identical results to run 0", run));
    }
    
    println!("✓ All {} runs produced identical results", RUNS);
}

#[test]
fn test_deterministic_results_different_instances() {
    const CANDIDATE_COUNT: usize = 30;
    const NOMINATOR_COUNT: usize = 300;
    
    println!("Testing deterministic results with different engine instances");
    
    let election_data = generate_synthetic_election_data(CANDIDATE_COUNT, NOMINATOR_COUNT);
    let config = ElectionConfiguration {
        active_set_size: 15,
        algorithm: AlgorithmType::SequentialPhragmen,
        overrides: None,
        block_number: None,
    };
    
    // Create multiple engine instances
    let engine1 = ElectionEngine::new();
    let engine2 = ElectionEngine::new();
    let engine3 = ElectionEngine::new();
    
    // Execute with each engine
    let result1 = engine1.execute(&config, &election_data)
        .expect("Election with engine1 should succeed");
    let result2 = engine2.execute(&config, &election_data)
        .expect("Election with engine2 should succeed");
    let result3 = engine3.execute(&config, &election_data)
        .expect("Election with engine3 should succeed");
    
    // All results should be identical
    compare_results_exact_match(&result1, &result2)
        .expect("Engine1 and engine2 should produce identical results");
    compare_results_exact_match(&result2, &result3)
        .expect("Engine2 and engine3 should produce identical results");
    
    println!("✓ All engine instances produced identical results");
}

