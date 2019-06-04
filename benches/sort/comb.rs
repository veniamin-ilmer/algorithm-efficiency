pub fn sort(arr: &mut [u32]) {
  let mut gap = arr.len();
  let mut sorted = false;
  
  while !sorted {
    gap = (gap * 10) / 13;  //Equivalent of gap = floor(gap / 1.3)
    if gap < 1 {
      gap = 1;
      sorted = true;
    }
    
    for i in 0 .. arr.len() - gap {
      if arr[i] > arr[i + gap] {
        arr.swap(i, i + gap);
        sorted = false;
      }
    }
  }
}
