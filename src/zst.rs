pub fn zst() {
    println!("============= ZST: Zero-Sized Types ===================");

    /*
    ZST (Zero-Sized Types) — это типы, которые занимают 0 байт в памяти, 
    но при этом играют важную роль в системе типов Rust.

    Используются:
        - статической проверки
        - Пустые коллекции
        - Реализация трейтов

    Встроенные ZST:
        ()	Пустой кортеж (unit-тип)
        PhantomData	Маркер для generic-логики
        [T; 0]	Пустой массив
        fn()	Функциональный тип без аргументов

    Практическое применение:
        - безопасное API
        - Статическая проверка
        - Контроль памяти
    */

    let db = Database::new();
    db.delete_user(AdminAccess);
    // db.delete_user(UserAccess); // ошибка компиляции: UserAccess не является ZST
}


struct AdminAccess; // ZST

struct Database {
    //...
}

impl Database {
    fn new() -> Self {
        Self {}
    }
    // только админам можно удалять пользователей
    fn delete_user(&self, _: AdminAccess) {
    }
}

