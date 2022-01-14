pub mod sub_a;
mod sub_b;

pub fn run() {
  println!("hoge");

  // サブモジュールの呼び出し
  sub_a::func_a();
  sub_b::func_b();
}
