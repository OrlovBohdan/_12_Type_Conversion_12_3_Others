#[test]

/*
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')' )
                                 .split(',')
                                 .map(|x| x.trim())
                                 .collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point { x: x_fromstr, y: y_fromstr })
    }
}
fn main() {
    // FILL in the blanks in two ways
    // DON'T change code anywhere else
    let p = __;
    assert_eq!(p.unwrap(), Point{ x: 3, y: 4} );

    println!("Success!");
}
*/

fn main() {
    // FILL in the blanks in two ways
    // DON'T change code anywhere else
    let p = Point::from_str("(3, 4)"); // Заповнено
    // або можна використовувати:
    // let p = "(3, 4)".parse::<Point>(); // Інший варіант заповнення

    assert_eq!(p.unwrap(), Point{ x: 3, y: 4} );

    println!("Success!");
}

use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')' )
            .split(',')
            .map(|x| x.trim())
            .collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point { x: x_fromstr, y: y_fromstr })
    }
}




/*
Перше заповнення: let p = Point::from_str("(3, 4)"); — ви безпосередньо викликаєте метод from_str, щоб створити об'єкт Point з рядка.

Другий варіант: let p = "(3, 4)".parse::<Point>(); — це альтернативний спосіб, який використовує метод parse, оскільки у нас є реалізація FromStr для Point.
*/