pub fn sort(arr: &mut [u32]) {
  if arr.len() <= 1 {
    return;
  }
	let mut sorted = false;
  while !sorted {
    sorted = true;
    for i in (1..arr.len() - 1).step_by(2) {
      if arr[i] > arr[i + 1] {
        arr.swap(i, i + 1);
        sorted = false;
      }
    }
    for i in (0..arr.len() - 1).step_by(2) {
      if arr[i] > arr[i + 1] {
        arr.swap(i, i + 1);
        sorted = false;
      }
    }
  }
}