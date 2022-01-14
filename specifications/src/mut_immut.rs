// 定数の定義
const NUMBER: i32 = 100_000;

pub fn run() {
  println!("===mut_immut run===");

  immut();
  mutable();
}

fn immut() {
  // 使用していない関数・変数には先頭に_をつけるとwarningが消える
  let _x = 1;
  // immutable error
  // _x = 2;

  // シャドーイング
  let y = 1;
  println!("y is: {:p}", &y);
  let y = y + 1;
  println!("y is: {:p}", &y);
  let y = y * 1;
  println!("y is: {:p}", &y);
  {
    let y = 0;
    println!("[scope in] y is: {}", y);
  }
  println!("[scope out] y is: {}", y);
}

fn mutable() {
  let mut x = 1;
  x = 3;
  println!("{}", x);

  x = NUMBER;
  println!("{}", x);

  println!("Memory adreess: {:p}", &NUMBER);
}
