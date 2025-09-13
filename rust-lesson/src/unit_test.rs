struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn compare_area(&self, other: &Rectangle) -> bool {
        self.width * self.height > other.width * other.height
    }
}

fn double_value(x: &mut i32) -> i32 {
    *x *= 2;
    *x
}

fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

// テスト用を実行した時だけコンパイルされる
#[cfg(test)]
// サブモジュールとしてtestsを定義
mod tests {
    // 親モジュールの関数や構造体を使うためにインポート
    // useとは、モジュールの中にある関数や構造体をスコープに持ってくるためのキーワード
    // superは親モジュールを指す

    use super::*;

    #[test]
    fn test_rectangle_area_comparison() {
        let rect1 = Rectangle {
            width: 5,
            height: 10,
        };
        let rect2 = Rectangle {
            width: 3,
            height: 15,
        };
        assert!(rect1.compare_area(&rect2));
    }

    #[test]
    fn test_double_value() {
        let mut value = 5;
        assert_eq!(double_value(&mut value), 10);
    }

    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle {
            width: 5,
            height: 10,
        };
        assert_ne!(rect.width * rect.height, 40);
    }

    #[test]
    fn test_greeting() {
        let name = "Alice";
        assert_eq!(greeting(name), "Hello, Alice!");
    }
}
