use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

use std::time::Duration;
use std::thread::sleep;
use std::fs::File;

fn main() {
    println!("Hey Patty, can you fullscreen this window for me?\nHit Enter when you've done that.");
    let mut key = String::new();
    io::stdin().read_line(&mut key).expect("Readline failed.");
    println!("Thanks! Here we go!");
    sleep(Duration::from_millis(1000));

    //Read picture file and print symbols
    let f = File::open("binary.txt").expect("Unable to open file.");
    let reader = BufReader::new(&f);
    for line in reader.lines(){
        println!("");
        let data_str = line.unwrap();
        let data = data_str.trim().split_whitespace();
        for i in data{
            if i == "1" {
                print!("*");
            } else {
                print!(" ");
            }
            io::stdout().flush();
        }
    sleep(Duration::from_millis(100));
    }
    println!("\n\n\nI hope you're having a great day! Merry Christmas again!");
    io::stdin().read_line(&mut key).expect("Readline failed.");
}
