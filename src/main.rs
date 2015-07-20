use std::io;
use std::io::prelude::*; // need this for the lines()
use std::collections::HashMap;

extern crate time;

fn main() {
    let reader = io::stdin();
    let lines: Vec<_> = reader.lock().lines().collect();
    let combos = create_strings();

    let mut map: HashMap<String, usize> = create_hash(&combos);

    //map.insert("aaaaa", 0);
    //*map.entry("aaaaa").or_insert(0) += 1;
    //*map.entry("aaaaa").or_insert(0) += 1;
    //println!("{:?}", map.get("aaaaa"));

    //println!("before: {:?}", time::get_time());
    //for line in lines {
        //let mut l = line.ok().unwrap();
        //for (key, value) in map.iter_mut() {
            //if problem(&l.trim(), key) {
                //*value += 1
            //}
        //}
    //}
    //println!("after: {:?}", time::get_time());
    println!("{:?}", map);
}

fn problem(word: &str, offensive: &str) -> bool {
    word.chars()
        .filter(|&ch| offensive.contains(ch) )
        .collect::<String>() == offensive
}

fn create_hash(sv: &Vec<String>) -> HashMap<String, usize> {
    let mut map: HashMap<String, usize> = HashMap::new();

    // TODO: complaining that borrowed s can't be moved, which makes sense, but I don't know how to
    // solve this issue. Do I need a Box? Reference counter? What?
    for s in sv.iter() {
        map.insert(*s, 0);
    }
    map
}

fn create_strings() -> Vec<String> {
    let alphabet = "abcdefghijklmnopqrstuvwxyz".to_string();
    let length = alphabet.len();
    let mut v: Vec<String> = Vec::new();

    // TODO: There has got to be a better way to do this. I just don't have a clue.
    for i in (0..length) {
        for ii in (0..length) {
            for iii in (0..length) {
                for iiii in (0..length) {
                    for iiiii in (0..length) {
                        let mut s: String = String::with_capacity(5);
                        s.push(alphabet.as_bytes()[i] as char);
                        s.push(alphabet.as_bytes()[ii] as char);
                        s.push(alphabet.as_bytes()[iii] as char);
                        s.push(alphabet.as_bytes()[iiii] as char);
                        s.push(alphabet.as_bytes()[iiiii] as char);
                        v.push(s);
                    }
                }
            }
        }
    }
    v
}

#[test]
fn test_problem() {
    assert!(!problem("misconduct", "snond"));
    assert!(!problem("mispronounced", "snond"));
    assert!(problem("synchronized", "snond"));
    assert!(problem("misfunctioned", "snond"));
    assert!(!problem("shotgunned", "snond"));
    assert!(problem("snond", "snond"));
}
