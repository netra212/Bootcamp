use std::fmt::format;
use std::fs::{self, File};
use std::io::{self, Read};
use std::num::ParseIntError;

fn main() {
    let v = vec!["one", "two", "three"];
    println!("{}", v[3]);

    // Implementing Result with File handling.
    let file = File::open("example.txt").unwrap(); // unwrap() will simply return value store inside the example.txt if open return Ok() variants otherwise it will panic if open return Err value. 

    // let file = match file {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Failed to open file: {:?}", error);
    //     }
    // };

    // The `expect()` method is similar to `unwrap()` method but we can pass the custom messaged in expect like this.
    let file1 = File::open("example1.txt").expect("Failed to open the file.!");

    // Propagating error.

    // Implementation of result_and_option.
    let first_line = read_first_line("example2.txt");

    // Implementation of Multiple error types (tutVid)
    let i = parse_file("example3.txt");

    match i {
        Ok(i) => println!("{i}"), 
        Err(e) => {
            match e {
                ParseFileError::File => {
                    //...
                }, 
                ParseFileError::Parse(e) => {
                    //...
                }
            }
        }
    }
}

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn get_user_id(username: &str) -> Result<i32, String> {
    if username.is_empty() {
        Err("Username cannot be empty".to_owned())
    } else {
        Ok(1)
    }
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?; // ? here is: unwrap valid value or return errornous value propagating them to the calling function. 
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // read_to_string reads the file in bytes and stored data in contents variable. 
    // File::open(filename)?.read_to_string(&mut contents)?;
    Ok(contents)
}

struct User {
    firstname: String,
    lastname: String,
}

fn get_initials(user: User) -> Option<String> {
    let first_initial = user.firstname.chars().next()?; // next return the optional value. chars() gives iterator here. 
    let last_initial = user.lastname.chars().next()?; // if next return the value with Some() then we will return the first name, otherwise next will simply return the None() variant
    Some(format!("{first_initial}.{last_initial}."))
}

// result_and_option.
fn read_first_line(filename: &str) -> Result<<Option<String>, io::Error> {
    fs::read_to_string(filename).map(|s| s.lines().next().map(|s| s.to_owned()))
    // s.lines() - return the iterator in a string. 
    // next() - gives the first line, next() method return an option (or Option type.) containing string slices because we might not have first line in the given file. and we want to convert that string_slices into owned string so that map is implemented in this case. 
    // map() - used to transfrom from one value to another value, here 

    // without implementation of the Option in return type, closure |s| -> were returning the option type but we were expecting Result type so in return type of function, return string value is wrapped within Option which contains Some() or None() variants. 
}

// result_and_option.
fn read_first_line1(filename: &str) -> <Option<String> {
    fs::read_to_string(filename).ok().and_then(|s| s.lines().next().map(|s| s.to_owned()))

    // ok() - takes a result type and convert to option type. 
    // and_then() - to convert that option into option containing first line in our file. 
    // method such as ok() & map() are called combinator because these can change the value. 
}

enum ParseFileError {
    File, 
    Parse(ParseIntError)
}

// multiple error types. 
fn parse_file(filename: &str) -> Result<i32, Box<dyn error::Error>> {
    // this function will take an file and attempt to parse them into an integer. 
    let s = fs::read_to_string(filename)?;
    let i = s.parse()?;
    Ok(i)
}

// multiple error types. 
fn parse_file(filename: &str) -> Result<i32, ParseFileError> {
    // this function will take an file and attempt to parse them into an integer. 
    let s = fs::read_to_string(filename).map_err(|e| {
        ParseFileError::File?;
    });
    // map_err: map the error of one type to another type. 
    // In this case, we map any variants of ParseFileError which is File. 
    let i = s.parse().map_err(|e| ParseFileError::Parse(e))?;
    Ok(i)
}

