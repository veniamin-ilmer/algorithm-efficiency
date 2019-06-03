use criterion::*;

use rand::thread_rng;
use rand::seq::SliceRandom;

mod bubble;
mod selection;
mod insertion;
mod cycle;

criterion_group!(random, random_benchmark);

criterion_main!(random);

fn random_benchmark(c: &mut Criterion) {
  let plot_config = PlotConfiguration::default();
//      .summary_scale(AxisScale::Logarithmic);

  let mut sort_function: fn(&mut [u32]) = bubble::sort;

  let benchmark_sort = move |b: &mut Bencher, size: &usize| {
    let vec_list = make_list_of_random_nums(*size);
    let mut count = 0_usize;
    b.iter_batched_ref(
      || {
        let mut random_vec = vec![0; *size];
        random_vec.clone_from_slice(&vec_list[count..size+count]);
        count = (count + 1) % 10000;
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
      "Bubble", move |b: &mut Bencher, size: &usize| { sort_function = bubble::sort; benchmark_sort(b, size) },
      (0usize..10).collect::<Vec<usize>>(),
    )
    .with_function("Selection", move |b: &mut Bencher, size: &usize| { sort_function = selection::sort; benchmark_sort(b, size) })
    .with_function("Insertion", move |b: &mut Bencher, size: &usize| { sort_function = insertion::sort; benchmark_sort(b, size) })
    .with_function("Cycle", move |b: &mut Bencher, size: &usize| { sort_function = cycle::sort; benchmark_sort(b, size) })
    .plot_config(plot_config),
  );
}

fn make_list_of_random_nums(size: usize) -> Vec<u32> {
  let mut random_vec: Vec<u32> = (0..10000+size as u32).collect();
  random_vec.shuffle(&mut thread_rng());
  random_vec
}