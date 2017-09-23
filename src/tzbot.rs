#![crate_name = "tzbot"]
#![crate_type = "lib"]

extern crate hourglass;
extern crate regex;
extern crate time;


use hourglass::Timezone;
use regex::Regex;
use std::collections::HashMap;
use std::option::Option;
use std::vec::Vec;

fn format_output(tz: String, hours: String, minutes: String, ampm: String, lookup: HashMap<String, String>, common: Vec<String>) -> String {
    let mut h_i = hours.parse::<i32>().unwrap();
    let m_i = minutes.parse::<i32>().unwrap();

    if &ampm == "PM" {
        h_i = h_i + 12;
    }

    let tz_full = lookup.get(&tz).unwrap();
    let local = Timezone::new(tz_full).unwrap();

    let now = time::now();
    let t_input = local.datetime(1900 + now.tm_year, 1 + now.tm_mon, now.tm_mday, h_i, m_i, 0, 0).unwrap();

    let mut result = format!("{}:{}{} {} is ", hours, minutes, ampm, tz);
    for current in common {
        if current == tz {
            continue;
        }
        let cur_val = lookup.get(&current).unwrap();
        let cur_tz = Timezone::new(&cur_val).unwrap();
        let cur_t = t_input.project(&cur_tz);
        println!("DBG:  {}: {:?}", current, cur_t);
        let hm = cur_t.format("%H:%M").unwrap();
        result = format!("{}{} {} / ", result, hm, current);

    }
    result.pop();
    result.pop();
    result.pop();

    result
}

pub fn convert(body_in: &str) -> Option<String> {
    let body = body_in.to_uppercase();

    let mut slookup = HashMap::new();

    slookup.insert("EST",        "America/New_York");
    slookup.insert("EASTERN",    "America/New_York");
    slookup.insert("CST",        "America/Chicago");
    slookup.insert("CENTRAL",    "America/Chicago");
    slookup.insert("MST",        "America/Denver");
    slookup.insert("MOUNTAIN",   "America/Denver");
    slookup.insert("PST",        "America/Los_Angeles");
    slookup.insert("PACIFIC",    "America/Los_Angeles");
    slookup.insert("CET",        "Europe/Berlin");
    slookup.insert("MSK",        "Europe/Moscow");
    slookup.insert("GMT",        "UTC");
    slookup.insert("UTC",        "UTC");

    // DST
    slookup.insert("EDT",        "America/New_York");
    slookup.insert("CDT",        "America/Chicago");
    slookup.insert("MDT",        "America/Denver");
    slookup.insert("PDT",        "America/Los_Angeles");
    slookup.insert("CEST",       "Europe/Berlin");

    let mut lookup = HashMap::new();
    for (key, val) in slookup.iter() {
        lookup.insert(key.to_string(), val.to_string());
    }

    let mut shorts = lookup.keys().fold(String::new(), |acc, k| { acc + k + "|" });
    shorts.pop();
    let common = vec!["PST".to_string(), "CST".to_string(), "EST".to_string(), "CET".to_string()];

    // (\d+):(\d+) (EST|CET|...)
    let re_fmt_base = format!("{} ({})", r"^(.*?)(\d+):(\d+)", shorts.to_uppercase());
    // (\d+):(\d+)(AM|PM| AM| PM) (EST|CET|...)
    let re_fmt_ampm = format!("{} ({})", r"^(.*?)(\d+):(\d+)(AM|PM| AM| PM)", shorts.to_uppercase());
    // (\d{1,3})(AM|PM| AM| PM) (EST|CET|...)
    let re_fmt_ampm_4d = format!("{} ({})", r"^(.*?)(\d{1,4})(AM|PM| AM| PM)", shorts.to_uppercase());

    let re_base = Regex::new(&re_fmt_base).unwrap();
    let re_ampm = Regex::new(&re_fmt_ampm).unwrap();
    let re_ampm_4d = Regex::new(&re_fmt_ampm_4d).unwrap();

    let result;

    let tz;
    let mut ampm = String::new();
    let hours;
    let minutes;

    if re_base.is_match(&body) {
        println!("DBG:  match:base");
        let parts = re_base.captures(&body).unwrap();
        hours = parts.get(2).unwrap().as_str();
        minutes = parts.get(3).unwrap().as_str();
        tz = parts.get(4).unwrap().as_str().to_string();
        //println!("[{}]:[{}] [{}]", hours, minutes, tz);
    } else if re_ampm.is_match(&body) {
        println!("DBG:  match:ampm");
        let parts = re_ampm.captures(&body).unwrap();
        hours = parts.get(2).unwrap().as_str();
        minutes = parts.get(3).unwrap().as_str();
        ampm = parts.get(4).unwrap().as_str().trim().to_string();
        tz = parts.get(5).unwrap().as_str().to_string();
        //println!("[{}]:[{}] {} [{}]", hours, minutes, ampm, tz);
    } else if re_ampm_4d.is_match(&body) {
        println!("DBG:  match:ampm_4d");
        let parts = re_ampm_4d.captures(&body).unwrap();

        let undecided = parts.get(2).unwrap().as_str().trim();
        if undecided.len() < 3 {
            hours = undecided;
            minutes = "00";
        } else if undecided.len() == 3 {
            hours = &undecided[0..1];
            minutes = &undecided[1..3];
        } else {
            hours = &undecided[0..2];
            minutes = &undecided[2..4];
        }

        ampm = parts.get(3).unwrap().as_str().trim().to_string();
        tz = parts.get(4).unwrap().as_str().to_string();
        //println!("[{}]:[{}] {} [{}] {}", hours, minutes, ampm, tz, undecided.len());
    } else {
        return None;
    }
    if lookup.contains_key(&tz) {
        result = format_output(tz, hours.to_string(), minutes.to_string(), ampm, lookup, common);
    } else {
        println!("TODO: Timezone not implemented: {}", tz);
        result = format!("Unknown timezone: {}", tz);
    }

    Some(result)
}

