
enum  IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}
fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4(x1, x2, x3, x4)
        => {println!("Ip V4: {}.{}.{}.{}", x1, x2, x3, x4)},

        IpAddrKind::V6(addr)
        => {println!("Ip V6: {}", addr)}
    };
}
#[derive(Debug)]
enum Message {
    Quite,  //  пустой элемент (unit struct)
    Move{x: i32, y: i32},   //  анонимная структура
    Write(String),  //  единичный кортеж (tuple)
    ChangeColor(i32, i32, i32) //   котреж из трех значений (tuple)
}

impl Message {
    fn call(&self) {
        let msg_code: u8 = match self {
            Message::ChangeColor(a, b, c) => {
                println!("{}, {}, {}", a, b, c);
                10
            },
            Message::Move { x, y} => {
                println!("{}, {}", x, y);
                20
            },
            Message::Quite => {
                30
            },
            Message::Write(msg) => {
                println!("{}", msg);
                40
            },
        };
        println!("Message code = {}", msg_code);
    }

}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    //...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}





//  Привязка значений к шаблонам
fn value_in_cents(coin: Coin) -> u8 {
    //  оператор match может возвращать значение
    //  каждая ветка должна вовзращать значения
    //  одного и того же типа
    let res = match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        //  у данного варианта есть значение UsState
        //  если значение coin совпадает с шаблоном,
        //  то state привяжется к значению параметра UsState
        //  и мы сможем использовать его внутри эотй ветки
        Coin::Quarter(state) => {
            println!("State quater from {:?}", state);
            25
        },
    };
    res
}



pub fn test() {
    //  MATCH

    let number: i32 = 13;
    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        _ => println!("Ain't special"),
    }

    let b: bool = true;
    let binary: u8 = match b {
        false => 0,
        true => 1
    };

    //  ENUMS

    let four: IpAddrKind = IpAddrKind::V4(127, 0, 0, 1);
    let six: IpAddrKind = IpAddrKind::V6("::1".to_string());
    route(four);
    route(six);
    let msg: Message = Message::Move {
        x: 14,
        y: 15
    };
    msg.call();
    //  OPTION
    //  Перечесление имеет два варианта:
    // *  Some<T> (параметризован) - если значение есть
    // *  None - пустое значение

    //  Тип будет выведен компилятором
    let option = Some(5);
    let some_str = Some("a string");

    //  тип в этом случае обязателен, так как компилятор
    //  не может определить тип варианта, который будет
    //  содержать Some, глядя только на значение None
    let absent_number: Option<i32> = None;
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Option::Some(i) => Some(i + 1),
            None => None
        }
    }
    let five: Option<i32> = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    //  ОПЕРАТОР _

    let n: u8 = 2;
    match n {
        1 => { println!("yes") },
        2 => { println!("yes yes") },
        3 => { println!("yes yes yes") },
        //  "_" означает "все остальное", поэтому разместив
        //  его после всех нужных нам значений, мы обработаем все варианты
        //  если хотя бы один вариант не будет обработан
        //  в блоке match, то будет CE
        //  оператор "_" вызывает деструктор на объекте, который мы матчили
        _ => ()
    };

    // Деструктуризация блоком MATCH
    let pair: (i32, i32) = (0, -2);
    match pair {
        (0, x) => println!("First: 0, second {}", x),
        (y, 0) => println!("Second: 0, first {}", y),
        _ => println!("Some values")
    };
    //  размер котрежа известен на этапе компиляции,
    //  поэтому в блок match идет копия pair
    println!("{:?}", pair);

    let col: Color = Color::RGB(122, 17, 40);

    match col {
        Color::Red => println!("Красный"),
        Color::RGB(r, g, b) => println!("r: {}, g: {}, b: {}", r, g, b),
        Color::HSV(h, s, v) => println!("h: {}, s: {}, v: {}", h, s, v),
        _ => println!("Ничего не совпало...")
    };
    println!("{:?}", col);

    let in_heap: String = "str_in_heap".to_string();
    match &in_heap[..] {
        "str" => println!("is str"),
        "in" => println!("a"),
        "heap" => println!("b"),
        _ => println!("none"),
    }

    // УКАЗАТЕЛИ И ССЫЛКИ ПРИ ДЕСТРУКТУРИЗАЦИИ

    let reference: &i32 = &4;
    match reference {
        // &val - это шаблон для ссылочной переменной
        &val => println!("Получаем значение через деструктуризацию {:?}", val),
        _ => println!(),
    };

    match *reference {
        //  здесь происходит передача владения
        //  в данном случае будет взята копия данных, так как типо reference - i32
        //  если бы это был объект, который не поддеридвает операцию копирования, произошл бы move, объект был бы больше не валиден
        val => println!("Получаем через разыменование {:?}", val),
    }

    //  Если изначально наша переменная не является ссылкой,
    //  деструктуризацию по ссылке можно сделать c помощью ключевых слов ref и mut ref

    let _not_ref = 3;

    let ref is_a_ref = 3;
    //  эквивалентно: let is_a_ref: &i32 = &3
    let ref mut is_a_mut_ref = 33;
    //  эквивалентно: let is_a_mut_ref: &mut i32 = &33

    //  ВАЖНО:
    //  оператор ref не позволяет создать изменяеммую ссылочную переменную:
    //  после создания ссылочной переменной не полуится переприсвоить:
    //  is_a_ref = &3 - CE
    //  is_a_mut_ref = &33 - CE


    let value = 5;
    let mut mut_value = 6;

    match value {
        // получаем ссылку
        ref r => println!("{}", r),
    }

    match mut_value {
        ref mut r => {
            *r += 20;
            println!("{}", r);
        },
    }

    let q: Option<String> = Some("Hell00".to_string());
    let mut l: usize = match q {
        //  перемещение q
        Some(str) => {println!("{}", str); str.len()},
        None => {println!("none..."); 0},
    };
    //  CE - q - не валидна
    // fixme: println!("{:?}", q);


    let mut r: Option<String> = Some("Hell00".to_string());
    let mut n: usize = match r {
        //  заимствование q
        Some(ref mut str) => {println!("{}", str); str.clear(); str.len()},
        None => {println!("none..."); 0},
    };
    //  q - валидна, так как происходило заимствование
    println!("{:?}", n);


    let mut x: Option<i32> = Some(123);
    //  внутри Some - примитив, автоматическое копирование
    match x {
        Some(x) => println!("value: {}", x),
        None => println!("none"),
    }
    //  валидность данных сохраняется
    println!("after match: {:?}", x);

    let x: (String, String) = (String::from("hello"), String::from("hello"));
    let res: String = match x {
        (ref a, ref b) if a.eq(b)  => {
            let mut r: String = a.clone();
            r.push_str(b);
            r
        },
        _ => "".to_string(),
    };
    //  валидно
    //  однако при конструкции (a, b) вместо (ref a, ref b) обе ветки оператора
    //  match забирают владение объектами, а дальнейшее использование x уже невозможно
    //  в данном случае будет происходить move так как внутри кортежа String
    println!("{:?}, {:?}", x, res);

    //
    let mut m: Message = Message::ChangeColor(64, 64, 64);
    match m {
        Message::ChangeColor(ref a, ref b, ref c) => a * b * c,
        Message::Quite => -1,
        Message::Write(ref msg) => 0,
        Message::Move{x: ref a, y: ref b} => a * b,
    };
    println!("{:?}", m);
}