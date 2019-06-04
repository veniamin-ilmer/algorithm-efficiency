#[path = "../benches/sort/heap.rs"]
mod heap;

fn main() {
  let mut arr = [4,2,1,5,9,2,3,7,6];
  heap::sort(&mut arr);
  println!("{:?}", arr);
}