pub fn box_vec_arc() {
    println!("=========== Box, Vec, Rc/Arc ================");
    /*
    1. Box<T> — указатель на данные в куче
    */

    let x = Box::new(42); // Аллокация в куче
    println!("{}", *x); // Разыменование
}