use std::fmt;

#[derive(Debug)]
struct Vector2D {
  x: isize,
  y: isize,
}

impl fmt::Display for Vector2D {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

fn main() {
  let vector = Vector2D{x: 2, y: 3};
  println!("{}", vector);
  println!("{:?}", vector);
  // println!("{:10.3b}", vector);
}