use std::vec::Vec;
use regex::Regex;

pub fn replace_in_body(body: String, replacements: Vec<(String,String)>) -> String {
    let mut result = body.to_string();
    for (pattern, replacement) in replacements {
        let rule = format!("\\{{\\{{{}\\}}\\}}",pattern);
        let re = Regex::new(&rule).unwrap();
        result = re.replace_all(&result, replacement).to_string();
    }
    result
}
