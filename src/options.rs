use std::collections::HashMap;
use std::io;
use std::str;

pub fn vectors() {
    let mut my_list = vec![
        2.0, 4.0, 6.0, 5.0, 4.1, 4.1, 4.1, 4.1, 4.1, 1.0, 1.0, 1.0, 2.0, 2.0, 4.1, 4.1, 5.0, 6.0,
        7.0, 8.0, 8.0, 9.0,
    ];

    let mean: f64 = my_list.iter().sum();
    let mean: f64 = mean / 2.0;
    my_list.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median_index: usize = my_list.len() / 2;
    let mut map = HashMap::new();
    let mut max_value = 0;
    let mut max_key = String::new();

    for float in &my_list {
        let count = map.entry(float.to_string()).or_insert(0);
        *count += 1;
        if *count > max_value {
            max_value = *count;
            max_key = format!("{}", float);
        }
    }

    println!(
        "The mean is {} the median is {}, and the most repetead value is {} with {} times",
        mean, my_list[median_index], max_key, map[&max_key]
    );
}

pub fn pig_latin() {
    let mut text = String::new();

    println!(" ============ Pig Latin Translator ================== \n Please Writte a word:   ");

    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read line");

    println!(
        "{}",
        text.split_whitespace() // This make an iterator of words ["word1", "word2"]
            .map(pigify) // This translate the each word in the iterator to a pig latin ord1way ... ord2awy
            /* this execute the join function to adds spaces between each splited word
             * Depending on how many word the text have its the way the initial strings would
             * growing
             **/
            .fold(String::new(), joinwords)
    );
}

fn pigify(word: &str) -> String {
    let mut chars = word.chars();
    let first_char = match chars.next() {
        Some(c) => c,
        _ => return String::new(),
    };

    match first_char {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", word),
        _ => format!("{}-{}ay", chars.as_str(), first_char),
    }
}

fn joinwords(mut current: String, next: String) -> String {
    /*
     * This function its called by a fold method but his only function its join two strings adding an
     * space between them then return both as one string example "Hello" and "World" this function
     * return "Hello World" Good, then if exist other word in the main the fold funcion acumulate
     * the returned string to the current parameter in example the next word its "Amigos" the
     * return will be "Hello World Amigos" and so on if exist more words in the text
     **/

    if !current.is_empty() {
        current.push(' ');
    }
    current.push_str(&next);
    println!("{}", current);
    current
}

// required trait for .lines()
use std::io::BufRead;
pub fn employees() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();
    let stdin = io::stdin();
    println!("Type 'Add <name> to <department>' to add an employee");
    println!("Type 'List <department>' to list the employees of a department");
    println!("Type 'All' to list all employees by department");
    println!("Type 'Quit' to quit");
    for line in stdin.lock().lines() {
        let input = line.expect("error: unable to read user input");
        match Command::from_input(&input) {
            // or_default is just a convenience, does the same as or_insert_with(Vec::default)
            Some(Command::Add { dept, name }) => employees.entry(dept).or_default().push(name),
            Some(Command::List(dept)) => match employees.get(&dept) {
                Some(names) => {
                    let mut names = names.clone();
                    names.sort();
                    for name in names {
                        println!("{}: {}", dept, name);
                    }
                }
                None => println!("I don't recognize that department!"),
            },
            Some(Command::All) => {
                for (dept, names) in &employees {
                    let mut names = names.clone();
                    names.sort();
                    for name in names {
                        println!("{}: {}", dept, name);
                    }
                }
            }
            Some(Command::Quit) => break,
            // consider using eprintln, which prints to stderr
            None => println!("Input error!"),
        }
    }
    println!("Have a nice day!");
}

enum Command {
    // Using named fields instead of Add(String, String) because dept and name
    // are the same type and could get mixed up.
    Add { dept: String, name: String },
    List(String),
    All,
    Quit,
}

impl Command {
    fn from_input(s: &str) -> Option<Self> {
        let words: Vec<&str> = s.trim().split_whitespace().collect();
        // "Slice destructuring / slice pattern matching" for more info
        match words.as_slice() {
            ["All"] => Some(Command::All),
            ["Quit"] => Some(Command::Quit),
            ["List", dept] => Some(Command::List(dept.to_string())),
            ["Add", name, "to", dept] => Some(Command::Add {
                dept: dept.to_string(),
                name: name.to_string(),
            }),
            _ => None,
        }
    }
}
