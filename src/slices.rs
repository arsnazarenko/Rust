//  функция, которая должны вернуть первое слово, найденной в строке
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
//  если в каечтве входого параметра указать &str,
//  то при передаче ссылки на строку, будет автоматически браться срез всей строки

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for i in 1..bytes.len() {
        if bytes[i] == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}


pub fn test() {
    //  Проблема:
    //  Мы возвращаем сам usize, но это число имеет значение только
    //  в контексте &String. Другими словами, поскольку это значение отдельное от
    //  String, то нет гарантии, что оно все ещё будет действительным в будущем.
    {
        let mut s: String = String::from("hello world");
        let word: usize = first_word(&s);
        s.clear();
    }

    //  Решение - СРЕЗЫ
    //  Во внутреннем вредставлении срез - это ссылка на n - байт строки
    //  s и длина среза
    //  Важно - строковые литералы - неизменяемы
    {
        let mut s: String = String::from("hello world");
        let hello: &str = &s[0..5];
        let hell: &str = &s[..4];
        let world: &str = &s[6..];
        let full_word_slice: &str = &s[..];
    }

    {
        let mut s: String = String::from("hello world");
        let first_word_slice: &str = first_word_slice(&s);
        //  Теперь результат метода связан с
        //  исходной строкой, так как содержит ссылку на строку
        //  s.clear подразумевает использование изменяемой ссылки на объект строки
        //  а значит если область видимости среза будет пересекаться с вызовом s.clear()
        //  то будет CE
        //  fixme: s.clear();  // CE
        println!("First word: {}", first_word_slice);
    }

    {
        let mut array: [i32; 5] = [1, 2, 3, 4, 5];
        let slice: &mut [i32] = &mut array[1..4];
        slice[2] = 5;
        assert_eq!(slice, &[2, 3, 5])
    }














}