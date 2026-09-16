mod entrada_usuario;
mod escalonador;
mod evento;
mod fila;
mod gerador_numerico;

use entrada_usuario::obter_dados_iniciais;
use escalonador::Escalonador;
use evento::{Evento, TipoEvento};
use fila::Fila;
use gerador_numerico::GeradorNumerico;

fn main() {
    let (dados, arquivo) = obter_dados_iniciais();
    let mut gerador = match arquivo {
        Some(arq) => GeradorNumerico::de_arquivo(arq),
        None => GeradorNumerico::new(),
    };

    if dados.quantidade_numeros == 0 {
        return;
    }

    let mut escalonador = Escalonador::new();
    let mut filas: Vec<Fila> = dados
        .filas
        .iter()
        .enumerate()
        .map(|(idx, config)| {
            Fila::new(
                idx + 1,
                config.servidores,
                config.capacidade,
                if idx == 0 { dados.min_chegada } else { 0.0 },
                if idx == 0 { dados.max_chegada } else { 0.0 },
                config.min_atendimento,
                config.max_atendimento,
                config.destinos.clone(),
            )
        })
        .collect();

    let limite = dados.quantidade_numeros;
    let mut tempo_anterior = 0.0;

    escalonador.adicionar(Evento::new(
        TipoEvento::Chegada,
        dados.primeira_chegada,
        -1,
        0,
    ));

    while let Some(evento) = escalonador.remover() {
        if gerador.contador() == limite {
            break;
        }

        let tempo_atual = evento.tempo();
        let tempo_decorrido = tempo_atual - tempo_anterior;

        for fila in &mut filas {
            fila.acumular_tempo(tempo_decorrido);
        }
        tempo_anterior = tempo_atual;

        match evento.tipo() {
            TipoEvento::Chegada => {
                let fila_nova = evento.fila_nova() as usize;

                if filas[fila_nova].entrada() {
                    if filas[fila_nova].tem_servidor_disponivel() {
                        filas[fila_nova].ocupar_servidor();
                        if let Some(prox_evento) = determinar_proximo_evento(
                            fila_nova,
                            &filas,
                            &mut gerador,
                            limite,
                            tempo_atual,
                        ) {
                            escalonador.adicionar(prox_evento);
                        } else {
                            break;
                        }
                    }
                }

                if fila_nova == 0 {
                    if let Some(intervalo) =
                        amostra(&mut gerador, dados.min_chegada, dados.max_chegada, limite)
                    {
                        escalonador.adicionar(Evento::new(
                            TipoEvento::Chegada,
                            tempo_atual + intervalo,
                            -1,
                            0,
                        ));
                    } else {
                        break;
                    }
                }
            }

            TipoEvento::Passagem => {
                let fila_anterior = evento.fila_anterior() as usize;
                let fila_nova = evento.fila_nova() as usize;

                filas[fila_anterior].saida();
                filas[fila_anterior].liberar_servidor();

                let entrou_nova = filas[fila_nova].entrada();

                if filas[fila_anterior].tem_cliente_na_espera() {
                    filas[fila_anterior].ocupar_servidor();
                    if let Some(prox_evento) = determinar_proximo_evento(
                        fila_anterior,
                        &filas,
                        &mut gerador,
                        limite,
                        tempo_atual,
                    ) {
                        escalonador.adicionar(prox_evento);
                    } else {
                        break;
                    }
                }

                if entrou_nova && filas[fila_nova].tem_servidor_disponivel() {
                    filas[fila_nova].ocupar_servidor();
                    if let Some(prox_evento) = determinar_proximo_evento(
                        fila_nova,
                        &filas,
                        &mut gerador,
                        limite,
                        tempo_atual,
                    ) {
                        escalonador.adicionar(prox_evento);
                    } else {
                        break;
                    }
                }
            }

            TipoEvento::Saida => {
                let fila_anterior = evento.fila_anterior() as usize;

                filas[fila_anterior].saida();
                filas[fila_anterior].liberar_servidor();

                if filas[fila_anterior].tem_cliente_na_espera() {
                    filas[fila_anterior].ocupar_servidor();
                    if let Some(prox_evento) = determinar_proximo_evento(
                        fila_anterior,
                        &filas,
                        &mut gerador,
                        limite,
                        tempo_atual,
                    ) {
                        escalonador.adicionar(prox_evento);
                    } else {
                        break;
                    }
                }
            }
        }
    }

    let tempo_total = tempo_anterior;

    for fila in filas.iter() {
        let nome_fila = format!("Q{}", fila.id());
        let cap_str = if fila.capacidade() < 0 {
            "inf".to_string()
        } else {
            fila.capacidade().to_string()
        };

        println!("\n=================================================");
        println!(
            "Fila:    {} (G/G/{}/{})",
            nome_fila,
            fila.servidores(),
            cap_str
        );
        if fila.min_chegada() > 0.0 || fila.max_chegada() > 0.0 {
            println!(
                "Chegada: {:.2} ... {:.2}",
                fila.min_chegada(),
                fila.max_chegada()
            );
        }
        println!(
            "Serviço: {:.2} ... {:.2}",
            fila.min_atendimento(),
            fila.max_atendimento()
        );
        println!("-------------------------------------------------");
        println!("Estado\t\tTempo\t\tProbabilidade");

        for (estado, &tempo) in fila.tempos_estados().iter().enumerate() {
            if tempo > 0.0 {
                let probabilidade = if tempo_total > 0.0 {
                    tempo / tempo_total * 100.0
                } else {
                    0.0
                };
                println!("{}\t\t{:.4}\t\t{:.2}%", estado, tempo, probabilidade);
            }
        }

        println!("-------------------------------------------------");
        println!("Número de perdas: {}", fila.perdas());
    }

    println!("\n=================================================");
    println!("Tempo total da simulação: {:.4}", tempo_total);
}

fn amostra(gerador: &mut GeradorNumerico, min: f64, max: f64, limite: usize) -> Option<f64> {
    if gerador.contador() == limite {
        return None;
    }
    let numero = gerador.proximo_numero();
    Some(min + (max - min) * numero)
}

fn determinar_proximo_evento(
    fila_origem: usize,
    filas: &[Fila],
    gerador: &mut GeradorNumerico,
    limite: usize,
    tempo_atual: f64,
) -> Option<Evento> {
    let fila = &filas[fila_origem];
    let tempo_servico = amostra(gerador, fila.min_atendimento(), fila.max_atendimento(), limite)?;
    let tempo_evento = tempo_atual + tempo_servico;

    let destinos = fila.destinos();
    if destinos.is_empty() {
        return Some(Evento::new(
            TipoEvento::Saida,
            tempo_evento,
            fila_origem as i32,
            -1,
        ));
    }

    let destino_escolhido = if destinos.len() == 1 {
        destinos[0].0
    } else {
        if gerador.contador() == limite {
            return None;
        }
        let aleatorio = gerador.proximo_numero();
        let mut acumulado = 0.0;
        let mut dest = destinos.last().unwrap().0;
        for &(d, prob) in destinos {
            acumulado += prob;
            if aleatorio <= acumulado {
                dest = d;
                break;
            }
        }
        dest
    };

    if destino_escolhido < 0 {
        Some(Evento::new(
            TipoEvento::Saida,
            tempo_evento,
            fila_origem as i32,
            -1,
        ))
    } else {
        Some(Evento::new(
            TipoEvento::Passagem,
            tempo_evento,
            fila_origem as i32,
            destino_escolhido,
        ))
    }
}
