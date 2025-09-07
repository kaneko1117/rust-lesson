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
}
