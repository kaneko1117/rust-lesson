// Debug traitを実装することで、構造体の内容をデバッグ表示できるようになる
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn create(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    // selfを参照渡しにしないと、所有権がムーブされてしまうので、
    // areaメソッドを実行したら、インスタンスが無効になってしまう
    fn area(&self) {
        println!("Area: {}", self.width * self.height);
    }
}

pub fn run() {
    let user1 = User {
        email: String::from("user1@example.com"),
        username: String::from("user1"),
        sign_in_count: 1,
        active: true,
    };

    // let user2 = user1;
    // println!("Username: {}, Email: {}", user2.username, user2.email);
    // user1の所有権はuser2にムーブされるので、user1は無効になる
    // ので、コンパイルエラーになる
    // println!("{}", user1);

    let mut user2 = user1;
    user2.username = String::from("user2");
    user2.email = String::from("user2@example.com");
    // # をつけると、構造体用にフォーマットされる
    println!("{:#?}", user2);
    let user2 = build_user(String::from("user22@example.com"), String::from("user22"));
    println!("{:#?}", user2);

    let rect = Rectangle::create(30, 50);
    println!("rect is {:#?}", rect);
    rect.area();
    println!("rect is {:#?}", rect);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}
