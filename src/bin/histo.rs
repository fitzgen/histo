//! Reads samples from stdin, one per line, and then prints the resulting
//! histogram.

extern crate histo;

use std::io::{self, BufRead, Write};
use std::process;

fn main() {
    if let Err(e) = try_main() {
        let mut stderr = io::stderr();
        let _ = write!(&mut stderr, "error: {}", e);
        process::exit(1);
    }
}

fn try_main() -> io::Result<()> {
    let mut hist = histo::Histogram::with_buckets(10);

    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut line = String::new();
    while stdin.read_line(&mut line)? > 0 {
        {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let sample: u64 = line.trim()
                .parse()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            hist.add(sample);
        }

        line.clear();
    }

    println!("{}", hist);
    Ok(())
}
