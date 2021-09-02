use std::fmt::Display;


//  ОБОБЩЕННЫЕ ТИПЫ В МЕТОДАХ
fn largest_32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &i in list {
        if i > largest {
            largest = i;
        }
    }
    largest
}
//  PartialOrd - типаж, в котором и определен оператор ' > '

//  Copy - типаж, который говрит о том, что после
//  перемещения данные остаются валидными - то есть просто копируются
//  такой типаж реализуют типы с известным размером, которые хранятся на стэке

//  1 вариант - только для стэковых объектов - Copy
fn largest_copy<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest: T = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//  2 вариант - для любых объектов
//  Так как типаж Clone верен для всех объектов, реализующих типаж Copy
//  Но происходит много ненужных аллокаций в случае объектов на куче
fn largest_clone<T: PartialOrd + Clone>(items: &[T]) -> T {
    let mut l = items[0].clone();
    for i in items {
        if *i > l {
            l = i.clone();
        }
    }
    l
}
//  3 вариант - возвращаем ссылку &T без лишних копирований и копирований
fn largest_ref<T: PartialOrd>(items: &[T]) -> &T {
    let mut largest_ref: &T = &items[0];
    for i in items {
        if *i > *largest_ref {
            largest_ref = i;
        }
    }
    largest_ref
}


//  ОБОБЩЕННЫЕ ТИПЫ В СТРУКТУРАХ
struct Point<T> {
    x: T,
    y: T
}
// impl<T> - показывает что T - это обобщенный тип
//  и методы реализуются для типа Point<T>
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
//  определение методов только для типа Point<f32>
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

//  ОГРАНИЧЕНИЕ ОБОБЩЕННЫХ МЕТОДОВ ТРЭЙТАМИ
struct Pair<T> {
    x: T,
    y: T
}
//  синтаксис impl<T> позволяет указать, для какого типа T будут реализованы методы
//  в данном случае для любого внтуренннего типа Т будут существовать метод new
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}
//  Pair<T> реализует метод cmp_display толлько если внутрениий тип Т реализует типажи Display и PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
//  ПОЛЕЗНО

//  Условная реализация типажа для любого типа, который реализует другой типаж!!
// impl<T: Display> ToString for T {
//     todo!()
// }

//  Рашифровка:
//  Метод to_string определенный типажом ToString можно вызвать для любого типа,
//  который реализует типаж Display


//  ОБОБЩЕННЫЕ ТИПЫ СТРУКТУР + ОБОБЩЕННЫЕ ПАРАМЕТРЫ МЕТОДОВ
#[derive(Debug)]
struct OtherPoint<T, U> {
    x: T,
    y: U
}

impl<T, U> OtherPoint<T, U> {
    fn mixup<V, W>(self: OtherPoint<T, U>, other: OtherPoint<V, W>) -> OtherPoint<T, W> {
        OtherPoint{
            x: self.x,
            y: other.y
        }
    }
}


pub fn main() {
    let vi = vec![1, 2, 3, 4, 5, 6];
    let vch = vec!['y', 'b', 'y', 'z', 'r', 'a'];
    let vs = vec!["a", "aaa", "ab", "abb"];
    println!("{}", largest_copy(&vi));
    println!("{}", largest_copy(&vch));
    println!("{}", largest_copy(&vs));


    let vstr = vec!["a".to_string(), "aaa".to_string(), "ab".to_string(), "abb".to_string()];
    println!("{}", largest_clone(&vstr));
    println!("{}", largest_clone(&vi));
    println!("{}", largest_clone(&vch));
    println!("{}", largest_clone(&vs));


    let largest_ref1 = largest_ref(&vstr);
    println!("{}", largest_ref1);
    let largest_ref2 = largest_ref(&vi);
    println!("{}", largest_ref2);
    let largest_ref3 = largest_ref(&vch);
    println!("{}", largest_ref3);
    let largest_ref4 = largest_ref(&vs);
    println!("{}", largest_ref4);



    let p: Point<i32> = Point {x: 1, y: 2};
    let pf: Point<f32> = Point {x: 10.0, y: 10.0};
    println!("x: {}", p.x());
    println!("x: {}", pf.x());
    println!("distance: {}", pf.distance_from_origin());




    let a: Pair<i32> = Pair::new(1, 4);
    let b: Pair<i32> = Pair::new(1, 4);
    let c: Pair<Pair<i32>> = Pair::new(a, b);

    let d: Pair<i32> = Pair::new(1, 4);
    let e: Pair<i32> = Pair {x: 1, y: 2};
    d.cmp_display();
    e.cmp_display();
    // c.cmp_display() -   CE;

    let p1 = OtherPoint { x: 5, y: 5.5 };
    let p2 = OtherPoint {x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);
    println!("{:?}", p3);

}

//