pub fn sort(in_vec: &mut [u32]) {
	let mut i = 1;
	while i < in_vec.len() {
    let x = in_vec[i];
    let mut j = i - 1;
    while in_vec[j] > x {
      in_vec[j + 1] = in_vec[j];
      if j == 0 { break; }
      j -= 1;
    }
    in_vec[j + 1] = x;
    i += 1;
	}
}