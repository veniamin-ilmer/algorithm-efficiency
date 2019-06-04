pub fn sort(arr: &mut [u32]) {
  //Sort an array a[0...n-1].
  let gaps = [701, 301, 132, 57, 23, 10, 4, 1];

  //Start with the largest gap and work down to a gap of 1
  for gap in &gaps {
    let gap = *gap;
    //Do a gapped insertion sort for this gap size.
    //The first gap elements a[0..gap-1] are already in gapped order
    //keep adding one more element until the entire array is gap sorted
    for i in gap..arr.len() {
      //add arr[i] to the elements that have been gap sorted
      //save arr[i] in temp and make a hole at position i
      let temp = arr[i];
      //shift earlier gap-sorted elements up until the correct location for a[i] is found
      let mut j = i;
      while j >= gap && arr[j - gap] > temp {
        arr[j] = arr[j - gap];
        j -= gap;
      }
      //put temp (the original a[i]) in its correct location
      arr[j] = temp
    }
  }
}