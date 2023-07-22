use criterion::{criterion_group, criterion_main, Criterion};

use rust_advert_of_code::solutions_2022::{day01, day02, day03, day04, day05};

fn bench_day_01(c: &mut Criterion) {
    c.bench_function("Adven of code Day 1, part one", |b| {
        b.iter(|| {
            day01::top_calories();
        });
    });

    c.bench_function("Adven of code Day 1, part two", |b| {
        b.iter(|| {
            day01::top_three_calories_itertools();
        });
    });
}

fn bench_day_02(c: &mut Criterion) {
    c.bench_function("Adven of code Day 2, part one", |b| {
        b.iter(|| day02::rock_paper_scissors(day02::game_result_p_1));
    });

    c.bench_function("Adven of code Day 2, part two", |b| {
        b.iter(|| day02::rock_paper_scissors(day02::game_result_p_1));
    });
}

fn bench_day_03(c: &mut Criterion) {
    c.bench_function("Adven of code Day 3, part one", |b| {
        b.iter(|| day03::rucksack_reorganization());
    });

    c.bench_function("Adven of code Day 3, part two", |b| {
        b.iter(|| day03::rucksack_reorganization_badges());
    });
}

fn bench_day_04(c: &mut Criterion) {
    c.bench_function("Adven of code Day 4, part one", |b| {
        b.iter(|| day04::camp_cleanup());
    });

    c.bench_function("Adven of code Day 4, part two", |b| {
        b.iter(|| day04::camp_cleanup_overlaps());
    });
}
fn bench_day_05(c: &mut Criterion) {
    c.bench_function("Adven of code Day 5, part one", |b| {
        b.iter(|| day05::supply_stacks());
    });

    // c.bench_function("Adven of code Day 4, part two", |b| {
    //     b.iter(|| day04::camp_cleanup_overlaps());
    // });
}

criterion_group!(
    benches,
    bench_day_01,
    bench_day_02,
    bench_day_03,
    bench_day_04,
    bench_day_05
);
criterion_main!(benches);
