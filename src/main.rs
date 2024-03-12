use leptess::LepTess;
use std::env;
use std::path::Path;

mod regex;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <image_path>", args[0]);
        std::process::exit(1);
    }

    let image_path = &args[1];
    let image_path = Path::new(image_path);

    if !image_path.exists() {
        eprintln!("No file: {:?}", image_path);
        std::process::exit(1);
    }

    let mut lt = match LepTess::new(None, "eng") {
        Ok(lt) => lt,
        Err(e) => {
            eprintln!("Tesseract missing: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = lt.set_image(image_path) {
        eprintln!("Tesseract error: {}", e);
        std::process::exit(1);
    }

    match lt.get_utf8_text() {
        Ok(text) => println!("{}", text),
        Err(e) => eprintln!("No text found: {}", e),
    }

    match lt.get_utf8_text() {
        Ok(text) => regex::extract_information(&text),
        Err(e) => eprintln!("No text found: {}", e),
    }

}
