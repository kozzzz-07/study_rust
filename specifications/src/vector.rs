pub fn run() {
  // let a1: [u8; 9000000] = [1; 9000000];

  // Vector スタック内では24byte(ptr,len,cap)取っている
  let mut v1 = vec![1, 2, 3, 4];
  let v2 = vec![5, 6, 7, 8];
  let mut v3 = vec![9, 10];

  println!("Stack address: {:p}", &v1);
  println!("Stack address: {:p}", &v2);

  // 実データの先頭アドレス（Heap）
  println!("Heap memory address: {:p}", v1.as_ptr());

  println!("v1 len: {}", v1.len());
  println!("v1 cap: {}", v1.capacity());
  v1.insert(1, 10);
  println!("v1 :{:?}", v1);
  v1.remove(0);
  println!("v1 :{:?}", v1);
  v1.append(&mut v3);
  println!("v1 :{:?}", v1);
  println!("v3 :{:?}", v3);
}
