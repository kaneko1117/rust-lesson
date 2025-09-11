pub fn run() {
    let st1 = String::from("x");
    let st2 = String::from("y");

    let result = get_longest(&st1, &st2);
    println!("The longest string is {}", result);

    let st3 = String::from("longest string");
    {
        let st4 = String::from("short");
        let result2 = get_longest(&st3, &st4);
        println!("The longest string is {}", result2);
    }
    // st4はスコープを抜けているので、参照できない
    // let result3 = get_longest(&st3, &st4);
}

//. generic lifetime annotationとは、関数の引数や戻り値にライフタイムを指定すること
//. ライフタイムとは、参照が有効な期間のこと
//. ライフタイムを指定することで、コンパイラが参照の有効期間をチェックできるようになる
//. 例えば、以下の関数は、xとyのライフタイムが同じであることを保証する
//. つまり、xとyのどちらかがスコープを抜けた場合、もう一方も無効になる
//. これにより、ダングリングポインタを防ぐことができる
fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 以下のような関数はコンパイルエラーになる
// なぜなら、let s = String::from("hello");で作られたsは関数のスコープを抜けると無効になる
// そのため、&sを返すことはできない
// fn dummy1<'a>() -> &'a str {
//     let s = String::from("hello");
//     &s
// }

// fn dummy2() -> i32 {
//     let i = 10;
//     i
// }
