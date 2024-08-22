use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let file_path = String::from("puzzle-input-day-1.txt");
    // Read puzzle input
    let mut file = File::open(&file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut total: i32 = 0;
    for line in contents.lines() {
        for word in line.split_whitespace() {
            let char_array: Vec<char> = word.chars().collect();

            // there are two numbers in this char array combine them to get the number
            let mut first_num = String::new();
            let mut last_num = String::new();
            for c in char_array {
                if c.is_numeric() {
                    if first_num.is_empty() {
                        // set string = to c
                        first_num = c.to_string();
                    } else {
                        last_num = c.to_string();
                    }
                }
            }

            if first_num.is_empty() {
                continue;
            }

            if last_num.is_empty() {
                //combine first num with first num
                last_num = first_num.clone();
            }

            first_num.push_str(&last_num);
            total += first_num.parse::<i32>().unwrap();
        }
    }

    println!("Total: {}", total);

    Ok(())
}
