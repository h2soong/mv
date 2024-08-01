use std::fs::File;
use std::io::{self, Read};
use markdown;

fn main() -> io::Result<()> {
    let mut file = File::open("C:/Users/h2soong/Desktop/school_学校/Review/2024_sigcomm/understanding.md")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("File contents: {}", contents);

	println!("html: {}",markdown::to_html(&contents));
    Ok(())
}

