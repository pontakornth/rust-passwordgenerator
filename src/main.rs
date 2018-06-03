extern crate rand;
use std::io;
use std::io::prelude::*;
use rand::prelude::*;
fn main() {
    println!("Hello, world!");
    let stdin = io::stdin();
    println!("Enter password length");
    let mut input = stdin.lock().lines();
    let length = input.next().unwrap().unwrap().parse::<i32>().unwrap();
    println!("Enter numbers of password");
    let number = input.next().unwrap().unwrap().parse::<i32>().unwrap();
    let p = generate_password(length,number);
    for i in p.iter() {
        println!("{}",i);
    }
}
fn generate_password(length : i32,number: i32) -> Vec<String> {
    let lowercase = "abcdefghijklmnopqrstuvwxyz";
    let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let listnumber = "0123456789";
    let other = "!\\\"#$%&'()*+,-./:;<=>?@[]^_{|}~";
    let all : Vec<char> = String::from(format!("{}{}{}{}",lowercase,uppercase,listnumber,other)).chars().collect();
    let mut password_list: Vec<String> = Vec::new();
    {
    for _ in 0..number {
        let mut password: String = String::from("");
        for _ in 0..length {
            password.push(*(thread_rng().choose(&all).unwrap()));
        }
        password_list.push(password);
    }}
    return password_list;
}