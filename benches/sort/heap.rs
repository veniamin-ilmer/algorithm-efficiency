pub fn sort(arr: &mut [u32]) {
  let n = arr.len();
  //Build a heap
  for i in (0 .. n / 2).rev() {
    heapify(arr, n, i);
  }
  //One by one extract an element from heap
  for i in (0 .. n).rev() {
    //Move current root to end
    arr.swap(0, i);
    //Call max heapify on the reduced heap
    heapify(arr, i, 0);
  }
}

//Heapify a subtree rooted with node i, which is an index in arr[].
//n is the size of the heap
fn heapify(arr: &mut [u32], n: usize, i: usize) {
  let mut largest = i;
  let left = 2 * i + 1;
  let right = 2 * i + 2;
  
  if left < n && arr[left] > arr[largest] {
    largest = left;
  }

  if right < n && arr[right] > arr[largest] {
    largest = right;
  }
  
  //If largest is not root
  if largest != i {
    arr.swap(i, largest);
    
    //Recursively heapify the affected sub-tree
    heapify(arr, n, largest);
  }
}
