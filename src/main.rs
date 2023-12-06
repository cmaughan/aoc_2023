use clap::Parser;
use itertools::Itertools;
use std::{error::Error, fs, process::Command};

fn format_ns(val: u64) -> String {
    if val < 1000 {
        format!("{}ns", val)
    } else {
        format!("{}μs", val / 1000)
    }
}
fn extract_microseconds(output: &str) -> (u64, u64, String) {
    let out = output
        .lines()
        .last()
        .unwrap()
        .split(",")
        .collect::<Vec<_>>();
    let p1 = out[0].parse::<u64>().unwrap();
    let p2 = out[1].parse::<u64>().unwrap();

    let text = format!("{0: <6}  {1: <6}", format_ns(p1), format_ns(p2));

    (p1, p2, text)
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

    if clargs.perf {
        println!("{0: <6} | {1: <6} | {2: <6}", "Day", "Part 1", "Part 2");
    }
    for day in &days {
        let mut args = vec!["run", "--release", "--bin", day];
        if clargs.perf {
            args.push("--");
            args.push("--perf");
        }
        let cmd = Command::new("cargo").args(args).output()?;
        let output = String::from_utf8(cmd.stdout)?;

        if clargs.perf {
            let res = extract_microseconds(&output);
            println!("{0:<6}   {1}", day, res.2);
            total_time += res.0 + res.1;
        } else {
            println!("Day {}: {}", day, output);
        }
    }

    if clargs.perf {
        println!("\nTotal time: {}μs", total_time as f64 / 1000.0);
    }
    Ok(())
}
