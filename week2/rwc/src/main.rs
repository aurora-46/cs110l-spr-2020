use std::env;
use std::process;
use std::fs::File; 
use std::io::{self, BufRead}; // For read_file_lines()

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    let file= File::open(filename).expect("can't open file {filename}");
    
    let mut line_count=0;
    let mut word_count=0;
    let mut char_count=0;

    for line in io::BufReader::new(file).lines() {
        let line=line.unwrap();
        
        line_count+=1;
        char_count+=1+line.len();

        let words:Vec<&str>=line.trim().split(' ').filter(|x|x.len()>0).collect();
        word_count+=words.len();
        
    }

    println!("{line_count} {word_count} {char_count}");
}
