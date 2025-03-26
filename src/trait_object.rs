pub fn trait_object() {
    println!("========= Trait Object ============");
    /*
    Trait Object — это механизм динамической диспетчеризации в Rust, 
    позволяющий работать с разными типами через единый интерфейс трейта.
    */
    
    // помещаем trait object в кучу с помощью Box
    let circle: Box<dyn Draw> = Box::new(Circle::new(65.0));
    circle.draw();
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