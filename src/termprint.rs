
use std::fmt;
use std::cmp::min;

use colored::{Colorize, Color,ColoredString};
use time::OffsetDateTime;
use std::collections::HashMap;

use serde_json::Value;

use reqwest::{Request, Response};

use termsize;

use crate::request_ext::{response_to_hashmap, request_to_hashmap,headers_to_hashmap};

pub use crate::request_ext::HttpMethod;

pub const LONG: usize = 120;
pub const MEDIUM: usize = 80;
pub const SHORT: usize = 30;
pub const SPACE: usize = 3;
pub const STD_WIDTH: usize = 20;
pub const RESET: &str = "\x1b[0m";
pub const MAX_WIDTH: usize = 120;
pub const MAX_COL_WIDTH: usize = 10;

trait ColoredItem {
    fn cinfo(&self) -> ColoredString;
    fn cvar(&self) -> ColoredString;
    fn ctitle(&self) -> ColoredString;
    // fn cheader(&self) -> ColoredString;  
    fn cerror(&self) -> ColoredString;
    fn cwarning(&self) -> ColoredString;
    fn citem(&self) -> ColoredString;
    fn cline(&self) -> ColoredString;
    fn column(&self,index:usize) -> ColoredString;
}

#[cfg(any(feature = "truecolors", feature = "colors256"))]
pub const COLUMN_COLORS: [(u8, u8, u8); 10] = [
    (255, 0, 0),    // Red
    (0, 255, 0),    // Green
    (0, 0, 255),    // Blue
    (255, 255, 0),  // Yellow
    (0, 255, 255),  // Cyan
    (255, 0, 255),  // Magenta
    (192, 192, 192),// Silver
    (128, 128, 128),// Gray
    (128, 0, 0),    // Maroon
    (128, 128, 0),  // Olive
];

#[cfg(feature = "mono")]
pub const COLUMN_COLORS: [(u8, u8, u8); 3] = [
    (75, 75, 75),    
    (150, 150, 150),   
    (255, 255, 255),    
];

#[cfg(feature = "basic")]
pub const COLUMN_COLORS: [Color; 5] = [
    Color::BrightBlue,  
    Color::BrightCyan,
    Color::BrightMagenta,    
    Color::BrightGreen,
    Color::BrightYellow,
];

#[cfg(feature = "basic")]
impl ColoredItem for &str {
    fn cinfo(&self) -> ColoredString {
        self.blue()
    }

    fn cvar(&self) -> ColoredString {
        self.bright_cyan()
    }

    fn ctitle(&self) -> ColoredString {
        self.bold().blue()
    }

    fn cerror(&self) -> ColoredString {
        self.bright_red().bold()
    }

    fn cwarning(&self) -> ColoredString {
        self.bright_yellow()
    }

    fn citem(&self) -> ColoredString {
        self.bright_cyan()
    }

    fn cline(&self) -> ColoredString {
        self.bright_blue().bold()
    }

    fn column(&self,index:usize) -> ColoredString {
        let col_color = COLUMN_COLORS[index % COLUMN_COLORS.len()];
        self.color(col_color)
    }
}

#[cfg(feature = "basic")]
impl ColoredItem for String {
    fn cinfo(&self) -> ColoredString {
        self.blue()
    }

    fn cvar(&self) -> ColoredString {
        self.bright_cyan()
    }

    fn ctitle(&self) -> ColoredString {
        self.bold().blue()
    }

    fn cerror(&self) -> ColoredString {
        self.bright_red().bold()
    }

    fn cwarning(&self) -> ColoredString {
        self.bright_yellow()
    }

    fn citem(&self) -> ColoredString {
        self.bright_cyan()
    }

    fn cline(&self) -> ColoredString {
        self.bright_blue().bold()
    }

    fn column(&self,index:usize) -> ColoredString {
        let col_color = COLUMN_COLORS[index % COLUMN_COLORS.len()];
        self.color(col_color)
    }
}

#[cfg(feature = "colors256")]
impl ColoredItem for &str {
    fn cinfo(&self) -> ColoredString {
        self.truecolor(51, 102, 255) 
    }

    fn cvar(&self) -> ColoredString {
        self.truecolor(0, 255, 255) 
    }

    fn ctitle(&self) -> ColoredString {
        self.truecolor(51, 102, 255) 
    }

    fn cerror(&self) -> ColoredString {
        self.bold().truecolor(255, 0, 0) 
    }

    fn cwarning(&self) -> ColoredString {
        self.truecolor(255, 255, 85) 
    }

    fn citem(&self) -> ColoredString {
        self.truecolor(0, 255, 255) 
    }

    fn cline(&self) -> ColoredString {
        self.truecolor(51, 102, 255) 
    }

    fn column(&self,index:usize) -> ColoredString {
        let color = COLUMN_COLORS[index % COLUMN_COLORS.len()];
        self.truecolor(color.0, color.1, color.2)
    }
}


#[cfg(feature = "truecolors")]
impl ColoredItem for &str {
    fn cinfo(&self) -> ColoredString {
        self.truecolor(51, 102, 255) 
    }

    fn cvar(&self) -> ColoredString {
        self.truecolor(0, 255, 255) 
    }

    fn ctitle(&self) -> ColoredString {
        self.truecolor(51, 102, 255) 
    }

    fn cerror(&self) -> ColoredString {
        self.bold().truecolor(255, 0, 0) 
    }

    fn cwarning(&self) -> ColoredString {
        self.truecolor(255, 255, 85) 
    }

    fn citem(&self) -> ColoredString {
        self.truecolor(0, 255, 255) 
    }

    fn cline(&self) -> ColoredString {
        self.truecolor(51, 102, 255) 
    }

    fn column(&self,index:usize) -> ColoredString {
        let color = COLUMN_COLORS[index % COLUMN_COLORS.len()];
        self.truecolor(color.0, color.1, color.2)
    }
}

#[cfg(feature = "mono")]
impl ColoredItem for &str {
    fn cinfo(&self) -> ColoredString {
        self.white()
    }

    fn cvar(&self) -> ColoredString {
        self.white()
    }

    fn ctitle(&self) -> ColoredString {
        self.white()
    }

    fn cerror(&self) -> ColoredString {
        self.bold().white()
    }

    fn cwarning(&self) -> ColoredString {
        self.white()
    }

    fn citem(&self) -> ColoredString {
        self.white()
    }

    fn cline(&self) -> ColoredString {
        self.white()
    }
    fn column(&self,index:usize) -> ColoredString {
        let color = COLUMN_COLORS[index % COLUMN_MONO_COLORS.len()];
        self.truecolor(color.0, color.1, color.2)
    }
}

pub fn index2rgb(index: usize) -> (u8, u8, u8) {
    if index < 16 {
        match index {
            0 => (0, 0, 0),
            1 => (205, 0, 0),
            2 => (0, 205, 0),
            3 => (205, 205, 0),
            4 => (0, 0, 238),
            5 => (205, 0, 205),
            6 => (0, 205, 205),
            7 => (229, 229, 229),
            8 => (127, 127, 127),
            9 => (255, 0, 0),
            10 => (0, 255, 0),
            11 => (255, 255, 0),
            12 => (92, 92, 255),
            13 => (255, 0, 255),
            14 => (0, 255, 255),
            15 => (255, 255, 255),
            _ => (0, 0, 0), // Default to black if out of range
        }
    } else if index < 232 {
        let r = ((index - 16) / 36 % 6) * 51;
        let g = ((index - 16) / 6 % 6) * 51;
        let b = ((index - 16) % 6) * 51;
        (r as u8, g as u8, b as u8)
    } else {
        // Grayscale range (232-255)
        let gray = (index as i16 - 232) * 10 + 8;
        (gray as u8, gray as u8, gray as u8)
    }
}

pub fn get_terminal_type() -> String {
    let term = std::env::var("TERM").unwrap_or_else(|_| "unknown".to_string());
    term
}

pub fn get_terminal_width() -> usize {
    termsize::get().unwrap().cols.into()
}

pub fn print_terminal_type() {
    let term = get_terminal_type();
    print_double_line(MEDIUM);
    print_info("Terminal type", &term);
    print_double_line(MEDIUM);   
    println!();
}
// UNUSED
// fn truncate_str(s: &str, max_len: usize) -> String {
//     if s.len() > max_len && max_len > 3 {
//         let truncated = s.chars().take(max_len - 3).collect::<String>();
//         format!("{}...", truncated)
//     } else {
//         s.to_string()
//     }
// }

pub fn print_all_colors() {
    print_title("All Clorored Colors");
    println!("{} {}", "Black".black(), "Bright Black".bright_black());
    println!("{} {}", "Red".red(), "Bright Red".bright_red());
    println!("{} {}", "Green".green(), "Bright Green".bright_green());
    println!("{} {}", "Yellow".yellow(), "Bright Yellow".bright_yellow());
    println!("{} {}", "Blue".blue(), "Bright Blue".bright_blue());  
    println!("{} {}", "Magenta".magenta(), "Bright Magenta".bright_magenta());
    println!("{} {}", "Cyan".cyan(), "Bright Cyan".bright_cyan());
    println!("{} {}", "White".white(), "Bright White".bright_white());
    println!();


}

pub fn print_index2rgb() {
    print_message("\nColor index to rgb:");
    print_line(MEDIUM);
    for i in 0..65{
        let rgb1 = index2rgb(i);
        let rgb2 = index2rgb(i+65);
        let rgb3 = index2rgb(i+130);    
        let rgb4 = index2rgb(i+195);
        let rgb1_str: String = format!("{:3}:({:3},{:3},{:3})  ",i,rgb1.0,rgb1.1,rgb1.2);
        let rgb2_str: String = format!("{:3}:({:3},{:3},{:3})  ",i+65,rgb2.0,rgb2.1,rgb2.2);
        let rgb3_str: String = format!("{:3}:({:3},{:3},{:3})  ",i+130,rgb3.0,rgb3.1,rgb3.2);
        let rgb4_str: String = format!("{:3}:({:3},{:3},{:3})  ",i+195,rgb4.0,rgb4.1,rgb4.2);
        print!("{} ", &rgb1_str.truecolor(rgb1.0 as u8,rgb1.1 as u8,rgb1.2 as u8));
        print!("{} ", &rgb2_str.truecolor(rgb2.0 as u8,rgb2.1 as u8,rgb2.2 as u8));
        print!("{} ", &rgb3_str.truecolor(rgb3.0 as u8,rgb3.1 as u8,rgb3.2 as u8));
        if i < 61 {
            print!("{}\n", &rgb4_str.truecolor(rgb4.0 as u8,rgb4.1 as u8,rgb4.2 as u8));
        }
        else {
            println!();
        }
    }
}

pub fn error(message: &str, info: Option<&str>, err_msg: Option<&str>) -> String {
    let basic_error = format!("{} {}","ERROR".cerror(),message.cerror());
    match (info,err_msg) {
        (Some(info_str),Some(err_str)) => format!("{}: {} - {}",basic_error, info_str.cerror(), err_str.cerror()),
        (Some(info_str),None) => format!("{}: {}",basic_error, info_str.cerror()),
        (None,Some(err_str)) => format!("{}: {}",basic_error, err_str.cerror()),
        (None,None) => format!("{}",basic_error)
    }
}

pub fn print_error(message: &str, info: Option<&str>, err_msg: Option<&str>) {
    println!("{}",error(message,info,err_msg));
}

pub fn warning(msg: &str) -> String {
    format!("{}: {}","WARNTING".cwarning().bold(),msg.cwarning())
}

pub fn print_warning(msg: &str)  {
    println!("{}",warning(msg));
}

pub fn info(key: &str, value: &str) -> String {
    format!("{}: {}",key.cinfo(),value.cvar())
}

pub fn print_info(key: &str, value: &str) {
    println!("{}",info(key, value));
}

pub fn write_info(f: &mut fmt::Formatter,key: &str, value: &str) {
    if let Err(e) = writeln!(f,"{}{}",key.cinfo(),value.cvar()) { 
        print_error("Error write",Some(&"write_info".to_string()),Some(&e.to_string()))
    };
}

pub fn message(txt: &str) -> String {
    format!("{}",txt.cinfo())
}                                                 

pub fn print_message(txt: &str)  {
    println!("{}",message(txt));
}

pub fn write_message(f: &mut fmt::Formatter,txt: &str)  {
    if let Err(e) = writeln!(f,"{}",message(txt)) { 
        print_error("Error write",Some(&"write_info".to_string()),Some(&e.to_string()))
    };
}

pub fn str_title(msg: &str) -> String {
    format!("{}\n",msg.ctitle().bold())
}

pub fn print_title(msg: &str)  {
    print!("{}",str_title(msg))
}
pub fn write_title(f: &mut fmt::Formatter,msg: &str)  {
    write!(f,"{}",str_title(msg)).unwrap()
}



pub fn write_header(f: &mut fmt::Formatter,headers: &Vec<&str>, widths:&Vec<usize>, start_index: usize, last_index: usize) {
    let output = make_header(headers,widths,start_index,last_index);
    writeln!(f,"{}",output).unwrap()
}

pub fn print_header(headers: &Vec<&str>, widths: &Vec<usize>, start_index: usize, last_index: usize) {
    println!("{}",make_header(headers,widths,start_index,last_index));
}

pub fn line(length: usize) -> String {
    format!("{}\n","─".repeat(length).as_str().cline())
    // format!("{:⎯<w$}", "".cline().bold())
}

pub fn print_line(length: usize) {
    print!("{}", line(length));
}

pub fn write_line(f: &mut fmt::Formatter,length: usize) {
    if let Err(e) = write!(f,"{}", line(length)) { 
        print_error("Error write",Some(&"write_line".to_string()),Some(&e.to_string()))
    };
}

pub fn double_line(length: usize) -> String {
    format!("{:═<length$}\n", "".cline())
}

pub fn print_double_line(length: usize) {
    print!("{}", double_line(length));
}

pub fn write_double_line(f: &mut fmt::Formatter,length: usize) {
    if let Err(e) = writeln!(f,"{}", double_line(length)) { 
        print_error("Error write",Some(&"write_double_line".to_string()),Some(&e.to_string()))
    };
}

pub fn print_map <T: std::fmt::Display> (title: &str,val: &str, map: T ) {
    print_line(MEDIUM);
    print_info(title, val);
    print_line(MEDIUM);
    println!("{}",map);
    print_line(MEDIUM);
}

pub fn str_key_value(key: &str, value: &str, max_klen:usize, max_vlen:usize) -> String {
    
    let parts: Vec<String> = value.chars()
                    .collect::<Vec<_>>()
                    .chunks(max_vlen)
                    .map(|chunk| chunk.iter().collect())
                    .collect();
    let mut output: String = "".to_string();
    for (i,p) in parts.iter().enumerate() {
        if i == 0 {
            output.push_str(&format!("{:max_klen$}: {}\n", key.cinfo(), p.cvar()));
        }
        else {
            output.push_str(&format!("{:max_klen$}  {}\n", " ", p.cvar()));
        }
    } 
    output
}

pub fn str_hashmap<K: std::fmt::Display,V: std::fmt::Display>(map: &HashMap<K,V>, title: Option<&str>) -> String {

    // let max_width = min(max_width, MAX_WIDTH); 
    let max_width = get_terminal_width();
    let max_k = map.keys().map(|key| key.to_string().len()).max().unwrap_or(max_width);
    let mut max_v = map.values().map(|val: &V| val.to_string().len()).max().unwrap_or(max_width);
    max_v = min(max_v, max_width - max_k - 3);  
    let line_len = max_k + max_v + 3;

    let mut output = String::new();
    // output.push_str(&line(line_len));
    if let Some(t) = title {
        output.push_str(&str_title(t));
        output.push_str(&line(line_len));
    }

    for (key, value) in map {
        output.push_str(&str_key_value(&key.to_string(), &value.to_string(), max_k, max_v));
    }
    output.push_str(&line(line_len));
    output
}


pub fn print_hashmap<K: std::fmt::Display,V: std::fmt::Display>(map: &HashMap<K,V>, title: Option<&str>) { 
    print!("{}",str_hashmap(map,title));
}

pub fn write_hashmap<K: std::fmt::Display,V: std::fmt::Display>(
    f: &mut fmt::Formatter,map: &HashMap<K,V>, title: Option<&str>) {
    let _ = writeln!(f,"{}",str_hashmap(map,title));
}

// In construction
pub fn str_struct<T: serde::Serialize>(title: &str, obj: &T) -> String {

    // let max_width = min(max_width, MAX_WIDTH);
    let max_width = get_terminal_width();
    let mut max_k: usize = 0;
    let mut max_v: usize = 0;

    let json = serde_json::to_value(obj).unwrap();
    let mut output = String::new();

    if let serde_json::Value::Object(map) = json {
        max_k = max_k.max(map.keys().map(|key| key.len()).max().unwrap_or(0)+1);
        max_v = max_v.max(map.values().map(|val| val.to_string().len()).max().unwrap_or(0)+1);
        max_v = min(max_v, max_width - max_k - 3);  
        let line_len = max_k + max_v + 3;

        // output.push_str(&line(line_len));
        output.push_str(&str_title(title));
        output.push_str(&line(line_len));

        for (key, value) in map {
            output.push_str(&str_key_value(&key.to_string(), &value.to_string(), max_k, max_v));
        }

        output.push_str(&line(line_len));
    }
    output
}

pub fn print_struct<T: serde::Serialize>(title: &str, obj: &T) {
    println!("{}", str_struct(title, obj));
}

pub fn write_struct<T: serde::Serialize>(f: &mut fmt::Formatter, title: &str, obj: &T) -> Result<(), std::fmt::Error> {
    writeln!(f, "{}", str_struct(title, obj))
}

fn get_keys_from_value(value: &Value) -> Vec<&str> {
    match value {
        Value::Object(map) => map.keys().map(|k| k.as_str()).rev().collect(),
        _ => vec![],
    }
}

fn get_values_from_value(value: &Value) -> Vec<String> {
    match value {
        // Value::Object(map) => map.values().map(|k| k.as_str().unwrap_or("")).collect(),
        Value::Object(map) => map.values().map(|k| k.to_string().replace("\"",""))
                                                      .rev().collect(),
        _ => vec![],
    }
}

pub fn print_vec_struct<T: serde::Serialize>(title: &str, vec: &Vec<T>) {
    
    let mut table: Vec<Vec<&str>> = Vec::with_capacity(vec.len()+1);
    let obj1 = serde_json::to_value(vec.get(0)).expect("Failed to serialize struct");
    table.push(get_keys_from_value(&obj1));

    let rows_values = vec.iter().map(|v| serde_json::to_value(v).unwrap()).collect::<Vec<Value>>();
    let rows_string = rows_values.iter().map(|v| get_values_from_value(v)).collect::<Vec<Vec<String>>>();
    let rows_str = rows_string.iter().rev()
                                       .map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<&str>>())
                                       .collect::<Vec<Vec<&str>>>();
    
    table.extend(rows_str);
    println!();
    print_title(&title);
    println!();
    print_table(table,true,None,None);
}

pub fn print_start_program(program_name: &str) -> OffsetDateTime {
    let max_width = get_terminal_width();
    let local = OffsetDateTime::now_utc();
    let local_str: &str = &format!("{}", local);
    println!("{RESET}\n");
    print_double_line(max_width);
    println!("{} {}: {}","Start".cinfo(),program_name.ctitle(),local_str.cvar());
    print_double_line(max_width);
    println!("\n");
    local
}

pub fn print_end_program(program_name: &str, start: OffsetDateTime) -> OffsetDateTime {
    let max_width = get_terminal_width();
    let local = OffsetDateTime::now_utc();
    let local_str: &str = &format!("{}", local);
    let duration_str: &str = &format!("{}",(local - start));
    println!("{RESET}\n");
    print_double_line(max_width);
    println!("{} {}: {} - {}","End ".cinfo(),program_name.cinfo().bold(),
                              local_str.cvar(),duration_str.cvar().bold());
    print_double_line(max_width);
    println!("\n");
    local
}

pub enum TreeBlock {
    Item, 
    End
}

pub fn print_tree_item(item: &str,block_type: TreeBlock ) {
    match block_type {
        TreeBlock::Item => println!("{} {}","├──".cline(),item.citem()),
        TreeBlock::End => println!("{} {}","└──".cline(),item.citem()),
    }
}



pub fn make_table(table: Vec<Vec<&str>>, header: bool,title: Option<&str>,column_width: Option<usize>) -> String {

    let max_width = get_terminal_width()-10;
    let max_col_width = column_width.unwrap_or(MAX_COL_WIDTH);

    let title = title.map_or("".to_string(), |t| format!("{}{}",t.ctitle(),"\n"));
    let col_widths = get_column_widths(&table,max_col_width);
    
    let mut output = String::from(title);

    let mut start_index = 0;
    while start_index < table[0].len() {
        let next_index = next_columns_segment(start_index, max_width, &col_widths);
        let table_len = col_widths[start_index..=next_index].iter().sum::<usize>() + SPACE * (1+next_index - start_index);
        let line = format!("{}\n","─".repeat(table_len).as_str().cline());
        for (i, row) in table.iter().enumerate() {
            if i == 0 {
                if header==false {
                    output.push_str(&line);
                } else {
                    output.push_str(&make_header(row, &col_widths,start_index,next_index));
                    output.push_str(&line);
                }
            } else {
                output.push_str(&make_row(row, &col_widths,start_index,next_index));
            }
        }
        output.push_str(&line);
        output.push_str("\n");

        start_index = next_index + 1;
    }
    output
}

pub fn table_from_string(
    table: Vec<Vec<String>>,  
    has_header: bool,title:Option<&str>,column_width:Option<usize>
)-> String {
    let table_vec: Vec<Vec<&str>> = table.iter().map(|row| row.iter().map(|s| s.as_str()).collect()).collect();
    make_table(table_vec, has_header,title,column_width)
}


pub fn print_table(data: Vec<Vec<&str>>, 
    has_header: bool,  
    title: Option<&str>,column_width: Option<usize>
) {
    println!("{}", make_table(data,has_header,title,column_width));
}

pub fn write_table(
    f: &mut fmt::Formatter, 
    data: Vec<Vec<&str>>, 
    has_header: bool, 
    title: Option<&str>,
    column_width: Option<usize>
) -> Result<(), std::fmt::Error> {
    writeln!(f, "{}", make_table(data, has_header, title,column_width))
}

pub fn make_header(
    headers: &Vec<&str>,
    widths: &Vec<usize>,
    start_index: usize,
    last_index: usize
) -> String {
    let mut output = String::new();
    let header_rows = (start_index..=last_index)
        .map(|i| headers[i].len() / widths[i])
        .max()
        .unwrap_or(0);

    for hr in 0..header_rows {
        for (i, cell) in headers[start_index..=last_index].iter().enumerate() {
            let padded = format!("{:width$}", cell, width = widths[i] * header_rows);
            let slice = &padded[hr*widths[i]..widths[i]*(hr+1)];
            output.push_str(&format!("{}{}", slice.column(i)," ".repeat(SPACE)));
        }
        output.push_str("\n");
    }
    output
}

pub fn make_row(
    row: &Vec<&str>,
    widths: &Vec<usize>,
    start_index: usize,
    last_index: usize
) -> String {
    let mut output = String::new();
    for (i,cell) in row[start_index..=last_index].iter().enumerate() {
        if cell.len() > widths[i] {
            let truncated = &format!("{}*",&cell[..widths[i]-1]);
            output.push_str(&format!("{:width$}{}", truncated.column(i)," ".repeat(SPACE), width=widths[i]));
        } else {
            output.push_str(&format!("{:width$}{}", cell.column(i)," ".repeat(SPACE), width=widths[i]));
        }
    }
    output.push_str("\n");
    output
}

fn next_columns_segment(
    start_index: usize,
    max_width: usize,
    col_widths: &Vec<usize>
) -> usize {
    let mut current_width = 0;
    for (i, &width) in col_widths.iter().enumerate().skip(start_index) {
        current_width += width + SPACE;
        if current_width > max_width {
            return i - 1;
        }
    }
    col_widths.len()-1
}

pub fn get_column_widths(
    table: &Vec<Vec<&str>>,
    column_width:usize
) -> Vec<usize> {
    let mut max_lengths: Vec<usize> = vec![0; table[0].len()];
    for row in table {
        for (i, cell) in row.iter().enumerate() {
            max_lengths[i] = min(max_lengths[i].max(cell.len()),column_width);
        }
    }
    max_lengths
}

pub fn print_request(request: &Request) {
    let req_map = request_to_hashmap(request);
    print_hashmap(&req_map, Some("Request"));
}

pub fn print_response(method: HttpMethod,response: &Response) {
    let req_map = response_to_hashmap(method, response);
    print_hashmap(&req_map, Some("Response"));
}

pub fn print_headers(headers: &reqwest::header::HeaderMap) {
    let headers_map = headers_to_hashmap(headers);
    print_hashmap(&headers_map, Some("Headers"));
}