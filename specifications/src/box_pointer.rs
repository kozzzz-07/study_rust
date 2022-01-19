enum List {
  Node(i32, Box<List>),
  Nil,
}

pub fn run() {
  // t1 = 32byte
  let t1: (i64, String) = (10, String::from("hello"));

  println!("Stack adress t1: {:p}", &t1);
  println!("Heap memory adress t1.1: {:p}", t1.1.as_ptr());
  println!("Len t1.1: {}", t1.1.len());
  println!("Cap t1.1: {}", t1.1.capacity());

  // タプルのデータはスタックに積まれず、ヒープにタプルの領域(32byte)をとる (例としてタプルを使用しているだけ)
  // box pointerはタプルの先頭アドレスを格納する
  let mut b1 = Box::new(t1);
  // 参照にすることで参照外しを行い変更ができる
  (*b1).1 += "world";
  println!("b1: {} {}", b1.0, b1.1);
  println!("Stack adress b1: {:p}", &b1);
  println!("Heap memory adress tuple data (t1): {:p}", b1);
}
