use advent_of_code_2022::{get_data, get_day, get_days};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    for i in get_days() {
        let (part1, part2, path) = get_day(i);
        c.bench_function(&format!("day {i} A"), |b| {
            b.iter(|| {
                part1(get_data(&path));
            })
        });

        c.bench_function(&format!("day {i} B"), |b| {
            b.iter(|| {
                part2(get_data(&path));
            })
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
