mod binary_search;
mod selection_sort;
use binary_search::binary_search_iter;

fn main() {
  let ints = [1, 5, 25, 30, 35, 39];
  let response = binary_search_iter(&ints, 35).unwrap();
  println!("{:?}", response);
  println!("{}", ints[response]);
}
