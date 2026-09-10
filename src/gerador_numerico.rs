use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const A: usize = 5673869;
const C: usize = 543;
const M: usize = 2_usize.pow(26);
const SEED: usize = 42;

pub struct GeradorNumerico {
    contador: usize,
    leitor: Option<BufReader<File>>,
    anterior: usize,
}

impl GeradorNumerico {
    pub fn new() -> Self {
        GeradorNumerico {
            contador: 0,
            anterior: SEED,
            leitor: None,
        }
    }

    pub const fn from_file(file: BufReader<File>) -> Self {
        GeradorNumerico {
            contador: 0,
            leitor: Some(file),
            anterior: SEED,
        }
    }

    pub fn next_number(&mut self) -> f64 {
        self.contador += 1;
        match &mut self.leitor {
            Some(leitor) => {
                let mut linha = String::new();
                leitor
                    .read_line(&mut linha)
                    .expect("erro ao ler um número do arquivo");
                linha.trim()
                    .parse::<f64>()
                    .expect("a linha não contém um número")
            }
            None => {
                let aux = (A * self.anterior + C) % M;
                self.anterior = aux;
                aux as f64 / M as f64
            }
        }
    }

    pub fn get_contador(&self) -> usize {
        self.contador
    }
}
