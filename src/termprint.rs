use std::fmt;

use colored::{Colorize,ColoredString};
use time::OffsetDateTime;
// use serde::Serialize;
// use serde_json::json;

pub const LONG: usize = 120;
pub const MEDIUM: usize = 80;
pub const SHORT: usize = 30;
pub const SPACE: usize = 3;

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

#[cfg(feature = "basic")]
impl ColoredItem for &str {
    fn cinfo(&self) -> ColoredString {
        self.blue()
    }

    fn cvar(&self) -> ColoredString {
        self.cyan()
    }

    fn ctitle(&self) -> ColoredString {
        self.bold().blue()
    }

    // fn cheader(&self) -> ColoredString {
    //     self.italic().blue()
    // }

    fn cerror(&self) -> ColoredString {
        self.red().bold()
    }

    fn cwarning(&self) -> ColoredString {
        self.yellow()
    }

    fn citem(&self) -> ColoredString {
        self.cyan()
    }

    fn cline(&self) -> ColoredString {
        self.blue()
    }
}

#[cfg(feature = "truecolor")]
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

    // fn cheader(&self) -> ColoredString {
    //     self.italic().truecolor(51, 102, 255)  
    // }

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

    // fn cheader(&self) -> ColoredString {
    //     self.italic().white() 
    // }

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
}

impl ColoredItem for String {
    fn cinfo(&self) -> ColoredString {
        self.as_str().cinfo()
    }

    fn cvar(&self) -> ColoredString {
        self.as_str().cvar()
    }

    fn ctitle(&self) -> ColoredString {
        self.as_str().ctitle()
    }

    // fn cheader(&self) -> ColoredString {
    //     self.as_str().cheader()
    // }

    fn cerror(&self) -> ColoredString {
        self.as_str().cerror()
    }

    fn cwarning(&self) -> ColoredString {
        self.as_str().cwarning()
    }

    fn citem(&self) -> ColoredString {
        self.as_str().citem()
    }

    fn cline(&self) -> ColoredString {
        self.as_str().cline()
    }

    fn column(&self,index:usize) -> ColoredString {
        self.as_str().column(index)
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


pub fn print_index2rgb() {
    print_message("\nColor index to rgb:");
    print_line(Some(MEDIUM));
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

pub fn key_value(key: &str, value: &str, width: Option<usize>) -> String {
    let w: usize = width.unwrap_or(key.len());
    format!("{:w$}: {}",key.cinfo(),value.cvar())
}

pub fn print_key_value(key: &str, value: &str, width: Option<usize>) {
    println!("{}",key_value(key,value,width));
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

pub fn info(key: &str, value: &str, width: Option<usize>) -> String {
    let w: usize = width.unwrap_or(key.len());
    format!("{:w$}{}",key.cinfo(),value.cvar())
}

pub fn print_info(key: &str, value: &str, width: Option<usize>) {
    println!("{}",info(key, value, width));
}

pub fn write_info(f: &mut fmt::Formatter,key: &str, value: &str, width: Option<usize>) {
    let w: usize = width.unwrap_or(key.len());
    if let Err(e) = writeln!(f,"{:w$}{}",key.cinfo(),value.cvar()) { 
        print_error("Error write",Some(&"write_info".to_string()),Some(&e.to_string()))
    };
}

pub fn message(txt: &str) -> String {
    format!("{}",txt.cinfo())
}

pub fn print_message(txt: &str)  {
    println!("{}",message(txt));
}

pub fn write_msg(f: &mut fmt::Formatter,txt: &str)  {
    writeln!(f,"{}",message(txt)).unwrap()
}

pub fn title(msg: &str) -> String {
    format!("{}",msg.ctitle().bold())
}

pub fn print_title(msg: &str)  {
    println!("{}",title(msg))
}
pub fn write_title(f: &mut fmt::Formatter,msg: &str) -> Result<(), std::fmt::Error> {
    writeln!(f,"{}",title(msg))
}

pub fn line(length: Option<usize>) -> String {
    let w: usize = length.unwrap_or(MEDIUM);
    format!("{:⎯<w$}", "".cline())
}

pub fn print_line(length: Option<usize>) {
    println!("{}", line(length));
}

pub fn write_line(f: &mut fmt::Formatter,length: Option<usize>) {
    if let Err(e) = writeln!(f,"{}", line(length)) { 
        print_error("Error write",Some(&"write_line".to_string()),Some(&e.to_string()))
    };
}

pub fn double_line(length: Option<usize>) -> String {
    let w: usize = length.unwrap_or(MEDIUM);
    format!("{:═<w$}", "".cline())
}

pub fn print_double_line(length: Option<usize>) {
    println!("{}", double_line(length));
}

pub fn write_double_line(f: &mut fmt::Formatter,length: Option<usize>) {
    if let Err(e) = writeln!(f,"{}", double_line(length)) { 
        print_error("Error write",Some(&"write_double_line".to_string()),Some(&e.to_string()))
    };
}

pub fn print_map <T: std::fmt::Display> (title: &str,val: &str, map: T ) {
    print_line(Some(MEDIUM));
    print_key_value(title, val , None);
    print_line(Some(MEDIUM));
    println!("{}",map);
    print_line(Some(MEDIUM));
}

pub fn print_struct<T: serde::Serialize>(title: &str, val: &str, obj: &T, width: Option<usize>) {
    println!("");
    print_line(width);
    print_key_value(&title.bold(), val, None);
    print_line(width);
    let json = serde_json::to_value(obj).unwrap();
    if let serde_json::Value::Object(map) = json {
        let max_key_length = map.keys().map(|key| key.len()).max().unwrap_or(0)+1;
        for (key, value) in map {
            let formatted_key = format!("{:w$}", key, w = max_key_length);
            print_key_value(&formatted_key, &value.to_string(), Some(max_key_length));
        }
    }
    print_line(width);
}

pub fn print_start_program(program_name: &str) -> OffsetDateTime {
    let local = OffsetDateTime::now_utc();
    let local_str: &str = &format!("{}", local);
    println!("\n");
    print_double_line(Some(MEDIUM - 2));
    println!("{} {}: {}","Start".cinfo(),program_name.ctitle(),local_str.cvar());
    print_double_line(Some(MEDIUM - 2));
    println!("\n");
    local
}

pub fn print_end_program(program_name: &str, start: OffsetDateTime) -> OffsetDateTime {
    let local = OffsetDateTime::now_utc();
    let local_str: &str = &format!("{}", local);
    let duration_str: &str = &format!("{}",(local - start));
    println!("\n");
    print_double_line(Some(MEDIUM));
    println!("{} {}: {} - {}","End ".cinfo(),program_name.cinfo().bold(),
                              local_str.cvar(),duration_str.cvar().bold());
    print_double_line(Some(MEDIUM));
    println!("\n");
    local
}

pub enum TreeBlock {
    Item, 
    End
}

pub fn print_tree_item(item: &str,block_type: TreeBlock ) {
    match block_type {
        TreeBlock::Item => println!("{} {}","├──".cline(),item.cvar()),
        TreeBlock::End => println!("{} {}","└──".cline(),item.cvar()),
    }
}

pub fn table(table: Vec<Vec<&str>>, header: bool) -> String {
    let mut max_lengths: Vec<usize> = vec![0; table[0].len()];
    for row in &table {
        for (i, cell) in row.iter().enumerate() {
            max_lengths[i] = max_lengths[i].max(cell.len());
        }
    }

    let mut output = String::new();
    let table_len = max_lengths.iter().sum::<usize>() + SPACE * (max_lengths.len() - 1);
    let line = format!("{}\n",&"─".repeat(table_len + SPACE).cline());
    for (i, row) in table.iter().enumerate() {
        if i == 1 {
            output.push_str(&line);
        }
        for (j, cell) in row.iter().enumerate() {
            let mut padded_cell = format!("{:width$}", cell.column(j), width = max_lengths[j]);
            if i ==0 && header {
                padded_cell = format!("{:width$}", cell.column(j).bold(), width = max_lengths[j]);
            }
            output.push_str(&padded_cell);
            output.push_str(&" ".repeat(SPACE));
        }
        output.push('\n');
    }
    output.push_str(&line);
    output
}

pub fn print_table(data: Vec<Vec<&str>>) {
    println!("{}", table(data,true));
}