use regex::Regex;

pub fn extract_information(text: &str) {

    let email_regex = Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap();
	let email_match: Vec<_> = email_regex.captures_iter(text).collect(); 

	if !email_match.is_empty() {
		println!("\nFound e-mails:");
		for cap in email_match {
			println!("{}", &cap[0]);
		}
	}

    // TODO: Enhance phone numbers matching regex
    let phone_regex = Regex::new(r"\b\d{10}\b").unwrap();
	let phone_match: Vec<_> = phone_regex.captures_iter(text).collect(); 

	if !phone_match.is_empty() {
		println!("\nPhone Numbers:");
		for cap in phone_match {
			println!("{}", &cap[0]);
		}
	}

    println!("")
}
