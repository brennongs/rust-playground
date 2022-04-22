fn notes() {
  // == Primitive Types ==
  // - integer (i8, i16, i32[d], i64, i128)
  // - unsigned integer (u8, u16, u32, u64, u128)
  // - float (f32, f64[d])
  // - boolean (bool)
  // - character (char)

  // Important to remember about primitive data types:
  // - INTEGERS CAN OVERFLOW - 
  // When you've allocated a u<n> or i<n>,
  // if you try to assign that variable to a value out of range
  // (i8 range is 0..=255)
  // it will loop back round to the minimum
  // (let n i8 = 256 // n == 0)

  // == Compound Types ==
  // # Tuples #
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let (_, y, __) = tup;

  println!("The value of y is: {}", y); // "The value of y is: 6.4"

  // can also use dot notation
  let one = tup.2;
  println!("The value of one is: {}", one); // what you'd expect

  // # Arrays #
  let typed_arr: [i32; 5] = [1, 2, 3, 4, 5];
  let assumed_arr = [3; 5]; // allocates array with 5 3's in it.
  let typed_arr_first = typed_arr[0];
  let assumed_arr_last = assumed_arr.last();
}

use std::io;

fn main() {
  let a = [1, 2, 3, 4, 5];

  println!("Please enter an array index.");

  let mut index = String::new();

  io::stdin()
    .read_line(&mut index)
    .expect("Failed to read line");

  let index: usize = index.trim()
    .parse()
    .expect("Index entered was not a number");

  // will panic at runtime if index is out of range
  let element = a[index];

  println!(
    "The falue of the element at index {}, is: {}",
    index,
    element
  );
}
