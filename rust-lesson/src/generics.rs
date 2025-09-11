struct Point<T>{
    x:T,
    y:T,
}

struct Point2<T,U>{
    x:T,
    y:U,
}

// implとは構造体に対してメソッドを実装するためのキーワード
impl<T,U>Point2<T,U>{
    // selfはそのメソッドが呼び出されたインスタンスを指す
    fn mixup<V,W>(self,other:Point2<V,W>)-> Point2<T,W>{
        Point2 { x:self.x, y: other.y }
    }
}

pub fn run() {
    let number_list = vec![34, 50, 25, 100, 65];
    // let mut largest = number_list[0];
    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    // println!("The largest number is {}", largest);

    
    // シングルクォーテーションで囲まれた文字はchar型
    // char型は4バイトで、１文字を表す
    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("{}", largest(number_list));
    println!("{}", largest(char_list));

    let p1 = Point {x:5, y:10};
    let p2 = Point {x:1.0, y:4.0};
    println!("p1.x = {}, p1.y = {}", p1.x, p1.y);
    println!("p2.x = {}, p2.y = {}", p2.x, p2.y);

    let p3 = Point2{x:32,y:1.3};
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let p4 = Point2{x:"hello",y:'c'};
    let p5 = p3.mixup(p4);
    println!("p5.x = {}, p5.y = {}", p5.x, p5.y);

}

// fn largest_i32(list:Vec<i32>) -> i32 {
//     let mut largest = list[0];
//     for number in list {
//         if number > largest {
//             largest = number;
//         }
//     }
//     largest
// }

fn largest<T: PartialOrd + Copy>(list:Vec<T>) -> T {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}