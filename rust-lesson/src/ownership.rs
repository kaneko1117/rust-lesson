pub fn run() {
    let s1 = String::from("hello");
    let s2 = s1;
    // s1の所有権はs2にムーブされるので、s1は無効になる
    // ので、コンパイルエラーになる
    // ちなみに所有権がムーブされる対象の型はStringのようなヒープ領域にデータを確保する型
    //　これに対して、i32のようなスタック領域にデータを確保する型はムーブされない = コピーされる
    // println!("{}", s1);
    println!("{}", s2);

    let i1 = 10;
    let i2 = i1;
    // i1の所有権はi2にムーブされないので、i1は有効
    println!("{}", i1);
    println!("{}", i2);
    println!("Stack address of i1 is {:p}", &i1);
    println!("Stack address of i2 is {:p}", &i2);

    let sl1 = "literal";
    let sl2 = sl1;
    println!("{}", sl1);
    println!("{}", sl2);
    println!("Stack address of sl1 is {:p}", &sl1);
    println!("Stack address of sl2 is {:p}", &sl2);

    // deep copyでクローンすることができる
    // クローンとはheap領域に確保されたデータをコピーすること
    //　ただし、クローンはコストが高いので、必要な場合にのみ使うべき
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);
    println!("Stack address of s3 is {:p}", &s3);
    println!("Heap memory address of s3 is {:p}", s3.as_ptr());
    println!("Stack address of s4 is {:p}", &s4);
    println!("Heap memory address of s4 is {:p}", s4.as_ptr());

    let s5 = String::from("hello");
    println!("Before take_ownership, s5 = {}", s5);
    println!("Stack address of s5 is {:p}", &s5);
    println!("Heap memory address of s5 is {:p}", s5.as_ptr());
    println!("Len of s5 is {}", s5.len());
    println!("Cap of s5 is {}", s5.capacity());
    take_ownership(s5);
    // s5の所有権はtake_ownershipにムーブされるので、s5は無効になる
    // ので、コンパイルエラーになる
    // println!("{}", s5);

    fn take_ownership(s: String) {
        println!("{}", s);
        println!("Stack address of s is {:p}", &s);
        // s5のヒープ領域のアドレスと同じになる
        println!("Heap memory address of s is {:p}", s.as_ptr());
        println!("Len of s is {}", s.len());
        println!("Cap of s is {}", s.capacity());
    }

    let s6 = String::from("hello");
    println!("Before take_giveback_ownership, s6 = {}", s6);
    println!("Stack address of s6 is {:p}", &s6);
    println!("Heap memory address of s6 is {:p}", s6.as_ptr());
    let s7 = take_giveback_ownership(s6);
    println!("After take_giveback_ownership, s7 = {}", s7);
    println!("Stack address of s7 is {:p}", &s7);
    println!("Heap memory address of s7 is {:p}", s7.as_ptr());

    fn take_giveback_ownership(s: String) -> String {
        // rustではreturnを使わなくても、最後のセミコロンなしの式がreturnされる
        //s6の所有権はsにムーブし、returnされるときにs7にムーブされる
        s
    }

    // 参照を使うことで、所有権をムーブせずに値を借用できる
    let s8 = String::from("hello");
    let len = calculate_length(&s8);
    // s8の所有権はcalculate_lengthにムーブされないので、s8は有効
    println!("The length of '{}' is {}.", s8, len);
    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    // ミュータブルな参照を使うことで、所有権をムーブせずに値を変更できる
    let mut s9 = String::from("hello");
    println!("Before change, s9 = {}", s9);
    change(&mut s9);
    println!("After change, s9 = {}", s9);
    fn change(s: &mut String) {
        s.push_str(", world");
    }

    // imutableな参照は複数持つことができる
    let s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("r1 = {}, r2 = {}", r1, r2);

    // imutableな参照とmutableな参照は共存することができない
    let mut s11 = String::from("hello");
    let r3 = &s11;
    // let r4 = &mut s11; // コンパイルエラー

    // mutableな参照は1つだけ持つことができる
    // たとえ、所有権者であっても参照で渡した値を読むことはできない
    let mut s12 = String::from("hello");
    let r5 = &mut s12;
    // let r6 = &mut s12; // コンパイルエラー
    // println!("s12 = {}", s12); // コンパイルエラー
    println!("r5 = {}", r5);
    // 参照のスコープはそれが最後に使われた場所までなので、ここでs12を読むことができる
    println!("s12 = {}", s12);

    let mut s13 = String::from("hello");
    let r1 = &s13;
    let r2 = &s13;
    println!("r1 = {}, r2 = {}", r1, r2);
    // r1とr2はここまでしか使われていないので、ここでmutableな参照を作ることができる
    let r3 = &mut s13;
    *r3 += ", world";
    println!("r3 = {}", s13);

}
