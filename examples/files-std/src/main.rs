use std::fs::File;
use std::io::Write;
use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    let string_name_file = "exampleFile.txt";
    let string_data_file = String::from("Hello std::File");

    // Create File
    let mut file = File::create(string_name_file)?;

    // Write string
    file.write_all(string_data_file.as_bytes())?;

    // Read data from file
    let mut content = String::new();
    let mut input_file = File::open(string_name_file)?;
    input_file.read_to_string(&mut content)?;

    // Print message
    println!("File {} has been written succesfully", string_name_file);
    println!("File content: {}", content);

    Ok(())
}
