#[test]

/*

struct Container(i32, i32);

// USING associated types to re-implement trait Contains.
// trait Contains {
//    type A;
//    type B;

trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    // Grab the first number.
    fn first(&self) -> i32 { self.0 }

    // Grab the last number.
    fn last(&self) -> i32 { self.1 }
}

fn difference<A, B, C: Contains<A, B>>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}
*/

fn main() {
    let number_1 = 3;
    let number_2 = 10;
    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
             &number_1, &number_2,
             container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}


struct Container(i32, i32);

// Переписываем трейт Contains с использованием ассоциированных типов
trait Contains {
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

// Реализуем трейт для Container с конкретными типами A и B
impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    fn first(&self) -> i32 { self.0 }
    fn last(&self) -> i32 { self.1 }
}

// Изменяем функцию difference для работы с контейнерами, которые реализуют Contains с ассоциированными типами
fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}


/*
Ассоциированные типы в трейте Contains:

Добавлены type A; и type B; как ассоциированные типы в трейте Contains. Это означает, что реализации этого трейта будут определять конкретные типы для A и B.
Реализация трейта Contains для Container:

В реализации трейта для Container указано, что A и B являются i32. Методы contains, first и last определяются, как и прежде.
Функция difference:

Функция difference теперь работает с любым типом, реализующим трейт Contains с ассоциированными типами.
*/

