mod data_source;

use sort_lib;

fn main() {
  let arr1 = &mut data_source::integer();
  sort_lib::insertion_sort(arr1);
  println!("{:?}", arr1);

  let arr2 = &mut data_source::floating_point();
  sort_lib::insertion_sort(arr2);
  println!("{:?}", arr2);

  let arr3 = &mut data_source::str();
  sort_lib::insertion_sort(arr3);
  println!("{:?}", arr3);

  let arr4 = &mut data_source::student();
  sort_lib::insertion_sort(arr4);
  println!("{:?}", arr4);
}
