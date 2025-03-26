use std::fs::File;
use std::io::{BufReader, Read};


pub fn buffer() -> std::io::Result<()>{

    /*
        BufReader / BufWriter
        Основной буфер (например, 8 КБ) хранится в стеке, если его размер известен на этапе компиляции.
        Но стандартные реализации (std::io::BufReader) используют выделение в куче для гибкости
    */
    println!("============== Буфер ===================");
    let file = File::open("../data/hello.html")?;
    let mut reader = BufReader::new(file);  // буфер в куче

    let mut content = String::new();
    reader.read_to_string(&mut content)?;



    let mut file2 = File::open("../data/data.txt")?;
    let mut stack_buf = [0u8; 4096]; // 4 КБ в стеке
    file2.read_exact(&mut stack_buf)?; // чтение в стековый буфер

    // let file = File::open("../data/data.txt")?;
    // let reader = BufReader::with_capacity(64 * 1024, file); // 64 КБ в куче


    Ok(())
}