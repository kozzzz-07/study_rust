pub fn run() {
  println!("===ownership run===");

  // let s1 = String::from("hello");
  // let s2 = s1;
  // // s1の所有権はS2にMoveされた
  // println!("{} {}", s1, s2); // s1 err

  let i1 = 1;
  let i2 = i1;
  // Stack内で値がコピーされているだけなのでエラーにならない
  println!("{} {}", i1, i2);
  println!("Stack address i1: {:p}", &i1);
  println!("Stack address i2: {:p}", &i2);

  println!("");

  // 文字列スライスもStackでコピーしているだけなのでエラーにならない
  let sl1 = "hello";
  let sl2 = sl1;
  println!("{} {}", sl1, sl2);
  println!("Stack address sl1: {:p}", &sl1);
  println!("Stack address sl2: {:p}", &sl2);

  println!("");

  // deep copyはヒープに文字列をコピーするのでエラーにならない
  let s3 = String::from("deep");
  let s4 = s3.clone();
  println!("{} {}", s3, s4);
  println!("Stack address s3: {:p}", &s3);
  println!("Stack address s4: {:p}", &s4);
  println!("Heap memory address s3: {:p}", s3.as_ptr());
  println!("Heap memory address s4: {:p}", s4.as_ptr());

  println!("");

  let s5 = String::from("hello");
  println!("Stack address s5: {:p}", &s5);
  println!("Heap memory address s5: {:p}", s5.as_ptr());
  println!("s5 len: {}", s5.len());
  println!("s5 cap: {}", s5.capacity());
  take_ownership(s5);
  // 所有権がs5からsへMoveされた
  // println!("s5: {}", s5); // err

  println!("");

  let s6 = String::from("hello");
  println!("Stack address s6: {:p}", &6);
  println!("Heap memory address s6: {:p}", s6.as_ptr());
  println!("s6 len: {}", s6.len());
  let s7 = take_giveback_ownership(s6);
  // 所有権はs7へ
  println!("s7: {}", s7);
  // スタックはs6と別だが、保有しているデータ（Heap）は同じ
  println!("Stack address s7: {:p}", &7);
  println!("Heap memory address s7: {:p}", s7.as_ptr());
  println!("s6 len: {}", s7.len());

  println!("");

  let s8 = String::from("hello");
  let len = cululate_length(&s8);
  // s8の所有権はs8のまま
  println!("The length of '{}' is {}", s8, len);

  println!("");

  let mut s9 = String::from("hello");
  change(&mut s9);
  println!("Changed s9: {}", s9);

  println!("");

  // イミュータブルなリファレンスは複数作成可能
  let s10 = String::from("hello");
  let r1 = &s10;
  let r2 = &s10;
  println!("s10: {} r1: {} r2: {}", s10, r1, r2);

  // イミュータブルなリファレンスとミュータブルなリファレンスは共存できない
  // let mut s10 = String::from("hello");
  // let r1 = &s10;
  // let r2 = &mut s10; // error
  // println!("s10: {} r1: {} r2: {}", s10, r1, r2);

  println!("");

  let mut s11 = String::from("hello");
  let r1 = &mut s11;
  // ミュータブルな参照が有効なスコープでは、所有権者であってもs11を読みにいけない
  // println!("s11: {}", s11); // error
  // println!("r1: {}", r1);

  // ミュータブルな参照のスコープ外であれば、アクセスが可能となる
  println!("r1: {}", r1);
  println!("s11: {}", s11);

  println!("");

  let mut s12 = String::from("hello");
  let r1 = &s12;
  let r2 = &s12;
  println!("{} and {}", r1, r2);
  // ここ以降でr1/r2が登場しなければミュータブルな参照が作成可能
  let r3 = &mut s12;
  // 参照外しで書き換え
  *r3 = String::from("hello_updated");
  println!("s12: {}", s12);
}

fn take_ownership(s: String) {
  println!("===call take_ownership()===");
  // sは所有権を持ち、スコープを抜けるときにdropする
  println!("Stack address s: {:p}", &s); // s5とは別
  println!("Heap memory address s5: {:p}", s.as_ptr()); // s5が参照していた先と同じ
  println!("s len: {}", s.len());
  println!("s cap: {}", s.capacity());
  println!("take_ownership arg s: {}", s)
}

fn take_giveback_ownership(s: String) -> String {
  println!("===call take_giveback_ownership()===");
  // returnは関数の中でセミコロンなしのもの
  s
}

fn cululate_length(s: &String) -> usize {
  println!("===call cululate_length()===");
  // 参照を受け取ることで、所有権をmoveせずに済む
  s.len()
}

fn change(s: &mut String) {
  println!("===call change()===");
  // ミュータブルな参照を受け取って、データを変更できる
  s.push_str("_world");
}
