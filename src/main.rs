mod termprint;
use std::{collections::HashMap, vec};

use serde::Serialize;

use termprint as tp;

mod request_ext;

use request_ext::{request_to_hashmap, response_to_hashmap, HttpMethod};

fn main() {
    let start = tp::print_start_program("Test termprint");

    tp::print_all_colors();

    tp::print_terminal_type();

    tp::message("Just a message");

    tp::print_info("Info with width: 20", "Value");

    tp::print_error(
        "error message",
        Some("error info"),
        Some("Additional err_msg"),
    );

    tp::print_warning("warning message");

    tp::print_info("Key", "Value");

    tp::print_title("Title");

    #[derive(Serialize)]
    struct Person {
        name: String,
        firstname: String,
        age: u32,
    }
    let person = Person {
        name: "Doe".to_string(),
        firstname: "John".to_string(),
        age: 42,
    };

    let person_long = Person {
        name: "MaryMaryMaryMaryMaryMaryMaryMaryMaryMaryMaryMaryMaryMaryMaryMaryMaryMaryMaryMaryMaryMaryMaryMaryMaryMary".to_string(),
        firstname: "HigginsHigginsHigginsHigginsHigginsHigginsHigginsHigginsHigginsHigginsHigginsHigginsHigginsHigginsHigginsHigginsHiggins".to_string(),
        age: 42,
};

    tp::print_struct(&format!("Struct: {}", "Person"), &person);
    tp::print_struct(&format!("Struct Long: {}", "Person"), &person_long);

    // tp::print_index2rgb();

    let table = vec![
        vec![
            "Name",
            "Age",
            "City",
            "Country of residence",
            "Street",
            "Professional Carreer",
            "Phone",
        ],
        vec![
            "Alice",
            "30",
            "New York",
            "USA",
            "123 Main St",
            "Engineer",
            "555-1234",
        ],
        vec![
            "Bob",
            "25",
            "Los Angeles",
            "USA",
            "456 Elm St",
            "Teacher",
            "555-5678",
        ],
        vec![
            "Charlie",
            "35",
            "Chicago",
            "USA",
            "789 Oak St",
            "Doctor",
            "555-9012",
        ],
    ];

    tp::print_table(table, true, Some("Table"), Some(10));

    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key23", "value2");
    map.insert("many key3", "valuevaluevaluevaluevaluevaluevaluevaluevaluevaluevaluevaluevaluevaluevaluevaluevaluevaluevaluevaluevaluevaluevaluevaluevaluevaluevaluevaluevaluevalue");
    tp::print_hashmap(&map, Some("Map"));

    println!("{}", tp::str_hashmap(&map, Some("Title row break")));

    let person2 = Person {
        name: "Mustermann".to_string(),
        firstname: "Eva".to_string(),
        age: 39,
    };

    let vec_persons = vec![person, person2];

    tp::print_vec_struct("Persons Struct", &vec_persons);

    let vec_str = vec!["Alice", "Bob", "Charlie"];
    tp::print_vec(&vec_str, Some("Vec"));

    tp::print_end_program("Test termprint", start);
}
