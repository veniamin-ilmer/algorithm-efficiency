use std::mem;

//Sort an array in place and return the number of writes.
pub fn sort(arr: &mut [u32]) {
	//Loop through the array to find cycles to rotate.
	for cycle_start in 0 .. arr.len() - 1 {
		let mut item = arr[cycle_start];
		
		//Find where to put the item.
		let mut pos = cycle_start;
		for i in cycle_start + 1 .. arr.len() {
			if arr[i] < item {
				pos += 1;
			}
		}
		
		//If the item is already there, this is not a cycle.
		if pos == cycle_start {
			continue;
		}
    
    //Otherwise, put the item there or right after any duplicates.
    while item == arr[pos] {
      pos += 1;
    }
    mem::swap(&mut arr[pos], &mut item);
    
    //Rotate the rest of the cycle.
    while pos != cycle_start {
      //Find where to put the item.
      pos = cycle_start;
      for i in cycle_start + 1 .. arr.len() {
        if arr[i] < item {
          pos += 1;
        }
      }
      
      //Put the tiem there or right after any duplicates.
      while item == arr[pos] {
        pos += 1;
      }
      mem::swap(&mut arr[pos], &mut item);
      
    }
	}
}