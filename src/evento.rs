#[derive(Clone, Copy)]
pub enum TipoEvento {
    Arrival,
    Departure,
    Passagem
}

#[derive(Clone, Copy)]
pub struct Evento {
    tipo_evento: TipoEvento,
    tempo: f64,
}

impl Evento {
    pub fn new(tipo_evento: TipoEvento, tempo: f64) -> Self {
        Self { tipo_evento, tempo }
    }

    pub const fn tempo(&self) -> f64 {
        self.tempo
    }

    pub const fn tipo(&self) -> TipoEvento {
        self.tipo_evento
    }
}
