use crate::book::Book;
use std::fmt;

#[derive(Debug)]
pub struct Library {
    pub books: Vec<Book>,
}

impl Library {
    // Borrow book by changing its status from available to unavailable
    pub fn borrow_book(&mut self, id: u32) -> Result<(), &'static str> {
        let idx = self.get_book_idx(id).ok_or("ID does not exist")?;
        
        let book = &mut self.books[idx];

        if book.available {
            book.available = false;
            Ok(())
        } else {
            Err("Book is already borrowed")
        }
    }

    // Return book by changing its status from unavailable to available
    pub fn return_book(&mut self, id: u32) -> Result<(), &'static str> {
        let idx = self.get_book_idx(id).ok_or("ID does not exist")?;
        
        let book = &mut self.books[idx];

        if !book.available {
            book.available = true;
            Ok(())
        } else {
            Err("Book is not borrowed")
        }
    }

    fn get_book_idx(&self, id: u32) -> Option<usize> {
        self.books.iter().position(|b| b.id == id)
    }
}

impl fmt::Display for Library {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.books.is_empty() {
            return write!(f, "");
        }

        let len = self.books.len();
        for book in &self.books[..len - 1] {
            write!(f, "{}\n\n", book)?;
        }

        write!(f, "{}", self.books[len - 1])
    }
}