pub fn sort(arr: &mut [u32]) {
  for i in 1 .. arr.len() {
    let x = arr[i]; //What we are going to move.
    let mut new_x_pos = i; //Where we will move it.
    for j in (0 .. i).rev() {
      if x >= arr[j] { break; } //We either hit the same, or a smaller number. Break. Everything further is already sorted.
      arr[j + 1] = arr[j];
      new_x_pos = j;  //We moved something. That means x will need to be moved.
    }
    arr[new_x_pos] = x;  //Move x to its new position.
	}
}