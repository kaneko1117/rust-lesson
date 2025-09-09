pub fn run(){
    // スタックの容量は8MB
    // 8MBを超えるとスタックオーバーフローになる
    // 例えば以下のような大きな配列を作るとスタックオーバーフローになる
    // let a1 : [u8 ; 9000000] = [1 ;9000000];

    // ベクター型とは動的配列のこと
    // スタックの中身はptr,len,capacityの3つの情報を持つ
    // ptrは実データのアドレスを指す
    // lenは要素数
    // capは確保できる容量についての情報
    let mut v1 = vec![1,2,3];
    let v2 = vec![4,5,6];
    let mut v3 = vec![7,8,9];
    println!("Stack address of v1 is {:p}", &v1);
    println!("Stack address of v2 is {:p}", &v2);
}