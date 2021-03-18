use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sublist::{sublist, Comparison};

fn huge_sublist_not_in_huge_list() {
    let v1: Vec<u64> = (10..1_000_001).collect();
    let v2: Vec<u64> = (1..1_000_000).collect();

    assert_eq!(Comparison::Unequal, sublist(&v1, &v2));
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("superlist of empty", |b| {
        b.iter(|| sublist(black_box(&['a', 's', 'd', 'f']), black_box(&[])))
    });
}

fn criterion_benchmark2(c: &mut Criterion) {
    c.bench_function("huge sublist not in huge list", |b| {
        b.iter(|| huge_sublist_not_in_huge_list())
    });
}

fn criterion_benchmark3(c: &mut Criterion) {
    let v1: Vec<u64> = (10..1_000_001).collect();
    let v2: Vec<u64> = (1..1_000_000).collect();
    c.bench_function("huge sublist with black box", |b| {
        b.iter(|| sublist(black_box(&v1), black_box(&v2)))
    });
}
fn criterion_benchmark4(c: &mut Criterion) {
    let v1: Vec<u64> = (10..1_000_001).collect();
    let v2: Vec<u64> = (1..1_000_000).collect();
    c.bench_function("huge sublist ", |b| {
        b.iter(|| sublist(&v1, &v2))
    });}

criterion_group!(
    benches,
    criterion_benchmark,
    criterion_benchmark2,
    criterion_benchmark3,
    criterion_benchmark4
);
criterion_main!(benches);
