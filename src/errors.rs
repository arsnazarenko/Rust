use std::fs::File;
use std::io::{ErrorKind, Read};
use std::io;
use std::fs;

pub fn main() {
    // Обраобтка ошибки открытия файла
    // Детально обрабатываем случай, когда файла нет, пробуем создать новый файл
    {
        let f = File::open("hello.txt");
        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error)
                }
            },
        };
    }

    {
        // unwrap
        let file: File = File::open("hello.txt").unwrap();

        // Вызов unwrap аналогичен следующему коду:
        let f: File = match File::open("hello.txt") {
            Ok(file) => file,
            Err(error) => panic!("{}", error),
        };
        // expect - аналогия unwrap с пользовательским текстом об ошибке
        let file: File = File::open("hello.txt").expect("My error text: failed to open file");

        // Вызов expect аналогичен следующему коду:
        let f: File = match File::open("hello.txt") {
            Ok(file) => file,
            Err(error) => panic!("My error text: failed to open file, error:{}", error),
        };
    }

    // ПРОБРОС ОШИБОК
    //  1.
    {
        fn read_username_from_file() -> Result<String, io::Error> {
            let f: Result<File, io::Error> = File::open("hello.txt");
            let mut f = match f {
                Ok(file) => file,
                Err(e) => return Err(e),   // ранний выход из функции
            };

            let mut s: String = String::new();


            match f.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }
        }
        read_username_from_file();
    }

    //  2.
    {
        fn read_username_from_file_short() -> Result<String, io::Error> {
            // operator ? делает следующее:
            // если результат Ok(v), то возвращает v,
            // если резльтат Err(e), делает return Err(e) - ранний выход из функции, возвращая Err(?)
            let mut f = File::open("hello.txt")?;
            let mut s = String::new();
            // Если Ok(v) , то возвращается значение v - в данном случае количество прочитанных байт
            // его мы игнорируем
            // иначе возвращается Err(e) с выходом из функции с помощью return
            f.read_to_string(&mut s)?;
            Ok(s)
        }
        read_username_from_file_short();
    }

    //  3.
    {
        fn read_username_from_file_short_short() -> Result<String, io::Error> {
            let mut s = String::new();
            File::open("hello.txt")?.read_to_string(&mut s)?;
            Ok(s)
        }
        read_username_from_file_short_short();
    }

    //  4.
    {
        fn read_str_from_file() -> Result<String, io::Error> {
            fs::read_to_string("hello.txt")
        }
        read_str_from_file();
    }
}