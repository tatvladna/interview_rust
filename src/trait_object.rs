pub fn trait_object() {
    println!("========= Trait Object ============");
    /*
    Trait Object — это механизм динамической диспетчеризации в Rust, 
    позволяющий работать с разными типами через единый интерфейс трейта.
    */

    // помещаем trait object в кучу с помощью Box
    let circle: Box<dyn Draw> = Box::new(Circle::new(65.0));
    circle.draw();

    /*
    Можно ли для любого трейта создать trait object?
        Нет, не для любого. Трейт должен быть object-safe (безопасным для использования в динамическом контексте).
        Нельзя: возвращать Self, использовать generic-методы

    */

    /*
    «Либо трейт, либо тип должен быть локально определён».
    "Через Newtype Pattern — создание локального типа-обёртки. 
    Например, struct MyVec(Vec<i32>) позволяет реализовать Display для MyVec, так как это локальный тип."
    */
}


trait Draw {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Self {
        Self { radius: radius }
    }
}

impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius: {}", self.radius);
    }
}