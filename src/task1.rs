#[test]

/*
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    // IMPLEMENT fmt method
}

fn main() {
    let origin = Point { x: 0, y: 0 };
    // FILL in the blanks
    assert_eq!(origin.__, "The point is (0, 0)");
    assert_eq!(format!(__), "The point is (0, 0)");

    println!("Success!");
}
*/

fn main() {
    let origin = Point { x: 0, y: 0 };

    // FILL in the blanks
    assert_eq!(origin.to_string(), "The point is (0, 0)"); // Заповнено
    assert_eq!(format!("{}", origin), "The point is (0, 0)"); // Заповнено

    println!("Success!");
}

use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    // IMPLEMENT fmt method
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The point is ({}, {})", self.x, self.y)
    }
}

/*
Метод fmt: Реалізація методу fmt пише рядок у форматі "The point is (x, y)", використовуючи write!.
Перше заповнення: assert_eq!(origin.to_string(), "The point is (0, 0)"); — тут ми викликаємо метод to_string(),
який використовує реалізацію fmt::Display.
Друге заповнення: assert_eq!(format!("{}", origin), "The point is (0, 0)"); — використовуємо макрос format!,
щоб створити рядок, використовуючи той же формат.
*/