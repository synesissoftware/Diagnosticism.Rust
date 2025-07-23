// benchmarks/doomgram.rs : evaluates costs of using `DoomGram`

#![allow(non_snake_case)]

use diagnosticism::diagnostics::DoomGram;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};


mod constants {

}


mod implementation {

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
);
criterion_main!(benches);
