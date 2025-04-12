mod benchmark_results;

use benchmark_results::{BenchMarkResult, BenchMarkResults};
use luau_parser::prelude::Parser;
use std::{
    fs::{self, File},
    io::{self, Read},
    path::Path,
    time::{Duration, Instant},
};

const ITERATIONS: usize = 1000;
const WARM_UP_ITERATIONS: usize = 50;

fn bench<'a>(parser: &mut Parser<'a>, uri: &str, content: &'a str) -> BenchMarkResult {
    parser.set_input(content);

    for _ in 0..WARM_UP_ITERATIONS {
        let _ = parser.parse(uri);
    }

    let mut total = Duration::default();
    let mut lowest = Duration::MAX;
    let mut highest = Duration::default();

    for _ in 0..ITERATIONS {
        let start = Instant::now();
        let _ = parser.parse(uri);
        let elapsed = start.elapsed();

        if elapsed < lowest {
            lowest = elapsed;
        } else if elapsed > highest {
            highest = elapsed;
        }

        total += elapsed;
    }

    BenchMarkResult {
        lowest,
        highest,
        total,
    }
}

fn process_files(src_dir: &Path, benchmark_results: &mut BenchMarkResults) -> io::Result<()> {
    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            process_files(&path, benchmark_results)?;
        } else if path.is_file() {
            let mut file = File::open(&path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;

            let mut parser = Parser::new(&content);
            let time_taken = bench(&mut parser, path.to_string_lossy().as_ref(), &content);

            benchmark_results.insert(path, time_taken);
        }
    }

    Ok(())
}

// (average, difference_1, difference_2)
fn calculate_data(benchmark_result: &BenchMarkResult) -> (Duration, Duration, Duration) {
    let average = benchmark_result.total / ITERATIONS as u32;
    let difference_1 = average - benchmark_result.lowest;
    let difference_2 = benchmark_result.highest - average;

    (average, difference_1, difference_2)
}

fn print_single_results(path: &Path, benchmark_result: &BenchMarkResult) {
    let (average, difference_1, difference_2) = calculate_data(benchmark_result);

    eprintln!("File at '{}'", path.display());
    eprintln!(
        "Parsed {} times in: {:?}",
        ITERATIONS, benchmark_result.total
    );
    eprintln!(
        "Average time/parse: {:?} (-{:?}/+{:?})",
        average, difference_1, difference_2
    );
}

fn get_percentage_change(a: &Duration, b: &Duration) -> String {
    let a_nanos = a.as_nanos() as f64;
    let b_nanos = b.as_nanos() as f64;

    if a > b {
        format!("+{:.2}%", ((a_nanos - b_nanos) / b_nanos) * 100.0)
    } else {
        format!("-{:.2}%", ((b_nanos - a_nanos) / a_nanos) * 100.0)
    }
}

fn print_and_compare_results(
    path: &Path,
    old_benchmark_result: &BenchMarkResult,
    benchmark_result: &BenchMarkResult,
) {
    let (average, difference_1, difference_2) = calculate_data(benchmark_result);
    let (old_average, old_difference_1, old_difference_2) = calculate_data(old_benchmark_result);

    let total_difference =
        get_percentage_change(&benchmark_result.total, &old_benchmark_result.total);
    let average_difference = get_percentage_change(&average, &old_average);
    let difference_1_difference = get_percentage_change(&difference_1, &old_difference_1);
    let difference_2_difference = get_percentage_change(&difference_2, &old_difference_2);

    eprintln!("File at '{}'", path.display());
    eprintln!(
        "Parsed {} times in: {:?}({})",
        ITERATIONS, benchmark_result.total, total_difference
    );
    eprintln!(
        "Average time/parse: {:?}({}) (-{:?}({})/+{:?}({}))",
        average,
        average_difference,
        difference_1,
        difference_1_difference,
        difference_2,
        difference_2_difference
    );
}

fn print_results(old: &BenchMarkResults, benchmark_results: &BenchMarkResults) {
    for (path, benchmark_result) in benchmark_results.iter() {
        if let Some(old_benchmark_result) = old.get(path) {
            print_and_compare_results(path, old_benchmark_result, benchmark_result);
        } else {
            print_single_results(path, benchmark_result);
        }

        eprintln!();
    }
}

fn main() -> io::Result<()> {
    let mut benchmark_results = BenchMarkResults::new();
    process_files(Path::new("test-code"), &mut benchmark_results)?;

    print_results(
        &BenchMarkResults::load().unwrap_or_default(),
        &benchmark_results,
    );

    benchmark_results.save()
}
