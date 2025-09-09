enum List {
    // Listは再帰的なデータ構造で、無限のサイズになる可能性があるのでコンパイルエラーになる
    // 再帰的なデータ構造を定義するには、ボックスポインタを使う必要がある
    // ボックスポインタは8バイトの固定長なので、コンパイルエラーにならない
    Node(i32, Box<List>),
    Nil,
}

pub fn run() {
    // スタックの容量は8MB
    // 8MBを超えるとスタックオーバーフローになる
    // 例えば以下のような大きな配列を作るとスタックオーバーフローになる
    // let a1 : [u8 ; 9000000] = [1 ;9000000];

    // ベクター型とは動的配列のこと
    // スタックの中身はptr,len,capacityの3つの情報を持つ
    // ptrは実データのアドレスを指す
    // lenは要素数
    // capは確保できる容量についての情報
    let mut v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    let mut v3 = vec![7, 8, 9];
    println!("Stack address of v1 is {:p}", &v1);
    println!("Stack address of v2 is {:p}", &v2);
    println!("Heep address of v1 is {:p}", v1.as_ptr());
    println!("Len of v1 is {}", v1.len());
    println!("Cap of v1 is {}", v1.capacity());
    v1.insert(1, 10);
    print!("{:?}\n", v1);
    v1.remove(1);
    print!("{:?}\n", v1);
    v1.append(&mut v3);
    print!("{:?}\n", v1);
    print!("{:?}\n", v3);

    // タプル
    let t1 = (10, String::from("hello"));
    println!("Stack address of t1 is {:p}", &t1);
    println!("Heap memory address of t1.1 is {:p}", &t1.1.as_ptr());
    println!("Len of t1.1 is {}", t1.1.len());
    println!("Cap of t1.1 is {}", t1.1.capacity());

    // ボックスポインタ
    // ヒープ領域にデータを確保する
    // ボックスポインタはスタックに確保され、データのポインタを持つ
    // ヒープに確保されたデータの所有権はボックスが持ち、データが所有していた実体はそのままデータが持つ
    let mut b1 = Box::new(t1);
    (*b1).1 += " world";
    println!("{} {}", b1.0, b1.1);
    println!("Stack address of b1 is {:p}", &b1);
    println!("Heap memory address of b1 is {:p}", b1);
}
