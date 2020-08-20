// use std::fs::{self, OpenOptions};
// use std::io;
// use std::path::{Path, PathBuf};
use std::env;
use std::fs;
// use toml_edit::{Document, value};
use toml_edit::Document;
use indexmap::IndexMap;

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

    let mut map = &mut IndexMap::new();
    for (key, val) in doc.iter() {
        print!("Key=[{: >30}]\t", key);
        print_item(val, key, map);
    }

    println!("Map=[{:?}]", map);
    
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

fn print_table(table: &dyn toml_edit::TableLike, key: &str, map: &mut IndexMap<String, String>) {
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
        },
        toml_edit::Value::String(s) => {
            &map.insert(key.to_string(), s.to_string());
            println!("String)={}", s)
        },
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
        toml_edit::Value::InlineTable(it) => print_table(it, key, map), //println!("InlineTable)={}", it),
    }
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
