pub enum TipoEvento {
    CHEGADA = 1,
    SAIDA = 2
}

pub struct Evento {
    tipo: TipoEvento,
    fim: f64
}

impl Evento {

    pub const fn new(tipo: TipoEvento, fim: f64) -> Self {
        Evento {
            tipo: tipo,
            fim: fim
        }
    }

    pub const fn get_fim(&self) -> f64 {
        self.fim
    }


}