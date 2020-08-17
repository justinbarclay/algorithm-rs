use std::cmp::Ordering::*;

pub fn binary_search_iter<T: Ord>(items: &[T], target: &T) -> Option<usize>{
  let mut low = 0;
  let mut high = items.len() - 1;

  while low < high {
    let mid = (low + high) / 2;
    match target.cmp(&items[mid]){
      Less => high = mid - 1,
      Greater => low = mid + 1,
      Equal => return Some(mid)
    }
  }

  None
}

pub fn binary_search_recursive<T: Ord>(items: &[T], target: &T) -> Option<usize>{
  binary_search_recursive_helper(items, target, 0, items.len() - 1)
}

fn binary_search_recursive_helper<T: Ord>(items: &[T], target: &T, left: usize, right: usize) -> Option<usize>{

  if left > right {
    return None
  }

  let mid = (left + right) / 2;

  match target.cmp(&items[mid]) {
    Less => binary_search_recursive_helper(items, target, left, mid - 1),
    Greater => binary_search_recursive_helper(items, target,mid + 1, right),
    Equal => Some(mid)
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn correct_binary_search_recur(){
    let correct_arr = [
      1, 10, 20, 47, 59, 63, 75, 88, 99,
      107, 120, 133, 155, 162, 176, 188,
      199, 200, 210, 222
    ];

    for i in 0..correct_arr.len() {
      assert_eq!(i, binary_search_recursive(&correct_arr, correct_arr[i]).unwrap());
    }
  }

  #[test]
  fn correct_binary_search_iter() {
    let correct_arr = [
      1, 10, 20, 47, 59, 63, 75, 88, 99,
      107, 120, 133, 155, 162, 176, 188,
      199, 200, 210, 222
    ];
    for i in 0..correct_arr.len() {
      assert_eq!(i, binary_search_iter(&correct_arr, correct_arr[i]).unwrap());
    }
  }
  #[test]
  fn incorrect_binary_search_iter() {
    let correct_arr = [
      1, 10, 20, 47, 59, 63, 75, 88, 99,
      107, 121, 133, 155, 162, 176, 188,
      199, 200, 210, 222
    ];
    for i in 0..correct_arr.len() {
      assert_eq!(None, binary_search_iter(&correct_arr, correct_arr[i] + 1000));
    }
  }
}
