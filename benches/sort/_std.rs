use bencher::Bencher;

use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn baseline(b: &mut Bencher) {
  let answer_vec: Vec<u32> = (0..1).collect();
  b.iter(|| {
    let mut test_vec: Vec<u32> = (0..1).collect();
    test_vec.sort_unstable();
    assert_eq!(test_vec, answer_vec);
  });
}

pub fn sorted(b: &mut Bencher) {
  let answer_vec: Vec<u32> = (0..10_000).collect();
  b.iter(|| {
    let mut test_vec: Vec<u32> = (0..10_000).collect();
    test_vec.sort_unstable();
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
    test_vec.sort_unstable();
    assert_eq!(test_vec, answer_vec);
    i_count = (i_count + 1) % 100;
  });
}

pub fn reverse(b: &mut Bencher) {
  let answer_vec: Vec<u32> = (0..10_000).collect();
  b.iter(|| {
    let mut test_vec: Vec<u32> = (0..10_000).rev().collect();
    test_vec.sort_unstable();
    assert_eq!(test_vec, answer_vec);
  });
}
