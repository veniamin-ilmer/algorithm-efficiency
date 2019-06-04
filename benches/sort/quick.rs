pub fn sort(arr: &mut [u32]) {
  quicksort(arr, 0, arr.len() - 1);
}

fn quicksort(arr: &mut [u32], lo: usize, hi: usize) {
  if lo < hi {
    let p = partition(arr, lo, hi);
    quicksort(arr, lo, p);
    quicksort(arr, p + 1, hi);
  }
}

fn partition(arr: &mut [u32], lo: usize, hi: usize) -> usize {
  let pivot = arr[lo + (hi - lo) / 2];
  let mut left = lo;
  let mut right = hi;
  loop {
    while arr[left] < pivot {
      left += 1;
    }
    while arr[right] > pivot {
      right -= 1;
    }
    if left >= right {
      return right;
    }
    arr.swap(left, right);
    left += 1;
    right -= 1;
  }
}