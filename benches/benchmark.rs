use criterion::{black_box, criterion_group, criterion_main, Criterion};
use algorithms::{quicksort::quicksort_naive,
                 selection_sort::selection_sort_naive};

use rand::prelude::*;

fn gen_random_ints(size: i64) -> Vec<i64>{
  let mut rng = rand::thread_rng();
  let mut nums: Vec<i64> = (1..size).collect();
  nums.shuffle(&mut rng);

  nums
}
pub fn quicksort_naive_bench(c: &mut Criterion){
  let unsorted = gen_random_ints(10_000);

  c.bench_function("quicksort naive", |b| b.iter(||{ quicksort_naive(black_box(&unsorted.clone()))
  }));
}

pub fn selection_sort_naive_bench(c: &mut Criterion){
  let unsorted = gen_random_ints(10_000);
  c.bench_function("selection sort naive", |b|
                   b.iter(||{
                     selection_sort_naive(black_box(&mut unsorted.clone())) }));
}

criterion_group!(benches, quicksort_naive_bench, selection_sort_naive_bench);
criterion_main!(benches);
