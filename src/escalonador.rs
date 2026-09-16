use std::collections::VecDeque;

use crate::evento::Evento;

pub struct Escalonador {
    eventos: VecDeque<Evento>,
}

impl Escalonador {
    pub fn new() -> Self {
        Escalonador {
            eventos: VecDeque::new(),
        }
    }

    pub fn adicionar(&mut self, evento: Evento) {
        let mut posicao = 0;
        while posicao < self.eventos.len() && self.eventos[posicao].tempo() <= evento.tempo() {
            posicao += 1;
        }
        self.eventos.insert(posicao, evento);
    }

    pub fn remover(&mut self) -> Option<Evento> {
        self.eventos.pop_front()
    }
}

