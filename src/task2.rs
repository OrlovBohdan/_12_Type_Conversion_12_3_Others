#[test]

/*
// To use `from_str` method, you need to introduce this trait into the current scope.
use std::str::FromStr;
fn main() {
    let parsed: i32 = "5".__.unwrap();
    let turbo_parsed = "10".__.unwrap();
    let from_str = __.unwrap();
    let sum = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);

    println!("Success!");
}
*/


// To use `from_str` method, you need to introduce this trait into the current scope.

fn main() {
    let parsed: i32 = "5".parse().unwrap(); // Заповнено
    let turbo_parsed: i32 = "10".parse().unwrap(); // Явно вказаний тип i32
    let from_str = i32::from_str("20").unwrap(); // Заповнено
    let sum = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);

    println!("Success!");
}

use std::str::FromStr;

/*
Перше заповнення: let parsed: i32 = "5".parse().unwrap(); — тут використовується метод parse() для перетворення рядка "5" в i32. unwrap() використовується для отримання значення, оскільки parse() повертає Result.

Друге заповнення: let turbo_parsed = "10".parse().unwrap(); — аналогічно, рядок "10" перетворюється в i32.

Третє заповнення: let from_str = i32::from_str("20").unwrap(); — використовується метод from_str для перетворення рядка "20" в i32.

Явний тип: Явно вказується тип для turbo_parsed як i32: let turbo_parsed: i32 = "10".parse().unwrap();. Це дозволяє компілятору зрозуміти, в який тип потрібно перетворити рядок "10".
*/