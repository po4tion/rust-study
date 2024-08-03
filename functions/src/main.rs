fn main() {
  println!("main");

  second_function();
  print_number(129);
  let return_value = circle_area(2.1);
  println!("{return_value}");
}

fn second_function() {
  println!("second_function")
}

fn print_number(x: i32) {
  println!("x = {x}")
}

const PI: f64 = 3.141592;

fn circle_area(radius: f64) -> f64 {
  // 식으로 평가되는 마지막 값은 세미콜론이 있으면 안된다.
  PI * radius * radius
}
