pub fn run() {
  // 文字列スライス型 スタック内では16byte取っている(ptr,len)(文字列は静的領域)
  let s1 = "あいう";
  // 先頭アドレス(参照)
  println!("Static memory adress: {:?}", s1.as_ptr());
  // バイト数
  println!("len: {:?}", s1.len());

  // String型 スタック内では24byte取っている(ptr,len,cap)(文字列はHeap領域)
  let mut s2 = String::from("hello");
  // 先頭アドレス（所有）
  println!("Static memory adress: {:?}", s2.as_ptr());
  // バイト数
  println!("len: {:?}", s2.len());
  // キャパシティバイト数
  println!("cap: {:?}", s2.capacity());

  s2.push_str("_hoge");
  println!("{}", s2);
}
