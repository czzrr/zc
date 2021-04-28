use std::slice::Iter;
use std::fmt;

#[derive(Debug)]
pub struct Book<> {
    pub title: String,
    pub id: u32,
    pub available: bool
}

impl Book {
    // Create a new Book from an iterator of title, id and availability
    pub fn new(mut book_info: Iter<&str>) -> Option<Book> {
        let title = book_info.next()?.to_string();

        let id: u32 = book_info.next()?.parse().ok()?;

        let available = match *book_info.next()? {
            "Available" => true,
            "Unavailable" => false,
            _ => return None,
        };

        Some(Book{ title, id, available })
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.available {
            "Available"
        } else {
            "Unavailable"
        };

        write!(f, "{}\n{}\n{}", self.title.as_str(), self.id, status)
    }
}