#!/usr/bin/env rust-script
//! Comprehensive benchmark runner for TopLang
//!
//! Runs benchmarks across multiple VM implementations and compares with Python

use std::path::Path;
use std::process::Command;
use std::time::Instant;

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct BenchmarkResult {
    name: String,
    vm_type: String,
    duration_ms: u128,
    success: bool,
    output: String,
}

#[derive(Debug)]
struct BenchmarkConfig {
    name: String,
    file: String,
    runs: usize,
}

fn main() {
    println!("╔═══════════════════════════════════════════════════╗");
    println!("║    TopLang Performance Benchmark Suite           ║");
    println!("╚═══════════════════════════════════════════════════╝");
    println!();

    let benchmarks = vec![
        BenchmarkConfig {
            name: "Fibonacci".to_string(),
            file: "benchmarks/toplang/fibonacci.top".to_string(),
            runs: 5,
        },
        BenchmarkConfig {
            name: "Primes".to_string(),
            file: "benchmarks/toplang/primes.top".to_string(),
            runs: 5,
        },
        BenchmarkConfig {
            name: "Array Sum".to_string(),
            file: "benchmarks/toplang/array_sum.top".to_string(),
            runs: 5,
        },
        BenchmarkConfig {
            name: "Nested Loops".to_string(),
            file: "benchmarks/toplang/nested_loops.top".to_string(),
            runs: 5,
        },
        BenchmarkConfig {
            name: "Factorial".to_string(),
            file: "benchmarks/toplang/factorial.top".to_string(),
            runs: 5,
        },
    ];

    let vm_types = vec![
        ("Interpreter", vec![]),
        ("Bytecode VM", vec!["--bytecode"]),
        ("NaN Boxing", vec!["--bytecode", "--nanbox"]),
    ];

    let mut all_results = Vec::new();

    for bench in &benchmarks {
        if !Path::new(&bench.file).exists() {
            println!("⚠️  Skipping {} (file not found)", bench.name);
            continue;
        }

        println!("📊 Benchmarking: {}", bench.name);
        println!("   File: {}", bench.file);
        println!("   Runs: {}", bench.runs);
        println!();

        for (vm_name, flags) in &vm_types {
            let results = run_benchmark(bench, vm_name, flags);

            if !results.is_empty() {
                let avg_ms =
                    results.iter().map(|r| r.duration_ms).sum::<u128>() / results.len() as u128;
                let min_ms = results.iter().map(|r| r.duration_ms).min().unwrap();
                let max_ms = results.iter().map(|r| r.duration_ms).max().unwrap();

                println!(
                    "   {:14} avg: {:4}ms  min: {:4}ms  max: {:4}ms",
                    vm_name, avg_ms, min_ms, max_ms
                );

                all_results.push((bench.name.clone(), vm_name.to_string(), avg_ms));
            }
        }
        println!();
    }

    // Print summary table
    print_summary_table(&all_results, &benchmarks);

    // Print speedup analysis
    print_speedup_analysis(&all_results, &benchmarks);
}

fn run_benchmark(config: &BenchmarkConfig, vm_type: &str, flags: &[&str]) -> Vec<BenchmarkResult> {
    let mut results = Vec::new();

    for _ in 0..config.runs {
        let start = Instant::now();

        let mut cmd = Command::new("./target/release/topc");
        cmd.arg(&config.file);
        for flag in flags {
            cmd.arg(flag);
        }

        let output = cmd.output();
        let duration = start.elapsed();

        match output {
            Ok(out) => {
                results.push(BenchmarkResult {
                    name: config.name.clone(),
                    vm_type: vm_type.to_string(),
                    duration_ms: duration.as_millis(),
                    success: out.status.success(),
                    output: String::from_utf8_lossy(&out.stdout).to_string(),
                });
            }
            Err(e) => {
                eprintln!("   ⚠️  Failed to run {}: {}", vm_type, e);
                return vec![];
            }
        }
    }

    results
}

fn print_summary_table(results: &[(String, String, u128)], benchmarks: &[BenchmarkConfig]) {
    println!("╔═══════════════════════════════════════════════════╗");
    println!("║              Summary Results (ms)                 ║");
    println!("╚═══════════════════════════════════════════════════╝");
    println!();
    println!("┌─────────────────┬────────────┬────────────┬────────────┐");
    println!("│ Benchmark       │ Interpreter│ Bytecode   │ NaN Boxing │");
    println!("├─────────────────┼────────────┼────────────┼────────────┤");

    for bench in benchmarks {
        let interp = results
            .iter()
            .find(|(n, v, _)| n == &bench.name && v == "Interpreter")
            .map(|(_, _, d)| *d);
        let bytecode = results
            .iter()
            .find(|(n, v, _)| n == &bench.name && v == "Bytecode VM")
            .map(|(_, _, d)| *d);
        let nanbox = results
            .iter()
            .find(|(n, v, _)| n == &bench.name && v == "NaN Boxing")
            .map(|(_, _, d)| *d);

        print!("│ {:15} │", bench.name);
        print!(" {:9} │", format_opt_ms(interp));
        print!(" {:9} │", format_opt_ms(bytecode));
        print!(" {:9} │", format_opt_ms(nanbox));
        println!();
    }

    println!("└─────────────────┴────────────┴────────────┴────────────┘");
    println!();
}

fn print_speedup_analysis(results: &[(String, String, u128)], benchmarks: &[BenchmarkConfig]) {
    println!("╔═══════════════════════════════════════════════════╗");
    println!("║            Speedup Analysis                       ║");
    println!("╚═══════════════════════════════════════════════════╝");
    println!();
    println!("┌─────────────────┬──────────────┬──────────────┐");
    println!("│ Benchmark       │ Bytecode/Int │ NanBox/Byte  │");
    println!("├─────────────────┼──────────────┼──────────────┤");

    for bench in benchmarks {
        let interp = results
            .iter()
            .find(|(n, v, _)| n == &bench.name && v == "Interpreter")
            .map(|(_, _, d)| *d);
        let bytecode = results
            .iter()
            .find(|(n, v, _)| n == &bench.name && v == "Bytecode VM")
            .map(|(_, _, d)| *d);
        let nanbox = results
            .iter()
            .find(|(n, v, _)| n == &bench.name && v == "NaN Boxing")
            .map(|(_, _, d)| *d);

        print!("│ {:15} │", bench.name);

        let speedup1 = if let (Some(i), Some(b)) = (interp, bytecode) {
            format!("{:.2}x", i as f64 / b as f64)
        } else {
            "N/A".to_string()
        };

        let speedup2 = if let (Some(b), Some(n)) = (bytecode, nanbox) {
            format!("{:.2}x", b as f64 / n as f64)
        } else {
            "N/A".to_string()
        };

        print!(" {:12} │", speedup1);
        print!(" {:12} │", speedup2);
        println!();
    }

    println!("└─────────────────┴──────────────┴──────────────┘");
    println!();

    // Calculate averages
    let avg_bytecode_speedup =
        calculate_avg_speedup(results, benchmarks, "Interpreter", "Bytecode VM");
    let avg_nanbox_speedup =
        calculate_avg_speedup(results, benchmarks, "Bytecode VM", "NaN Boxing");
    let total_speedup = calculate_avg_speedup(results, benchmarks, "Interpreter", "NaN Boxing");

    println!("📈 Average Speedups:");
    println!(
        "   Bytecode VM vs Interpreter: {:.2}x",
        avg_bytecode_speedup
    );
    println!("   NaN Boxing vs Bytecode:     {:.2}x", avg_nanbox_speedup);
    println!(
        "   NaN Boxing vs Interpreter:  {:.2}x (total)",
        total_speedup
    );
    println!();
}

fn calculate_avg_speedup(
    results: &[(String, String, u128)],
    benchmarks: &[BenchmarkConfig],
    baseline: &str,
    target: &str,
) -> f64 {
    let mut speedups = Vec::new();

    for bench in benchmarks {
        let base = results
            .iter()
            .find(|(n, v, _)| n == &bench.name && v == baseline)
            .map(|(_, _, d)| *d);
        let tgt = results
            .iter()
            .find(|(n, v, _)| n == &bench.name && v == target)
            .map(|(_, _, d)| *d);

        if let (Some(b), Some(t)) = (base, tgt) {
            speedups.push(b as f64 / t as f64);
        }
    }

    if speedups.is_empty() {
        0.0
    } else {
        speedups.iter().sum::<f64>() / speedups.len() as f64
    }
}

fn format_opt_ms(ms: Option<u128>) -> String {
    match ms {
        Some(m) => format!("{}ms", m),
        None => "N/A".to_string(),
    }
}
