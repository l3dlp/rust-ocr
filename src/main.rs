use leptess::LepTess;
use std::env;
use std::path::Path;

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
}
