#[test]

/*
trait Pilot {
    fn fly(&self) -> String;
}

trait Wizard {
    fn fly(&self) -> String;
}

struct Human;

impl Pilot for Human {
    fn fly(&self) -> String {
        String::from("This is your captain speaking.")
    }
}

impl Wizard for Human {
    fn fly(&self) -> String {
        String::from("Up!")
    }
}

impl Human {
    fn fly(&self) -> String {
        String::from("*waving arms furiously*")
    }
}

fn main() {
    let person = Human;

    assert_eq!(__, "This is your captain speaking.");
    assert_eq!(__, "Up!");

    assert_eq!(__, "*waving arms furiously*");

    println!("Success!");
}
*/

fn main() {
    let person = Human;

    // Используем явное приведение типа к трейту Pilot
    assert_eq!(Pilot::fly(&person), "This is your captain speaking.");
    // Используем явное приведение типа к трейту Wizard
    assert_eq!(Wizard::fly(&person), "Up!");

    // Вызываем метод fly структуры Human
    assert_eq!(person.fly(), "*waving arms furiously*");

    println!("Success!");
}

trait Pilot {
    fn fly(&self) -> String;
}

trait Wizard {
    fn fly(&self) -> String;
}

struct Human;

impl Pilot for Human {
    fn fly(&self) -> String {
        String::from("This is your captain speaking.")
    }
}

impl Wizard for Human {
    fn fly(&self) -> String {
        String::from("Up!")
    }
}

impl Human {
    fn fly(&self) -> String {
        String::from("*waving arms furiously*")
    }
}



/*
Первый assert_eq!: Для вызова метода fly из трейта Pilot используется Pilot::fly(&person).
Второй assert_eq!: Для вызова метода fly из трейта Wizard используется Wizard::fly(&person).
Третий assert_eq!: Для вызова метода fly из структуры Human просто используется person.fly().
*/