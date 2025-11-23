//! Performance test: Concurrent election execution

use offline_election::engine::ElectionEngine;
use offline_election::models::election_config::ElectionConfiguration;
use offline_election::types::AlgorithmType;
use crate::common::data_generator::generate_large_scale_election_data;
use crate::common::benchmark_utils::measure_execution_time;
use tokio::task;

#[test]
#[ignore] // May be slow
async fn test_concurrent_election_execution() {
    const CANDIDATE_COUNT: usize = 100;
    const NOMINATOR_COUNT: usize = 1_000;
    const CONCURRENT_TASKS: usize = 10;
    
    println!("Testing {} concurrent election executions", CONCURRENT_TASKS);
    
    let engine = ElectionEngine::new();
    let config = ElectionConfiguration {
        active_set_size: 50,
        algorithm: AlgorithmType::SequentialPhragmen,
        overrides: None,
        block_number: None,
    };
    
    // Generate election data
    let election_data = generate_large_scale_election_data(
        CANDIDATE_COUNT,
        NOMINATOR_COUNT,
        AlgorithmType::SequentialPhragmen,
    );
    
    // Spawn concurrent tasks
    let mut handles = Vec::new();
    
    for i in 0..CONCURRENT_TASKS {
        // Create new engine instance for each task (engine is stateless)
        let config_clone = config.clone();
        let data_clone = election_data.clone();
        
        let handle = task::spawn(async move {
            let engine = ElectionEngine::new();
            let (result, duration) = measure_execution_time(|| {
                engine.execute(&config_clone, &data_clone)
            });
            
            (i, result, duration)
        });
        
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    let mut results = Vec::new();
    for handle in handles {
        let (task_id, result, duration) = handle.await.unwrap();
        results.push((task_id, result, duration));
    }
    
    // Verify all executions succeeded
    for (task_id, result, duration) in &results {
        assert!(result.is_ok(), "Concurrent execution {} should succeed", task_id);
        let election_result = result.as_ref().unwrap();
        assert_eq!(election_result.selected_validators.len(), 50);
        println!("Task {} completed in {:?}", task_id, duration);
    }
    
    // Verify all results are identical (deterministic)
    let first_result = results[0].1.as_ref().unwrap();
    for (task_id, result, _) in &results[1..] {
        let current_result = result.as_ref().unwrap();
        
        // Compare selected validators
        let first_validators: Vec<&String> = first_result.selected_validators.iter()
            .map(|v| &v.account_id)
            .collect();
        let current_validators: Vec<&String> = current_result.selected_validators.iter()
            .map(|v| &v.account_id)
            .collect();
        
        assert_eq!(
            first_validators, current_validators,
            "Concurrent execution {} should produce identical results",
            task_id
        );
    }
    
    println!("✓ All {} concurrent executions completed successfully and produced identical results", CONCURRENT_TASKS);
}

