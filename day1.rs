use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let path = Path::new("input.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {}
    }
    
    // this part I actually wrote
    let answer:u32 = s.split('\n').map(|line| {
        let digits: Vec<u32> = (line).chars()
            .filter(|c| (*c).is_digit(10))
            .filter_map(|c| c.to_digit(10))
            .collect();
        10 * digits.first().copied().unwrap_or(0) + digits.last().copied().unwrap_or(0)
    }).sum();
    print!("answer: {}\n", answer)
}