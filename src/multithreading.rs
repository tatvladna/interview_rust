use std::thread;
use std::sync::{Arc, Mutex};
use std::rc::Rc;

pub fn multithreading() {
    println!("============== Многопоточность ================");
    /*
        1. Безопасность памяти без сборщика мусора
            Система владения предотвращает гонки данных на этапе компиляции
            Правило одного писателя: либо одна изменяемая ссылка (&mut), либо несколько неизменяемых (&)
        2. Типажи для многопоточности
            Send: разрешает передачу владения между потоками
            Sync: разрешает совместный доступ между потоками через ссылки
        3. Стандартные примитивы
            std::thread: создание потоков
            Каналы (std::sync::mpsc): межпоточная коммуникация
        4. Атомарные операции
            std::sync::atomic: атомарные типы для lock-free программирования
        5. Мьютексы с гарантиями
            Mutex и RwLock: защита данных с проверкой на этапе компиляции
        6. Высокоуровневые абстракции
            rayon: параллельные итераторы
            crossbeam: улучшенные каналы и scoped threads
        7. Статическая проверка гонок данных
            Компилятор Rust предотвращает:
                Одновременный доступ на запись и чтение
                Передачу небезопасных ссылок между потоками
                Утечки ресурсов
    */

    send_example();
    sync_example();
    no_data_races();
    rc_example();
    arc_example();
    mutex_guard_example();

}




// Send - можно передавать владение между потоками
fn send_example() {
    let data = vec!["100", "200", "300", "400", "500"]; // Vec реализует Send
    
    thread::spawn(move || {  // move передает владение в поток
        println!("Data in thread: {:?}", data);
    }).join().unwrap();
}

// Sync - можно использовать ссылки из нескольких потоков
fn sync_example() {
    let data = &[1, 2, 3];  // Срез реализует Sync
    
    thread::scope(|s| {
        s.spawn(|| println!("Thread 1: {:?}", data));
        s.spawn(|| println!("Thread 2: {:?}", data));
    });
}

fn no_data_races() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // Клонируем Arc для каждого потока
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            // Компилятор не позволит использовать counter без блокировки
            let mut num = counter.lock().unwrap();
            *num += 1;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

// Rc - только для однопоточного кода
fn rc_example() {
    let data = Rc::new(5);
    let _clone1 = Rc::clone(&data);
    let _clone2 = Rc::clone(&data);
    // Нельзя передать между потоками - ошибка компиляции
    // thread::spawn(move || println!("{}", data));

    println!("Original: {}", data);
    println!("Clone 1: {}", _clone1);
    println!("Clone 2: {}", _clone2);
}

// Arc - для многопоточного кода
fn arc_example() {
    let data = Arc::new(5);
    let data1 = Arc::clone(&data);
    let data2 = Arc::clone(&data);

    let handle1 = thread::spawn(move || println!("Thread 1: {}", data1));
    let handle2 = thread::spawn(move || println!("Thread 2: {}", data2));
    
    handle1.join().unwrap(); // ожидаем завершение
    handle2.join().unwrap(); // всех потоков
}


// Mutex гарантирует, что только один поток в данный момент может изменять данные:
fn mutex_guard_example() {
    let m = Mutex::new(10);
    
    {
        let mut guard = m.lock().unwrap();
        *guard += 1;
    }
    
    println!("{:?}", m.lock().unwrap());
}