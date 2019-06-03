#[path = "../benches/sort/insertion.rs"]
mod insertion;

fn main() {
  let mut arr = [4,2,1,5,9,2,3,7,6];
  insertion::sort(&mut arr);
  println!("{:?}", arr);
}