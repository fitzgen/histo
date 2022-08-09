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

fn io_error(error: impl Into<Box<dyn std::error::Error + Send + Sync>>) -> io::Error {
    io::Error::new(io::ErrorKind::Other, error)
}

const HELP_TEXT: &'static str = "\
Reads data from `stdin` and prints a histogram of the data to `stdout`.

The input data format must be one unsigned 64-bit integer per line.

USAGE:
    histo [OPTIONS] [FLAGS]

OPTIONS:
    -n, --buckets BUCKETS
        Configure the number of buckets in the histogram

FLAGS:
    -h, --help
        Prints help information

    -V, --version
        Prints version information

EXAMPLE:

    $ head data.txt
    3
    1
    50
    38
    1
    38
    1
    38
    39
    4
    $ histo < data.txt
    # Number of samples = 154653
    # Min = 1
    # Max = 58
    #
    # Mean = 4.407945529669717
    # Standard deviation = 5.8241339617399985
    # Variance = 33.92053640429325
    #
    # Each ∎ is a count of 2484
    #
     1 ..  7 [ 124222 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
     7 .. 13 [  18556 ]: ∎∎∎∎∎∎∎
    13 .. 19 [   7117 ]: ∎∎
    19 .. 25 [   2059 ]:
    25 .. 31 [   1245 ]:
    31 .. 37 [    284 ]:
    37 .. 43 [    882 ]:
    43 .. 49 [    122 ]:
    49 .. 55 [    135 ]:
    55 .. 61 [     31 ]:
";

fn help_text() {
    eprintln!("histo {}\n\n{}", env!("CARGO_PKG_VERSION"), HELP_TEXT);
}

fn fail_with_help_text(msg: impl AsRef<str>) -> ! {
    eprintln!("{}", msg.as_ref());
    eprintln!();
    help_text();
    process::exit(1)
}

fn try_main() -> io::Result<()> {
    let mut buckets = 10;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-n" | "--buckets" => {
                let arg = args.next().unwrap_or_else(|| {
                    fail_with_help_text(format!("missing number of buckets for `{arg}`"))
                });
                buckets = arg.parse().map_err(|e| io_error(e))?;
            }
            "-h" | "--help" => {
                help_text();
                process::exit(0);
            }
            "-V" | "--version" => {
                println!("histo {}", env!("CARGO_PKG_VERSION"));
                process::exit(0);
            }
            _ => fail_with_help_text(format!("unknown CLI argument: `{arg}`")),
        }
    }

    let mut hist = histo::Histogram::with_buckets(buckets);

    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut line = String::new();
    while stdin.read_line(&mut line)? > 0 {
        {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let sample: u64 = line.trim().parse().map_err(|e| io_error(e))?;

            hist.add(sample);
        }

        line.clear();
    }

    println!("{}", hist);
    Ok(())
}
