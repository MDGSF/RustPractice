use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Mutex;

// s ms us ns ps
// 1s = 10^3 ms
// 1s = 10^6 us
// 1s = 10^9 ns
// 1s = 10^12ps

fn bench_atomic(c: &mut Criterion) {
    let mut i = 0;
    let a = AtomicUsize::new(i);
    c.bench_function("atomic compare_exchange", |b| {
        b.iter(|| {
            let _ret = a.compare_exchange(i, i + 1, Ordering::SeqCst, Ordering::SeqCst);
            i += 1;
        })
    });
}

fn bench_mutex(c: &mut Criterion) {
    let lock = Mutex::new(0_u32);
    c.bench_function("mutex lock", |b| {
        b.iter(|| {
            *lock.lock().unwrap() += 1;
        })
    });
}

criterion_group!(benches, bench_atomic, bench_mutex);
criterion_main!(benches);
