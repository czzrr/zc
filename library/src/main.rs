fn main() {
    if let Err(e) = library::run("library.txt") {
        println!("Error: {}", e);
    }
}