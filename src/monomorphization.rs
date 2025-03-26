use std::ops::AddAssign;

pub fn monomorphization() {
    /*
    Мономорфизация — это процесс в компиляторе Rust, при котором обобщённый (generic) 
    код превращается в конкретные реализации для каждого используемого типа на этапе компиляции. ->  высокая производительность

    При написании обобщенной функции (с использованием generics), 
    компилятор Rust генерирует отдельную версию этой функции для каждого конкретного типа, который используется в коде.

    + понятнее
    + высокая происзводительность
    - большой размер бинарника
    - Не подходит для гетерогенных коллекций

    Vec<i32> и Vec<String> — разные типы? → Потому что мономорфизация генерирует отдельные реализации.
    */

    let my_vec = vec![10, 3, 2, 90, 21, 1223];

    match sum_lst(&my_vec) {
        Some(s) => println!("Sum of vector elements: {}", s),
        None => println!("Vector is empty"),
    }

    // Пример с другим вектором: функция sum_lst вызывается для нового объекта
    let my_vec2 = vec![29, 32, 22333, 232, 9007, 45];
    match sum_lst(&my_vec2) {
        Some(s) => println!("Sum of vector elements: {}", s),
        None => println!("Vector is empty"),
    }

    // переместим вектора в кучу
    if let Some(boxed_data) = process_data(my_vec, my_vec2) {
        println!("Data in heap: {:?}", boxed_data);
    }
}


// чтобы работа += нужно AddAssign<T>
// Clone - чтобы можно было скопировать первый элеемнт 
fn sum_lst<T: AddAssign<T> + Clone>(lst: &[T]) -> Option<T> {
    let mut sum = lst.first()?.clone();
    for num in lst[1..].iter().cloned() {
        sum += num;
    }
    Some(sum)
}


fn process_data(vec1: Vec<i32>, vec2: Vec<i32>) -> Option<Box<Vec<i32>>> {
    if !vec1.is_empty() || !vec2.is_empty() {
        let combined: Vec<i32> = vec1.into_iter().chain(vec2.into_iter()).collect();
        Some(Box::new(combined)) 
    } else {
        None
    }
}