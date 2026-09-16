#[derive(Clone)]
pub struct Fila {
    id: usize,
    servidores: i32,
    capacidade: i32,
    min_chegada: f64,
    max_chegada: f64,
    min_atendimento: f64,
    max_atendimento: f64,
    clientes: i32,
    servidores_ocupados: i32,
    perdas: i32,
    tempos_estados: Vec<f64>,
    destinos: Vec<(i32, f64)>,
}

impl Fila {
    pub fn new(
        id: usize,
        servidores: i32,
        capacidade: i32,
        min_chegada: f64,
        max_chegada: f64,
        min_atendimento: f64,
        max_atendimento: f64,
        destinos: Vec<(i32, f64)>,
    ) -> Self {
        Fila {
            id,
            servidores,
            capacidade,
            min_chegada,
            max_chegada,
            min_atendimento,
            max_atendimento,
            clientes: 0,
            servidores_ocupados: 0,
            perdas: 0,
            tempos_estados: vec![0.0],
            destinos,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn servidores(&self) -> i32 {
        self.servidores
    }

    pub fn capacidade(&self) -> i32 {
        self.capacidade
    }

    pub fn perdas(&self) -> i32 {
        self.perdas
    }

    pub fn tempos_estados(&self) -> &[f64] {
        &self.tempos_estados
    }

    pub fn min_chegada(&self) -> f64 {
        self.min_chegada
    }

    pub fn max_chegada(&self) -> f64 {
        self.max_chegada
    }

    pub fn min_atendimento(&self) -> f64 {
        self.min_atendimento
    }

    pub fn max_atendimento(&self) -> f64 {
        self.max_atendimento
    }

    pub fn destinos(&self) -> &[(i32, f64)] {
        &self.destinos
    }

    pub fn pode_entrar(&self) -> bool {
        self.capacidade < 0 || self.clientes < self.capacidade
    }

    pub fn tem_servidor_disponivel(&self) -> bool {
        self.servidores_ocupados < self.servidores
    }

    pub fn tem_cliente_na_espera(&self) -> bool {
        self.clientes > self.servidores_ocupados
    }

    pub fn ocupar_servidor(&mut self) {
        self.servidores_ocupados += 1;
    }

    pub fn liberar_servidor(&mut self) {
        if self.servidores_ocupados > 0 {
            self.servidores_ocupados -= 1;
        }
    }

    pub fn entrada(&mut self) -> bool {
        if self.pode_entrar() {
            self.clientes += 1;
            true
        } else {
            self.perdas += 1;
            false
        }
    }

    pub fn saida(&mut self) {
        if self.clientes > 0 {
            self.clientes -= 1;
        }
    }

    pub fn acumular_tempo(&mut self, tempo_decorrido: f64) {
        if tempo_decorrido <= 0.0 {
            return;
        }
        let estado = self.clientes as usize;
        if estado >= self.tempos_estados.len() {
            self.tempos_estados.resize(estado + 1, 0.0);
        }
        self.tempos_estados[estado] += tempo_decorrido;
    }
}