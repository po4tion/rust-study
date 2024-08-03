fn main() {
  let mut x = 3;
  println!("x의 값은 {x}입니다");

  x = 7;

  println!("x의 값은 {x}입니다");

  const PI: f32 = 3.141592;

  println!("PI의 값은 {PI}입니다");

  let y = 5;
  println!("y의 값은 {y}입니다");

  let y = y + 5;
  println!("y의 값은 {y}입니다");

  {
    let y = y * 2;
    println!("y의 값은 {y}입니다");
  }

  println!("y의 값은 {y}입니다");
}
