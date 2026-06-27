// benchmarks/time_format.rs : evaluates costs of `nanoseconds_to_string()`

#![allow(non_snake_case)]

use std::hint::black_box;

use diagnosticism::nanoseconds_to_string;

use criterion::{
    criterion_group,
    criterion_main,
    BatchSize,
    Criterion,
};


/// Representative nanosecond counts spanning each output band and formatting
/// path (integer-only, fractional, large whole, zero, negative, explicit `+`).
const REPRESENTATIVE_VALUES : [(i64, &str); 12] = [
    (0, "zero"),
    (9, "9 ns"),
    (789, "789 ns"),
    (6_789, "6.789 µs"),
    (6_000, "6 µs"),
    (123_456_789, "123.4 ms"),
    (123_000_000, "123 ms"),
    (9_123_456_789, "9.123 s"),
    (9_000_000_000, "9 s"),
    (77_777_777_777_777_777, "77e16 ns → s"),
    (-123_456_789, "negative 123.4 ms"),
    (999_772_000, "999.7 ms edge"),
];


fn bench_nanoseconds_to_string(
    c : &mut Criterion,
    nanoseconds : i64,
    format_spec : &str,
    label : &str,
) {
    let id = format!("`nanoseconds_to_string()` [{label}]");

    c.bench_function(&id, |b| {
        b.iter(|| {
            let s = black_box(nanoseconds_to_string(black_box(nanoseconds), black_box(format_spec)));

            black_box(s)
        })
    });
}


pub fn BENCHMARK_nanoseconds_to_string_zero(c : &mut Criterion) {
    bench_nanoseconds_to_string(c, 0, "", "zero");
}


pub fn BENCHMARK_nanoseconds_to_string_ns(c : &mut Criterion) {
    bench_nanoseconds_to_string(c, 9, "", "9 ns");
    bench_nanoseconds_to_string(c, 789, "", "789 ns");
}


pub fn BENCHMARK_nanoseconds_to_string_us(c : &mut Criterion) {
    bench_nanoseconds_to_string(c, 6_789, "", "6.789 µs");
    bench_nanoseconds_to_string(c, 6_000, "", "6 µs");
}


pub fn BENCHMARK_nanoseconds_to_string_ms(c : &mut Criterion) {
    bench_nanoseconds_to_string(c, 123_456_789, "", "123.4 ms");
    bench_nanoseconds_to_string(c, 123_000_000, "", "123 ms");
    bench_nanoseconds_to_string(c, 999_772_000, "", "999.7 ms edge");
}


pub fn BENCHMARK_nanoseconds_to_string_s(c : &mut Criterion) {
    bench_nanoseconds_to_string(c, 9_123_456_789, "", "9.123 s");
    bench_nanoseconds_to_string(c, 9_000_000_000, "", "9 s");
    bench_nanoseconds_to_string(c, 77_777_777_777_777_777, "", "77e16 ns → s");
}


pub fn BENCHMARK_nanoseconds_to_string_negative(c : &mut Criterion) {
    bench_nanoseconds_to_string(c, -123_456_789, "", "negative 123.4 ms");
}


pub fn BENCHMARK_nanoseconds_to_string_explicit_plus(c : &mut Criterion) {
    bench_nanoseconds_to_string(c, 999_772_000, "+", "999.7 ms with +");
}


pub fn BENCHMARK_nanoseconds_to_string_mixed_workload(c : &mut Criterion) {
    c.bench_function("`nanoseconds_to_string()` [mixed representative values]", |b| {
        b.iter_batched(
            || 0usize,
            |index| {
                let (nanoseconds, _label) = REPRESENTATIVE_VALUES[index % REPRESENTATIVE_VALUES.len()];

                let s = black_box(nanoseconds_to_string(black_box(nanoseconds), ""));

                black_box(s);

                index + 1
            },
            BatchSize::SmallInput,
        )
    });
}


criterion_group!(
    benches,
    BENCHMARK_nanoseconds_to_string_zero,
    BENCHMARK_nanoseconds_to_string_ns,
    BENCHMARK_nanoseconds_to_string_us,
    BENCHMARK_nanoseconds_to_string_ms,
    BENCHMARK_nanoseconds_to_string_s,
    BENCHMARK_nanoseconds_to_string_negative,
    BENCHMARK_nanoseconds_to_string_explicit_plus,
    BENCHMARK_nanoseconds_to_string_mixed_workload,
);
criterion_main!(benches);
