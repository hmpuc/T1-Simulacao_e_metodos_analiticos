use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// números do java.util.Random
const A: u64 = 25214903917;
const C: u64 = 11;
const M: u64 = 1_u64 << 48; // 2^48
const SEED: u64 = 1;

pub struct GeradorNumerico {
    contador: usize,
    leitor: Option<BufReader<File>>,
    anterior: u64,
}

impl GeradorNumerico {
    pub fn new() -> Self {
        GeradorNumerico {
            contador: 0,
            anterior: SEED,
            leitor: None,
        }
    }

    pub const fn de_arquivo(arquivo: BufReader<File>) -> Self {
        GeradorNumerico {
            contador: 0,
            leitor: Some(arquivo),
            anterior: SEED,
        }
    }

    pub fn proximo_numero(&mut self) -> Option<f64> {
        match &mut self.leitor {
            Some(leitor) => {
                loop {
                    let mut linha = String::new();
                    let bytes = leitor
                        .read_line(&mut linha)
                        .expect("erro ao ler um número do arquivo");
                    if bytes == 0 {
                        return None;
                    }
                    let trimmed = linha.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    self.contador += 1;
                    return Some(
                        trimmed
                            .parse::<f64>()
                            .expect("a linha não contém um número válido"),
                    );
                }
            }
            None => {
                self.contador += 1;
                let aux = ((A as u128 * self.anterior as u128 + C as u128) & ((M as u128) - 1)) as u64;
                self.anterior = aux;
                Some(aux as f64 / M as f64)
            }
        }
    }

    pub fn contador(&self) -> usize {
        self.contador
    }
}
