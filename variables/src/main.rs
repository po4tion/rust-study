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

  let tuples = (2, true, 1.14);
  let (a, b, c) = tuples;
  let first = tuples.0;
  let second = tuples.1;
  let third = tuples.2;

  println!("tuples의 구조 분해 값은 {a} {b} {c}입니다");
  println!(
    "tuples의 구조 분해 값은 {0} {1} {2}입니다",
    tuples.0, tuples.1, tuples.2
  );

  let arrays = [1, 2, 3, 4, 5];
  println!("array의 첫번째 인덱스 값은 {0}입니다", arrays[0]);

  let threes = [3; 100];
  let last = threes[99];

  println!("{last}");
}
