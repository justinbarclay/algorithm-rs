use std::cmp::Ordering::*;

#[allow(dead_code)]
pub fn quicksort_naive<T: Copy + Ord>(list: &[T]) -> Vec<T>{
  if list.len() < 2{
    return list.to_vec();
  }
  let pivot_point = list.len()/2;
  let pivot = list[pivot_point];

  let mut lesser = Vec::new();
  let mut greater = Vec::new();
  let mut equal = Vec::new();
  equal.push(pivot);

  for item in list.iter().skip(1){
    match item.cmp(&pivot) {
      Less => lesser.push(item.clone()),
      Greater => greater.push(item.clone()),
      Equal => equal.push(item.clone())
    }
  }

  [quicksort_naive(&lesser),
   equal,
   quicksort_naive(&greater)].concat()
}

#[cfg(test)]
mod test{
  use super::*;

  #[test]
  fn quicksort_naive_should_pass(){
    let unsorted = vec![
      691040, 83642, 993016, 707058, 383979, 710110, 173194, 338622, 305874, 33962, 237030,
      632260, 481490, 413149, 741138, 762856, 345386, 478834, 20818, 624760, 458200, 237089,
      887982, 994813, 667671, 365350, 375510, 420190, 884819, 928030, 542702, 950454, 717027,
      721050, 318440, 334246, 302284, 721610, 851794, 583184, 822652, 667434, 864259, 869115,
      340056, 618758, 53453, 565501, 793777, 466741,
    ];

    let sorted = vec![
      20818, 33962, 53453, 83642, 173194, 237030, 237089, 302284, 305874, 318440, 334246, 338622,
      340056, 345386, 365350, 375510, 383979, 413149, 420190, 458200, 466741, 478834, 481490,
      542702, 565501, 583184, 618758, 624760, 632260, 667434, 667671, 691040, 707058, 710110,
      717027, 721050, 721610, 741138, 762856, 793777, 822652, 851794, 864259, 869115, 884819,
      887982, 928030, 950454, 993016, 994813,
    ];
    assert_eq!(
      sorted,
      quicksort_naive(&unsorted)
    );
  }
}
