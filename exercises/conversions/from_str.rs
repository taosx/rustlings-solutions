// This does practically the same thing that TryFrom<&str> does.
// Additionally, upon implementing FromStr, you can use the `parse` method
// on strings to generate an object of the implementor type.
// You can read more about it at https://doc.rust-lang.org/std/str/trait.FromStr.html
use std::str::FromStr;

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// Steps:
// 1. If the length of the provided string is 0, then return an error
// 2. Split the given string on the commas present in it
// 3. Extract the first element from the split operation and use it as the name
// 4. If the name is empty, then return an error
// 5. Extract the other element from the split operation and parse it into a `usize` as the age
//    with something like `"4".parse::<usize>()`.
// If while parsing the age, something goes wrong, then return an error
// Otherwise, then return a Result of a Person object
impl FromStr for Person {
    type Err = String;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.len() < 1 {
            return Err("Can't get Person from empty string".to_string());
        }

        let parts: Vec<&str> = s.split(",").collect();
        if parts.len() != 2 {
            return Err("string doesn't containg name and string".to_string());
        }

        let name = if parts[0] != "" {
            String::from(parts[0])
        } else {
            return Err("name can't be empty string".to_string());
        };

        let age: Result<usize, _> = parts[1].parse();
        if !age.is_ok() {
            return Err("can not parse age field".to_string());
        }
        let age = age.unwrap();

        Ok(Person { name, age })
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert!("".parse::<Person>().is_err());
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    #[should_panic]
    fn missing_age() {
        "John,".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_age() {
        "John,twenty".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_comma_and_age() {
        "John".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_name() {
        ",1".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_name_and_age() {
        ",".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_name_and_invalid_age() {
        ",one".parse::<Person>().unwrap();
    }
}
