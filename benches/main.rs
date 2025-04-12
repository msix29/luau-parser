use luau_parser::prelude::Parser;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Read},
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

const ITERATIONS: usize = 1000;
const WARM_UP_ITERATIONS: usize = 50;

type BenchmarkResults = HashMap<PathBuf, BenchMarkResult>;

#[derive(Debug)]
struct BenchMarkResult {
    lowest: Duration,
    highest: Duration,
    total: Duration,
}

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

fn process_files(src_dir: &Path, benchmark_results: &mut BenchmarkResults) -> io::Result<()> {
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

fn print_results(benchmark_results: &BenchmarkResults) {
    for (path, benchmark_result) in benchmark_results.iter() {
        let average = benchmark_result.total / ITERATIONS as u32;
        let difference_1 = average - benchmark_result.lowest;
        let difference_2 = benchmark_result.highest - average;
        eprintln!("File at '{}'", path.display());
        eprintln!(
            "Parsed {} times in: {:?}",
            ITERATIONS, benchmark_result.total
        );
        eprintln!(
            "Average time/parse: {:?} (-{:?}/+{:?})",
            average, difference_1, difference_2
        );
        eprintln!();
    }
}

fn main() -> io::Result<()> {
    let mut benchmark_results = HashMap::new();
    process_files(Path::new("test-code"), &mut benchmark_results)?;

    print_results(&benchmark_results);

    Ok(())
}
