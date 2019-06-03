pub fn sort(in_vec: &mut [u32]) {
	for i in 0 .. in_vec.len() - 1 {
    let mut smallest = i;
    for j in i + 1 .. in_vec.len() {
      if in_vec[j] < in_vec[smallest] {
        smallest = i; //Found new minimum. Remember its index.
      }
    }
	
    //Found something smaller than our previous minimum.
    if smallest != i {
      in_vec.swap(i, smallest);
    }
  }
}