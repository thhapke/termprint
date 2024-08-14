mod termprint;
use std::collections::HashMap;

use serde::Serialize;

use termprint as tp;

fn main() {

    let start = tp::print_start_program("Test termprint");

    tp::print_terminal_type();

    tp::message("Just a message");

    tp::print_info("Info with width: 20","Value",Some(20));

    tp::print_error("error message", Some("error info"), Some("Additional err_msg"));

    tp::print_warning("warning message");

    tp::print_info("Key","Value",None);

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

    tp::print_struct("Struct", "Person", &person, None);

   tp::print_all_colors();

    tp::print_index2rgb();

    let table = vec![
        vec!["Name", "Age", "City"],
        vec!["Alice", "30", "New York"],
        vec!["Bob", "25", "Los Angeles"],
        vec!["Charlie", "35", "Chicago"],
    ];
    tp::print_table(table);

    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key23", "value2");
    map.insert("many key3", "value3");
    tp::print_hashmap(&map,Some("Map"));
    tp::print_hashmap(&map,None);

    tp::print_end_program("Test termprint", start);

}