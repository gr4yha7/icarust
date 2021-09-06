fn main() {
  println!("hello {}!", "jamal");
  println!("hello {name}!", name="tellorX");
  println!("hello {0} and {1}!", "peter", "paul");
  println!("hello {:b}!", 30);
  println!("hello {number:>width$}!", number=1, width=5);
  println!("hello {number:0>width$}!", number=1, width=5);

  #[derive(Debug)]
  // #[allow(dead_code)]
  struct Structure(i32);

  eprintln!("This struct `{:?}` won't print", Structure(2));

  let pi = 3.141592;
  println!("Pi is roughly {0:.3}", pi);
  let name = "ruth";
  println!("Ruth trimmed down to 3 characters is {0:.3}", name);
}