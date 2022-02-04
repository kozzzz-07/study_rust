mod box_pointer;
mod lifetime_danglingpointer;
mod mut_immut;
mod ownership;
mod string;
mod tuple;
mod vars;
mod vector;

fn main() {
    println!("Hello, world!");

    // モジュールの呼び出し
    vars::run();
    // サブモジュールの呼び出し
    vars::sub_a::func_a();

    mut_immut::run();

    tuple::run();

    string::run();

    vector::run();

    box_pointer::run();

    ownership::run();

    lifetime_danglingpointer::run();
}
