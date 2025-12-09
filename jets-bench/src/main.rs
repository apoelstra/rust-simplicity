//! Script to parse results from benchmarks folder and generate worst
//! case results for each jets
//!
//! We rely on criterion directory structure in 0.4.0

use std::{collections::BTreeMap, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct MeasurementData {
    // The timestamp of when these measurements were saved.
    pub datetime: String,
    // The number of iterations in each sample
    pub iterations: Vec<f64>,
    // The measured values from each sample
    pub values: Vec<f64>,
    // The average values from each sample, ie. values / iterations
    pub avg_values: Vec<f64>,
    // The statistical estimates from this run
    pub estimates: Estimates,
}

#[derive(Debug, Deserialize)]
pub struct Estimates {
    pub mean: Estimate,
    pub median: Estimate,
    pub median_abs_dev: Estimate,
    pub slope: Option<Estimate>,
    pub std_dev: Estimate,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Estimate {
    /// The confidence interval for this estimate
    pub confidence_interval: ConfidenceInterval,
    /// The point estimate for this estimate
    pub point_estimate: f64,
    /// The standard error of this estimate
    pub standard_error: f64,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct ConfidenceInterval {
    pub confidence_level: f64,
    pub lower_bound: f64,
    pub upper_bound: f64,
}

#[derive(Clone, PartialEq, Serialize, Debug)]
pub struct BenchResult {
    best_name: String,
    best_value: f64,
    median_name: String,
    median_value: f64,
    worst_name: String,
    worst_value: f64,
}

fn main() -> Result<(), String> {
    // Simplicity root is the grand-parent directory of this file
    let mut bench_results = BTreeMap::new();
    let simplicity_root = std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    // Obtain the path to the criterion directory
    let criterion_dir = simplicity_root
        .join("jets-bench")
        .join("target")
        .join("criterion")
        .join("data")
        .join("main");

    // enumerate all folders in criterion directory
    let jets_iter = match std::fs::read_dir(criterion_dir) {
        Ok(dir) => dir.map(|e| e.unwrap().path()),
        Err(e) => {
            return Err(format!(
                "Failed to open criterion data directory (did you run `cargo criterion`?): {}",
                e
            ))
        }
    };

    for jet in jets_iter {
        let jet_name = jet.file_name().unwrap().to_str().unwrap().to_string();
        let mut estimates = vec![];
        // Iterate over all files in jets directory
        for jet_folder in std::fs::read_dir(&jet).expect(&format!("reading directory {}", jet.to_string_lossy())).map(|e| e.unwrap().path()) {
            let dist_name = jet_folder.file_name().unwrap().to_string_lossy().to_string();
            // Parse the file
            // Filter all files that start with measurements
            let mut measurement_files = jet_folder
                .read_dir()
                .unwrap()
                .map(|e| e.unwrap().path())
                .filter(|e| {
                    e.file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .starts_with("measurement_")
                })
                .collect::<Vec<_>>();
            // We expect only one file
            assert_eq!(
                measurement_files.len(),
                1,
                "Found more than one measurement file for {} / {}; please delete the target/criterion directory before running the benchmarks.",
                jet_name, dist_name,
            );
            let file = measurement_files.pop().unwrap();

            let file = std::fs::File::open(file).unwrap();
            let data: MeasurementData = serde_cbor::from_reader(file).unwrap();

            println!("{} {}: {}",
                jet_name,
                dist_name,
                data.estimates.mean.point_estimate,
            );

            estimates.push((
                data.estimates.mean.point_estimate,
                dist_name,
            ));
        }
        estimates.sort_by_key(|(w, _name)| (*w * 1_000_000.0) as u64);

        let n_ests = estimates.len();
        bench_results.insert(
            jet.file_name().unwrap().to_str().unwrap().to_string(),
            BenchResult {
                best_name: estimates[0].1.clone(),
                best_value: estimates[0].0,
                worst_name: estimates[n_ests - 1].1.clone(),
                worst_value: estimates[n_ests - 1].0,
                median_name: estimates[n_ests / 2].1.clone(),
                median_value: estimates[n_ests / 2].0,
            },
        );
    }

    // Haskell requires debug formatted strings.
    let bitcoin_bench_results: BTreeMap<_, _> = bench_results
        .iter()
        .filter_map(|(k, v)| {
            let jet = k.strip_prefix("bitcoin_")?;
            let debug_str = simplicity::jet::Bitcoin::from_str(jet).unwrap();
            Some((format!("{:?}", debug_str), v))
        })
        .collect();

    let elements_bench_results: BTreeMap<_, _> = bench_results
        .iter()
        .filter_map(|(k, v)| {
            let jet = k.strip_prefix("elements_")?;
            let debug_str = simplicity::jet::Elements::from_str(jet).unwrap();
            Some((format!("{:?}", debug_str), v))
        })
        .collect();

    // Create a file output json file with jet_bench_timestamp.json
    if bitcoin_bench_results.is_empty() {
        println!("Found no bitcoin jet results.");
    } else {
        let timestamp = chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S");
        let output_path = simplicity_root
            .join("data")
            .join(format!("bitcoin_jet_bench_{}.json", timestamp));
        let mut output_file = std::fs::File::create(&output_path).unwrap();
        serde_json::to_writer_pretty(&mut output_file, &bitcoin_bench_results).unwrap();
        println!("Output Bitcoin jet results to {}", output_path.to_string_lossy());
    }

    // Create a file output json file with jet_bench_timestamp.json
    if elements_bench_results.is_empty() {
        println!("Found no elements jet results.");
    } else {
        let timestamp = chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S");
        let output_path = simplicity_root
            .join("data")
            .join(format!("elements_jet_bench_{}.json", timestamp));
        let mut output_file = std::fs::File::create(&output_path).unwrap();
        serde_json::to_writer_pretty(&mut output_file, &elements_bench_results).unwrap();
        println!("Output Elements jet results to {}", output_path.to_string_lossy());
    }

    Ok(())
}
