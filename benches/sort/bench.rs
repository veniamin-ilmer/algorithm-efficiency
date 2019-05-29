//!Sorting Vecs of u32.
//!
//!* *Baseline*: 1 item.
//!* *Sorted*: 10,000 items, fully sorted already.
//!* *Random*: 10,000 items, randomly shuffled. (The shuffling is not part of the time).
//!* *Reverse*: 10,000 items, sorted in reverse order.

#[macro_use]
extern crate bencher;

mod _std;
mod _bubble_sort;

benchmark_group!(baseline, _std::baseline,
                           _bubble_sort::baseline);
benchmark_group!(sorted,   _std::sorted,
                           _bubble_sort::sorted);
benchmark_group!(random,   _std::random,
                           _bubble_sort::random);
benchmark_group!(reverse,  _std::reverse,
                           _bubble_sort::reverse);

benchmark_main!(baseline, sorted, random, reverse);
