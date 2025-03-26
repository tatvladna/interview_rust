pub fn static_dyn() {
    /* 1. Статическая и динамическая диспетчеризация: 
        * Статическая = generic, обычные функции, impl Trait:
            + скорость, innline-оптимизация
            - увеличивает размер бинарного файла (из-за дублирования кода)
        * Динамическая = Trait Objects (&dyn Trait, Box<dyn Trait>), указатели на трейт-объекты (&dyn Trait, Box<dyn Trait>, Rc<dyn Trait> и т. д.), сырые указатели *const dyn Trait:
            + меньший размер бинарника
            + можно работать с разными типами через интерфейс
            - накладной вопрос: поиск в vtable (таблица виртуальных методов)
            - невозможность виртуальных методов

        Если нужна максимальная производительность → статическая.
        Если нужна гетерогенная коллекция (разные типы в одном массиве) → динамическая (Vec<Box<dyn Trait>>)
    */
    println!("========== Статическая и динамическая диспетчеризация ==================");
    let alice = Woman::new("Alice", 30, 50_000);
    let name = "Bob".to_string();
    let bob = Man::new(name, 35, 50);

    match alice {
        Ok(woman) => {
            let res = static_info(woman);
            println!("{}", res);
        }
        Err(e) => println!("Error creating woman: {}", e),
    }

    match bob {
        Ok(man) => {
            let res = dyn_info(&man); // в динамической диспетчерезации все по ссылкам :)
            println!("{}", res);
        }
        Err(e) => println!("Error creating man: {}", e),
    }
}


trait PeopleInfo {
    fn make_money(&self) -> u32;
    fn get_name(&self) -> &str;
    fn get_age(&self) -> u8;
}

#[derive(Debug)]
struct Woman {
    name: String,
    age: u8,
    salary: u32,
}

impl Woman {
    pub fn new(name: impl Into<String>, age: u8, salary: u32) -> Result<Self, &'static str> {
        if age > 120 {
            return Err("Age is unrealistically high");
        }
        if salary > 1_000_000 {
            return Err("Salary is too high");
        }
        
        Ok(Self {
            name: name.into(),
            age,
            salary,
        })
    }
}

// или без проверок
impl Man {
    pub fn new(name: impl Into<String>, age: u8, salary: u32) -> Result<Self, &'static str> {
        if age > 50 {
            return Err("Мужчины не живут так долго");
        }

        if salary > 100 {
            return Err("Мужчины не получают такую высокую зарплату");
        }
        Ok (Self {
            name: name.into(),
            age,
            salary,
        })
    }
}

impl PeopleInfo for Woman {
    fn make_money(&self) -> u32 {
        self.salary
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_age(&self) -> u8 {
        self.age
    }
}

impl PeopleInfo for Man {
    fn make_money(&self) -> u32 {
        self.salary
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_age(&self) -> u8 {
        self.age
    }
}


struct Man {
    name: String,
    age: u8,
    salary: u32,
}



// rомпилятор генерирует отдельные функции для женщины и мужчины
fn static_info<T: PeopleInfo>(people: T) -> String {
    format!(
        "Name: {}, Age: {}, Salary: {}",
        people.get_name(),
        people.get_age(),
        people.make_money()
    )
}

// вызов определяется в runtime через vtable
fn dyn_info(people: &dyn PeopleInfo) -> String {
    format!(
    "Name: {}, Age: {}, Salary: {}",
    people.make_money(),
    people.get_age(),
    people.get_name(),
    )
}


