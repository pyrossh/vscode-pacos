mod rust

enum Shape {
  Circle(int)
  Square(int)
  Rectangle(int, int)
}

trait Comparable {
  fn compareTo()
}

struct Data {
	age: str
}

impl Data {
  fn get(&self, i: int) -> int  {
    return self.age
  }
}

fn main() {
  let data = "123"
  let d = println!("${}")
  for i in 0..1 {
    println(i)
  }
}

fn a(a: i32, b: f32, c: str) 