mod vars;

fn main() {
    println!("Hello, world!");

    // モジュールの呼び出し
    vars::run();
    // サブモジュールの呼び出し
    vars::sub_a::func_a();
}
