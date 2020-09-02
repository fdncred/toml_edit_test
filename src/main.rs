// use std::fs::{self, OpenOptions};
// use std::io;
// use std::path::{Path, PathBuf};
use std::env;
use std::fs;
// use toml_edit::{Document, value};
use indexmap::IndexMap;
use toml_edit::Document;
// use serde::{Serialize, Deserialize};
// use serde_json::{Result, Value};
use ansi_term::{Color, Style};


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap();
    let mut doc = contents.parse::<Document>().expect("Invalid TOML file.");

    // let green = "\u{1b}[32m";
    // let reset = "\u{1b}[0m";

    // for (key, val) in doc.iter() {
    //     println!("{}Key{} = [{:#?}] {}Value{} = [{:#?}]",
    //     green, reset, key,
    //     green, reset, val);
    // }

    let map = &mut IndexMap::new();
    for (key, val) in doc.iter() {
        print!("Key=[{: >30}]\t", key);
        print_item(val, key, map);
    }

    println!("Map\n{:?}", map);

    // test writing a key/value pair and saving it to see
    // if it maintains the comments
    doc["TestKey"] = toml_edit::value("TestValue");
    let outstr = doc.to_string_in_original_order();
    fs::write("foo.toml", outstr).unwrap();
}

fn print_item(item: &toml_edit::Item, key: &str, map: &mut IndexMap<String, String>) {
    print!("Item(");
    match item {
        toml_edit::Item::Value(v) => print_value(v, key, map),
        toml_edit::Item::Table(t) => print_table(t, key, map),
        toml_edit::Item::ArrayOfTables(a) => println!("ArrayOfTables-{:?}", a),
        toml_edit::Item::None => println!("None"),
    }
}

fn print_table(
    table: &dyn toml_edit::TableLike,
    _table_key: &str,
    map: &mut IndexMap<String, String>,
) {
    print!("Table)=\n");
    for (key, value) in table.iter() {
        print!("\tKey=[{: >30}] ", key);
        print_item(value, key, map);
    }
}

// This works too
// fn print_value(value: &toml_edit::Value) {
//     println!("Value={}", value);
// }

fn print_value(value: &toml_edit::Value, key: &str, map: &mut IndexMap<String, String>) {
    // print!("DecorPrefix=[{}]", value.decor().prefix());
    // print!("DecorSuffix=[{}]", value.decor().suffix());
    match value {
        toml_edit::Value::Integer(i) => {
            &map.insert(key.to_string(), i.to_string());
            println!("Integer)={}", i)
        }
        toml_edit::Value::String(s) => {
            &map.insert(key.to_string(), s.to_string());
            println!("String)={}", s)
        }
        toml_edit::Value::Float(f) => {
            &map.insert(key.to_string(), f.to_string());
            println!("Float)={}", f)
        }
        toml_edit::Value::DateTime(dt) => {
            &map.insert(key.to_string(), dt.to_string());
            println!("DateTime)={}", dt)
        }
        toml_edit::Value::Boolean(b) => {
            &map.insert(key.to_string(), b.to_string());
            println!("Boolean)={}", b)
        }
        toml_edit::Value::Array(a) => print_array(&a, key, map),
        toml_edit::Value::InlineTable(it) => {
            let style: IndexMap<&str, String> =
                it.iter()
                .map(|(k, v)|
                (k, v.to_string())).collect();
            println!("StyleMap = [{:#?}]", &style);
            let ansi_style = create_style(style);
            println!("Style=[{:#?}]", ansi_style);
            print_table(it, key, map) //println!("InlineTable)={}", it),
        }
    }
}

fn get_color_from_string(color_str: &str) -> ansi_term::Color {
    match color_str.as_ref() {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "purple" => Color::Purple,
        "cyan" => Color::Cyan,
        "white" => Color::White,
        // starts with #
        // is a number
        // starts with RGB
        _ => Color::White,
    }
}

fn create_style(style_map: IndexMap<&str, String>) -> Style {
    let mut style = Style::default();
    for (k, v) in style_map.iter() {
        println!("K=[{}] V=[{}]", k, v.trim());
        let key = k.trim();
        let value =v.trim().trim_matches('"');
        match key {
            "fg" => style = style.fg(get_color_from_string(value)),
            "bg" => style = style.on(get_color_from_string(value)),
            "is_bold" => style = if value == "true" { style.bold() } else { style },
            "is_dimmed" => style = if value == "true" { style.dimmed() } else { style },
            "is_italic" => style = if value == "true" { style.italic() } else { style },
            "is_underline" => style = if value == "true" { style.underline() } else { style },
            "is_blink" => style = if value == "true" { style.blink() } else { style },
            "is_reverse" => style = if value == "true" { style.reverse() } else { style },
            "is_hidden" => style = if value == "true" { style.hidden() } else { style },
            "is_strikethrough" => style = if value == "true" { style.strikethrough() } else { style },
            _ => (),
        };
    }

    println!("InStyle=[{:?}]", &style);
    style
}

fn print_array(array: &toml_edit::Array, key: &str, map: &mut IndexMap<String, String>) {
    print!("Array)=");
    let mut counter = 0;
    for value in array.iter() {
        if counter == 0 {
            println!("");
            counter = counter + 1;
        }
        &map.insert(key.to_string(), value.to_string());
        println!("\t\t\tValue: {}", value);
    }
}
