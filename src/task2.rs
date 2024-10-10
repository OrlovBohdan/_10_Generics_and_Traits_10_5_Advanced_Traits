#[test]

/*

use std::ops::Sub;

#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

// FILL in the blank in three ways: two of them use the default generic  parameters, the other one not.
// Notice that the implementation uses the associated type `Output`.
impl __ {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
        Point { x: 1, y: 3 });

    println!("Success!");
}
*/

fn main() {
    assert_eq!(Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
               Point { x: 1, y: 3 });

    println!("Success!");
}
use std::ops::Sub;

#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

// FILL in the blank in three ways: two of them use the default generic  parameters, the other one not.
// Notice that the implementation uses the associated type `Output`.
//1
/*impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}*/
//2
/*impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}*/
//3
impl<T: Sub<Output = T>> Sub<Point<T>> for Point<T> {
    type Output = Self;

    fn sub(self, other: Point<T>) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/*
Вариант 1: Указание T напрямую, ограничив его как Sub<Output = T>
Этот вариант позволяет ограничить тип T, чтобы он реализовывал трейт Sub с результатом того же типа

Вариант 2: Использование Self = Point<T> как значения по умолчанию для типа Rhs в Sub<Rhs>
Здесь мы применяем дефолтный параметр Rhs = Self, что позволяет определить вычитание для Point<T> и указать Self в качестве выходного типа.

Вариант 3: Определение Sub<Rhs = Self> с типом по умолчанию
Здесь мы можем явно задать тип Rhs в Sub, что аналогично второму варианту, но с явной установкой параметра:
*/



