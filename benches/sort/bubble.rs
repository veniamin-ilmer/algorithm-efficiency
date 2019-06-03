pub fn sort(in_vec: &mut [u32]) {
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