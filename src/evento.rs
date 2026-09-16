#[derive(Clone, Copy)]
pub enum TipoEvento {
    Chegada,
    Saida,
    Passagem,
}

#[derive(Clone, Copy)]
pub struct Evento {
    tipo_evento: TipoEvento,
    tempo: f64,
    fila_anterior: i32,
    fila_nova: i32,
}

impl Evento {
    pub fn new(tipo_evento: TipoEvento, tempo: f64, fila_anterior: i32, fila_nova: i32) -> Self {
        Evento {
            tipo_evento,
            tempo,
            fila_anterior,
            fila_nova,
        }
    }

    pub const fn tempo(&self) -> f64 {
        self.tempo
    }

    pub const fn tipo(&self) -> TipoEvento {
        self.tipo_evento
    }

    pub const fn fila_anterior(&self) -> i32 {
        self.fila_anterior
    }

    pub const fn fila_nova(&self) -> i32 {
        self.fila_nova
    }
}
