use criterion::Criterion;
use criterion::*;

use rand::thread_rng;
use rand::seq::SliceRandom;

mod bubble;
mod selection;

criterion_group!(random, random_benchmark);

criterion_main!(random);

fn random_benchmark(c: &mut Criterion) {
  let plot_config = PlotConfiguration::default()
      .summary_scale(AxisScale::Logarithmic);

  let mut sort_function: fn(&mut [u32]) = bubble::sort;

  let benchmark_sort = move |b: &mut Bencher, size: &u32| {
    let vec_list = make_list_of_random_nums(*size);
    let mut count = 0;
    b.iter_batched(
      || {
        let random_vec = vec_list[count].clone();
        count = (count + 1) % 100;
        random_vec
      },
      |random_vec| {
        let mut random_vec = black_box(random_vec);
        sort_function(&mut random_vec);
      },
      BatchSize::SmallInput
    )
  };

  c.bench(
    "Sort",
    ParameterizedBenchmark::new(
      "Bubble", move |b: &mut Bencher, size: &u32| { sort_function = bubble::sort; benchmark_sort(b, size) },
      vec![1u32, 10, 100, 1_000],
    )
    .with_function("Selection", move |b: &mut Bencher, size: &u32| { sort_function = selection::sort; benchmark_sort(b, size) })
    .with_function("Insertion", move |b: &mut Bencher, size: &u32| { sort_function = selection::sort; benchmark_sort(b, size) })
    .plot_config(plot_config),
  );
}

fn make_list_of_random_nums(size: u32) -> Vec<Vec<u32>> {
  let mut random_vec: Vec<u32> = (0..size).collect();
  let mut vec_list = vec![];
  for _count in 0..100 {
    random_vec.shuffle(&mut thread_rng());
    vec_list.push(random_vec.clone());
  }
  vec_list
}