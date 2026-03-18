use std::fs::File;
use std::io::Write;

fn main() -> Result<(), std::io::Error> {
    let string_name_file = "exampleFile.txt";
    let string_data_file = String::from("Hello std::File");

    // Create File
    let mut file = File::create(string_name_file)?;

    // Write string
    file.write_all(string_data_file.as_bytes())?;

    // Print message
    println!("File {} has been written succesfully", string_name_file);
    Ok(())
}

/*fn main() -> Result<(), std::io::Error> {

    let file_path = "example.txt";  // файл для записи
    let text = String::from("Hello METANIT.COM!");    // текст для записи
    let mut output_file = File::create(file_path)?; // создаем файл
    output_file.write_all(text.as_bytes())?;     // записываем в файл текст

    let mut content = String::new();    // строка для считывания файла
    let mut input_file = File::open(file_path)?;   // открываем файл
    input_file.read_to_string(&mut content)?;      // считываем содержимое файла в строку
    println!("File content: {}", content);      // выводим считанный текст на консоль
    Ok(())
}*/
