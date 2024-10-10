#[test]

/*

use std::fmt;

// DEFINE a newtype `Pretty` here


impl fmt::Display for Pretty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.0.clone() + ", world")
    }
}

fn main() {
    let w = Pretty("hello".to_string());
    println!("w = {}", w);
}*/
fn main() {
    let w = Pretty("hello".to_string());
    println!("w = {}", w);
}
use std::fmt;

// Определяем новый тип `Pretty` как структуру
struct Pretty(String);

impl fmt::Display for Pretty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.0.clone() + ", world")
    }
}
/*
Определение нового типа Pretty: Мы создаем структуру Pretty, которая содержит одно поле типа String.

Реализация трейт fmt::Display: Мы реализуем метод fmt, который форматирует вывод.
В данном случае он выводит строку, добавляя ", world" к содержимому поля self.0 и обрамляя его кавычками.

Использование нового типа в main: Мы создаем экземпляр Pretty с "hello", а затем выводим его с помощью println!.
Это вызывает метод fmt, и результат будет заключен в кавычки с добавленным текстом.
*/

