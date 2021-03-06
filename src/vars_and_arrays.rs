use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!(
        "Первый элемент среза: {0}\n\
            Длина среза: {1}", slice[0], slice.len());
}
//  размер массива является частью типа, его обязательно указывать
fn analyze_array(array: [i32; 500]) {
    println!(
        "Первый элемент среза: {0}\n\
            Длина среза: {1}", array[0], array.len());
}


pub fn test() {
    //  ПРЕМЕННЫЕ НА СТЭКЕ

    //  Примитивы
    let a: i32 = 123;
    //  в случае с примитивами, происходит копирование данных, старые переменные не инвалидируются
    //  потому что значения хранятся на стэке, их размер известен заранее
    //  аналогично с ссылками и указателями, т. к. они имеют фиксированный размер
    let b: i32 = a;
    let mut r: &i32 = &a;
    println!("{0}, {1}, {2}", a, b, *r);
    r = &b;
    println!("{0}, {1}, {2}", a, b, *r);

    // Массивы и срезы

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500]; //   инициализация 0 всех элементов
    let copy_ys: [i32; 500] = ys;   // копирование
    println!("Перывй элемент: {}", xs[0]);
    println!("второй элемент: {}", xs[1]);
    println!("Размер массива: {}", xs.len());

    //  память под массив выделяется на стэке
    println!("Массив занимает {} байт", mem::size_of_val(&xs));
    //  заимствование массива как среза
    analyze_slice(&xs);
    //  копирование массива в функцию, НЕ ПЕРЕМЕЩЕНИЕ
    analyze_array(ys);
    // переменная остается валидной
    analyze_slice(&ys);
}