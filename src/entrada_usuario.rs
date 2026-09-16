use std::{
    fs::File,
    io::{self, BufReader, Write},
};

#[derive(Clone)]
pub struct ConfiguracaoFila {
    pub servidores: i32,
    pub capacidade: i32,
    pub min_atendimento: f64,
    pub max_atendimento: f64,
}

#[derive(Clone)]
pub struct DadosIniciais {
    pub quantidade_numeros: usize,
    pub primeira_chegada: f64,
    pub min_chegada: f64,
    pub max_chegada: f64,
    pub filas: Vec<ConfiguracaoFila>,
}

pub fn obter_dados_iniciais() -> (DadosIniciais, Option<BufReader<File>>) {
    let quantidade_numeros = read_usize("Quantidade de números aleatórios: ");
    let primeira_chegada = read_f64("Tempo da primeira chegada: ");
    let min_chegada = read_f64("Tempo mínimo entre chegadas externas: ");
    let max_chegada = read_f64("Tempo máximo entre chegadas externas: ");

    let num_filas = read_usize("Quantidade de filas: ");
    let num_filas = if num_filas == 0 { 1 } else { num_filas };

    let mut filas = Vec::with_capacity(num_filas);

    for i in 1..=num_filas {
        println!("\n--- Configuração da Fila Q{} ---", i);
        let servidores = read_i32(&format!("Quantidade de servidores da Fila Q{}: ", i));
        let capacidade = read_i32(&format!("Capacidade da Fila Q{} (-1 para ilimitada): ", i));
        let min_atendimento = read_f64(&format!("Tempo mínimo de atendimento da Fila Q{}: ", i));
        let max_atendimento = read_f64(&format!("Tempo máximo de atendimento da Fila Q{}: ", i));

        filas.push(ConfiguracaoFila {
            servidores,
            capacidade,
            min_atendimento,
            max_atendimento,
        });
    }

    print!("\nArquivo de números aleatórios (Enter para gerar): ");
    io::stdout().flush().unwrap();
    let mut path = String::new();
    io::stdin().read_line(&mut path).unwrap();

    let file = if path.trim().is_empty() {
        None
    } else {
        Some(BufReader::new(File::open(path.trim()).expect("Erro ao abrir arquivo de números aleatórios")))
    };

    (
        DadosIniciais {
            quantidade_numeros,
            primeira_chegada,
            min_chegada,
            max_chegada,
            filas,
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
    read_line(text)
        .parse()
        .expect("Por favor, informe um número inteiro positivo válido.")
}

fn read_i32(text: &str) -> i32 {
    read_line(text)
        .parse()
        .expect("Por favor, informe um número inteiro válido.")
}

fn read_f64(text: &str) -> f64 {
    read_line(text)
        .parse()
        .expect("Por favor, informe um número decimal (ponto flutuante) válido.")
}

