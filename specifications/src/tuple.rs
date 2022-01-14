pub fn run() {
  println!("===tuple run===");

  let t1 = (100, 200, "hoge");
  // 取得方法は二種類
  let (x, y, z) = t1;
  println!("{} {} {}", x, y, z);
  println!("{} {} {}", t1.0, t1.1, t1.2);

  // 入れ子
  let mut t2 = ((0, 1), (2, 3));

  // a = (0,1), b = (1,2)
  let (a, b) = t2;
  println!("{:?} {:?}", a, b);
  println!("{:?} {:?}", a.0, a.1);

  let ((ref mut ptr_0, ref mut ptr_1), _) = t2;
  println!("ptr_0 ptr_1: {} {}", ptr_0, ptr_1);

  // 参照外しでアクセスする
  *ptr_0 = 10;
  *ptr_1 = -1;
  println!("{:?}", t2);
}
