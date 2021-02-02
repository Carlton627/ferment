mod modules;
mod models;

use std::fs;
use std::error::Error;

use modules::lexer;

pub fn extension_checker(env_args: &Vec<String>) -> Result<String, &'static str> {
    let filename_extension: Vec<&str> = env_args[1].split('.').collect();
    let filename = filename_extension[0].clone();
    let extension = filename_extension[1].clone();

    if extension == "fer" {
        Ok(String::from(filename) + "." + extension)
    } else {
        Err("Please use extension .fer for program files")
    }
}

pub fn run(file: String) -> Result<(), Box<dyn Error>> {
    let program = fs::read_to_string(file)?;
    let tokens = lexer::tokenizer(program)?;
    for token in tokens {
        println!("{:?}", token.token_type);
        println!("{:?}", token.value);
        println!("")
    }
    Ok(())
}

