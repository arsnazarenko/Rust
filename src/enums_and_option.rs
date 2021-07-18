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

enum Message {
    Quite,  //  пустой элемент (unit struct)
    Move{x: i32, y: i32},   //  анонимная структура
    Write(String),  //  единичный кортеж (tuple)
    ChangeColor(i32, i32, i32) //   котреж из трех значений (tuple)
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

impl Message {
    fn call(&self) {
        let msg_code: u8 = match self {

            Message::ChangeColor(ref a, ref b, ref c) => {
                10
            },
            Message::Move { ref x, ref y} => {
                20
            },
            Message::Quite => {
                30
            },
            Message::Write(ref msg) => {
                40
            },
        };
        println!("Message code = {}", msg_code);
    }
}

pub fn test() {
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
        3 => { println!("yes yes") },
        //  "_" означает "все остальное", поэтому разместив
        //  его после всех нужных нам значений, мы обработаем все варианты
        //  если хотя бы один вариант не будет обработан
        //  в блоке match, то будет CE
        _ => ()
    };
}






