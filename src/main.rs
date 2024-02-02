use std::io;
use std::fs;

fn main() {
    let mut counter = 0;
    let mut directory = String::new();

    println!("What directory would you like to UwU? ");

    io::stdin().read_line(&mut directory).expect("Error reading from STDIN");

    let mut directory = directory.trim_end().to_string();
    directory.push_str("\\UwU");

    loop {
        let mut file_name = String::clone(&directory);
        file_name.push_str(&(counter.to_string() + ".png"));

        fs::copy("uwu.png", &file_name).expect("Failed to copy data into file");
        counter += 1;
    }
}
