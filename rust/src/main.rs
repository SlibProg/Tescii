use std::{
    io,
    sync::{LazyLock, Mutex},
    thread::sleep,
    time::Duration,
};

use figlet_rs::FIGlet;

static TYPE: LazyLock<Mutex<FIGlet>> = LazyLock::new(|| Mutex::new(FIGlet::standard().unwrap()));
static TEXT: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("nothing".to_string()));

fn main() {
    clearscreen::clear().unwrap();

    let _ascii = r#"
        ______               _ _
       /_  __/__  __________(_|_)
        / / / _ \/ ___/ ___/ / /
       / / /  __(__  ) /__/ / /
      /_/  \___/____/\___/_/_/
        "#;

    println!("{}", _ascii);
    println!("Text to Ascii Art in Rust");
    println!("");
    sleep(Duration::from_secs(2));
    println!("Welcome to TESCII!");
    sleep(Duration::from_secs(1));
    println!("An Text to Ascii Art convertor!");
    sleep(Duration::from_secs(1));
    println!("Written in Rust, and made by Slib.");
    sleep(Duration::from_secs(3));
    clearscreen::clear().unwrap();
    sleep(Duration::from_secs(2));
    println!("Ascii Art Type:");
    println!("(Write down the numbers to select)");
    println!("");
    println!("0 - Standart");
    println!("1 - Small");
    println!("2 - Big");
    println!("3 - Slant (Standart but Italic)");
    sleep(Duration::from_secs(1));
    println!("Select:");
    let mut r#type = TYPE.lock().unwrap();
    let mut choosen_type = String::new();
    io::stdin().read_line(&mut choosen_type).expect("Error");

    if choosen_type.trim() == "0" {
        *r#type = FIGlet::standard().unwrap();
    } else if choosen_type.trim() == "1" {
        *r#type = FIGlet::big().unwrap();
    } else if choosen_type.trim() == "2" {
        *r#type = FIGlet::big().unwrap();
    } else if choosen_type.trim() == "3" {
        *r#type = FIGlet::slant().unwrap();
    };
    drop(r#type);
    text();
}

fn text() {
    clearscreen::clear().unwrap();
    println!("Type selected!");
    sleep(Duration::from_secs(1));
    println!("Now, write your text:");
    let mut text = TEXT.lock().unwrap();
    let mut message = String::new();
    io::stdin().read_line(&mut message).expect("Error");

    *text = message.trim().to_string();
    drop(text);
    show();
}

fn show() {
    clearscreen::clear().unwrap();
    let text = TEXT.lock().unwrap();
    let r#type = TYPE.lock().unwrap();
    sleep(Duration::from_millis(500));
    println!("{}", r#type.convert(&text).unwrap());
}
