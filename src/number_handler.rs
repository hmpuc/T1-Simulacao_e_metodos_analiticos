use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const A: usize = 5673869;
const C: usize = 543;
const M: usize = 2_usize.pow(26);
const SEED: usize = 42;

pub struct NumberHandler {
    count: usize,
    reader: Option<BufReader<File>>,
    previous: usize,
}

impl NumberHandler {
    pub fn new() -> Self {
        NumberHandler {
            count: 0,
            previous: SEED,
            reader: None,
        }
    }

    pub const fn from_file(file: BufReader<File>) -> Self {
        NumberHandler {
            count: 0,
            reader: Some(file),
            previous: SEED,
        }
    }

    pub fn next_number(&mut self) -> f64 {
        self.count += 1;
        match &mut self.reader {
            Some(reader) => {
                let mut line = String::new();
                reader
                    .read_line(&mut line)
                    .expect("erro ao ler um número do arquivo");
                line.trim()
                    .parse::<f64>()
                    .expect("a linha não contém um número")
            }
            None => {
                let aux = (A * self.previous + C) % M;
                self.previous = aux;
                aux as f64 / M as f64
            }
        }
    }

    pub fn get_count(&self) -> usize {
        self.count
    }
}
