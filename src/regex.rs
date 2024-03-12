use regex::Regex;

pub fn extract_information(text: &str) {

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
