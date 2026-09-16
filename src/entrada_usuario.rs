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
    pub destinos: Vec<(i32, f64)>, // -1 saída
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
            destinos: Vec::new(),
        });
    }

    if num_filas == 1 {
        filas[0].destinos = vec![(-1, 1.0)];
    } else {
        println!("\n--- Roteamento entre Filas ---");
        println!("1: Em série");
        println!("2: Personalizado");
        let opcao = read_line_default("Escolha o tipo de roteamento [1]: ", "1");

        if opcao == "2" {
            for i in 0..num_filas {
                println!("\nDestinos para a Fila Q{}:", i + 1);
                let quant_destinos = read_usize("  Quantidade de destinos possíveis: ");
                let mut destinos = Vec::with_capacity(quant_destinos);
                for d in 0..quant_destinos {
                    let dest_id = read_i32(&format!(
                        "    Destino {} (número da fila de 1 a {}, ou -1 para Saída): ",
                        d + 1,
                        num_filas
                    ));
                    let dest_index = if dest_id > 0 { dest_id - 1 } else { -1 };
                    let prob = read_f64("    Probabilidade: ");
                    destinos.push((dest_index, prob));
                }
                filas[i].destinos = destinos;
            }
        } else {
            for i in 0..num_filas {
                if i + 1 < num_filas {
                    filas[i].destinos = vec![((i + 1) as i32, 1.0)];
                } else {
                    filas[i].destinos = vec![(-1, 1.0)];
                }
            }
        }
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

fn read_line_default(text: &str, default: &str) -> String {
    let line = read_line(text);
    if line.is_empty() {
        default.to_string()
    } else {
        line
    }
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
        .expect("Por favor, informe um número decimal válido.")
}
