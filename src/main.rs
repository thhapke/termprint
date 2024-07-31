mod termprint;
use serde::Serialize;

use termprint as tp;

fn main() {

    let start = tp::print_start_program("Test termprint");

    tp::message("Just a message");

    tp::print_info("Info with width: 20","Value",Some(20));

    tp::print_error("error message", Some("error info"), Some("Additional err_msg"));

    tp::print_warning("warning message");

    tp::print_key_value("Key","Value",None);

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

    tp::print_index2rgb();

    tp::print_end_program("Test termprint", start);

    let table = vec![
        vec!["Name", "Age", "City"],
        vec!["Alice", "30", "New York"],
        vec!["Bob", "25", "Los Angeles"],
        vec!["Charlie", "35", "Chicago"],
    ];
    tp::print_table(table)
}