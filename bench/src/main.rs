use std::collections::HashMap;
use std::sync::Mutex;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use rayon::prelude::*;

static TEXT: &str = include_str!("../../tests/gulliver.txt");

fn count_words() {
    let mut map = HashMap::<&str, usize>::new();
    for word in TEXT.split_whitespace() {
        *map.entry(word).or_default() += 1;
    }
}

fn do_self(n: usize) {
    let interner = symbol_table::SymbolTable::new();
    (0..n).into_par_iter().for_each(|_| {
        for word in TEXT.split_whitespace() {
            interner.intern(word);
        }
    })
}

#[cfg(feature = "global")]
fn do_self_global(n: usize) {
    (0..n).into_par_iter().for_each(|_| {
        for word in TEXT.split_whitespace() {
            let _ = symbol_table::GlobalSymbol::from(word);
        }
    })
}

fn do_string_interner(n: usize) {
    let interner = Mutex::new(string_interner::StringInterner::default());
    (0..n).into_par_iter().for_each(|_| {
        for word in TEXT.split_whitespace() {
            interner.lock().unwrap().get_or_intern(word);
        }
    })
}

fn do_string_interner_buffer(n: usize) {
    use string_interner::{backend::*, *};
    let interner = Mutex::new(StringInterner::<BufferBackend>::new());
    (0..n).into_par_iter().for_each(|_| {
        for word in TEXT.split_whitespace() {
            interner.lock().unwrap().get_or_intern(word);
        }
    })
}

fn do_lasso(n: usize) {
    use lasso::*;
    let interner = ThreadedRodeo::<Spur>::new();
    (0..n).into_par_iter().for_each(|_| {
        for word in TEXT.split_whitespace() {
            interner.get_or_intern(word);
        }
    })
}

#[allow(clippy::type_complexity)]
static BENCHES: &[(fn(usize), &str)] = &[
    (do_self, "self"),
    #[cfg(feature = "global")]
    (do_self_global, "self-global"),
    (do_string_interner, "string-interner"),
    (do_string_interner_buffer, "string-interner-buffer"),
    (do_lasso, "lasso"),
];

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("count-words", |b| b.iter(count_words));
    for (f, name) in BENCHES {
        let mut group = c.benchmark_group(*name);
        for n in [1, 2, 4, 8] {
            let id = BenchmarkId::from_parameter(n);
            group.bench_with_input(id, &n, |b, &n| b.iter(|| f(n)));
        }
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(10000, Output::Flamegraph(None)));
    targets = criterion_benchmark
}
criterion_main!(benches);
