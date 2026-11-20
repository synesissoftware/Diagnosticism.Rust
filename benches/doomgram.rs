// benchmarks/doomgram.rs : evaluates costs of using `DoomGram`

#![allow(non_snake_case)]

use std::hint::black_box;

use diagnosticism::DoomGram;

use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};


fn doomgram_empty() -> DoomGram {
    DoomGram::default()
}


fn doomgram_single() -> DoomGram {
    let mut dg = DoomGram::default();

    dg.push_event_time_ms(13);

    dg
}


fn doomgram_uniform() -> DoomGram {
    let mut dg = DoomGram::default();

    dg.push_event_time_s(1);
    dg.push_event_time_s(1);
    dg.push_event_time_s(1);

    dg
}


fn doomgram_min_mean_max() -> DoomGram {
    let mut dg = DoomGram::default();

    dg.push_event_time_s(1);
    dg.push_event_time_s(2);

    dg
}


fn doomgram_uniform_spread() -> DoomGram {
    let mut dg = DoomGram::default();

    dg.push_event_time_ns(9);
    dg.push_event_time_ns(80);
    dg.push_event_time_ns(700);
    dg.push_event_time_us(6);
    dg.push_event_time_us(50);
    dg.push_event_time_us(400);
    dg.push_event_time_ms(3);
    dg.push_event_time_ms(20);
    dg.push_event_time_ms(100);
    dg.push_event_time_s(9);
    dg.push_event_time_s(80);
    dg.push_event_time_s(700);

    dg
}


fn doomgram_overflowed() -> DoomGram {
    let mut dg = DoomGram::default();

    dg.push_event_time_us(18446744073709550);
    dg.push_event_time_us(1);
    dg.push_event_time_us(0);
    dg.push_event_time_us(1);

    dg
}


fn bench_to_mmm(
    c : &mut Criterion,
    dg : DoomGram,
    label : &str,
) {
    let id = format!("`DoomGram::to_mmm()` [{label}]");

    c.bench_function(&id, |b| {
        b.iter(|| {
            let s = black_box(black_box(&dg).to_mmm());

            black_box(s)
        })
    });
}


fn bench_to_nmmm(
    c : &mut Criterion,
    dg : DoomGram,
    label : &str,
) {
    let id = format!("`DoomGram::to_nmmm()` [{label}]");

    c.bench_function(&id, |b| {
        b.iter(|| {
            let s = black_box(black_box(&dg).to_nmmm());

            black_box(s)
        })
    });
}


pub fn BENCHMARK_DoomGram_Default(c : &mut Criterion) {
    c.bench_function("`DoomGram::default()`", |b| {
        b.iter(|| {
            let dg = black_box(DoomGram::default());

            let _ = black_box(dg);
        })
    });
}

pub fn BENCHMARK_DoomGram_event_count(c : &mut Criterion) {
    c.bench_function("`DoomGram::event_count()`", |b| {
        b.iter(|| {
            let dg = black_box(DoomGram::default());

            let _ = black_box(black_box(&dg).event_count());
        })
    });
}

pub fn BENCHMARK_DoomGram_push_ns(c : &mut Criterion) {
    c.bench_function("`DoomGram::push_event_time_ns()`", |b| {
        b.iter(|| {
            let mut dg = black_box(DoomGram::default());

            let _ = black_box(black_box(&mut dg).push_event_time_ns(123));
        })
    });
}

pub fn BENCHMARK_DoomGram_push_us(c : &mut Criterion) {
    c.bench_function("`DoomGram::push_event_time_us()`", |b| {
        b.iter(|| {
            let mut dg = black_box(DoomGram::default());

            let _ = black_box(black_box(&mut dg).push_event_time_us(123));
        })
    });
}

pub fn BENCHMARK_DoomGram_push_ms(c : &mut Criterion) {
    c.bench_function("`DoomGram::push_event_time_ms()`", |b| {
        b.iter(|| {
            let mut dg = black_box(DoomGram::default());

            let _ = black_box(black_box(&mut dg).push_event_time_ms(123));
        })
    });
}

pub fn BENCHMARK_DoomGram_push_s(c : &mut Criterion) {
    c.bench_function("`DoomGram::push_event_time_s()`", |b| {
        b.iter(|| {
            let mut dg = black_box(DoomGram::default());

            let _ = black_box(black_box(&mut dg).push_event_time_s(123));
        })
    });
}


pub fn BENCHMARK_DoomGram_to_mmm(c : &mut Criterion) {
    bench_to_mmm(c, doomgram_empty(), "empty");
    bench_to_mmm(c, doomgram_single(), "single");
    bench_to_mmm(c, doomgram_uniform(), "uniform");
    bench_to_mmm(c, doomgram_min_mean_max(), "min-mean-max");
    bench_to_mmm(c, doomgram_uniform_spread(), "uniform spread");
    bench_to_mmm(c, doomgram_overflowed(), "overflow");
}


pub fn BENCHMARK_DoomGram_to_nmmm(c : &mut Criterion) {
    bench_to_nmmm(c, doomgram_empty(), "empty");
    bench_to_nmmm(c, doomgram_single(), "single");
    bench_to_nmmm(c, doomgram_uniform(), "uniform");
    bench_to_nmmm(c, doomgram_min_mean_max(), "min-mean-max");
    bench_to_nmmm(c, doomgram_uniform_spread(), "uniform spread");
    bench_to_nmmm(c, doomgram_overflowed(), "overflow");
}


criterion_group!(
    benches,
    // construction
    BENCHMARK_DoomGram_Default,
    BENCHMARK_DoomGram_event_count,
    // populating
    BENCHMARK_DoomGram_push_ns,
    BENCHMARK_DoomGram_push_us,
    BENCHMARK_DoomGram_push_ms,
    BENCHMARK_DoomGram_push_s,
    // formatting
    BENCHMARK_DoomGram_to_mmm,
    BENCHMARK_DoomGram_to_nmmm,
);
criterion_main!(benches);
