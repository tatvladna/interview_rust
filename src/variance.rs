use std::cell::Cell;

pub fn variance() {
    println!("=============== Вариантность ====================");
    /*
    Ковариантность и контравариантность в Rust — это важные концепции системы типов, которые определяют, 
    как типы соотносятся при наследовании или композиции.

    1. Ковариантность
        &'static str — подтип &'a str (так как 'static ⊂ 'a)
    2. Контравариантность
        &'static str ⊃ &str
    3. Инвариантность: никакие замены типов не допускаются
        Cell<T>, RefCell<T> (изменяемые данные) 
        *mut T (сырые указатели)

    Rust использует эти правила, чтобы гарантировать отсутствие висячих ссылок (безопасность памяти) и других небезопасных ситуаций.
    */

    let static_str: &'static str = "Hello";
    let f: fn(&'static str) = print_any_string; // Контравариантность!
    
    apply_to_string(f, static_str);

    // инвариантность
    let cell = Cell::new("Hello");
    let cell_ref: &Cell<&str> = &cell;
    
    let bad_string = String::from("World");
    cell_ref.set(&bad_string);

    println!("{:?}", cell_ref);
    println!("{:?}", cell);
}


fn apply_to_string<'a>(f: fn(&'a str), s: &'a str) {
    f(s)
}

fn print_any_string(text: &str) { // Принимает любой &str
    println!("{}", text);
}
