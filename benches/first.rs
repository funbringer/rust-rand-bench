use criterion::{criterion_group, criterion_main, Criterion};
use rand::distributions::{Distribution, Standard};

fn bench_simple<T>(c: &mut Criterion)
where
    Standard: Distribution<T>,
{
    let name = ["simple", std::any::type_name::<T>()].join(" ");
    c.bench_function(&name, |b| {
        b.iter(|| {
            let _value = rand::random::<T>();
        })
    });
}

fn bench_cached<T>(c: &mut Criterion)
where
    Standard: Distribution<T>,
{
    let name = ["cached", std::any::type_name::<T>()].join(" ");
    let mut rng = rand::thread_rng();
    c.bench_function(&name, |b| {
        b.iter(|| {
            use rand::Rng;
            let _value = rng.gen::<T>();
        })
    });
}

fn bench_range_simple(c: &mut Criterion) {
    c.bench_function("range_simple", |b| {
        b.iter(|| {
            use rand::Rng;
            let _value = rand::thread_rng().gen_range(0..=9999);
        })
    });
}

fn bench_range_cached(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    c.bench_function("range_cached", |b| {
        b.iter(|| {
            use rand::Rng;
            let _value = rng.gen_range(0..=9999);
        })
    });
}

criterion_group!(
    benches,
    bench_simple::<u32>,
    bench_cached::<u32>,
    bench_simple::<[u8; 32]>,
    bench_cached::<[u8; 32]>,
    bench_range_simple,
    bench_range_cached,
);

criterion_main!(benches);
