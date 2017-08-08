#![feature(test)]

extern crate histo;
extern crate test;

#[bench]
fn bench_adding_samples(b: &mut test::Bencher) {
    b.iter(|| {
        let mut h = histo::Histogram::with_buckets(10);
        for i in 0..100 {
            h.add(i);
            h.add(i * i);
            h.add(i * i * i);
        }
        test::black_box(h);
    });
}

#[bench]
fn bench_formatting(b: &mut test::Bencher) {
    use std::string::ToString;

    let mut h = histo::Histogram::with_buckets(10);

    for i in 0..100 {
        h.add(i);
        h.add(i * i);
        h.add(i * i * i);
    }

    b.iter(|| {
        test::black_box(h.to_string());
    });
}
