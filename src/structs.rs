
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

struct Color (i32, i32, i32);   // котрежные структуры
struct Point (i32, i32, i32);   // без названия полей

#[derive(Debug)]    // аннотация, для подключения функциональности печати отладочной информации о структуре
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    //  Ассоциированный функции (без self)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }

    fn area(&self) -> u32 {
        self.width * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}

fn build_user(email: String, username: String) -> User {
    User {
        username: username,
        email: email,
        sign_in_count: 1,
        active: true
    }   // без ; - неявный вовзрат
}

fn build_user_short(email: String, username: String) -> User {
    //  сокращенная инициализация
    User {
        username,
        email,
        sign_in_count: 1,
        active: true
    }   // без ; - неявный вовзрат
}

pub fn test() {
    let mut user1: User = User {
        username: String::from("Nazarenko"),
        email: String::from("rust@edu.com"),
        sign_in_count: 144,
        active: true,
    };
    user1.email = String::from("rust-itmo@edu.com");
    let bad_user: User = build_user("vzroslaya21@rambler.com".to_string(),
                          "Karaseva".to_string());
    let good_user: User = build_user_short("i_love_sneakers_and_rap@gmail.com".to_string(),
                                    "Georgiy".to_string());


    //  СИНТАКСИС СОЗДАНИЯ СТРУКТУРЫ С ПОМОЩЬЮ СИНТАКСИСА ОБНОВЛЕНИЯ
    let new_user: User = User {
        //  в поля username и email установятся указанные значения, все остальные из ..varname
        username: String::from("Sergey Klimenkov"),
        email: String::from("serge_klim@itmo.ru"),
        ..bad_user
    };

    let black: Color = Color(0, 0, 0);
    let end: Point = Point(0, 0, 0);
    println!("r: {}, g: {}, b:{}", black.0, black.1, black.2);
    println!("x: {}, y: {}, z:{}", end.0, end.1, end.2);

    //  ПРИМЕР ИСПОЛЬЗОВАНИЯ СТРУКТУР
    let rect1: Rectangle = Rectangle { width: 30, height: 50};
    println!("The area of rectangle is {} square pixels.", area(&rect1));
    println!("rect1 is {:?}", rect1);   // {:?} - формат вывода известный как Debug

    let rectangle: Rectangle = Rectangle::square(5);
}