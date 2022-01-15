mod mut_immut;
mod string;
mod tuple;
mod vars;

fn main() {
    println!("Hello, world!");

    // モジュールの呼び出し
    vars::run();
    // サブモジュールの呼び出し
    vars::sub_a::func_a();

    mut_immut::run();

    tuple::run();

    string::run();
}
