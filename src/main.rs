mod person;

use person::*;
use person::people::People as ManyPersons;

fn main() {
    let mut users = vec!["Tom", "Sam", "Bob"];

    print_vec(&users);
    users.push("Alice");

    print_vec(&users);

    println!("size {}", users.len());

    print_vec(&users);

    users.remove(2);

    print_vec(&users);

    println!("{}", users.pop().unwrap());

    print_vec(&users);

    let aa = Person {
        first_name: "Tom".to_string(),
        last_name: "Tom".to_string(),
    };
    let pep = ManyPersons {};
}

fn print_vec(users: &Vec<&str>) {
    for user in users {
        print!("{} ", user);
    }
    println!();
}
