// from_into.rs
//
// The From trait is used for value-to-value conversions. If From is implemented
// correctly for a type, the Into trait should work conversely. You can read
// more about it at https://doc.rust-lang.org/std/convert/trait.From.html
//
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        let (name, age) = match s.split_once(',') {
            Some((name, age)) => (name.trim(), age.trim()),
            _ => return Person::default(),
        };

        if let Ok(age) = age.parse::<usize>() {
            if name.len() > 0 {
                return Person {
                    name: String::from(name),
                    age,
                };
            }
        }

        Person::default()
    }
}

fn main() {
    // Use the `from` function
    let p1 = Person::from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

