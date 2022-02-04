pub fn run() {
  println!("===lifetime-danglingpointer run===");

  lifetime();
  lifetime2();

  danglingpointer();
}

fn lifetime() {
  // ライフタイムが終了するタイミングは実態と参照で違う
  {
    let x = 5; // 'b（実態）

    let r = &x; // 'a（参照）

    println!("r: {}", r); // 'aのライフタイムが終了する（参照が最後に使われた直後）
  } // 'bのライフタイムが終了する（スコープを抜ける時にdrop）
}

fn lifetime2() {
  {
    let mut s = String::from("hello"); // 'b
    let r = &mut s; // 'a

    println!("r: {}", r); // 'aのライフタイムは終了しない

    // println!("r: {}", s); // error

    println!("r: {}", r); // 'aのライフタイムが終了する
    println!("s: {}", s);
  }
}

fn danglingpointer() {
  // ダングリングポインタ
  // 'aが'bより長生きしてはならない

  // let r; // 'a
  // {
  //   let x = 5; // 'b
  //   r = &x;
  // } // 'bがdrop
  // println!("r: {}", r); // 'aのライフタイムが終了
}
