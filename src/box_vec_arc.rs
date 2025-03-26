use std::rc::Rc;
use std::sync::Arc;
use std::thread;


pub fn box_vec_arc() {
    println!("=========== Box, Vec, Rc/Arc ================");
    /*
    1. Box<T> — указатель на данные в куче
    2. Vec<T> — динамический массив
    3. Rc<T>/Arc<T> — подсчёт ссылок
        Rc - однопоточность
        Arc - многопоточность
    */

    let x = Box::new(42); // Аллокация в куче
    println!("{}", *x); // Разыменование


    
    let a = Rc::new(42);
    let b = a.clone(); // Увеличивает счётчик
    println!("a: {}, b: {}", a, b);


    let val = Arc::new(60);
    let val_clone = Arc::clone(&val);

    thread::spawn(move || {
        println!("{}", val_clone); // безопасно для другого потока
    }).join().unwrap();


    let v = vec![1, 2, 3];
    println!("Size of Vec: {}", std::mem::size_of_val(&v));
    println!("Size of data: {}", v.len() * std::mem::size_of::<i32>());
}