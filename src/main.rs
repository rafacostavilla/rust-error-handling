use std::{fmt::Error, fs::File, io::ErrorKind};
fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = 
    match greeting_file_result {
        Ok(file) => file,
        Err(error) => 
        match error.kind() {
          ErrorKind::NotFound => 
          match  File::create("hello.txt"){
              Ok(new_file) => new_file,
              Err(error) => panic!("Problem creating file: {error:?}"),
          },
          other_error => panic!("Problem opening the file: {other_error:?}"),   
        },
    };

    //Alternative way using closures

    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else( |error|{
    //             panic!("Problem creating file: {error:?}");
    //         })
    //     } else {
    //         panic!("Problem creating file: {error:?}");
    //     }
    // });

}
