use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    const FILE_PATH: &str = "puzzle-input-day-1.txt";
    let contents = read_file(FILE_PATH)?;

    println!("Total: {}", first_and_last_number_solution(&contents));
    println!(
        "Total with digits: {}",
        first_and_last_number_or_digit_solution(&contents)
    );

    Ok(())
}

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(&file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn first_and_last_number_solution(contents: &str) -> i32 {
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
    return total;
}

/**
 * Native solution
 * I look left to right
 * When I see a number or a digit I add it to the array
 * until I reach the end of the array
 * I then combine the first and last numer to make a two digit number
 * and add it to the total
 */
fn first_and_last_number_or_digit_solution(content: &str) -> i32 {
    let mut total: i32 = 0;

    for line in content.lines() {
        // integer array
        let mut numbers: Vec<char> = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        let length = chars.len();
        let mut i = 0;
        while i < length {
            if chars[i].is_numeric() {
                numbers.push(chars[i]);
                i += 1;
                continue;
            }

            match chars[i] {
                'z' => {
                    // check if zero is contained
                    // if we have enough length in the array to check zero we check for a pattern match
                    if i + 3 < length {
                        if chars[i + 1] == 'e' && chars[i + 2] == 'r' && chars[i + 3] == 'o' {
                            numbers.push('0');
                        }
                    }
                }
                'o' => {
                    // check if one is contained
                    if i + 2 < length && chars[i + 1] == 'n' && chars[i + 2] == 'e' {
                        numbers.push('1');
                    }
                }
                't' => {
                    // check if two is contained
                    if i + 2 < length && chars[i + 1] == 'w' && chars[i + 2] == 'o' {
                        numbers.push('2');
                    } else if i + 4 < length
                        && chars[i + 1] == 'h'
                        && chars[i + 2] == 'r'
                        && chars[i + 3] == 'e'
                        && chars[i + 4] == 'e'
                    {
                        // check if three is contained
                        numbers.push('3');
                    }
                }
                'f' => {
                    // check if four is contained
                    if i + 3 < length
                        && chars[i + 1] == 'o'
                        && chars[i + 2] == 'u'
                        && chars[i + 3] == 'r'
                    {
                        numbers.push('4');
                    } else if i + 3 < length
                        && chars[i + 1] == 'i'
                        && chars[i + 2] == 'v'
                        && chars[i + 3] == 'e'
                    {
                        numbers.push('5');
                    }
                }
                's' => {
                    // check if six is contained
                    if i + 2 < length && chars[i + 1] == 'i' && chars[i + 2] == 'x' {
                        numbers.push('6');
                    } else if i + 4 < length
                        && chars[i + 1] == 'e'
                        && chars[i + 2] == 'v'
                        && chars[i + 3] == 'e'
                        && chars[i + 4] == 'n'
                    {
                        numbers.push('7');
                    }
                }
                'e' => {
                    // check if eight is contained
                    if i + 4 < length
                        && chars[i + 1] == 'i'
                        && chars[i + 2] == 'g'
                        && chars[i + 3] == 'h'
                        && chars[i + 4] == 't'
                    {
                        numbers.push('8');
                    }
                }
                'n' => {
                    // check if nine is contained
                    if i + 3 < length
                        && chars[i + 1] == 'i'
                        && chars[i + 2] == 'n'
                        && chars[i + 3] == 'e'
                    {
                        numbers.push('9');
                    }
                }
                _ => {}
            }
            i += 1;
        }

        if numbers.is_empty() {
            println!("No numbers found");
            continue;
        }

        if numbers.len() > 0 {
            let mut num = String::new();
            num.push(numbers[0]);
            num.push(numbers[numbers.len() - 1]);
            total += num.parse::<i32>().unwrap();
            continue;
        }
    }

    return total;
}
