use leptess::LepTess;
use std::env;
use std::path::Path;
use regex::Regex;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <image_path>", args[0]);
        std::process::exit(1);
    }

    let image_path = &args[1];
    let image_path = Path::new(image_path);

    if !image_path.exists() {
        eprintln!("Le fichier spécifié n'existe pas : {:?}", image_path);
        std::process::exit(1);
    }

    let mut lt = match LepTess::new(None, "eng") {
        Ok(lt) => lt,
        Err(e) => {
            eprintln!("Erreur lors de l'initialisation de Tesseract: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = lt.set_image(image_path) {
        eprintln!("Erreur lors de la définition de l'image pour Tesseract: {}", e);
        std::process::exit(1);
    }

    match lt.get_utf8_text() {
        Ok(text) => println!("{}", text),
        Err(e) => eprintln!("Erreur lors de la reconnaissance du texte: {}", e),
    }

    match lt.get_utf8_text() {
        Ok(text) => extract_information(&text),
        Err(e) => eprintln!("Erreur lors de la reconnaissance des téléphones et emails : {}", e),
    }

}

fn extract_information(text: &str) {

    let email_regex = Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap();
    println!("\nEmails trouvés:");
    for cap in email_regex.captures_iter(text) {
        println!("{}", &cap[0]);
    }

    // TODO: Enhance phone numbers matching regex
    let phone_regex = Regex::new(r"\b\d{10}\b").unwrap();
    println!("\nNuméros de téléphone trouvés:");
    for cap in phone_regex.captures_iter(text) {
        println!("{}", &cap[0]);
    }

    println!("")
}
