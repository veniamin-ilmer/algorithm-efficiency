use criterion::*;

use rand::thread_rng;
use rand::seq::SliceRandom;

mod bubble;
mod selection;
mod insertion;
mod insertions2;
mod cycle;
mod comb;
mod odd_even;
mod shell;
mod quick;
mod heap;

criterion_group!(random, random_benchmark);

criterion_main!(random);

fn random_benchmark(c: &mut Criterion) {
  let plot_config = PlotConfiguration::default();
//      .summary_scale(AxisScale::Logarithmic);

  let benchmark_sort = move |b: &mut Bencher, size: &usize, sort_function: &Fn(&mut [u32])|  {
    b.iter_batched_ref(
      || {
        let mut random_vec: Vec<u32> = (0..*size as u32).collect();
        random_vec.shuffle(&mut thread_rng());
        random_vec
      },
      |random_vec| {
        let mut random_vec = black_box(random_vec);
        (*sort_function)(&mut random_vec);
      },
      BatchSize::SmallInput
    )
  };

  c.bench(
    "Sort",
    ParameterizedBenchmark::new(
      "Insertion2", move |b: &mut Bencher, size: &usize| { benchmark_sort(b, size, &insertions2::sort) },
      (1usize..20).collect::<Vec<usize>>(),
    )
//    .with_function("Selection", move |b: &mut Bencher, size: &usize| { benchmark_sort(b, size, &selection::sort) })
    .with_function("Insertion", move |b: &mut Bencher, size: &usize| { benchmark_sort(b, size, &insertion::sort) })
//    .with_function("Cycle", move |b: &mut Bencher, size: &usize| { benchmark_sort(b, size, &cycle::sort) })
//    .with_function("Comb", move |b: &mut Bencher, size: &usize| { benchmark_sort(b, size, &comb::sort) })
//    .with_function("Odd-Even", move |b: &mut Bencher, size: &usize| { benchmark_sort(b, size, &odd_even::sort) })
//    .with_function("Shell", move |b: &mut Bencher, size: &usize| { benchmark_sort(b, size, &shell::sort) })
//    .with_function("Quick", move |b: &mut Bencher, size: &usize| { benchmark_sort(b, size, &quick::sort) })
//    .with_function("Heap", move |b: &mut Bencher, size: &usize| { benchmark_sort(b, size, &heap::sort) })
    .plot_config(plot_config),
  );
}