use std::fmt;

use colored::Colorize;
use colored::customcolors::CustomColor;

use reqwest::{Response, Request};
use reqwest::header::HeaderMap;

use time::OffsetDateTime;

const BLUE4: CustomColor = CustomColor{r:137, g:209,b:255};
const BLUE7: CustomColor = CustomColor{r:0, g:112,b:242};
const RED7: CustomColor = CustomColor{r:210, g:10,b:10};
const RED4: CustomColor = CustomColor{r:255, g:140,b:178};
const MANGO2: CustomColor = CustomColor{r:255, g:243,b:184};

// pub const INFO: CustomColor = BLUE7;
// pub const VAR: CustomColor = BLUE4;
// pub const LINE: CustomColor = BLUE4;
// pub const ERROR: CustomColor = RED7;
// pub const ERROR_PINK: CustomColor = RED4;
// pub const WARNING: CustomColor = MANGO2;
// pub const TREE: CustomColor = BLUE7;

pub const INFO: CustomColor = BLUE7;
pub const VAR: CustomColor = BLUE4;
pub const LINE: CustomColor = BLUE4;
pub const ERROR: CustomColor = RED7;
pub const ERROR_PINK: CustomColor = RED4;
pub const WARNING: CustomColor = MANGO2;
pub const TREE: CustomColor = BLUE7;
pub const LONG: usize = 120;
pub const MEDIUM: usize = 80;
pub const SHORT: usize = 30;


pub fn print_key_value(key: &str, value: &str, width1: Option<usize>) {
    let width: usize = width1.unwrap_or(key.len());
    println!("{:width$}: {}",key.custom_color(INFO),value.custom_color(VAR));
}

pub fn key_value(key: &str, value: &str, width1: Option<usize>) -> String {
    let width: usize = width1.unwrap_or(key.len());
    format!("{:width$}: {}",key.custom_color(INFO),value.custom_color(VAR))
}

pub fn error(message: &str) -> String {
    format!("ERROR: {}",message.custom_color(ERROR))
}

pub fn print_error(message: &str, info: Option<&str>, err_msg: Option<&str>) {
    if info.is_none() & err_msg.is_none() {
        println!("{} {}","ERROR ".to_string().bold().custom_color(ERROR),message.custom_color(ERROR));
    }
    else {
        print!("{} {}","ERROR:".to_string().bold().custom_color(ERROR),message.custom_color(ERROR));
    }
    if let Some(info_str) =  info {
        println!(": {}",info_str.custom_color(ERROR));
    }
    if let Some(err_str) =  err_msg {
        println!("{}",err_str.custom_color(ERROR_PINK));
    }
}

pub fn warning(message: &str) -> String {
    format!("Warning: {}",message.custom_color(WARNING))
}

pub fn print_warning(message: &str)  {
    println!("Warning: {}",message.custom_color(WARNING));
}

pub fn info(key: &str, value: &str, width1: Option<usize>) -> String {
    let width: usize = width1.unwrap_or(key.len());
    format!("{:width$}{}",key.custom_color(INFO),value.custom_color(VAR))
}

pub fn print_info(key: &str, value: &str, width1: Option<usize>) {
    let width: usize = width1.unwrap_or(key.len());
    println!("{:width$}{}",key.custom_color(INFO),value.custom_color(VAR));
}
pub fn write_info(f: &mut fmt::Formatter,key: &str, value: &str, width1: Option<usize>) {
    let width: usize = width1.unwrap_or(key.len());
    if let Err(e) =writeln!(f,"{:width$}{}",key.custom_color(INFO),value.custom_color(VAR)) { 
        print_error("Error write",Some(&"write_info".to_string()),Some(&e.to_string()))
    };
}

pub fn msg(msg: &str) -> String {
    format!("{}",msg.custom_color(INFO))
}

pub fn print_msg(msg: &str)  {
    println!("{}",msg.custom_color(INFO))
}

pub fn write_msg(f: &mut fmt::Formatter,msg: &str)  {
    writeln!(f,"{}",msg.custom_color(INFO)).unwrap()
}

pub fn print_title(msg: &str)  {
    println!("{}",msg.custom_color(INFO).bold())
}
pub fn write_title(f: &mut fmt::Formatter,msg: &str)  {
    writeln!(f,"{}",msg.custom_color(INFO).bold()).unwrap()
    
}
// pub fn print_info(key: &str, value: &str, width1: Option<usize>) -> String {
//     let width: usize = width1.unwrap_or(key.len());
//     println!("{:width$}: {}",key.custom_color(INFO),value.custom_color(VAR))
// }

pub fn print_line(length: Option<usize>) {
    let width: usize = length.unwrap_or(MEDIUM);
    println!("{:⎯<width$}", "".custom_color(LINE));
}

pub fn write_line(f: &mut fmt::Formatter,length: Option<usize>) {
    let width: usize = length.unwrap_or(MEDIUM);
    if let Err(e) = writeln!(f,"{:⎯<width$}", "".custom_color(LINE)) { 
        print_error("Error write",Some(&"write_line".to_string()),Some(&e.to_string()))
    };
}

pub fn print_double_line(length: Option<usize>) {
    let width: usize = length.unwrap_or(MEDIUM);
    println!("{:═<width$}", "".custom_color(LINE));
}

pub fn write_double_line(f: &mut fmt::Formatter,length: Option<usize>) {
    let width: usize = length.unwrap_or(MEDIUM);
    if let Err(e) = writeln!(f,"{:═<width$}", "".custom_color(LINE)) { 
        print_error("Error write",Some(&"write_double_line".to_string()),Some(&e.to_string()))
    };
}

pub fn print_map <T: std::fmt::Display> (title: &str,val: &str, map: T ) {
    print_line(Some(LONG));
    print_key_value(title, val , None);
    print_line(Some(LONG));
    println!("{}",map);
    print_line(Some(LONG));
}

pub enum HttpMethod {
    GET,
    PUT,
    DELETE,
    POST,
}

impl HttpMethod {
    fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
        }
    }
}

pub fn print_response(method: HttpMethod, response: &Response) {
    let width: usize = 23;
    print_line(Some(LONG));
    print_title(&String::from("Response"));
    print_line(Some(LONG));
    print_info("Status", response.status().as_str(),Some(width));
    print_info("Method",method.as_str(),Some(width));
    print_info("URL",response.url().as_str(),Some(width));
    print_title("Headers");
    for (key, value) in response.headers() {
        print_info(&key.to_string(), &value.to_str().unwrap(), Some(width));
    }
    print_line(Some(LONG));
}

pub fn print_headers(headers: &HeaderMap, title: Option<&str>) {
    let width: usize = 20;
    print_line(Some(LONG));
    match title {
        Some(t)=> print_title(t),
        None => print_title("Headers")
    }
    print_line(Some(LONG));
    for (key, value) in headers {
        print_info(&key.to_string(), &value.to_str().unwrap(),Some(width));
    }
    print_line(Some(LONG));
}

pub fn write_headers(f: &mut fmt::Formatter,headers: &HeaderMap, title: Option<&str>) {
    let width: usize = 20;
    write_line(f,Some(LONG));
    match title {
        Some(t)=> write_title(f,t),
        None => write_title(f,"Headers")
    }
    write_line(f,Some(LONG));
    for (key, value) in headers {
        info(&key.to_string(), &value.to_str().unwrap(),Some(width));
    }
    write_line(f,Some(LONG));
}

pub fn print_request(request:&Request) {
    let width: usize = 20;
    print_line(Some(LONG));
    print_title(&String::from("Request"));
    print_line(Some(LONG));
    print_info("Method",request.method().as_str(),Some(width));
    print_info("URL",request.url().as_str(),Some(width));
    print_title("Headers");
    for (key, value) in request.headers() {
        print_info(&key.to_string(), &value.to_str().unwrap(), Some(width));
    }
    print_line(Some(LONG));
}

pub fn print_start_program(program_name: &str) -> OffsetDateTime {
    let local = OffsetDateTime::now_utc();
    println!("\n");
    print_double_line(Some(LONG));
    println!("{} {}: {}",String::from("Start").custom_color(INFO),program_name.bold().custom_color(INFO),local.to_string().custom_color(INFO));
    print_double_line(Some(LONG));
    println!("\n");
    local
}

pub fn print_end_program(program_name: &str, start: OffsetDateTime) -> OffsetDateTime {
    let local = OffsetDateTime::now_utc();
    let duration = (local - start).to_string();
    println!("\n");
    print_double_line(Some(LONG));
    println!("{} {}: {} - {}",String::from("End ").custom_color(INFO),program_name.bold().custom_color(INFO),
                              local.to_string().custom_color(VAR),duration.custom_color(VAR).bold());
    print_double_line(Some(LONG));
    println!("\n");
    local
}

pub enum TreeBlock {
    Item, 
    End
}

pub fn print_tree_item(item: &str,block_type: TreeBlock ) {
    match block_type {
        TreeBlock::Item => println!("{} {}","├──".custom_color(INFO),item.custom_color(VAR)),
        TreeBlock::End => println!("{} {}","└──".custom_color(INFO),item.custom_color(VAR)),
    }
}