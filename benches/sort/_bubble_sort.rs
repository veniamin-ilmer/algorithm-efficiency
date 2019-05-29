use bencher::Bencher;

use rand::thread_rng;
use rand::seq::SliceRandom;

fn sort(in_vec: &mut [u32]) {
	let mut n = in_vec.len();
	while n > 1 {
		let mut new_n = 0;
		for i in 1..n {
			if in_vec[i - 1] > in_vec[i] {
				in_vec.swap(i - 1, i);
				new_n = i;
			}
		}
		n = new_n;
	}
}

pub fn baseline(b: &mut Bencher) {
  let answer_vec: Vec<u32> = (0..1).collect();
  b.iter(|| {
    let mut test_vec: Vec<u32> = (0..1).collect();
	sort(&mut test_vec);
    assert_eq!(test_vec, answer_vec);
  });
}

pub fn sorted(b: &mut Bencher) {
  let answer_vec: Vec<u32> = (0..10_000).collect();
  b.iter(|| {
    let mut test_vec: Vec<u32> = (0..10_000).collect();
	sort(&mut test_vec);
    assert_eq!(test_vec, answer_vec);
  });
}

pub fn random(b: &mut Bencher) {
  let answer_vec: Vec<u32> = (0..10_000).collect();
  let mut test_vec: Vec<u32> = (0..10_000).collect();
  let mut test_vecs = Vec::new();
  
  //Set up randoms
  for _ in 0..100 {
	test_vec.shuffle(&mut thread_rng());
    test_vecs.push(test_vec.to_vec());  //Record for later use.
  }
  let mut i_count = 0;
  
  b.iter(|| {
    test_vec = test_vecs[i_count].to_vec();
	sort(&mut test_vec);
    assert_eq!(test_vec, answer_vec);
    i_count = (i_count + 1) % 100;
  });
}

pub fn reverse(b: &mut Bencher) {
  let answer_vec: Vec<u32> = (0..10_000).collect();
  b.iter(|| {
    let mut test_vec: Vec<u32> = (0..10_000).rev().collect();
	sort(&mut test_vec);
    assert_eq!(test_vec, answer_vec);
  });
}
