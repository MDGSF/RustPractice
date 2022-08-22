use benches_demo_1::queue::Queue;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_queue(c: &mut Criterion) {
    let q = Queue::with_capacity(8);
    c.bench_function("RwLock VecDeque", |b| {
        b.iter(|| {
            q.push(black_box(123));
            q.pop();
        })
    });
}

criterion_group!(benches, bench_queue);
criterion_main!(benches);
