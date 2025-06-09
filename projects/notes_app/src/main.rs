use std::{fs::OpenOptions, io::{self, Write, BufRead, BufReader}};

fn main() {
    println!("1) Add Note\n2) View Notes");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => {
            let mut note = String::new();
            println!("Enter your note:");
            io::stdin().read_line(&mut note).unwrap();

            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("notes.txt")
                .unwrap();

            writeln!(file, "{}", note.trim()).unwrap();
        }
        "2" => {
            let file = OpenOptions::new().read(true).open("notes.txt").unwrap();
            let reader = BufReader::new(file);

            println!("Your Notes:");
            for (i, line) in reader.lines().enumerate() {
                println!("{}. {}", i + 1, line.unwrap());
            }
        }
        _ => println!("Invalid choice"),
    }
}
