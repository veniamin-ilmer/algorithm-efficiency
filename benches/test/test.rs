    use criterion::*;

    criterion_group!(test, timing);
    criterion_main!(test);

    fn timing(c: &mut Criterion) {
      use std::{thread, time};

      let delay = time::Duration::from_millis(100);
      
      c.bench_function("timing", move |b| {
        b.iter(|| {
          thread::sleep(delay);
          black_box(1) + 1
          });
      });
    }