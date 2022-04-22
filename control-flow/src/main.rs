fn main() {
  // let cond = true;
  // won't compile with mismatched types
  // let number = if cond { 5 } else { "siz" };

  // fizzbuzz(1000);

  // loops
  let mut counter = 0;
  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2;
    }
  };
  print!("The result is {}", result);

}

// fn fizzbuzz(x: i32) {
//   let mut current = 0;

//   while current <= x {
//     current += 1;

//     if current % 3 == 0 { print!("fizz") }
//     if current % 5 == 0 { print!("buzz") }
//     if !(current % 5 == 0 || current % 3 == 0) {
//       print!("{}", current)
//     }
//     println!()
//   }
// }