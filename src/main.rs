use clap::Parser;
use itertools::Itertools;
use std::{error::Error, fs, process::Command};

fn extract_microseconds(output: &str) -> Result<usize, Box<dyn Error>> {
    let out = output.lines().last().unwrap();
    let time = if out.ends_with("ms") {
        out["Time: ".len()..out.len() - 2].parse::<usize>()? * 1000
    } else {
        out["Time: ".len()..out.len() - 3].parse::<usize>()?
    };
    Ok(time)
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value_t = false)]
    perf: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let clargs = Cli::parse();
    let days = fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bin/"))?
        .filter_map(|p| p.ok()?.path().file_stem()?.to_str().map(str::to_string))
        .sorted()
        .collect::<Vec<_>>();
    let mut total_time = 0;

    for day in &days {
        let mut args = vec!("run", "--release", "--bin", day);
        if clargs.perf {
            args.push("--");
            args.push("--perf");
        }
        let cmd = Command::new("cargo")
            .args(args)
            .output()?;
        let output = String::from_utf8(cmd.stdout)?;

        if clargs.perf {
            let ms = extract_microseconds(&output)?;
            println!("Day {}: {}μs", day, ms);
            total_time += ms;
        } else {
            println!("Day {}: {}", day, output);
        }
    }

    if clargs.perf {
        println!("\nTotal time: {}ms", total_time as f64 / 1000.0);
    }
    Ok(())
}
