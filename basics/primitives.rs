fn main() {
  let exists: bool = false;
  let n = 2828299292i64;
  let mut name = "john";
  let array: [i8; 5] = [-1, 2, 4, 0, 4];
  let tup: (f32, bool, char, u32) = (31.9, true, 'Å', 45);
  name = "shadowy";

  println!("Boolean - exists: {}", exists);
  println!("64-bit integer - n: {}", n);
  println!("String name - name: {}", name);
  println!("Array first - array: {}", array[0]);
  println!("Tuple - first: {}", tup.0);
  println!("Tuple - second: {}", tup.1);
  println!("Tuple - third: {}", tup.2);
  println!("Tuple - last: {}", tup.3);
}