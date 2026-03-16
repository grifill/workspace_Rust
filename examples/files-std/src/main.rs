use std::fs::File;
use std::io::{Write};

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