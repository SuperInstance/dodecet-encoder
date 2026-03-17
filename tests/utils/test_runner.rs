// Test Runner Module
// Provides utilities for running comprehensive test suites

use std::process::Command;
use std::time::Instant;

pub struct TestRunner {
    test_results: Vec<TestResult>,
}

pub struct TestResult {
    suite_name: String,
    test_name: String,
    passed: bool,
    duration_ms: u64,
}

impl TestRunner {
    pub fn new() -> Self {
        Self {
            test_results: Vec::new(),
        }
    }

    /// Run all integration tests
    pub fn run_integration_tests(&mut self) -> Result<(), String> {
        println!("\n" + "=".repeat(80));
        println!("RUNNING INTEGRATION TESTS");
        println!("=".repeat(80));

        let start = Instant::now();

        // Run cargo test for integration tests
        let output = Command::new("cargo")
            .args(&["test", "--test", "wasm_integration"])
            .output()
            .map_err(|e| format!("Failed to run integration tests: {}", e))?;

        let duration = start.elapsed();

        let passed = output.status.success();
        println!("\nIntegration Tests: {} ({:.2}s)",
                 if passed { "✓ PASSED" } else { "✗ FAILED" },
                 duration.as_secs_f64());

        self.test_results.push(TestResult {
            suite_name: "Integration".to_string(),
            test_name: "WASM Integration".to_string(),
            passed,
            duration_ms: duration.as_millis() as u64,
        });

        if passed {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    /// Run all performance benchmarks
    pub fn run_performance_benchmarks(&mut self) -> Result<(), String> {
        println!("\n" + "=".repeat(80));
        println!("RUNNING PERFORMANCE BENCHMARKS");
        println!("=".repeat(80));

        let start = Instant::now();

        // Run cargo test for benchmarks
        let output = Command::new("cargo")
            .args(&["test", "--test", "benchmarks", "--", "--nocapture"])
            .output()
            .map_err(|e| format!("Failed to run benchmarks: {}", e))?;

        let duration = start.elapsed();

        let passed = output.status.success();
        println!("\nPerformance Benchmarks: {} ({:.2}s)",
                 if passed { "✓ PASSED" } else { "✗ FAILED" },
                 duration.as_secs_f64());

        self.test_results.push(TestResult {
            suite_name: "Performance".to_string(),
            test_name: "Benchmarks".to_string(),
            passed,
            duration_ms: duration.as_millis() as u64,
        });

        if passed {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    /// Run all compatibility tests
    pub fn run_compatibility_tests(&mut self) -> Result<(), String> {
        println!("\n" + "=".repeat(80));
        println!("RUNNING COMPATIBILITY TESTS");
        println!("=".repeat(80));

        let start = Instant::now();

        // Run cargo test for compatibility tests
        let output = Command::new("cargo")
            .args(&["test", "--test", "browser_tests"])
            .output()
            .map_err(|e| format!("Failed to run compatibility tests: {}", e))?;

        let duration = start.elapsed();

        let passed = output.status.success();
        println!("\nCompatibility Tests: {} ({:.2}s)",
                 if passed { "✓ PASSED" } else { "✗ FAILED" },
                 duration.as_secs_f64());

        self.test_results.push(TestResult {
            suite_name: "Compatibility".to_string(),
            test_name: "Browser Tests".to_string(),
            passed,
            duration_ms: duration.as_millis() as u64,
        });

        if passed {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    /// Run WASM package validation tests
    pub fn run_wasm_package_tests(&mut self) -> Result<(), String> {
        println!("\n" + "=".repeat(80));
        println!("RUNNING WASM PACKAGE TESTS");
        println!("=".repeat(80));

        let start = Instant::now();

        // Run cargo test for WASM package tests
        let output = Command::new("cargo")
            .args(&["test", "--test", "wasm_package_tests"])
            .output()
            .map_err(|e| format!("Failed to run WASM package tests: {}", e))?;

        let duration = start.elapsed();

        let passed = output.status.success();
        println!("\nWASM Package Tests: {} ({:.2}s)",
                 if passed { "✓ PASSED" } else { "✗ FAILED" },
                 duration.as_secs_f64());

        self.test_results.push(TestResult {
            suite_name: "WASM Package".to_string(),
            test_name: "Validation".to_string(),
            passed,
            duration_ms: duration.as_millis() as u64,
        });

        if passed {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    /// Run all test suites
    pub fn run_all_tests(&mut self) -> Result<(), String> {
        println!("\n" + "=".repeat(80));
        println!("COMPREHENSIVE TEST SUITE - DODECET ENCODER WEEK 2");
        println!("Testing & Validation");
        println!("=".repeat(80));

        let total_start = Instant::now();

        // Run all test suites
        let results = vec![
            self.run_integration_tests(),
            self.run_performance_benchmarks(),
            self.run_compatibility_tests(),
            self.run_wasm_package_tests(),
        ];

        let total_duration = total_start.elapsed();

        // Generate summary report
        self.generate_summary_report(total_duration);

        // Check if all tests passed
        let all_passed = results.iter().all(|r| r.is_ok());

        if all_passed {
            Ok(())
        } else {
            Err("Some test suites failed".to_string())
        }
    }

    /// Generate summary report
    fn generate_summary_report(&self, total_duration: std::time::Duration) {
        println!("\n" + "=".repeat(80));
        println!("TEST SUMMARY REPORT");
        println!("=".repeat(80));

        println!("\n## Test Suites");
        for result in &self.test_results {
            let status = if result.passed { "✓ PASSED" } else { "✗ FAILED" };
            println!("{} - {}: {} ({:.2}s)",
                     result.suite_name,
                     result.test_name,
                     status,
                     result.duration_ms as f64 / 1000.0);
        }

        let passed_count = self.test_results.iter().filter(|r| r.passed).count();
        let total_count = self.test_results.len();

        println!("\n## Overall Results");
        println!("Total Test Suites: {}/{} passed", passed_count, total_count);
        println!("Total Duration: {:.2}s", total_duration.as_secs_f64());

        if passed_count == total_count {
            println!("\n✓ ALL TESTS PASSED - System is production-ready!");
        } else {
            println!("\n✗ SOME TESTS FAILED - Review required");
        }

        println!("\n" + "=".repeat(80));
    }
}

impl Default for TestRunner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test_runner_tests {
    use super::*;

    #[test]
    fn test_test_runner_creation() {
        let runner = TestRunner::new();
        assert_eq!(runner.test_results.len(), 0);
    }

    #[test]
    fn test_test_runner_default() {
        let runner = TestRunner::default();
        assert_eq!(runner.test_results.len(), 0);
    }
}
