pub fn sort(arr: &mut [u32]) {
	let mut n = arr.len();
	while n > 1 {
		let mut new_n = 0;
		for i in 1..n {
      //If this pair is out of order
			if arr[i - 1] > arr[i] {
        //Swap them and remember something changed
				arr.swap(i - 1, i);
				new_n = i;
			}
		}
		n = new_n;
	}
}