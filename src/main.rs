extern crate qrcode;
extern crate arboard;

use std::{env,io};
use qrcode::QrCode;
use qrcode::render::unicode;
use arboard::Clipboard;

fn main() {

    let mut source = "stdin";

    for argument in env::args() {
        if (argument == "-c") {
            source = "clipboard"
        }
    }

    let content = read_content(source);

    let code = QrCode::new(content).unwrap();
    let image = code.render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    println!("{}", image);
}

fn read_content(source: &str) -> String {
    if (source == "clipboard") {
        let mut clipboard = Clipboard::new().unwrap();
        return clipboard.get_text().unwrap().to_string();
    }

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    return buffer;

}