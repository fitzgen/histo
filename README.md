# `histo`

[![Rust](https://github.com/fitzgen/histo/actions/workflows/rust.yml/badge.svg)](https://github.com/fitzgen/histo/actions/workflows/rust.yml) [![histo on crates.io](https://img.shields.io/crates/v/histo.svg)](https://crates.io/crates/histo) [![histo on docs.rs](https://docs.rs/histo/badge.svg)](https://docs.rs/histo/)

Histograms with a configurable number of buckets, and a terminal-friendly
`Display`.

This crate provides a `Histogram` type that allows configuration of the number
of buckets that will be used, regardless of the range of input samples. This is
useful when displaying a `Histogram` (for example, when printing it to a
terminal) but it sacrifices fancy tracking of precision and significant figures.

It uses O(n) memory.

```rust
extern crate histo;
use histo::Histogram;

// Create a histogram that will have 10 buckets.
let mut histogram = Histogram::with_buckets(10);

// Adds some samples to the histogram.
for sample in 0..100 {
    histogram.add(sample);
    histogram.add(sample * sample);
}

// Iterate over buckets and do stuff with their range and count.
for bucket in histogram.buckets() {
    do_stuff(bucket.start(), bucket.end(), bucket.count());
}

// And you can also `Display` a histogram!
println!("{}", histogram);

// Prints:
//
// ```
// # Number of samples = 200
// # Min = 0
// # Max = 9801
// #
// # Mean = 1666.5000000000005
// # Standard deviation = 2641.2281518263426
// # Variance = 6976086.1499999985
// #
// # Each ∎ is a count of 2
// #
//    0 ..  980 [ 132 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
//  980 .. 1960 [  13 ]: ∎∎∎∎∎∎
// 1960 .. 2940 [  10 ]: ∎∎∎∎∎
// 2940 .. 3920 [   8 ]: ∎∎∎∎
// 3920 .. 4900 [   7 ]: ∎∎∎
// 4900 .. 5880 [   7 ]: ∎∎∎
// 5880 .. 6860 [   6 ]: ∎∎∎
// 6860 .. 7840 [   6 ]: ∎∎∎
// 7840 .. 8820 [   5 ]: ∎∎
// 8820 .. 9800 [   6 ]: ∎∎∎
// ```
```

## Install and Usage

To use the `histo` crate in your Rust project, add it to your `Cargo.toml` file:

```toml
[dependencies]
histo = "0.1.0"
```

The `histo` crate also comes with the command line `histo` tool:

```commands
$ cargo install histo
$ tail samples.txt
1
2
3
4
5
1
2
3
4
5
$ histo < samples.txt
# Number of samples = 150
# Min = 1
# Max = 10
#
# Mean = 5.833333333333334
# Standard deviation = 1.9301698255737905
# Variance = 3.7255555555555566
#
# Each ∎ is a count of 1
#
 1 ..  2 [  3 ]: ∎∎∎
 2 ..  3 [  3 ]: ∎∎∎
 3 ..  4 [  3 ]: ∎∎∎
 4 ..  5 [ 31 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 5 ..  6 [ 28 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 6 ..  7 [ 29 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 7 ..  8 [ 29 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 8 ..  9 [  8 ]: ∎∎∎∎∎∎∎∎
 9 .. 10 [  8 ]: ∎∎∎∎∎∎∎∎
10 .. 11 [  8 ]: ∎∎∎∎∎∎∎∎
```
