use std::{fs::File, io::{BufRead, BufReader}};

use rand::{Rng, rngs::ThreadRng};


pub struct NumberHandler {
    count: i128,
    rand: Option<ThreadRng>,
    reader: Option<BufReader<File>>
}

impl NumberHandler {
    pub fn new() -> Self {
        NumberHandler {
            count: 0,
            rand: Some(rand::rng()),
            reader: None
        }
    }

    pub const fn from_file(file: BufReader<File>) -> Self {
        NumberHandler {
            count: 0,
            rand: None,
            reader: Some(file)
        }
    }

    pub fn next_number(&mut self) -> f64 {
        self.count += 1;
        match &mut self.rand {
            Some(rand) => rand.next_u32() as f64 / u32::MAX as f64,
            None => {
                match &mut self.reader {
                    Some(reader) => {
                        let mut line= String::new();
                        reader.read_line(&mut line);
                        line.parse::<f64>().expect("Line doesn't contain a number")
                    },
                    None => panic!("Number Handler doesn't have a valid method to generate a number")
                }
            }
        }
    }
}