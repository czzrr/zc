use std::io;
use std::fs;
use std::error::Error;

mod book;
mod library;

use crate::book::Book;
use crate::library::Library;

pub fn run(filename: &str) -> Result<(), Box<dyn Error>> {
    let books = read_books(filename)?;
    let mut library = Library{ books };

    println!("Welcome to the library!");
    println!("----------------------");
    println!("{}", library);
    println!("----------------------");

    loop {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)?;

        if user_input.to_ascii_lowercase().contains("quit") {
            break;
        }

        // Process query and print error in case there's any
        if let Err(e) = process_query(&user_input, &mut library) {
            println!("Error when processing query: {}", e);
        }
    }

    write_books(filename, library.books)?;

    Ok(())
}

fn write_books(filename: &str, books: Vec<Book>) -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();
    for book in &books {
        contents = contents + book.to_string().as_str() + "\n\n";
    }

    fs::write(filename, contents.trim())?;

    Ok(())
}

fn read_books(filename: &str) -> Result<Vec<Book>, Box<dyn Error>> {
    let contents: String = fs::read_to_string(filename)?;
    
    let lines: Vec<&str> = contents
        .lines()
        .collect();

    let books: Option<Vec<Book>> = lines
        .split(|line| *line == "")
        .map(|v| Book::new(v.iter()))
        .collect();

    books.ok_or("Error when creating books".into())
}

fn process_query(query: &str, library: &mut Library) -> Result<(), &'static str> {
    let mut query = query.split_whitespace();

    let command = query.next()
        .ok_or("Command not provided")?
        .to_ascii_lowercase();

    if command != "borrow" && command != "return" {
        return Err("Invalid command")
    }

    let id: u32 = match query.next()
        .ok_or("ID not provided")?
        .parse::<u32>() {
        Ok(n) => n,
        Err(_) => return Err("Error when parsing ID"),
    };

    if command == "borrow" {
        match library.borrow_book(id) {
            Ok(_) => println!("Book with ID {} borrowed", id),
            Err(e) => println!("{}", e),
        }
    } else {
        match library.return_book(id) {
            Ok(_) => println!("Book with ID {} returned", id),
            Err(e) => println!("{}", e),
        }
    }

    //println!("{}", library);

    Ok(())
}