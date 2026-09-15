mod evento;
mod escalonador;
mod gerador_numerico;
mod fila;
mod user_input;


use evento::{Evento, TipoEvento};
use escalonador::Escalonador;
use gerador_numerico::GeradorNumerico;
use fila::Fila;
use user_input::get_initial_data;

fn main() {
    let (data, file) = get_initial_data();
    let mut numbers = match file {
        Some(file) => GeradorNumerico::from_file(file),
        None => GeradorNumerico::new(),
    };

    let mut event_handler = Escalonador::new();
    let mut queue = Fila::new(data.servers, data.capacity, data.min_arrival, data.max_arrival, data.min_service, data.max_service);
    let mut current_time: f64;
    let mut previous_time = 0.0;
    let mut time_per_state = vec![0.0];
    let mut losses = 0;

    if data.count == 0 {
        return;
    }

    event_handler.add(Evento::new(TipoEvento::Chegada, data.first_arrival));

    while let Some(event) = event_handler.remove() {
        if numbers.get_contador() == data.count {
            break;
        } 
        current_time = event.tempo();

        let state = queue.length();
        if state >= time_per_state.len() {
            time_per_state.resize(state + 1, 0.0);
        }
        time_per_state[state] += current_time - previous_time;
        previous_time = current_time;

        match event.event_type() {
            EventType::Arrival => {
                if queue.add() {
                    if queue.has_available_server() {
                        queue.occupy_server();
                        let Some(service_time) =
                            sample(&mut numbers, data.min_service, data.max_service, data.count)
                        else {
                            break;
                        };
                        event_handler.add(Evento::new(
                            TipoEvento::Saida,
                            current_time + service_time,
                        ));
                    }
                } else {
                    losses += 1;
                }

                if let Some(interval) = sample(&mut numbers, data.min_arrival, data.max_arrival, data.count) {
                    event_handler.schedule(Event::new(EventType::Arrival, current_time + interval));
                } else {
                    break;
                }
            }
            EventType::Departure => {
                queue.remove();
                queue.release_server();

                if queue.has_waiting_client() {
                    queue.occupy_server();
                    let Some(service_time) =
                        sample(&mut numbers, data.min_service, data.max_service, data.count)
                    else {
                        break;
                    };
                    event_handler.schedule(Event::new(
                        EventType::Departure,
                        current_time + service_time,
                    ));
                }
            }
        }
    }

    let total_time: f64 = time_per_state.iter().sum();

    println!("Fila:    Q1 (G/G/{}/{})", data.servers, data.capacity);
    println!(
        "Chegada: {:.2} ... {:.2}",
        data.min_arrival, data.max_arrival
    );
    println!(
        "Serviço: {:.2} ... {:.2}",
        data.min_service, data.max_service
    );
    println!("\n-------------------------------");
    println!("Estado\t\tTempo\t\tProbabilidade");

    for (state, time) in time_per_state.iter().enumerate() {
        if *time > 0.0 {
            let probability = time / total_time * 100.0;
            println!("{}\t\t{:.2}\t\t{:.2}%", state, time, probability);
        }
    }

    println!("\n-------------------------------");
    println!("\nNúmero de perdas: {losses}");
    println!("\nTempo médio da simulação: {:.2}", total_time);
}

fn sample(numbers: &mut NumberHandler, min: f64, max: f64, limit: usize) -> Option<f64> {
    if numbers.get_count() == limit {
        return None;
    } 
    let number = numbers.next_number();
    return Some(min + (max - min) * number);
}
