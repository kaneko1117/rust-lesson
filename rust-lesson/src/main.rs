mod vars;

// constはどのスコープでも使える
// 対してletはブロックの中でしか使えない
const MAX_POINTS: u32 = 100_000;

fn main() {
    // println!("Hello, world!");
    vars::run();

    // rustはデフォルトで変数はimmutable
    let x = 5;
    // x = 10; // これはエラーになる
    let mut _y = 10; // mutをつけるとmutableになる
    _y = 20; // これはOK
    println!("x: {}, y: {}", x, _y);

    // 整数の型はデフォルトでi32
    // アンダーバーを入れるとunusedの警告が出なくなる
    let _i1 = 3;

    // 少数の型はデフォルトでf64
    let _f1 = 3.0;

    println!("{}", usize::BITS);
    println!("Memory address of const is {:p}", &MAX_POINTS);

    let i2: i64 = 1;
    let i3: i64 = 2;

    let s1 = "Helloこんにちは";
    let s2 = "World";

    // 文字列リテラルはstaticに確保される
    // 16バイトで確保される
    // ptr,lenの2つの情報を持つ
    // ptrは実データのアドレスを指す
    // lenは文字列の長さ
    println!("Stack address of s1 is {:p}", &s1);
    println!("Stack address of s2 is {:p}", &s2);

    // as_ptr()でstaticに確保されている実データのアドレスを取得できる
    println!("Static memory address of s1 : {:?}",s1.as_ptr());
    println!("Static memory address of s2 : {:?}",s2.as_ptr());

    println!("Len of s1 : {}",s1.len());
    println!("Len of s2 : {}",s2.len());

    // String型は動的に文字列を扱えるのでheapに確保される
    // 24バイトで確保される
    // ptr,len,capacityの3つの情報を持つ
    // capは確保できる容量についての情報
    // rustが勝手に決めてくれる
    let mut ss1 = String::from("hello");
    let mut ss2 = String::from("hello world");
    println!("Stack address of s1 is {:p}", &ss1);
    println!("Stack address of s2 is {:p}", &ss2);
    println!("Heep memory address of s1 is {:p}", ss1.as_ptr());
    println!("Heep memory address of s2 is {:p}", ss2.as_ptr());
    println!("Len of s1 : {}, cap : {}",ss1.len(),ss1.capacity());
    println!("Len of s2 : {}, cap : {}",ss2.len(),ss2.capacity());

    ss1.push_str(" world");
    println!("After push_str");
    println!("Stack address of s1 is {:p}", &ss1);
    println!("Heep memory address of s1 is {:p}", ss1.as_ptr());
    println!("Len of s1 : {}, cap : {}",ss1.len(),ss1.capacity());  

}
