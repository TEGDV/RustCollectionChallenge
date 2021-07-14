mod options;
use std::io;

fn main() {
    let mut chose = String::new();

    println!("Select an option \n 1 => Vector Sorting \n 2 => Piglatin \n 3 => Employees App \n");

    io::stdin()
        .read_line(&mut chose)
        .expect("Failed to read line");

    match chose.as_bytes()[0] {
        b'1' => options::vectors(),
        b'2' => options::pig_latin(),
        b'3' => options::employees(),
        _ => (),
    }
}
