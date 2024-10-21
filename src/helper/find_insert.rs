// import vector
use std::vec::Vec;
use regex::Regex;


pub fn find_insert(body:String)-> Vec<String>{
  let rule = Regex::new("\\{\\{inject_(.+)\\}\\}");

  let mut params: Vec<String> = Vec::new();
  for cap in rule.unwrap().captures_iter(&body) {
    params.push(cap[1].to_string());
  }

  params
}