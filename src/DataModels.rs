use std::fs::{OpenOptions};
use std::io::{BufRead, BufWriter};
use eframe::egui::{Color32, Frame};
use std::io::Write;
use std::sync::mpsc::{Sender,Receiver};
use rust_xlsxwriter::{Workbook, XlsxError};

pub struct ConverterBody {
    pub input : String,
    pub separator : String,
    pub smart_mode : bool
}
pub struct FilterBody {
    pub filter : String,
    pub main_body : String,
}

pub struct LogsBlock {
    pub display_logs : Vec<String>,
    pub logs_sender : Sender<String>,
    pub logs_receiver : Receiver<String>
}

pub struct MainBody {
    pub excel_writer : bool,
    pub converter : ConverterBody,
    pub way_to_file : String,
    pub filter : FilterBody,
    pub position : Switch,
    pub logs : LogsBlock // For pushing logs in the bottom field.
}
#[derive(PartialEq)]
pub enum Switch {
    Converter,
    Filter
}

impl MainBody {
    pub fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        return MainBody {
            excel_writer: false,
            converter: ConverterBody {
                input: "".to_string(),
                separator : "".to_string(),
                smart_mode: false
            },
            way_to_file: r#"C:\Users\User\Desktop\text.txt"#.to_string(),
            filter: FilterBody {
                filter: "".to_string(),
                main_body: "".to_string(),
            },
            position : Switch::Converter,
            logs: LogsBlock {
                display_logs: Vec::new(),
                logs_sender: tx,
                logs_receiver: rx,
            },
        }
    }
}

pub fn return_central_frame() -> Frame {
    return Frame {
        inner_margin: Default::default(),
        outer_margin: Default::default(),
        rounding: Default::default(),
        shadow: Default::default(),
        fill: Color32::from_rgb(21, 27, 31),
        stroke: Default::default(),
    }
}

pub fn convert_and_write(text : String, path : String, separator : String, logs : Sender<String>, condition : bool) -> () {
    tokio::spawn(async move {
        let formed = text.trim().replace(&separator, "\n");

        let _ = logs.send("Данные переформатированы.".to_string());

        let mut transformed : Vec<&str> = formed.split('\n').collect();
        transformed.retain(|value| value.len() == 21 && check_if_code(value));

        let _ = logs.send(format!("Всего кодов извлечено : {}", transformed.len()));

        if condition {
            excel_writer(transformed, logs, path)
        }
        else {
            println!("Transformed : {:#?}", transformed);

            let mut buffer : String = String::new();
            let _ = transformed.iter().for_each(|value|
                buffer.push_str(add_n(value).as_str())
            );

            let _ = logs.send("Данные готовы для записи в документ".to_string());

            match OpenOptions::new().write(true).append(true).open(path) {
                Ok(file) => {
                    let mut f = BufWriter::new(file);
                    write!(f, "{}", buffer).unwrap();
                    let _ = logs.send("Файл был записан конвертированными данными".to_string());
                }
                Err(_) => {
                    let _ = logs.send("Произошла ошибка при открытии файла для записи.".to_string());
                }
            };
        }
    });
}

pub fn filter_and_write(to_filter : String, main_body : String, path : String, logs : Sender<String>, condition : bool) -> () {
    tokio::spawn(async move {
        if to_filter != "".to_string() && main_body != "".to_string() {
            let filter_vec: Vec<&str> = to_filter.trim().split('\n').collect::<Vec<&str>>();
            let _ = logs.send(format!("Найдено линий в строке для выреза : {}", filter_vec.len()));
            let mut main_vec: Vec<&str> = main_body.trim().split('\n').collect::<Vec<&str>>();
            let _ = logs.send(format!("Найдено линий в главной строке : {}", main_vec.len()));
            for element in filter_vec {
                main_vec.retain(|unf| unf != &element)
            };
            let _ = logs.send(format!("Осталось линий после фильтрации : {}", main_vec.len()));
            if condition {
                excel_writer(main_vec, logs, path)
            }
            else {
                let mut write_string: String = String::new();
                for elements in main_vec {
                    write_string.push_str(elements);
                    write_string.push('\n')
                };
                match OpenOptions::new().write(true).append(true).open(path.trim()) {
                    Ok(file) => {
                        let mut f = BufWriter::new(file);
                        write!(f, "{}", write_string).unwrap();
                        let _ = logs.send("Файл был записан отфильтрованными данными".to_string());
                    },
                    Err(_) => {
                        let _ = logs.send("Произошла ошибка при открытии файла для записи.".to_string());
                    }
                };
            }
        }
        else {
            write_log("Ошибка, заполните два верхних поля данными", logs)
        }
    });
}

pub fn clear_logs(logs : Sender<String>) -> () {let _ = logs.send("$CLEARLOGSCOMMAND$$".to_string());}

pub fn write_log(text : &str, logs : Sender<String>) -> () {let _ = logs.send(text.to_string());}

pub fn add_n(text : &str) -> String {
    let mut buffer : String = String::from(text);
    buffer.push('\n');
    return buffer
}

pub fn path_extractor(path : String) -> Result<String, ()> {
    let mut vec = path.split('.').collect::<Vec<&str>>();
    let last = vec.last_mut().unwrap();
    *last = ".xlsx";
    let mut buffer : String = String::new();
    vec.into_iter().for_each(|value| buffer.push_str(value));
    return Ok(buffer)
}

pub fn check_if_code(object : &&str) -> bool {
    let mut counter : u8 = 0;
    let _ : Vec<_>= object.chars().map(|value| if value.is_numeric() {
        counter += 1
    }).collect();
    if counter >= 7 {
        return true
    }
    else {
        return false
    }
}

pub fn excel_writer(transformed : Vec<&str>, logs : Sender<String>, path : String) -> () {
    let mut workbook = Workbook::new();
    let work_sheet = workbook.add_worksheet();
    work_sheet.set_column_width(0, 40).unwrap();
    let _ = transformed.into_iter().enumerate().for_each(|(a, b)| {
        work_sheet.write(a as u32, 0, b).unwrap();
    });
    match path_extractor(path) {
        Ok(value) => {
            match workbook.save(&value) {
                Ok(_) => {
                    write_log(format!("Файл успешно записан {}", &value).as_str(), logs);
                }
                Err(_) => {
                    write_log("Ошибка записи файла", logs)
                }
            }
        }
        Err(_) => {write_log("Ошибка формирования пути для записи.", logs)}
    }
}