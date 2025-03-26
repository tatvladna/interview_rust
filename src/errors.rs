use std::fs::File;
use std::io::Read;

pub fn errors() {
    println!("========= Обработка ошибок ============");
    /*
    Основные способы обработок ошибок:
        - match
        - unwrap(): для прототипов
        - expect(): как unwrap() + возможность написать сообщение
        - ?: элегантно
    */

    match read_file("../data/data.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    match read_file("../data/nonexistent.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    let my_vec = vec!["a", "b", "c", "d", "e"];
    match read_vec(&my_vec) {
        Some(first_element) => println!("First element: {}", first_element),
        None => eprintln!("Vector is empty"),
    }
    let mut empty_vec = Vec::new();
    empty_vec.push(800);
    empty_vec.pop();
    match read_vec(&empty_vec) {
        Some(first_elem) => println!("{}", first_elem),
        None => println!("Vector is empty"),
    }

    divide(15, 3);

    let file = File::open("../Cargo.toml").unwrap(); // есть риск паники
    println!("Cargo.toml content: {:?}", file.metadata());

}

// Result<T, E> — Основной тип для обработки ошибок
// он не вызывает панику и не останавливает программу, просто выводит ошибку
fn read_file(path: &str) -> Result<String, std::io::Error> {
    // ? автоматически возвращает Err если было ошибка
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Option<T> — Для отсутствующих значений
fn read_vec<T>(vec: &[T]) -> Option<&T> {
    if !vec.is_empty() {
        vec.first()
    } else {
        None
    }

}

// panic! — Для невосстановимых ошибок
// Когда нужно, чтобы программа завершилась
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero!");
    }
    a / b
}