use std::collections::VecDeque;

use crate::fila::eventos::Evento;

#[path ="./eventos.rs"]
mod eventos;

pub struct Fila {
    elements: VecDeque<Evento>,
}

impl Fila {
    pub fn new(chegada: Evento) -> Self {
        let mut elements = VecDeque::new();
        elements.push_back(chegada);
        Fila {
            elements: elements,
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn add(&mut self, event: Evento) {
        let mut start = 0_usize;
        let mut end = self.elements.len() - 1;
        while start < end {
            let mid = start + (end - start) / 2;

            let compare_event = self.elements.get(mid).expect("Algorithm shouldn't be able to have a invalid position");
            if compare_event.get_fim() <= event.get_fim() {
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }
        self.elements.insert(start, event);
    }

    pub fn remove_first(&mut self) -> Option<Evento> {
        self.elements.pop_front()
    }
}