use std::{
    fs::File,
    io::{self, BufReader, Write},
};

pub fn get_initial_data() -> (InitialData, Option<BufReader<File>>) {
    let count = read_usize("Quantidade de números aleatórios: ");
    let min_arrival = read_f64("Tempo mínimo entre chegadas: ");
    let max_arrival = read_f64("Tempo máximo entre chegadas: ");
    let min_service = read_f64("Tempo mínimo de atendimento: ");
    let max_service = read_f64("Tempo máximo de atendimento: ");
    let first_arrival = read_f64("Tempo da primeira chegada: ");
    let capacity = read_i64("Capacidade da fila (-1 para ilimitada): ");
    let servers = read_i64("Quantidade de servidores: ");

    print!("Arquivo de números aleatórios (Enter para gerar): ");
    io::stdout().flush().unwrap();
    let mut path = String::new();
    io::stdin().read_line(&mut path).unwrap();

    let file = if path.trim().is_empty() {
        None
    } else {
        Some(BufReader::new(File::open(path.trim()).unwrap()))
    };

    (
        InitialData {
            count,
            min_arrival,
            max_arrival,
            min_service,
            max_service,
            first_arrival,
            capacity,
            servers,
        },
        file,
    )
}

fn read_line(text: &str) -> String {
    print!("{text}");
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn read_usize(text: &str) -> usize {
    read_line(text).parse().unwrap()
}
fn read_i64(text: &str) -> i64 {
    read_line(text).parse().unwrap()
}
fn read_f64(text: &str) -> f64 {
    read_line(text).parse().unwrap()
}

pub struct InitialData {
    pub count: usize,
    pub min_arrival: f64,
    pub max_arrival: f64,
    pub min_service: f64,
    pub max_service: f64,
    pub first_arrival: f64,
    pub capacity: i32,
    pub servers: i32,
}
