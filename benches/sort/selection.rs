pub fn sort(arr: &mut [u32]) {
  //Could do arr.len() - 1 because single element is also min element.
	for i in 0 .. arr.len() - 1 {
    //Assume the first element is the smallest.
    let mut smallest = i;
    //Test against elements after i to find the smallest.
    for j in i + 1 .. arr.len() {
      //If this element is less, then it is the new minimum.
      if arr[j] < arr[smallest] {
        //Found new minimum. Remember its index.
        smallest = j;
      }
    }
	
    //If this element is less, then it is the new minimum.
    if smallest != i {
      //Found new minimum; remember its index.
      arr.swap(i, smallest);
    }
  }
}
