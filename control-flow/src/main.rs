fn main() {
  let x = 4;

  if x % 2 == 0 {
    println!("짝수 값은 {x} 입니다");
  } else {
    println!("홀수 값은 {x} 입니다");
  }

  let mut counter = 0;

  let return_loop = loop {
    println!("반복!");

    counter += 1;

    if counter == 3 {
      break counter;
    }
  };

  println!("return_loop 값은 {return_loop} 입니다.");

  let mut while_counter = 0;

  while while_counter < 3 {
    println!("while 반복");
    while_counter += 1;
  }

  println!("while_counter 값은 {while_counter} 입니다.");

  let xs = [1, 2, 3, 4, 5];

  for item in xs {
    println!("{item}");
  }

  for item in 17..20 {
    println!("{item}");
  }

  for item in (17..20).rev() {
    println!("{item}");
  }
}
