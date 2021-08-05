use core::mem;
use std::cmp::max;
use std::collections::HashMap;

pub fn main() {
    //  VECTOR
    //  Создание, доабвление элементов
    {
        let mut c: Vec<i32> = Vec::new();
        c.push(1);
        c.push(2);
        c.push(3);
        c.push(4);
        //  Макрос для задания начальных значений вектора
        let mut v = vec![1i16, 2, 3];
        //  создать вектор, пзаполненный 100 нулями
        v = vec![0i16; 100];
    }   // <--  память будет освобобждена здесь

    //  Чтение данных из вектора
    {
        let v: Vec<i32> = vec![11, 22, 33, 44, 55, 66, 77, 88, 99, 100];
        let var: &i32 = &v[3];
        let var_by_get = match v.get(3) {
            Some(third) => third,
            None => return, //  вернет magic тип '!', который приводится к любому типо
        };
        println!("{}, {}", var, var_by_get);

        //fixme: let does_not_exit: &i32 = &v[100];  // вызовет panic и программа аварийно завершит работу

        //  Удобно, если в вашей программе предусматривается какое либо поведение при обращению по несуществующему индексу
        //  Пример - пользователь отправляет по сети запрос для получения i-го элемента коллекции
        //  При выходе за размеры коллекции, ситуация обрабатывается в match и пользователю отправляется ответ с предложением
        //  повторить попытку запроса а также информаци о текущем размере коллекции на сервере
        let does_not_exit_by_get: Option<&i32> = v.get(100); //  вернет None
    }

    //  BORROW CHECKER при работе с коллекциями

    //  изменяемые и неизменяемые ссылки на вектор
    //  имеют пересекающиеся жизненные циклы, что является CE
    {
        let mut v: Vec<i32> =  vec![1, 2, 3, 4, 5];
        let first: &i32 = &v[0];// <--------------------- начало жизни first
        v.push(6);// <------------------------------ начало и конец & mut r - изменяемой ссылки на vector

        //fixme: println!("First element: {}", first);// <-------- конец жизни first
    }

    //  корректный код
    //  жизненные циклы изм. и неизм. ссылки не пересекаются
    {
        let mut v: Vec<i32> =  vec![1, 2, 3, 4, 5];
        let first: &i32 = &v[0];// <--------------------- начало и конец жизни first
        v.push(6);// <------------------------------ начало и конец & mut r - изменяемой ссылки на vector
    }

    //  ПЕРЕБОР ЗНАЧЕНИЙ ВЕКТОРА
    {
        let mut v: Vec<i32> = vec![10, 20, 30, 40, 50, 60, 70];
        for i in &v {
            println!("{}", i);
        }

        for j in &mut v {
            *j *= 10;
            println!("{}", j);
        }
    }
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    {

        let enum_size: usize = mem::size_of::<SpreadsheetCell>();
        let str_size: usize = mem::size_of::<String>();
        let i32_size: usize = mem::size_of::<i32>();
        let f64_size: usize = mem::size_of::<i64>();
        let enum_max_option_size: usize = max(max(i32_size, str_size), f64_size);
        println!("Size of any SpreadsheetCell variable: {}\n
        Max of enum fields: {}\n
        String: {}\n
        Int: {}\n
        Float: {}\n
        Memory for storage enum option: {}",
            enum_size,
            enum_max_option_size,
            str_size,
            i32_size,
            f64_size,
            enum_size - enum_max_option_size)
    }

    {
        let mut v: Vec<SpreadsheetCell> = vec![
            SpreadsheetCell::Float(2.2),
            SpreadsheetCell::Int(23),
            SpreadsheetCell::Text(String::from("hi"))
        ];
        for var in &v {
            match var {
                SpreadsheetCell::Float(f) => println!("{}", f),
                SpreadsheetCell::Int(i) => println!("{}", i),
                SpreadsheetCell::Text(str) => println!("{}", str),
            }
        }

        //  HASHMAP
        {

            let mut scores = HashMap::new();
            scores.insert(String::from("Blue"), 10);
            scores.insert(String::from("Yellow"), 50);


            let teams = vec![String::from("Blue"), String::from("Yellow"), String::from("Red")];
            let init_scores = vec![10, 20, 50];
            //  Указание типа HashMap<_,_> здесь обязательно, поскольку метод коллект собирает данные во множество структур
            //  по типу компилятор понимает нужный типо конструируемой коллекции, а типы ключа и значения компилятор выодит сам,
            //  поэтому хватит <_, _>
            let mut scores: HashMap<_,_> = teams.into_iter().zip(init_scores.into_iter()).collect();

            scores.entry(String::from("Blue")).or_insert(50);
            scores.entry(String::from("White")).or_insert(80);

            println!("{:?}", scores);

        }
        //  ХЭШ-МАПА и владение
        //  Для значений с владением - move()
        //  Для типов с трэйтом copy - копирование
        {
            let name = String::from("Ars");
            let surname = String::from("Naz");

            let mut map = HashMap::<String, String>::new();
            map.insert(name, surname);  // move

            //fixme: println!("{}, {}", name, surname);// - CE
            let x = String::from("Ars");
            match map.get(&x) {
                Some(surname) => println!("{}", surname),
                None => (),
            };

            for (ref key, value) in &map {
                println!("{}, {}", key, value);
            }


        }

        {
            let text = "hello world wonderful world";
            let mut map = HashMap::<&str, i32>::new();
            for word in text.split_whitespace() {
                let count = map.entry(word).or_insert(0);
                *count += 1;
            }
            println!("{:?}", map)

        }

    }


}