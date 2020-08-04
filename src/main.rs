// use std::fs::{self, OpenOptions};
// use std::io;
// use std::path::{Path, PathBuf};
use std::env;
use std::fs;
// use toml_edit::{Document, value};
use toml_edit::Document;

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

    for (key, val) in doc.iter() {
        print!("Key=[{: >30}]\t", key);
        print_item(val);
    }

    // test writing a key/value pair and saving it to see
    // if it maintains the comments
    doc["TestKey"] = toml_edit::value("TestValue");
    let outstr = doc.to_string_in_original_order();
    fs::write("foo.toml", outstr).unwrap();
}

fn print_item(item: &toml_edit::Item) {
    print!("Item(");
    match item {
        toml_edit::Item::Value(v) => print_value(v),
        toml_edit::Item::Table(t) => print_table(t),
        toml_edit::Item::ArrayOfTables(a) => println!("ArrayOfTables-{:?}", a),
        toml_edit::Item::None => println!("None"),
    }
}

fn print_table(table: &dyn toml_edit::TableLike) {
    print!("Table)=\n");
    for (key, value) in table.iter() {
        print!("\tKey=[{: >30}] ", key);
        print_item(value);
    }
}

// This works too
// fn print_value(value: &toml_edit::Value) {
//     println!("Value={}", value);
// }

fn print_value(value: &toml_edit::Value) {
    // print!("DecorPrefix=[{}]", value.decor().prefix());
    // print!("DecorSuffix=[{}]", value.decor().suffix());
    match value {
        toml_edit::Value::Integer(i) => println!("Integer)={}", i),
        toml_edit::Value::String(s) => println!("String)={}", s),
        toml_edit::Value::Float(f) => println!("Float)={}", f),
        toml_edit::Value::DateTime(dt) => println!("DateTime)={}", dt),
        toml_edit::Value::Boolean(b) => println!("Boolean)={}", b),
        toml_edit::Value::Array(a) => print_array(&a),
        toml_edit::Value::InlineTable(it) => print_table(it), //println!("InlineTable)={}", it),
    }
}

fn print_array(array: &toml_edit::Array) {
    print!("Array)=");
    let mut counter = 0;
    for value in array.iter() {
        if counter == 0 {
            println!("");
            counter = counter + 1;
        }
        println!("\t\t\tValue: {}", value);
    }
}
