use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader, Write},
    path::Path,
};

#[derive(Clone)]
pub struct ConfiguracaoFila {
    pub servidores: i32,
    pub capacidade: i32,
    pub min_chegada: f64,
    pub max_chegada: f64,
    pub min_atendimento: f64,
    pub max_atendimento: f64,
    pub destinos: Vec<(i32, f64)>, // -1 saída
}

#[derive(Clone)]
pub struct DadosIniciais {
    pub quantidade_numeros: usize,
    pub primeira_chegada: f64,
    pub filas: Vec<ConfiguracaoFila>,
}

struct FilaDef {
    nome: String,
    servidores: i32,
    capacidade: i32,
    min_chegada: f64,
    max_chegada: f64,
    min_atendimento: f64,
    max_atendimento: f64,
    saidas: Vec<(String, f64)>,
}

fn split_chave_valor(s: &str) -> (&str, &str) {
    if let Some(pos) = s.find(':') {
        let chave = s[..pos].trim();
        let valor = s[pos + 1..].trim();
        (chave, valor)
    } else {
        (s.trim(), "")
    }
}

fn remover_comentario(s: &str) -> &str {
    if let Some(pos) = s.find('#') {
        &s[..pos]
    } else {
        s
    }
}

fn limpar_valor(val: &str) -> &str {
    val.trim().trim_matches(|c| c == '\'' || c == '"')
}

fn resolver_destino(nome: &str, mapa: &HashMap<String, usize>) -> i32 {
    let trimmed = nome.trim();
    let lower = trimmed.to_lowercase();
    if lower == "saida" || lower == "saída" || lower == "exit" || lower == "-1" {
        return -1;
    }
    if let Some(&idx) = mapa.get(trimmed) {
        return idx as i32;
    }
    if let Some(&idx) = mapa.get(&lower) {
        return idx as i32;
    }
    if lower.starts_with('q') {
        if let Ok(num) = lower[1..].parse::<usize>() {
            if num >= 1 {
                return (num - 1) as i32;
            }
        }
    }
    if let Ok(num) = lower.parse::<usize>() {
        if num >= 1 {
            return (num - 1) as i32;
        }
    }
    -1
}

pub fn carregar_de_yaml(caminho: &str) -> (DadosIniciais, Option<BufReader<File>>) {
    let path = Path::new(caminho);
    if !path.is_file() {
        eprintln!("Arquivo não encontrado: {}", caminho);
        std::process::exit(1);
    }

    let conteudo = std::fs::read_to_string(path).unwrap_or_else(|err| {
        eprintln!("Erro ao ler o arquivo '{}': {}", caminho, err);
        std::process::exit(1);
    });

    let mut primeira_chegada = 0.0;
    let mut quantidade_numeros = 100_000;
    let mut arquivo_numeros: Option<String> = None;
    let mut filas_defs: Vec<FilaDef> = Vec::new();

    let mut em_filas = false;
    let mut em_saidas = false;
    let mut saidas_indent = 0;

    for linha in conteudo.lines() {
        let sem_comentario = remover_comentario(linha);
        if sem_comentario.trim().is_empty() {
            continue;
        }

        let indent = sem_comentario.len() - sem_comentario.trim_start().len();
        let limpo = sem_comentario.trim();

        if indent == 0 {
            em_filas = false;
            em_saidas = false;
            let (chave, valor) = split_chave_valor(limpo);
            match chave {
                "filas" => em_filas = true,
                "chegada_inicial" | "chegada" | "primeira_chegada" => {
                    let v = limpar_valor(valor);
                    if let Ok(val) = v.parse::<f64>() {
                        primeira_chegada = val;
                    }
                }
                "quantidade_numeros" => {
                    let v = limpar_valor(valor);
                    if let Ok(val) = v.parse::<usize>() {
                        quantidade_numeros = val;
                    }
                }
                "arquivo_numeros" => {
                    let v = limpar_valor(valor);
                    if !v.is_empty() {
                        arquivo_numeros = Some(v.to_string());
                    }
                }
                _ => {}
            }
            continue;
        }

        if em_filas {
            if em_saidas {
                if indent > saidas_indent {
                    let (dest, prob_str) = split_chave_valor(limpo);
                    let prob: f64 = limpar_valor(prob_str).parse().unwrap_or(0.0);
                    if let Some(fila_atual) = filas_defs.last_mut() {
                        fila_atual.saidas.push((dest.to_string(), prob));
                    }
                    continue;
                } else {
                    em_saidas = false;
                }
            }

            let (chave, valor) = split_chave_valor(limpo);
            if valor.is_empty() && chave == "saidas" {
                em_saidas = true;
                saidas_indent = indent;
            } else if valor.is_empty() {
                // Nova fila (ex: "Q1:", "Q2:")
                filas_defs.push(FilaDef {
                    nome: chave.to_string(),
                    servidores: 1,
                    capacidade: -1,
                    min_chegada: 0.0,
                    max_chegada: 0.0,
                    min_atendimento: 0.0,
                    max_atendimento: 0.0,
                    saidas: Vec::new(),
                });
            } else if let Some(fila_atual) = filas_defs.last_mut() {
                let v = limpar_valor(valor);
                match chave {
                    "servidores" => fila_atual.servidores = v.parse().unwrap_or(1),
                    "capacidade" => {
                        fila_atual.capacidade = if v.is_empty() || v == "inf" || v == "-1" {
                            -1
                        } else {
                            v.parse().unwrap_or(-1)
                        }
                    }
                    "min_chegada" => fila_atual.min_chegada = v.parse().unwrap_or(0.0),
                    "max_chegada" => fila_atual.max_chegada = v.parse().unwrap_or(0.0),
                    "min_atendimento" => fila_atual.min_atendimento = v.parse().unwrap_or(0.0),
                    "max_atendimento" => fila_atual.max_atendimento = v.parse().unwrap_or(0.0),
                    _ => {}
                }
            }
        }
    }

    let mut nome_para_indice = HashMap::new();
    for (idx, f) in filas_defs.iter().enumerate() {
        nome_para_indice.insert(f.nome.clone(), idx);
        nome_para_indice.insert(f.nome.to_lowercase(), idx);
    }

    let mut filas = Vec::with_capacity(filas_defs.len());
    for f in filas_defs {
        let mut destinos = Vec::new();
        if f.saidas.is_empty() {
            destinos.push((-1, 1.0));
        } else {
            let mut soma_prob = 0.0;
            for (alvo, prob) in f.saidas {
                let idx_alvo = resolver_destino(&alvo, &nome_para_indice);
                destinos.push((idx_alvo, prob));
                soma_prob += prob;
            }
            if soma_prob < 1.0 - 1e-6 {
                destinos.push((-1, ((1.0 - soma_prob) * 10000.0).round() / 10000.0));
            }
        }

        filas.push(ConfiguracaoFila {
            servidores: f.servidores,
            capacidade: f.capacidade,
            min_chegada: f.min_chegada,
            max_chegada: f.max_chegada,
            min_atendimento: f.min_atendimento,
            max_atendimento: f.max_atendimento,
            destinos,
        });
    }

    let leitor_arquivo = if let Some(ref nome_arq) = arquivo_numeros {
        let caminho_direto = Path::new(nome_arq);
        let caminho_final = if caminho_direto.is_file() {
            caminho_direto.to_path_buf()
        } else if let Some(pai) = path.parent() {
            let relativo = pai.join(nome_arq);
            if relativo.is_file() {
                relativo
            } else {
                eprintln!("Arquivo de números não encontrado: {}", nome_arq);
                std::process::exit(1);
            }
        } else {
            eprintln!("Arquivo de números não encontrado: {}", nome_arq);
            std::process::exit(1);
        };

        let f = File::open(&caminho_final).unwrap_or_else(|err| {
            eprintln!(
                "Erro ao abrir arquivo de números '{}': {}",
                caminho_final.display(),
                err
            );
            std::process::exit(1);
        });
        Some(BufReader::new(f))
    } else {
        None
    };

    (
        DadosIniciais {
            quantidade_numeros,
            primeira_chegada,
            filas,
        },
        leitor_arquivo,
    )
}

pub fn obter_dados_iniciais() -> (DadosIniciais, Option<BufReader<File>>) {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let caminho = &args[1];
        let path = Path::new(caminho);
        if !path.is_file() {
            eprintln!("Arquivo não encontrado: {}", caminho);
            std::process::exit(1);
        }
        return carregar_de_yaml(caminho);
    }

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
            min_chegada: if i == 1 { min_chegada } else { 0.0 },
            max_chegada: if i == 1 { max_chegada } else { 0.0 },
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
