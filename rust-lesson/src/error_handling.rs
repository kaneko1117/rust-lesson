pub fn run() {
    let res = division_option(10.0, 0.0);
    match res {
        Some(v) => println!("Result: {}", v),
        None => println!("Error: Division by zero"),
    }

    let res = division_result(10.0, 0.0);
    match res {
        Ok(v) => println!("Result: {}", v),
        Err(e) => println!("Error: {}", e),
    }

    let numbers = vec![1, 2, 3];
    match sum(&numbers) {
        Some(v) => println!("Sum: {}", v),
        None => println!("Error: Not enough elements"),
    }
    let numbers = vec![1, 2];
    match sum(&numbers) {
        Some(v) => println!("Sum: {}", v),
        None => println!("Error: Not enough elements"),
    }
}

fn division_option(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 { None } else { Some(x / y) }
}

fn division_result(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err("Division by zero".into())
    } else {
        Ok(x / y)
    }
}

fn sum(a: &[i32]) -> Option<i32> {
    // ?はアウトオブインデックスでエラーになった場合、キャッチしてNoneを返す
    let a0 = a.get(0)?;
    let a1 = a.get(1)?;
    let a2 = a.get(2)?;
    Some(a0 + a1 + a2)
}
